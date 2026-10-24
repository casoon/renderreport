//! Engine configuration

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the rendering engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    /// Paths to search for template packs
    pub pack_paths: Vec<PathBuf>,
    /// Additional font directories
    pub font_paths: Vec<PathBuf>,
    /// Enable embedded fonts (bundled with library)
    pub use_embedded_fonts: bool,
    /// Enable system fonts
    pub use_system_fonts: bool,
    /// Cache directory for compiled templates
    pub cache_dir: Option<PathBuf>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            pack_paths: std::iter::once(PathBuf::from("./packs"))
                .chain(home_dir().map(|home| home.join(".renderreport/packs")))
                .collect(),
            font_paths: vec![],
            use_embedded_fonts: true,
            use_system_fonts: true,
            cache_dir: None,
        }
    }
}

impl EngineConfig {
    /// Create a new configuration builder
    pub fn builder() -> EngineConfigBuilder {
        EngineConfigBuilder::default()
    }

    /// Add a pack search path
    pub fn add_pack_path(&mut self, path: impl Into<PathBuf>) {
        self.pack_paths.push(path.into());
    }

    /// Add a font directory
    pub fn add_font_path(&mut self, path: impl Into<PathBuf>) {
        self.font_paths.push(path.into());
    }
}

/// Builder for engine configuration
#[derive(Default)]
pub struct EngineConfigBuilder {
    config: EngineConfig,
}

impl EngineConfigBuilder {
    pub fn pack_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.config.pack_paths.push(path.into());
        self
    }

    pub fn font_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.config.font_paths.push(path.into());
        self
    }

    pub fn use_embedded_fonts(mut self, value: bool) -> Self {
        self.config.use_embedded_fonts = value;
        self
    }

    pub fn use_system_fonts(mut self, value: bool) -> Self {
        self.config.use_system_fonts = value;
        self
    }

    pub fn cache_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.config.cache_dir = Some(path.into());
        self
    }

    pub fn build(self) -> EngineConfig {
        self.config
    }
}

/// The user's home directory (`HOME`, or `USERPROFILE` on Windows).
fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .filter(|home| !home.is_empty())
        .map(PathBuf::from)
}
