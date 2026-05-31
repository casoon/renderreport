//! Render request and output types

use crate::pack::PackId;
use crate::theme::Theme;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

fn default_true() -> bool {
    true
}

/// Page layout configuration for a render request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageSetup {
    /// Paper format: "a4" | "a3" | "letter" | "legal" (default: "a4")
    #[serde(default)]
    pub paper: Option<String>,
    /// Page orientation: "portrait" | "landscape" (default: "portrait")
    #[serde(default)]
    pub orientation: Option<String>,
    /// Margin override as a raw Typst expression, e.g. `"20pt"` or `"(top: 24pt)"`.
    /// When absent the theme's `page.margin*` tokens are used.
    #[serde(default)]
    pub margin: Option<String>,
    /// Show running header on content pages (default: true)
    #[serde(default = "default_true")]
    pub show_header: bool,
    /// Show running footer on content pages (default: true)
    #[serde(default = "default_true")]
    pub show_footer: bool,
    /// Custom header logo asset name
    #[serde(default)]
    pub header_logo: Option<String>,
}

impl Default for PageSetup {
    fn default() -> Self {
        Self {
            paper: None,
            orientation: None,
            margin: None,
            show_header: true,
            show_footer: true,
            header_logo: None,
        }
    }
}

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
    /// Page layout configuration
    #[serde(default)]
    pub page_setup: PageSetup,
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
            page_setup: PageSetup::default(),
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
