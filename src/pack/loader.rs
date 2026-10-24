//! Pack loading from filesystem and embedded sources

use std::path::{Path, PathBuf};

use crate::components::ComponentCatalog;
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
        for (name, entry) in &manifest.templates {
            let content = read_listed_file(path, "templates", &entry.file)?;
            if !defines_function(&content, "report") {
                return Err(Error::Pack(PackError::InvalidStructure(format!(
                    "templates/{} does not define `#let report(meta, body)`",
                    entry.file
                ))));
            }
            pack.templates.insert(name.clone(), content);
        }

        // Load components. The manifest key is the component id; the file
        // defines the Typst function of that name, or for an override of a
        // built-in component, the built-in's function name.
        for (id, entry) in &manifest.components {
            let content = read_listed_file(path, "components", &entry.file)?;
            let fn_name = ComponentCatalog::get(id).map_or(id.as_str(), |d| d.fn_name);
            if !defines_function(&content, fn_name) {
                return Err(Error::Pack(PackError::InvalidStructure(format!(
                    "components/{} does not define `#let {fn_name}(data)`",
                    entry.file
                ))));
            }
            pack.components.insert(id.clone(), content);
        }

        // Load themes
        for (name, entry) in &manifest.themes {
            let content = read_listed_file(path, "themes", &entry.file)?;
            let theme: Theme = toml::from_str(&content)?;

            if entry.default {
                pack.default_theme = Some(theme.clone());
            }

            pack.themes.insert(name.clone(), theme);
        }

        for (name, entry) in &manifest.templates {
            if let Some(theme) = &entry.default_theme {
                if !pack.themes.contains_key(theme) {
                    return Err(Error::Pack(PackError::InvalidStructure(format!(
                        "template `{name}` names default theme `{theme}`, which pack.toml does not list"
                    ))));
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

/// Reads `<pack>/<dir>/<file>`, a file the manifest lists; a missing file is
/// an invalid pack rather than something to skip.
fn read_listed_file(pack_path: &Path, dir: &str, file: &str) -> crate::Result<String> {
    let file_path = pack_path.join(dir).join(file);
    if !file_path.is_file() {
        return Err(Error::Pack(PackError::InvalidStructure(format!(
            "pack.toml lists {dir}/{file}, which does not exist"
        ))));
    }
    Ok(std::fs::read_to_string(&file_path)?)
}

/// Whether Typst `source` defines a function `name` with `#let name(`.
pub(crate) fn defines_function(source: &str, name: &str) -> bool {
    source
        .lines()
        .any(|line| line.trim_start().starts_with(&format!("#let {name}(")))
}

impl Default for PackLoader {
    fn default() -> Self {
        Self::new(&[PathBuf::from("./packs")])
    }
}
