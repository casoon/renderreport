//! Typst World implementation for embedded compilation
//!
//! This module provides the necessary infrastructure for running Typst
//! as an embedded library without CLI.

use std::cell::OnceCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{OnceLock, RwLock};

use fontdb::Database;
use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::{Library, World};

use crate::engine::EngineConfig;

/// Typst World implementation for RenderReport
pub struct ReportWorld {
    /// The main source file
    main: Source,
    /// The standard library
    library: typst::utils::LazyHash<Library>,
    /// Font book
    book: typst::utils::LazyHash<FontBook>,
    /// Loaded fonts
    fonts: Vec<Font>,
    /// File slots for virtual filesystem
    slots: RwLock<HashMap<FileId, FileSlot>>,
    /// Root path for file resolution
    root: PathBuf,
    /// Current datetime
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
    /// Create a new world with the given source
    pub fn new(source: String, config: &EngineConfig) -> Self {
        let main = Source::detached(source);
        let library = typst::utils::LazyHash::new(Library::default());

        // Initialize font database
        let mut fontdb = Database::new();

        if config.use_system_fonts {
            fontdb.load_system_fonts();
        }

        for path in &config.font_paths {
            if path.is_dir() {
                fontdb.load_fonts_dir(path);
            } else if path.is_file() {
                let _ = fontdb.load_font_file(path);
            }
        }

        // Convert fontdb to Typst fonts
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
            main,
            library,
            book: typst::utils::LazyHash::new(book),
            fonts,
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
        let id = FileId::new(None, VirtualPath::new(path.as_ref()));
        let mut slots = self.slots.write().unwrap();
        let slot = slots.entry(id).or_insert_with(FileSlot::new);
        let _ = slot.bytes.set(Ok(Bytes::new(content.into())));
    }
}

impl World for ReportWorld {
    fn library(&self) -> &typst::utils::LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &typst::utils::LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main.id() {
            return Ok(self.main.clone());
        }

        let slots = self.slots.read().unwrap();
        if let Some(slot) = slots.get(&id) {
            if let Some(result) = slot.source.get() {
                return result.clone();
            }
        }
        drop(slots);

        let path = id.vpath().as_rooted_path();
        let full_path = self.root.join(path.strip_prefix("/").unwrap_or(path));

        let result = std::fs::read_to_string(&full_path)
            .map_err(|e| FileError::from_io(e, &full_path))
            .map(|text| Source::new(id, text));

        let mut slots = self.slots.write().unwrap();
        let slot = slots.entry(id).or_insert_with(FileSlot::new);
        let _ = slot.source.set(result.clone());

        result
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        let slots = self.slots.read().unwrap();
        if let Some(slot) = slots.get(&id) {
            if let Some(result) = slot.bytes.get() {
                return result.clone();
            }
        }
        drop(slots);

        let path = id.vpath().as_rooted_path();
        let full_path = self.root.join(path.strip_prefix("/").unwrap_or(path));

        let result = std::fs::read(&full_path)
            .map_err(|e| FileError::from_io(e, &full_path))
            .map(Bytes::new);

        let mut slots = self.slots.write().unwrap();
        let slot = slots.entry(id).or_insert_with(FileSlot::new);
        let _ = slot.bytes.set(result.clone());

        result
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
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
