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

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
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

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
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
    /// Optional minimum height for each grid item
    #[serde(default)]
    pub item_min_height: Option<String>,
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
            item_min_height: None,
        }
    }

    pub fn with_item_min_height(mut self, min_height: impl Into<String>) -> Self {
        self.item_min_height = Some(min_height.into());
        self
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
        "grid-component"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Flow group with optional soft keep-together behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowGroup {
    /// Ordered child components or raw content blocks
    pub items: Vec<serde_json::Value>,
    /// Spacing between items
    #[serde(default)]
    pub spacing: Option<String>,
    /// Keep the group together if its measured height stays under this threshold
    #[serde(default)]
    pub keep_together_if_under: Option<String>,
}

impl Default for FlowGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl FlowGroup {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            spacing: None,
            keep_together_if_under: None,
        }
    }

    pub fn add_item(mut self, content: serde_json::Value) -> Self {
        self.items.push(content);
        self
    }

    pub fn with_spacing(mut self, spacing: impl Into<String>) -> Self {
        self.spacing = Some(spacing.into());
        self
    }

    pub fn with_keep_together_if_under(mut self, threshold: impl Into<String>) -> Self {
        self.keep_together_if_under = Some(threshold.into());
        self
    }
}

impl Component for FlowGroup {
    fn component_id(&self) -> &'static str {
        "flow-group"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Page break for multi-page reports
/// Inspired by BIRT Page Setup
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBreak;

impl Default for PageBreak {
    fn default() -> Self {
        Self::new()
    }
}

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

impl Default for KeyValueList {
    fn default() -> Self {
        Self::new()
    }
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

/// Table of Contents component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableOfContents {
    /// TOC heading title
    pub title: String,
    /// Maximum heading depth to include
    pub depth: u8,
    /// Font size for TOC entries (e.g. "9pt", "10pt")
    #[serde(default)]
    pub font_size: Option<String>,
}

impl TableOfContents {
    pub fn new() -> Self {
        Self {
            title: "Inhaltsverzeichnis".to_string(),
            depth: 3,
            font_size: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_depth(mut self, depth: u8) -> Self {
        self.depth = depth;
        self
    }

    pub fn with_font_size(mut self, size: impl Into<String>) -> Self {
        self.font_size = Some(size.into());
        self
    }
}

impl Default for TableOfContents {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for TableOfContents {
    fn component_id(&self) -> &'static str {
        "table-of-contents"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── SectionHeaderSplit ──────────────────────────────────────────────────────

fn default_level_two() -> u8 { 2 }
fn default_outlined() -> bool { true }
fn default_divider_below() -> bool { true }

/// Section header with 1/3 title and 2/3 body text side-by-side
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionHeaderSplit {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub eyebrow: Option<String>,
    #[serde(default = "default_level_two")]
    pub level: u8,
    #[serde(default = "default_outlined")]
    pub outlined: bool,
    #[serde(default = "default_divider_below")]
    pub divider_below: bool,
}

impl SectionHeaderSplit {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            eyebrow: None,
            level: 2,
            outlined: true,
            divider_below: true,
        }
    }

    pub fn with_eyebrow(mut self, eyebrow: impl Into<String>) -> Self {
        self.eyebrow = Some(eyebrow.into());
        self
    }

    pub fn with_level(mut self, level: u8) -> Self {
        self.level = level.clamp(1, 6);
        self
    }

    pub fn no_divider(mut self) -> Self {
        self.divider_below = false;
        self
    }
}

impl Component for SectionHeaderSplit {
    fn component_id(&self) -> &'static str {
        "section-header-split"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── PhaseBlock ──────────────────────────────────────────────────────────────

/// A phase block for roadmap/plan displays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseBlock {
    pub phase_number: u8,
    pub phase_label: String,
    pub description: String,
    pub items: Vec<String>,
    #[serde(default)]
    pub accent_color: Option<String>,
    #[serde(default)]
    pub item_count_total: Option<usize>,
}

impl PhaseBlock {
    pub fn new(
        number: u8,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            phase_number: number,
            phase_label: label.into(),
            description: description.into(),
            items: Vec::new(),
            accent_color: None,
            item_count_total: None,
        }
    }

    pub fn with_items(mut self, items: Vec<String>) -> Self {
        self.items = items;
        self
    }

    pub fn with_total(mut self, total: usize) -> Self {
        self.item_count_total = Some(total);
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.accent_color = Some(color.into());
        self
    }
}

impl Component for PhaseBlock {
    fn component_id(&self) -> &'static str {
        "phase-block"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── ChecklistPanel ──────────────────────────────────────────────────────────

/// A single row in a ChecklistPanel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistRow {
    pub label: String,
    pub diagnosis: String,
    #[serde(default)]
    pub status: Option<String>,
}

impl ChecklistRow {
    pub fn new(label: impl Into<String>, diagnosis: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            diagnosis: diagnosis.into(),
            status: None,
        }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

/// Card showing label–diagnosis pairs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistPanel {
    #[serde(default)]
    pub title: Option<String>,
    pub rows: Vec<ChecklistRow>,
}

impl ChecklistPanel {
    pub fn new(rows: Vec<ChecklistRow>) -> Self {
        Self { title: None, rows }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for ChecklistPanel {
    fn component_id(&self) -> &'static str {
        "checklist-panel"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── MetricStrip ─────────────────────────────────────────────────────────────

/// Item inside a MetricStrip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStripItem {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub accent_color: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

impl MetricStripItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            unit: None,
            accent_color: None,
            status: None,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn with_accent(mut self, color: impl Into<String>) -> Self {
        self.accent_color = Some(color.into());
        self
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

/// Horizontal row of equal KPI cells
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricStrip {
    pub items: Vec<MetricStripItem>,
    #[serde(default)]
    pub compact: bool,
}

impl MetricStrip {
    pub fn new(items: Vec<MetricStripItem>) -> Self {
        Self { items, compact: false }
    }

    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }
}

impl Component for MetricStrip {
    fn component_id(&self) -> &'static str {
        "metric-strip"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── ImpactGrid ──────────────────────────────────────────────────────────────

/// A single card inside an ImpactGrid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactGridCard {
    pub label: String,
    pub headline: String,
    pub body: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

impl ImpactGridCard {
    pub fn new(
        label: impl Into<String>,
        headline: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            label: label.into(),
            headline: headline.into(),
            body: body.into(),
            status: None,
            icon: None,
        }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Three-column impact card layout (user / risk / conversion)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactGrid {
    pub user: ImpactGridCard,
    pub risk: ImpactGridCard,
    pub conversion: ImpactGridCard,
    #[serde(default)]
    pub title: Option<String>,
}

impl ImpactGrid {
    pub fn new(user: ImpactGridCard, risk: ImpactGridCard, conversion: ImpactGridCard) -> Self {
        Self { user, risk, conversion, title: None }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for ImpactGrid {
    fn component_id(&self) -> &'static str {
        "impact-grid"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── SpotlightCard ───────────────────────────────────────────────────────────

/// General-purpose spotlight card with left severity stripe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotlightCard {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub variant: Option<String>,
    #[serde(default)]
    pub eyebrow: Option<String>,
    #[serde(default)]
    pub metric: Option<String>,
    #[serde(default)]
    pub detail: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
}

impl SpotlightCard {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            variant: None,
            eyebrow: None,
            metric: None,
            detail: None,
            action: None,
        }
    }

    pub fn with_variant(mut self, variant: impl Into<String>) -> Self {
        self.variant = Some(variant.into());
        self
    }

    pub fn with_eyebrow(mut self, eyebrow: impl Into<String>) -> Self {
        self.eyebrow = Some(eyebrow.into());
        self
    }

    pub fn with_metric(mut self, metric: impl Into<String>) -> Self {
        self.metric = Some(metric.into());
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
}

impl Component for SpotlightCard {
    fn component_id(&self) -> &'static str {
        "spotlight-card"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── ComparisonBlock ─────────────────────────────────────────────────────────

fn default_label_left() -> String {
    "Before".into()
}
fn default_label_right() -> String {
    "After".into()
}

/// Before/after code or text comparison block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonBlock {
    pub wrong: String,
    pub right: String,
    #[serde(default)]
    pub wrong_label: Option<String>,
    #[serde(default)]
    pub right_label: Option<String>,
    #[serde(default = "default_label_left")]
    pub label_left: String,
    #[serde(default = "default_label_right")]
    pub label_right: String,
    #[serde(default)]
    pub is_code: bool,
    #[serde(default)]
    pub note: Option<String>,
}

impl ComparisonBlock {
    pub fn new(wrong: impl Into<String>, right: impl Into<String>) -> Self {
        Self {
            wrong: wrong.into(),
            right: right.into(),
            wrong_label: None,
            right_label: None,
            label_left: "Before".into(),
            label_right: "After".into(),
            is_code: false,
            note: None,
        }
    }

    pub fn code(mut self) -> Self {
        self.is_code = true;
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn with_labels(
        mut self,
        wrong_label: impl Into<String>,
        right_label: impl Into<String>,
    ) -> Self {
        self.wrong_label = Some(wrong_label.into());
        self.right_label = Some(right_label.into());
        self
    }

    pub fn with_side_labels(
        mut self,
        label_left: impl Into<String>,
        label_right: impl Into<String>,
    ) -> Self {
        self.label_left = label_left.into();
        self.label_right = label_right.into();
        self
    }
}

impl Component for ComparisonBlock {
    fn component_id(&self) -> &'static str {
        "comparison-block"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── ComparisonCluster ───────────────────────────────────────────────────────

/// A single item in a ComparisonCluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonClusterItem {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub sub: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

impl ComparisonClusterItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            sub: None,
            status: None,
        }
    }

    pub fn with_sub(mut self, sub: impl Into<String>) -> Self {
        self.sub = Some(sub.into());
        self
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

fn default_three() -> usize { 3 }

/// Grid of comparison items with optional title
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonCluster {
    pub items: Vec<ComparisonClusterItem>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default = "default_three")]
    pub columns: usize,
}

impl ComparisonCluster {
    pub fn new(items: Vec<ComparisonClusterItem>) -> Self {
        Self { items, title: None, columns: 3 }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = columns;
        self
    }
}

impl Component for ComparisonCluster {
    fn component_id(&self) -> &'static str {
        "comparison-cluster"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── SideLabel ───────────────────────────────────────────────────────────────

/// Two-column layout: left 1/3 heading, right 2/3 content (bullets or text).
///
/// Ideal for structured information blocks where a label identifies a category
/// and associated details are listed alongside it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SideLabel {
    /// Section heading shown in the left column
    pub heading: String,
    /// Optional smaller subheading below the main heading
    #[serde(default)]
    pub subheading: Option<String>,
    /// Bullet items shown in the right column (preferred for lists)
    #[serde(default)]
    pub items: Vec<String>,
    /// Plain text shown in the right column when no items are provided
    #[serde(default)]
    pub text: Option<String>,
    /// Whether to render a horizontal divider below this block
    #[serde(default)]
    pub divider: bool,
}

impl SideLabel {
    pub fn new(heading: impl Into<String>) -> Self {
        Self {
            heading: heading.into(),
            subheading: None,
            items: Vec::new(),
            text: None,
            divider: false,
        }
    }

    pub fn with_subheading(mut self, subheading: impl Into<String>) -> Self {
        self.subheading = Some(subheading.into());
        self
    }

    pub fn add_item(mut self, item: impl Into<String>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_divider(mut self) -> Self {
        self.divider = true;
        self
    }
}

impl Component for SideLabel {
    fn component_id(&self) -> &'static str {
        "side-label"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Horizontal benefit strip (3–5 points)
/// For: Product features, value proposition, marketing intro
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitStrip {
    /// Benefit titles
    pub titles: Vec<String>,
    /// Benefit descriptions
    pub descriptions: Vec<String>,
    /// Icons (optional, one per item)
    #[serde(default)]
    pub icons: Vec<String>,
    /// Number of columns (2-5, defaults to 3)
    #[serde(default = "default_benefit_columns")]
    pub columns: usize,
}

fn default_benefit_columns() -> usize {
    3
}

impl BenefitStrip {
    pub fn new() -> Self {
        Self {
            titles: Vec::new(),
            descriptions: Vec::new(),
            icons: Vec::new(),
            columns: default_benefit_columns(),
        }
    }

    pub fn add_benefit(mut self, title: impl Into<String>, description: impl Into<String>) -> Self {
        self.titles.push(title.into());
        self.descriptions.push(description.into());
        self
    }

    pub fn add_benefit_with_icon(mut self, icon: impl Into<String>, title: impl Into<String>, description: impl Into<String>) -> Self {
        self.titles.push(title.into());
        self.descriptions.push(description.into());
        self.icons.push(icon.into());
        self
    }

    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(2).min(5);
        self
    }
}

impl Component for BenefitStrip {
    fn component_id(&self) -> &'static str {
        "benefit-strip"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Pricing / Plan card
/// For: Service packages, pricing tiers, audit levels, feature bundles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingCard {
    /// Plan name
    pub name: String,
    /// Price / cost (e.g. "$99" or "Free")
    pub price: String,
    /// Optional billing period (e.g. "/month")
    #[serde(default)]
    pub billing_period: Option<String>,
    /// Brief description
    #[serde(default)]
    pub description: Option<String>,
    /// Features included
    #[serde(default)]
    pub features: Vec<String>,
    /// CTA label (e.g. "Choose Plan")
    #[serde(default)]
    pub cta_label: Option<String>,
    /// Highlight this card as recommended
    #[serde(default)]
    pub highlighted: bool,
}

impl PricingCard {
    pub fn new(name: impl Into<String>, price: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            price: price.into(),
            billing_period: None,
            description: None,
            features: Vec::new(),
            cta_label: None,
            highlighted: false,
        }
    }

    pub fn with_billing_period(mut self, period: impl Into<String>) -> Self {
        self.billing_period = Some(period.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn add_feature(mut self, feature: impl Into<String>) -> Self {
        self.features.push(feature.into());
        self
    }

    pub fn with_cta(mut self, label: impl Into<String>) -> Self {
        self.cta_label = Some(label.into());
        self
    }

    pub fn highlighted(mut self) -> Self {
        self.highlighted = true;
        self
    }
}

impl Component for PricingCard {
    fn component_id(&self) -> &'static str {
        "pricing-card"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Recommendation card
/// Lighter than Finding: recommendation + impact + effort + priority
/// For: Audit recommendations, best practices, improvements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationCard {
    /// Recommendation title
    pub title: String,
    /// Brief description
    pub description: String,
    /// Impact level (high | medium | low)
    #[serde(default)]
    pub impact: Option<String>,
    /// Effort required (high | medium | low)
    #[serde(default)]
    pub effort: Option<String>,
    /// Priority (critical | high | medium | low)
    #[serde(default)]
    pub priority: Option<String>,
}

impl RecommendationCard {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            impact: None,
            effort: None,
            priority: None,
        }
    }

    pub fn with_impact(mut self, impact: impl Into<String>) -> Self {
        self.impact = Some(impact.into());
        self
    }

    pub fn with_effort(mut self, effort: impl Into<String>) -> Self {
        self.effort = Some(effort.into());
        self
    }

    pub fn with_priority(mut self, priority: impl Into<String>) -> Self {
        self.priority = Some(priority.into());
        self
    }
}

impl Component for RecommendationCard {
    fn component_id(&self) -> &'static str {
        "recommendation-card"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Step card row
/// Numbered steps displayed horizontally (2-4 steps)
/// For: "How it works", process overview, methodology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepCardRow {
    /// Step titles
    pub titles: Vec<String>,
    /// Step descriptions
    pub descriptions: Vec<String>,
    /// Number of columns (2-4, defaults to 3)
    #[serde(default = "default_step_columns")]
    pub columns: usize,
}

fn default_step_columns() -> usize {
    3
}

impl StepCardRow {
    pub fn new() -> Self {
        Self {
            titles: Vec::new(),
            descriptions: Vec::new(),
            columns: default_step_columns(),
        }
    }

    pub fn add_step(mut self, title: impl Into<String>, description: impl Into<String>) -> Self {
        self.titles.push(title.into());
        self.descriptions.push(description.into());
        self
    }

    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = columns.max(2).min(4);
        self
    }
}

impl Component for StepCardRow {
    fn component_id(&self) -> &'static str {
        "step-card-row"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Columns layout component
/// Asymmetric two-column layout with flexible width ratio (e.g. 60/40, 70/30)
/// For: Text + image, side-by-side content, mixed layouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Columns {
    /// Left column content (raw Typst or component references)
    pub left: String,
    /// Right column content (raw Typst or component references)
    pub right: String,
    /// Left column width ratio (0.0-1.0, defaults to 0.6 for 60/40)
    #[serde(default = "default_left_width")]
    pub left_width: f64,
    /// Gap between columns
    #[serde(default)]
    pub gap: Option<String>,
}

fn default_left_width() -> f64 {
    0.6
}

impl Columns {
    pub fn new(left: impl Into<String>, right: impl Into<String>) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
            left_width: default_left_width(),
            gap: None,
        }
    }

    pub fn with_ratio(mut self, left_width: f64) -> Self {
        self.left_width = left_width.max(0.2).min(0.8);
        self
    }

    pub fn with_gap(mut self, gap: impl Into<String>) -> Self {
        self.gap = Some(gap.into());
        self
    }
}

impl Component for Columns {
    fn component_id(&self) -> &'static str {
        "columns"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Device preview component for PDF cover pages.
/// Shows a desktop screenshot (3/4 width) and a mobile screenshot (1/4 width)
/// side by side with Apple-style device frames.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevicePreview {
    /// Asset key for the desktop screenshot (registered via builder.asset())
    pub desktop_src: String,
    /// Asset key for the mobile screenshot (registered via builder.asset())
    pub mobile_src: String,
    /// Height of the mockup block in points
    #[serde(default = "default_device_preview_height")]
    pub height_pt: f64,
    /// Desktop column ratio 0.0–1.0 (defaults to 0.75 for 3/4 layout)
    #[serde(default = "default_desktop_ratio")]
    pub desktop_ratio: f64,
}

fn default_device_preview_height() -> f64 {
    195.0
}

fn default_desktop_ratio() -> f64 {
    0.70
}

impl DevicePreview {
    pub fn new(desktop_src: impl Into<String>, mobile_src: impl Into<String>) -> Self {
        Self {
            desktop_src: desktop_src.into(),
            mobile_src: mobile_src.into(),
            height_pt: default_device_preview_height(),
            desktop_ratio: default_desktop_ratio(),
        }
    }

    pub fn with_height(mut self, height_pt: f64) -> Self {
        self.height_pt = height_pt;
        self
    }
}

impl Component for DevicePreview {
    fn component_id(&self) -> &'static str {
        "device-preview"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// FAQ list component
/// Question-answer pairs for FAQs, knowledge bases, product docs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqList {
    /// FAQ items
    pub items: Vec<FaqItem>,
    /// Optional title
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaqItem {
    /// Question
    pub question: String,
    /// Answer
    pub answer: String,
}

impl FaqList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            title: None,
        }
    }

