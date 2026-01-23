//! Template pack system
//!
//! Packs contain templates, components, themes, and assets.
//! They can be loaded from the filesystem, embedded, or (later) from remote sources.

mod loader;
mod manifest;

pub use loader::PackLoader;
pub use manifest::PackManifest;

use crate::theme::Theme;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Unique identifier for a pack
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackId(pub String);

impl PackId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

impl std::fmt::Display for PackId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for PackId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// A template pack containing templates, components, themes, and assets
#[derive(Debug, Clone)]
pub struct Pack {
    /// Pack identifier
    pub id: PackId,
    /// Pack manifest
    pub manifest: PackManifest,
    /// Root path (if loaded from filesystem)
    pub root_path: Option<PathBuf>,
    /// Default theme from pack
    pub default_theme: Option<Theme>,
    /// Available themes
    pub themes: HashMap<String, Theme>,
    /// Template files (name -> content)
    pub templates: HashMap<String, String>,
    /// Component overrides (name -> content)
    pub components: HashMap<String, String>,
    /// Asset files (name -> bytes)
    pub assets: HashMap<String, Vec<u8>>,
}

impl Pack {
    /// Create a new empty pack
    pub fn new(id: impl Into<String>, manifest: PackManifest) -> Self {
        Self {
            id: PackId::new(id),
            manifest,
            root_path: None,
            default_theme: None,
            themes: HashMap::new(),
            templates: HashMap::new(),
            components: HashMap::new(),
            assets: HashMap::new(),
        }
    }

    /// Get a template by name
    pub fn get_template(&self, name: &str) -> Option<&String> {
        self.templates.get(name)
    }

    /// Get a component by name
    pub fn get_component(&self, name: &str) -> Option<&String> {
        self.components.get(name)
    }

    /// Get a theme by name
    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }

    /// Get an asset by name
    pub fn get_asset(&self, name: &str) -> Option<&Vec<u8>> {
        self.assets.get(name)
    }

    /// List available templates
    pub fn list_templates(&self) -> Vec<&String> {
        self.templates.keys().collect()
    }

    /// List available themes
    pub fn list_themes(&self) -> Vec<&String> {
        self.themes.keys().collect()
    }
}
