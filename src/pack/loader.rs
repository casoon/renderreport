//! Pack loading from filesystem and embedded sources

use std::path::{Path, PathBuf};

use crate::error::{Error, PackError};
use crate::pack::{Pack, PackManifest};
use crate::theme::Theme;

/// Loader for template packs
pub struct PackLoader {
    /// Search paths for packs
    search_paths: Vec<PathBuf>,
}

impl PackLoader {
    /// Create a new pack loader with given search paths
    pub fn new(search_paths: &[PathBuf]) -> Self {
        Self {
            search_paths: search_paths.to_vec(),
        }
    }

    /// Add a search path
    pub fn add_path(&mut self, path: impl Into<PathBuf>) {
        self.search_paths.push(path.into());
    }

    /// Load a pack by ID
    pub fn load(&self, pack_id: &str) -> crate::Result<Pack> {
        // Search for pack in paths
        for base_path in &self.search_paths {
            let pack_path = base_path.join(pack_id);
            if pack_path.exists() && pack_path.is_dir() {
                return self.load_from_path(&pack_path, pack_id);
            }
        }

        Err(Error::Pack(PackError::NotFound(pack_id.to_string())))
    }

    /// Load a pack from a specific path
    pub fn load_from_path(&self, path: &Path, pack_id: &str) -> crate::Result<Pack> {
        // Read manifest
        let manifest_path = path.join("pack.toml");
        if !manifest_path.exists() {
            return Err(Error::Pack(PackError::InvalidStructure(
                "Missing pack.toml manifest".into(),
            )));
        }

        let manifest_content = std::fs::read_to_string(&manifest_path)?;
        let manifest = PackManifest::from_toml(&manifest_content)?;

        let mut pack = Pack::new(pack_id, manifest.clone());
        pack.root_path = Some(path.to_path_buf());

        // Load templates
        let templates_dir = path.join("templates");
        if templates_dir.exists() {
            for entry in manifest.templates.values() {
                let template_path = templates_dir.join(&entry.file);
                if template_path.exists() {
                    let content = std::fs::read_to_string(&template_path)?;
                    let name = template_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    pack.templates.insert(name, content);
                }
            }
        }

        // Load components
        let components_dir = path.join("components");
        if components_dir.exists() {
            for (name, entry) in &manifest.components {
                let component_path = components_dir.join(&entry.file);
                if component_path.exists() {
                    let content = std::fs::read_to_string(&component_path)?;
                    pack.components.insert(name.clone(), content);
                }
            }
        }

        // Load themes
        let themes_dir = path.join("themes");
        if themes_dir.exists() {
            for (name, entry) in &manifest.themes {
                let theme_path = themes_dir.join(&entry.file);
                if theme_path.exists() {
                    let content = std::fs::read_to_string(&theme_path)?;
                    let theme: Theme = toml::from_str(&content)?;

                    if entry.default {
                        pack.default_theme = Some(theme.clone());
                    }

                    pack.themes.insert(name.clone(), theme);
                }
            }
        }

        // Load assets
        let assets_dir = path.join("assets");
        if assets_dir.exists() {
            Self::load_assets_recursive(&assets_dir, &assets_dir, &mut pack)?;
        }

        Ok(pack)
    }

    /// Recursively load assets from a directory
    fn load_assets_recursive(
        base_dir: &Path,
        current_dir: &Path,
        pack: &mut Pack,
    ) -> crate::Result<()> {
        for entry in std::fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                Self::load_assets_recursive(base_dir, &path, pack)?;
            } else if path.is_file() {
                let relative_path = path
                    .strip_prefix(base_dir)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .to_string();

                let bytes = std::fs::read(&path)?;
                pack.assets.insert(relative_path, bytes);
            }
        }

        Ok(())
    }

    /// List available packs in search paths
    pub fn list_available(&self) -> Vec<String> {
        let mut packs = Vec::new();

        for base_path in &self.search_paths {
            if let Ok(entries) = std::fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() && path.join("pack.toml").exists() {
                        if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                            packs.push(name.to_string());
                        }
                    }
                }
            }
        }

        packs
    }
}

impl Default for PackLoader {
    fn default() -> Self {
        Self::new(&[PathBuf::from("./packs")])
    }
}
