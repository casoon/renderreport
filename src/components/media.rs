//! Media components: images, logos, device previews and watermarks.

use super::catalog::{ComponentCategory, LayoutHint};
use super::Component;
use crate::component_catalog;
use serde::{Deserialize, Serialize};

component_catalog! {
    id: "image",
    // Avoids a collision with Typst's built-in `image(...)`.
    fn_name: "report-image",
    data: Image,
    template: include_str!("../../templates/components/image.typ"),
    description: "Image with optional caption and alignment",
    category: ComponentCategory::MediaAssets,
    layout_hint: LayoutHint::KeepTogether,
}

/// Image component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// Image source (path or URL)
    pub src: String,
    /// Alt text
    #[serde(default)]
    pub alt: Option<String>,
    /// Caption
    #[serde(default)]
    pub caption: Option<String>,
    /// Width (e.g., "100%", "50%", "200pt")
    #[serde(default)]
    pub width: Option<String>,
}

impl Image {
    pub fn new(src: impl Into<String>) -> Self {
        Self {
            src: src.into(),
            alt: None,
            caption: None,
            width: None,
        }
    }

    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn with_width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Set an alternative description of the image for Assistive Technology
    /// (screen readers). Images without `alt` are rendered as decorative PDF
    /// artifacts (see `templates/components/image.typ`) — set this whenever
    /// the image conveys content rather than pure decoration.
    pub fn with_alt(mut self, alt: impl Into<String>) -> Self {
        self.alt = Some(alt.into());
        self
    }
}

impl Component for Image {
    fn component_id(&self) -> &'static str {
        "image"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "watermark",
    data: Watermark,
    template: include_str!("../../templates/components/watermark.typ"),
    description: "Page-level watermark overlay",
    category: ComponentCategory::MediaAssets,
    layout_hint: LayoutHint::KeepTogether,
}

/// Watermark for background content
/// Inspired by Pentaho Watermark Band
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watermark {
    /// Watermark text
    pub text: String,
    /// Rotation angle in degrees
    #[serde(default = "default_rotation")]
    pub rotation: f64,
    /// Opacity (0.0-1.0)
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    /// Font size
    #[serde(default = "default_watermark_size")]
    pub size: String,
}

fn default_rotation() -> f64 {
    -45.0
}

fn default_opacity() -> f64 {
    0.1
}

fn default_watermark_size() -> String {
    "48pt".into()
}

impl Watermark {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            rotation: -45.0,
            opacity: 0.1,
            size: "48pt".into(),
        }
    }

    pub fn confidential() -> Self {
        Self::new("CONFIDENTIAL")
    }

    pub fn draft() -> Self {
        Self::new("DRAFT")
    }

    pub fn with_rotation(mut self, degrees: f64) -> Self {
        self.rotation = degrees;
        self
    }

    pub fn with_opacity(mut self, opacity: f64) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

impl Component for Watermark {
    fn component_id(&self) -> &'static str {
        "watermark"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "logo-strip",
    data: LogoStrip,
    template: include_str!("../../templates/components/logo_strip.typ"),
    description: "Horizontal strip of client/partner logos",
    category: ComponentCategory::MediaAssets,
    layout_hint: LayoutHint::KeepTogether,
}

/// Logo strip component
/// Display logos of customers, partners, or certifications
/// For: Trust building, social proof, partner logos, client lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoStrip {
    /// Logo labels/names
    pub labels: Vec<String>,
    /// Number of columns (2-6, defaults to 4)
    #[serde(default = "default_logo_columns")]
    pub columns: usize,
    /// Optional title
    #[serde(default)]
    pub title: Option<String>,
}

fn default_logo_columns() -> usize {
    4
}

impl Default for LogoStrip {
    fn default() -> Self {
        Self::new()
    }
}

impl LogoStrip {
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            columns: default_logo_columns(),
            title: None,
        }
    }

    pub fn add_logo(mut self, label: impl Into<String>) -> Self {
        self.labels.push(label.into());
        self
    }

    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = columns.clamp(2, 6);
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for LogoStrip {
    fn component_id(&self) -> &'static str {
        "logo-strip"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
