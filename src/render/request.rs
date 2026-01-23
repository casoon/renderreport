//! Render request and output types

use crate::pack::PackId;
use crate::theme::Theme;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Request to render a report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderRequest {
    /// Template identifier
    pub template_id: String,
    /// Optional pack identifier
    #[serde(default)]
    pub pack_id: Option<PackId>,
    /// Report title
    #[serde(default)]
    pub title: Option<String>,
    /// Report subtitle
    #[serde(default)]
    pub subtitle: Option<String>,
    /// Theme overrides
    #[serde(default)]
    pub theme: Option<Theme>,
    /// Report components
    #[serde(default)]
    pub components: Vec<serde_json::Value>,
    /// Asset references (name -> path)
    #[serde(default)]
    pub assets: HashMap<String, PathBuf>,
    /// Additional metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl RenderRequest {
    /// Create a new render request
    pub fn new(template_id: impl Into<String>) -> Self {
        Self {
            template_id: template_id.into(),
            pack_id: None,
            title: None,
            subtitle: None,
            theme: None,
            components: Vec::new(),
            assets: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
}

/// Output from rendering
#[derive(Debug)]
pub enum RenderOutput {
    /// PDF bytes
    Pdf(Vec<u8>),
    /// Future: HTML output
    #[allow(dead_code)]
    Html(String),
}

impl RenderOutput {
    /// Convert to bytes (for PDF)
    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            RenderOutput::Pdf(bytes) => bytes,
            RenderOutput::Html(html) => html.into_bytes(),
        }
    }

    /// Check if output is PDF
    pub fn is_pdf(&self) -> bool {
        matches!(self, RenderOutput::Pdf(_))
    }
}
