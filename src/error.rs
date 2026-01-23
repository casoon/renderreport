//! Error types for RenderReport

use thiserror::Error;

/// Main error type for RenderReport
#[derive(Error, Debug)]
pub enum Error {
    #[error("Pack error: {0}")]
    Pack(#[from] PackError),

    #[error("Render error: {0}")]
    Render(#[from] RenderError),

    #[error("Theme error: {0}")]
    Theme(#[from] ThemeError),

    #[error("Component error: {0}")]
    Component(#[from] ComponentError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML parse error: {0}")]
    TomlParse(#[from] toml::de::Error),
}

/// Errors related to template packs
#[derive(Error, Debug)]
pub enum PackError {
    #[error("Pack not found: {0}")]
    NotFound(String),

    #[error("Invalid pack manifest: {0}")]
    InvalidManifest(String),

    #[error("Pack version mismatch: required {required}, found {found}")]
    VersionMismatch { required: String, found: String },

    #[error("Missing required component: {0}")]
    MissingComponent(String),

    #[error("Invalid pack structure: {0}")]
    InvalidStructure(String),
}

/// Errors during rendering
#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Typst compilation failed: {0}")]
    TypstCompilation(String),

    #[error("Missing required data field: {0}")]
    MissingData(String),

    #[error("Invalid template: {0}")]
    InvalidTemplate(String),

    #[error("Asset not found: {0}")]
    AssetNotFound(String),

    #[error("Font not found: {0}")]
    FontNotFound(String),
}

/// Errors related to theming
#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("Invalid token name: {0}")]
    InvalidTokenName(String),

    #[error("Invalid token value for {token}: {message}")]
    InvalidTokenValue { token: String, message: String },

    #[error("Theme not found: {0}")]
    NotFound(String),
}

/// Errors related to components
#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Unknown component type: {0}")]
    UnknownType(String),

    #[error("Invalid component data: {0}")]
    InvalidData(String),

    #[error("Component validation failed: {0}")]
    ValidationFailed(String),
}

/// Convenience Result type
pub type Result<T> = std::result::Result<T, Error>;
