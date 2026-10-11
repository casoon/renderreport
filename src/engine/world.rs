//! Typst World implementation for embedded compilation
//!
//! This module provides the necessary infrastructure for running Typst
//! as an embedded library without CLI. Fonts, the virtual filesystem and
//! dates come from [`embedded_typst`].

use std::path::{Path, PathBuf};
use std::sync::Arc;

use embedded_typst::{EmbeddedWorld, Fonts};
use typst::diag::FileResult;
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};
use typst::{Library, World};

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

/// Cached font data, built once at engine construction time. Fonts found on
/// disk are only read when a document uses them.
pub struct FontCache {
    fonts: Arc<Fonts>,
}

impl FontCache {
    /// Build font cache from engine configuration.
    pub fn new(config: &EngineConfig) -> Self {
        Self::with_extra_fonts(config, &[])
    }

    /// Build font cache from engine configuration, plus additional
    /// caller-supplied font byte buffers (e.g. `include_bytes!`-embedded
    /// fonts), loaded the same way as `EMBEDDED_FONTS`.
    ///
    /// For a caller that sets `use_system_fonts(false)` and wants a fully
    /// self-contained binary — no dependency on font files present on the
    /// machine that runs it — with its own theme fonts rather than the
    /// crate's built-in Fira Sans fallback.
    pub fn with_extra_fonts(config: &EngineConfig, extra_fonts: &[&[u8]]) -> Self {
        let mut fonts = Fonts::builder();

        // There is no filesystem and there are no system fonts on wasm32.
        #[cfg(not(target_arch = "wasm32"))]
        {
            if config.use_system_fonts {
                fonts = fonts.system_fonts();
            }
            for path in &config.font_paths {
                fonts = fonts.path(path);
            }
        }

        if config.use_embedded_fonts {
            for data in EMBEDDED_FONTS {
                fonts = fonts.data(*data);
            }
        }

        for data in extra_fonts {
            fonts = fonts.data(data.to_vec());
        }

        Self {
            fonts: Arc::new(fonts.build()),
        }
    }
}

/// Typst World implementation for RenderReport
pub struct ReportWorld(EmbeddedWorld);

impl ReportWorld {
    /// Create a new world using a pre-built font cache.
    pub fn new(source: String, font_cache: Arc<FontCache>) -> Self {
        let main = VirtualPath::new("report.typ").expect("\"report.typ\" is a valid virtual path");
        let world = EmbeddedWorld::new(main, source, font_cache.fonts.clone());
        let root = std::env::current_dir().unwrap_or_default();
        Self(world.with_roots(vec![root]))
    }

    /// Set the root path for file resolution
    pub fn with_root(self, root: impl Into<PathBuf>) -> Self {
        Self(self.0.with_roots(vec![root.into()]))
    }

    /// Add a virtual file
    pub fn add_file(&self, path: impl AsRef<Path>, content: impl Into<Vec<u8>>) {
        let path_str = path.as_ref().to_string_lossy();
        let Ok(vpath) = VirtualPath::new(path_str.as_ref()) else {
            return;
        };
        self.0.add_file(vpath, content.into());
    }
}

impl World for ReportWorld {
    fn library(&self) -> &typst::utils::LazyHash<Library> {
        self.0.library()
    }

    fn book(&self) -> &typst::utils::LazyHash<FontBook> {
        self.0.book()
    }

    fn main(&self) -> FileId {
        self.0.main()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.0.source(id)
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.0.file(id)
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.0.font(index)
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        self.0.today(offset)
    }
}
