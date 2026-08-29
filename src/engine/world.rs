//! Typst World implementation for embedded compilation
//!
//! This module provides the necessary infrastructure for running Typst
//! as an embedded library without CLI.

use std::cell::OnceCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock, RwLock};

use fontdb::Database;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::{Library, LibraryExt, World};

use crate::engine::EngineConfig;

/// Fonts bundled with the crate, used as a last-resort fallback when neither
/// system fonts nor explicit font paths provide a usable face (e.g. inside a
/// Wasm sandbox with no filesystem/system fonts). Fira Sans (OFL-licensed,
/// static weights) — see `assets/fonts/OFL.txt`.
const EMBEDDED_FONTS: &[&[u8]] = &[
    include_bytes!("../../assets/fonts/FiraSans-Regular.ttf"),
    include_bytes!("../../assets/fonts/FiraSans-Bold.ttf"),
    include_bytes!("../../assets/fonts/FiraSans-Italic.ttf"),
    include_bytes!("../../assets/fonts/FiraSans-BoldItalic.ttf"),
];

/// Cached font data, built once at engine construction time.
pub struct FontCache {
    pub book: typst::utils::LazyHash<FontBook>,
    pub fonts: Vec<Font>,
}

impl FontCache {
    /// Build font cache from engine configuration.
    pub fn new(config: &EngineConfig) -> Self {
        let mut fontdb = Database::new();

        if config.use_system_fonts {
            fontdb.load_system_fonts();
        }

        for path in &config.font_paths {
            if path.is_dir() {
                fontdb.load_fonts_dir(path);
            } else if path.is_file() {
                if let Err(e) = fontdb.load_font_file(path) {
                    eprintln!("warning: failed to load font file {}: {e}", path.display());
                }
            }
        }

        if config.use_embedded_fonts {
            for data in EMBEDDED_FONTS {
                fontdb.load_font_data(data.to_vec());
            }
        }

        let mut book = FontBook::new();
        let mut fonts = Vec::new();

        for face in fontdb.faces() {
            let source_data: Option<Vec<u8>> = match &face.source {
                fontdb::Source::File(path) => std::fs::read(path).ok(),
                fontdb::Source::Binary(data) => {
                    let slice: &[u8] = data.as_ref().as_ref();
                    Some(slice.to_vec())
                }
                fontdb::Source::SharedFile(_, data) => {
                    let slice: &[u8] = data.as_ref().as_ref();
                    Some(slice.to_vec())
                }
            };

            if let Some(data) = source_data {
                let buffer = Bytes::new(data);
                for font in Font::iter(buffer) {
                    book.push(font.info().clone());
                    fonts.push(font);
                }
            }
        }

        Self {
            book: typst::utils::LazyHash::new(book),
            fonts,
        }
    }
}

/// Typst World implementation for RenderReport
pub struct ReportWorld {
    /// The main source file
    main: Source,
    /// The standard library
    library: typst::utils::LazyHash<Library>,
    /// Shared font cache
    font_cache: Arc<FontCache>,
    /// File slots for virtual filesystem
    slots: RwLock<HashMap<FileId, FileSlot>>,
    /// Root path for file resolution
    root: PathBuf,
    /// Current datetime (unused on wasm32 — no OS clock to query)
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    now: OnceLock<Option<Datetime>>,
}

/// Slot for lazy file loading
struct FileSlot {
    source: OnceCell<FileResult<Source>>,
    bytes: OnceCell<FileResult<Bytes>>,
}

impl FileSlot {
    fn new() -> Self {
        Self {
            source: OnceCell::new(),
            bytes: OnceCell::new(),
        }
    }
}

// Safety: FileSlot's OnceCell only allows single initialization
unsafe impl Send for FileSlot {}
unsafe impl Sync for FileSlot {}

impl ReportWorld {
    /// Create a new world using a pre-built font cache.
    pub fn new(source: String, font_cache: Arc<FontCache>) -> Self {
        let path = VirtualPath::new("report.typ").expect("\"report.typ\" is a valid virtual path");
        let id = FileId::new(RootedPath::new(VirtualRoot::Project, path));
        let main = Source::new(id, source);
        let library = typst::utils::LazyHash::new(Library::default());

        Self {
            main,
            library,
            font_cache,
            slots: RwLock::new(HashMap::new()),
            root: std::env::current_dir().unwrap_or_default(),
            now: OnceLock::new(),
        }
    }

    /// Set the root path for file resolution
    pub fn with_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.root = root.into();
        self
    }

    /// Add a virtual file
    pub fn add_file(&self, path: impl AsRef<Path>, content: impl Into<Vec<u8>>) {
        let path_str = path.as_ref().to_string_lossy();
        let Ok(vpath) = VirtualPath::new(path_str.as_ref()) else {
            return;
        };
        let id = FileId::new(RootedPath::new(VirtualRoot::Project, vpath));
        let mut slots = self.slots.write().unwrap();
        let slot = slots.entry(id).or_insert_with(FileSlot::new);
        slot.bytes.get_or_init(|| Ok(Bytes::new(content.into())));
    }

    /// Resolve `id` from cache, falling back to `load` against its on-disk path
    /// and caching the result in the `FileSlot` field selected by `cell`.
    fn cached_or_load<T: Clone>(
        &self,
        id: FileId,
        cell: impl Fn(&FileSlot) -> &OnceCell<FileResult<T>>,
        load: impl FnOnce(&Path) -> FileResult<T>,
    ) -> FileResult<T> {
        let slots = self.slots.read().unwrap();
        if let Some(slot) = slots.get(&id) {
            if let Some(result) = cell(slot).get() {
                return result.clone();
            }
        }
        drop(slots);

        let path_str = id.vpath().get_with_slash();
        let path = Path::new(path_str);
        let full_path = self.root.join(path.strip_prefix("/").unwrap_or(path));
        let result = load(&full_path);

        let mut slots = self.slots.write().unwrap();
        let slot = slots.entry(id).or_insert_with(FileSlot::new);
        cell(slot).get_or_init(|| result.clone()).clone()
    }
}

impl World for ReportWorld {
    fn library(&self) -> &typst::utils::LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &typst::utils::LazyHash<FontBook> {
        &self.font_cache.book
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main.id() {
            return Ok(self.main.clone());
        }

        self.cached_or_load(
            id,
            |slot| &slot.source,
            |full_path| {
                std::fs::read_to_string(full_path)
                    .map_err(|e| FileError::from_io(e, full_path))
                    .map(|text| Source::new(id, text))
            },
        )
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.cached_or_load(
            id,
            |slot| &slot.bytes,
            |full_path| {
                std::fs::read(full_path)
                    .map_err(|e| FileError::from_io(e, full_path))
                    .map(Bytes::new)
            },
        )
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.font_cache.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        // `std::time::SystemTime::now()` (used internally by `chrono::Local`)
        // panics on wasm32-unknown-unknown — there is no OS clock to query.
        // None of our templates call `datetime.today()`, so skip it there.
        #[cfg(target_arch = "wasm32")]
        {
            None
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            *self.now.get_or_init(|| {
                let now = chrono::Local::now();
                Datetime::from_ymd_hms(
                    now.format("%Y").to_string().parse().ok()?,
                    now.format("%m").to_string().parse().ok()?,
                    now.format("%d").to_string().parse().ok()?,
                    now.format("%H").to_string().parse().ok()?,
                    now.format("%M").to_string().parse().ok()?,
                    now.format("%S").to_string().parse().ok()?,
                )
            })
        }
    }
}