    pub fn add_item(mut self, question: impl Into<String>, answer: impl Into<String>) -> Self {
        self.items.push(FaqItem {
            question: question.into(),
            answer: answer.into(),
        });
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for FaqList {
    fn component_id(&self) -> &'static str {
        "faq-list"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Use-case card component
/// Single use case: context + problem + solution
/// For: Product documentation, case studies, application examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseCaseCard {
    /// Use case title
    pub title: String,
    /// Context / industry / audience
    pub context: String,
    /// The problem
    pub problem: String,
    /// The solution / how we help
    pub solution: String,
    /// Optional outcome / result
    #[serde(default)]
    pub outcome: Option<String>,
}

impl UseCaseCard {
    pub fn new(
        title: impl Into<String>,
        context: impl Into<String>,
        problem: impl Into<String>,
        solution: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            context: context.into(),
            problem: problem.into(),
            solution: solution.into(),
            outcome: None,
        }
    }

    pub fn with_outcome(mut self, outcome: impl Into<String>) -> Self {
        self.outcome = Some(outcome.into());
        self
    }
}

impl Component for UseCaseCard {
    fn component_id(&self) -> &'static str {
        "use-case-card"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
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
        self.columns = columns.max(2).min(6);
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

/// Pull quote component
/// Large, visually prominent single quote (centered, full-width)
/// For: Key statements, expert opinions, marketing headlines
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullQuote {
    /// The quote text
    pub quote: String,
    /// Optional attribution (author, source)
    #[serde(default)]
    pub attribution: Option<String>,
}

impl PullQuote {
    pub fn new(quote: impl Into<String>) -> Self {
        Self {
            quote: quote.into(),
            attribution: None,
        }
    }

