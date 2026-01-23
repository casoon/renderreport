//! Advanced components inspired by JasperReports, Eclipse BIRT, and Pentaho Reporting

use super::Component;
use serde::{Deserialize, Serialize};

/// List component that iterates over data records
/// Inspired by JasperReports List Component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    /// List title
    #[serde(default)]
    pub title: Option<String>,
    /// Items to display
    pub items: Vec<ListItem>,
    /// Layout style (vertical, horizontal, grid)
    #[serde(default = "default_list_layout")]
    pub layout: String,
    /// Columns for grid layout
    #[serde(default = "default_columns")]
    pub columns: usize,
    /// Show item numbers/bullets
    #[serde(default)]
    pub numbered: bool,
}

fn default_list_layout() -> String {
    "vertical".into()
}
fn default_columns() -> usize {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    /// Item content
    pub content: String,
    /// Optional icon/indicator
    #[serde(default)]
    pub icon: Option<String>,
    /// Nested items
    #[serde(default)]
    pub children: Vec<ListItem>,
}

impl List {
    pub fn new() -> Self {
        Self {
            title: None,
            items: Vec::new(),
            layout: "vertical".into(),
            columns: 1,
            numbered: false,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn add_item(mut self, content: impl Into<String>) -> Self {
        self.items.push(ListItem {
            content: content.into(),
            icon: None,
            children: Vec::new(),
        });
        self
    }

    pub fn add_item_with_icon(
        mut self,
        content: impl Into<String>,
        icon: impl Into<String>,
    ) -> Self {
        self.items.push(ListItem {
            content: content.into(),
            icon: Some(icon.into()),
            children: Vec::new(),
        });
        self
    }

    pub fn grid_layout(mut self, columns: usize) -> Self {
        self.layout = "grid".into();
        self.columns = columns;
        self
    }

    pub fn numbered(mut self) -> Self {
        self.numbered = true;
        self
    }
}

impl Component for List {
    fn component_id(&self) -> &'static str {
        "list"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Horizontal separator/divider
/// Inspired by BIRT Band Elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divider {
    /// Divider style (solid, dashed, dotted, double)
    #[serde(default = "default_divider_style")]
    pub style: String,
    /// Thickness
    #[serde(default = "default_divider_thickness")]
    pub thickness: String,
    /// Color
    #[serde(default)]
    pub color: Option<String>,
    /// Spacing above
    #[serde(default = "default_spacing")]
    pub spacing_above: String,
    /// Spacing below
    #[serde(default = "default_spacing")]
    pub spacing_below: String,
}

fn default_divider_style() -> String {
    "solid".into()
}
fn default_divider_thickness() -> String {
    "0.5pt".into()
}
fn default_spacing() -> String {
    "12pt".into()
}

impl Divider {
    pub fn new() -> Self {
        Self {
            style: "solid".into(),
            thickness: "0.5pt".into(),
            color: None,
            spacing_above: "12pt".into(),
            spacing_below: "12pt".into(),
        }
    }

    pub fn dashed() -> Self {
        Self {
            style: "dashed".into(),
            ..Self::new()
        }
    }

    pub fn dotted() -> Self {
        Self {
            style: "dotted".into(),
            ..Self::new()
        }
    }

    pub fn thick() -> Self {
        Self {
            thickness: "2pt".into(),
            ..Self::new()
        }
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl Component for Divider {
    fn component_id(&self) -> &'static str {
        "divider"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Grid/Multi-column layout container
/// Inspired by Pentaho Block/Row Layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid {
    /// Grid title
    #[serde(default)]
    pub title: Option<String>,
    /// Number of columns
    pub columns: usize,
    /// Grid items (each item is a cell)
    pub items: Vec<GridItem>,
    /// Gap between columns
    #[serde(default = "default_grid_gap")]
    pub column_gap: String,
    /// Gap between rows
    #[serde(default = "default_grid_gap")]
    pub row_gap: String,
}

fn default_grid_gap() -> String {
    "16pt".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridItem {
    /// Item content (can be nested components)
    pub content: serde_json::Value,
    /// Column span (for merged cells)
    #[serde(default = "default_span")]
    pub colspan: usize,
}

fn default_span() -> usize {
    1
}

impl Grid {
    pub fn new(columns: usize) -> Self {
        Self {
            title: None,
            columns,
            items: Vec::new(),
            column_gap: "16pt".into(),
            row_gap: "16pt".into(),
        }
    }

    pub fn add_item(mut self, content: serde_json::Value) -> Self {
        self.items.push(GridItem {
            content,
            colspan: 1,
        });
        self
    }

    pub fn add_item_with_span(mut self, content: serde_json::Value, colspan: usize) -> Self {
        self.items.push(GridItem { content, colspan });
        self
    }
}

impl Component for Grid {
    fn component_id(&self) -> &'static str {
        "grid"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Page break for multi-page reports
/// Inspired by BIRT Page Setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBreak;

impl PageBreak {
    pub fn new() -> Self {
        Self
    }
}

impl Component for PageBreak {
    fn component_id(&self) -> &'static str {
        "page-break"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::json!({})
    }
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

/// Progress bar / completion indicator
/// Inspired by JasperReports Chart Components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressBar {
    /// Bar label
    pub label: String,
    /// Current value
    pub value: f64,
    /// Maximum value
    #[serde(default = "default_max")]
    pub max: f64,
    /// Show percentage
    #[serde(default = "default_true")]
    pub show_percentage: bool,
    /// Bar color
    #[serde(default)]
    pub color: Option<String>,
}

fn default_max() -> f64 {
    100.0
}
fn default_true() -> bool {
    true
}

impl ProgressBar {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            max: 100.0,
            show_percentage: true,
            color: None,
        }
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn percentage(&self) -> f64 {
        (self.value / self.max * 100.0).min(100.0)
    }
}

impl Component for ProgressBar {
    fn component_id(&self) -> &'static str {
        "progress-bar"
    }
    fn to_data(&self) -> serde_json::Value {
        let mut data = serde_json::to_value(self).unwrap_or_default();
        if let serde_json::Value::Object(ref mut map) = data {
            map.insert("percentage".into(), serde_json::json!(self.percentage()));
        }
        data
    }
}

/// Key-Value pairs display
/// Inspired by BIRT Parameter elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValueList {
    /// List title
    #[serde(default)]
    pub title: Option<String>,
    /// Key-value pairs
    pub items: Vec<KeyValuePair>,
    /// Layout (horizontal, vertical)
    #[serde(default = "default_kv_layout")]
    pub layout: String,
}

fn default_kv_layout() -> String {
    "vertical".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
    #[serde(default)]
    pub highlight: bool,
}

impl KeyValueList {
    pub fn new() -> Self {
        Self {
            title: None,
            items: Vec::new(),
            layout: "vertical".into(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn add(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.items.push(KeyValuePair {
            key: key.into(),
            value: value.into(),
            highlight: false,
        });
        self
    }

    pub fn add_highlighted(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.items.push(KeyValuePair {
            key: key.into(),
            value: value.into(),
            highlight: true,
        });
        self
    }
}

impl Component for KeyValueList {
    fn component_id(&self) -> &'static str {
        "key-value-list"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
