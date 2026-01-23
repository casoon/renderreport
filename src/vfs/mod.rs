//! Virtual filesystem for asset resolution
//!
//! Provides a unified interface for accessing files from different sources:
//! - Embedded resources
//! - Filesystem
//! - Memory (for dynamically generated content)
//! - Future: Remote sources

use std::collections::HashMap;
use std::path::PathBuf;

/// Virtual filesystem entry
#[derive(Debug, Clone)]
pub enum VfsEntry {
    /// In-memory content
    Memory(Vec<u8>),
    /// Filesystem path
    File(PathBuf),
    /// Embedded resource (static bytes)
    Embedded(&'static [u8]),
}

impl VfsEntry {
    /// Read the content as bytes
    pub fn read(&self) -> std::io::Result<Vec<u8>> {
        match self {
            VfsEntry::Memory(bytes) => Ok(bytes.clone()),
            VfsEntry::File(path) => std::fs::read(path),
            VfsEntry::Embedded(bytes) => Ok(bytes.to_vec()),
        }
    }

    /// Read the content as string
    pub fn read_to_string(&self) -> std::io::Result<String> {
        let bytes = self.read()?;
        String::from_utf8(bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

/// Virtual filesystem for managing report assets
#[derive(Debug, Default)]
pub struct Vfs {
    /// Mounted entries
    entries: HashMap<String, VfsEntry>,
    /// Base paths for filesystem resolution
    base_paths: Vec<PathBuf>,
}

impl Vfs {
    /// Create a new empty VFS
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a base path for filesystem resolution
    pub fn add_base_path(&mut self, path: impl Into<PathBuf>) {
        self.base_paths.push(path.into());
    }

    /// Mount in-memory content
    pub fn mount_memory(&mut self, path: impl Into<String>, content: Vec<u8>) {
        self.entries.insert(path.into(), VfsEntry::Memory(content));
    }

    /// Mount a filesystem path
    pub fn mount_file(&mut self, vpath: impl Into<String>, fpath: impl Into<PathBuf>) {
        self.entries
            .insert(vpath.into(), VfsEntry::File(fpath.into()));
    }

    /// Mount embedded content
    pub fn mount_embedded(&mut self, path: impl Into<String>, content: &'static [u8]) {
        self.entries
            .insert(path.into(), VfsEntry::Embedded(content));
    }

    /// Check if a path exists
    pub fn exists(&self, path: &str) -> bool {
        if self.entries.contains_key(path) {
            return true;
        }

        // Check base paths
        for base in &self.base_paths {
            if base.join(path).exists() {
                return true;
            }
        }

        false
    }

    /// Read file content
    pub fn read(&self, path: &str) -> std::io::Result<Vec<u8>> {
        // Check mounted entries first
        if let Some(entry) = self.entries.get(path) {
            return entry.read();
        }

        // Check base paths
        for base in &self.base_paths {
            let full_path = base.join(path);
            if full_path.exists() {
                return std::fs::read(&full_path);
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", path),
        ))
    }

    /// Read file as string
    pub fn read_to_string(&self, path: &str) -> std::io::Result<String> {
        let bytes = self.read(path)?;
        String::from_utf8(bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// List all mounted paths
    pub fn list_mounted(&self) -> Vec<&String> {
        self.entries.keys().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_mount() {
        let mut vfs = Vfs::new();
        vfs.mount_memory("test.txt", b"Hello, World!".to_vec());

        assert!(vfs.exists("test.txt"));
        assert_eq!(vfs.read_to_string("test.txt").unwrap(), "Hello, World!");
    }

    #[test]
    fn test_not_found() {
        let vfs = Vfs::new();
        assert!(!vfs.exists("nonexistent.txt"));
        assert!(vfs.read("nonexistent.txt").is_err());
    }
}