    pub fn with_attribution(mut self, attribution: impl Into<String>) -> Self {
        self.attribution = Some(attribution.into());
        self
    }
}

impl Component for PullQuote {
    fn component_id(&self) -> &'static str {
        "pull-quote"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Big number component
/// Large metric display for marketing/impact stats
/// For: Key metrics, conversion rates, impact statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BigNumber {
    /// The number/value (e.g. "+23%", "10x", "500ms")
    pub value: String,
    /// Description/label
    pub label: String,
    /// Optional context or explanation
    #[serde(default)]
    pub context: Option<String>,
}

impl BigNumber {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            context: None,
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

impl Component for BigNumber {
    fn component_id(&self) -> &'static str {
        "big-number"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Glossary list component
/// Term-definition pairs for reference, appendix, or terminology
/// For: Glossaries, abbreviations, technical terms, standards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryList {
    /// Glossary items
    pub items: Vec<GlossaryItem>,
    /// Optional title
    #[serde(default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlossaryItem {
    /// Term/abbreviation
    pub term: String,
    /// Definition
    pub definition: String,
}

impl GlossaryList {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            title: None,
        }
    }

    pub fn add_item(mut self, term: impl Into<String>, definition: impl Into<String>) -> Self {
        self.items.push(GlossaryItem {
            term: term.into(),
            definition: definition.into(),
        });
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for GlossaryList {
    fn component_id(&self) -> &'static str {
        "glossary-list"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Diagnosis Panel ─────────────────────────────────────────────────────────

/// A single row in a DiagnosisPanel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosisRow {
    pub label: String,
    pub diagnosis: String,
    #[serde(default)]
    pub status: Option<String>,
}

impl DiagnosisRow {
    pub fn new(label: impl Into<String>, diagnosis: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            diagnosis: diagnosis.into(),
            status: None,
        }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

/// Card with label–diagnosis rows and optional status indicators
/// For: Technical overview, module diagnosis, quick health checks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosisPanel {
    #[serde(default)]
    pub title: Option<String>,
    pub rows: Vec<DiagnosisRow>,
}

impl DiagnosisPanel {
    pub fn new(rows: Vec<DiagnosisRow>) -> Self {
        Self { title: None, rows }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for DiagnosisPanel {
    fn component_id(&self) -> &'static str {
        "diagnosis-panel"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Dominant Issue Spotlight ────────────────────────────────────────────────

/// Full-width spotlight for a single dominant issue
/// For: Highlighting the biggest problem in an audit report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DominantIssueSpotlight {
    pub title: String,
    pub severity: String,
    pub body: String,
    pub user_impact: String,
    pub recommendation: String,
    #[serde(default)]
    pub eyebrow: Option<String>,
    #[serde(default)]
    pub affected_count: Option<u32>,
}

impl DominantIssueSpotlight {
    pub fn new(
        title: impl Into<String>,
        severity: impl Into<String>,
        body: impl Into<String>,
        user_impact: impl Into<String>,
        recommendation: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            severity: severity.into(),
            body: body.into(),
            user_impact: user_impact.into(),
            recommendation: recommendation.into(),
            eyebrow: None,
            affected_count: None,
        }
    }

    pub fn with_eyebrow(mut self, eyebrow: impl Into<String>) -> Self {
        self.eyebrow = Some(eyebrow.into());
        self
    }

    pub fn with_affected_count(mut self, count: u32) -> Self {
        self.affected_count = Some(count);
        self
    }
}

impl Component for DominantIssueSpotlight {
    fn component_id(&self) -> &'static str {
        "dominant-issue-spotlight"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Wrong/Right Block ───────────────────────────────────────────────────────

/// Before/after comparison block (wrong vs. right)
/// For: Code examples, best practice comparisons, fix demonstrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrongRightBlock {
    pub wrong: String,
    pub right: String,
    #[serde(default)]
    pub wrong_label: Option<String>,
    #[serde(default)]
    pub right_label: Option<String>,
    #[serde(default)]
    pub is_code: bool,
    #[serde(default)]
    pub note: Option<String>,
}

impl WrongRightBlock {
    pub fn new(wrong: impl Into<String>, right: impl Into<String>) -> Self {
        Self {
            wrong: wrong.into(),
            right: right.into(),
            wrong_label: None,
            right_label: None,
            is_code: false,
            note: None,
        }
    }

    pub fn code(mut self) -> Self {
        self.is_code = true;
        self
    }

    pub fn with_labels(
        mut self,
        wrong_label: impl Into<String>,
        right_label: impl Into<String>,
    ) -> Self {
        self.wrong_label = Some(wrong_label.into());
        self.right_label = Some(right_label.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

impl Component for WrongRightBlock {
    fn component_id(&self) -> &'static str {
        "wrong-right-block"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
