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
    #[serde(skip_serializing_if = "Option::is_none")]
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

// ─── DiagnosisPanel ──────────────────────────────────────────────────────────

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

/// Card showing label–diagnosis pairs
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

// ─── ImpactTriad ─────────────────────────────────────────────────────────────

/// A single card inside an ImpactTriad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactCard {
    pub label: String,
    pub headline: String,
    pub body: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

impl ImpactCard {
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
pub struct ImpactTriad {
    pub user: ImpactCard,
    pub risk: ImpactCard,
    pub conversion: ImpactCard,
    #[serde(default)]
    pub title: Option<String>,
}

impl ImpactTriad {
    pub fn new(user: ImpactCard, risk: ImpactCard, conversion: ImpactCard) -> Self {
        Self { user, risk, conversion, title: None }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for ImpactTriad {
    fn component_id(&self) -> &'static str {
        "impact-triad"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── DominantIssueSpotlight ──────────────────────────────────────────────────

/// Full-width spotlight for the most important finding
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
    ) -> Self {
        Self {
            title: title.into(),
            severity: severity.into(),
            body: body.into(),
            user_impact: String::new(),
            recommendation: String::new(),
            eyebrow: None,
            affected_count: None,
        }
    }

    pub fn with_impact(mut self, impact: impl Into<String>) -> Self {
        self.user_impact = impact.into();
        self
    }

    pub fn with_recommendation(mut self, rec: impl Into<String>) -> Self {
        self.recommendation = rec.into();
        self
    }

    pub fn with_count(mut self, count: u32) -> Self {
        self.affected_count = Some(count);
        self
    }

    pub fn with_eyebrow(mut self, eyebrow: impl Into<String>) -> Self {
        self.eyebrow = Some(eyebrow.into());
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

// ─── WrongRightBlock ─────────────────────────────────────────────────────────

/// Before/after code or text comparison block
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
}

impl Component for WrongRightBlock {
    fn component_id(&self) -> &'static str {
        "wrong-right-block"
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subheading: Option<String>,
    /// Bullet items shown in the right column (preferred for lists)
    #[serde(default)]
    pub items: Vec<String>,
    /// Plain text shown in the right column when no items are provided
    #[serde(skip_serializing_if = "Option::is_none")]
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
