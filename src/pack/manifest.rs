//! Pack manifest definition
//!
//! The manifest file (pack.toml) describes a template pack.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pack manifest (pack.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackManifest {
    /// Pack metadata
    pub pack: PackMeta,
    /// Compatibility requirements
    #[serde(default)]
    pub compatibility: CompatibilityInfo,
    /// Available templates
    #[serde(default)]
    pub templates: HashMap<String, TemplateEntry>,
    /// Available themes
    #[serde(default)]
    pub themes: HashMap<String, ThemeEntry>,
    /// Component overrides
    #[serde(default)]
    pub components: HashMap<String, ComponentEntry>,
}

/// Pack metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackMeta {
    /// Pack name
    pub name: String,
    /// Pack version (semver)
    pub version: String,
    /// Human-readable description
    #[serde(default)]
    pub description: Option<String>,
    /// Pack authors
    #[serde(default)]
    pub authors: Vec<String>,
    /// Pack capabilities/features
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// License identifier (SPDX)
    #[serde(default)]
    pub license: Option<String>,
    /// Repository URL
    #[serde(default)]
    pub repository: Option<String>,
    /// Keywords for discovery
    #[serde(default)]
    pub keywords: Vec<String>,
}

/// Compatibility requirements
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    /// Minimum renderreport engine version
    #[serde(default)]
    pub min_engine_version: Option<String>,
    /// Maximum renderreport engine version
    #[serde(default)]
    pub max_engine_version: Option<String>,
    /// Required Typst version (if using Typst-specific features)
    #[serde(default)]
    pub typst_version: Option<String>,
}

/// Template entry in manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateEntry {
    /// Template file path (relative to pack root)
    pub file: String,
    /// Human-readable description
    #[serde(default)]
    pub description: Option<String>,
    /// Default theme for this template
    #[serde(default)]
    pub default_theme: Option<String>,
    /// Required data schema (JSON Schema reference)
    #[serde(default)]
    pub schema: Option<String>,
}

/// Theme entry in manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeEntry {
    /// Theme file path (relative to pack root)
    pub file: String,
    /// Human-readable description
    #[serde(default)]
    pub description: Option<String>,
    /// Whether this is the default theme
    #[serde(default)]
    pub default: bool,
}

/// Component entry in manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentEntry {
    /// Component file path (relative to pack root)
    pub file: String,
    /// Human-readable description
    #[serde(default)]
    pub description: Option<String>,
    /// Whether this overrides a standard component
    #[serde(default)]
    pub overrides: Option<String>,
}

impl PackManifest {
    /// Parse manifest from TOML string
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// Serialize manifest to TOML string
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Get the default theme name
    pub fn default_theme_name(&self) -> Option<&str> {
        self.themes
            .iter()
            .find(|(_, entry)| entry.default)
            .map(|(name, _)| name.as_str())
    }
}

impl Default for PackManifest {
    fn default() -> Self {
        Self {
            pack: PackMeta {
                name: "unnamed".into(),
                version: "0.1.0".into(),
                description: None,
                authors: vec![],
                capabilities: vec![],
                license: None,
                repository: None,
                keywords: vec![],
            },
            compatibility: CompatibilityInfo::default(),
            templates: HashMap::new(),
            themes: HashMap::new(),
            components: HashMap::new(),
        }
    }
}
