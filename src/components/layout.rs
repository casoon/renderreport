//! Layout components: page structure, sections, grids and spacing.

use super::catalog::{ComponentCategory, LayoutHint};
use super::Component;
use crate::component_catalog;
use serde::{Deserialize, Serialize};

component_catalog! {
    id: "section",
    data: Section,
    template: include_str!("../../templates/components/section.typ"),
    description: "Document section heading with optional subtitle",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::KeepWithNext,
}

/// Section component for document structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    /// Section title
    pub title: String,
    /// Heading level (1-6)
    #[serde(default = "default_level")]
    pub level: u8,
    /// Section content (nested components)
    #[serde(default)]
    pub content: Vec<serde_json::Value>,
}

fn default_level() -> u8 {
    1
}

impl Section {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            level: 1,
            content: Vec::new(),
        }
    }

    pub fn with_level(mut self, level: u8) -> Self {
        self.level = level.clamp(1, 6);
        self
    }

    pub fn add_component(mut self, component: impl Component) -> Self {
        self.content.push(component.as_item());
        self
    }
}

impl Component for Section {
    fn component_id(&self) -> &'static str {
        "section"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Cover Page ─────────────────────────────────────────────────────────────

/// One module's score for the cover gauge strip.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverModuleGauge {
    pub name: String,
    pub score: u32,
}

impl CoverModuleGauge {
    pub fn new(name: impl Into<String>, score: u32) -> Self {
        Self {
            name: name.into(),
            score,
        }
    }
}

component_catalog! {
    id: "cover-page",
    data: CoverPage,
    template: include_str!("../../templates/components/cover_page.typ"),
    description: "Full-page cover with title, logo, and metadata",
    category: ComponentCategory::Layout,
    // The cover is the first page and emits its own trailing page break, so it
    // must NOT request a leading break (that would produce a blank page 1).
    layout_hint: LayoutHint::KeepTogether,
}

/// Professional cover page with score preview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverPage {
    pub brand: String,
    pub title: String,
    pub domain: String,
    pub subtitle: String,
    pub date: String,
    pub score: u32,
    pub grade: String,
    /// Optional brand logo asset path (e.g. a registered SVG). When set, it is
    /// rendered in the top-left brand slot instead of the plain `brand` text —
    /// enabling white-label custom logos on the cover.
    #[serde(default)]
    pub logo_src: String,
    /// Plain-language condition phrase shown next to the score (replaces the
    /// A–F grade column when present).
    #[serde(default)]
    pub band_phrase: String,
    /// Localized section labels. Empty → template falls back to a German default.
    #[serde(default)]
    pub score_label: String,
    #[serde(default)]
    pub findings_label: String,
    #[serde(default)]
    pub modules_label: String,
    /// Suffix after the critical count, e.g. "kritisch" / "Critical".
    #[serde(default)]
    pub label_critical: String,
    /// Full "no critical" pill text, e.g. "0 kritisch" / "0 Critical".
    #[serde(default)]
    pub label_no_critical: String,
    #[serde(default)]
    pub total_issues: u32,
    #[serde(default)]
    pub critical_issues: u32,
    #[serde(default)]
    pub modules: Vec<String>,
    /// Per-module scores rendered as a circular-gauge strip on the cover.
    #[serde(default)]
    pub module_gauges: Vec<CoverModuleGauge>,
    /// Computed from score thresholds
    #[serde(default)]
    computed_status: String,
}

impl CoverPage {
    pub fn new(title: &str, domain: &str, score: u32, grade: &str) -> Self {
        let computed_status = if score >= 75 {
            "good"
        } else if score >= 40 {
            "warning"
        } else {
            "bad"
        }
        .to_string();

        Self {
            brand: String::new(),
            title: title.into(),
            domain: domain.into(),
            subtitle: String::new(),
            date: String::new(),
            score,
            grade: grade.into(),
            logo_src: String::new(),
            band_phrase: String::new(),
            score_label: String::new(),
            findings_label: String::new(),
            modules_label: String::new(),
            label_critical: String::new(),
            label_no_critical: String::new(),
            total_issues: 0,
            critical_issues: 0,
            modules: vec![],
            module_gauges: vec![],
            computed_status,
        }
    }

    pub fn with_logo(mut self, logo_src: impl Into<String>) -> Self {
        self.logo_src = logo_src.into();
        self
    }

    pub fn with_band_phrase(mut self, phrase: impl Into<String>) -> Self {
        self.band_phrase = phrase.into();
        self
    }

    pub fn with_labels(
        mut self,
        score_label: impl Into<String>,
        findings_label: impl Into<String>,
        modules_label: impl Into<String>,
        label_critical: impl Into<String>,
        label_no_critical: impl Into<String>,
    ) -> Self {
        self.score_label = score_label.into();
        self.findings_label = findings_label.into();
        self.modules_label = modules_label.into();
        self.label_critical = label_critical.into();
        self.label_no_critical = label_no_critical.into();
        self
    }

    pub fn with_module_gauges(mut self, gauges: Vec<CoverModuleGauge>) -> Self {
        self.module_gauges = gauges;
        self
    }

    pub fn with_brand(mut self, brand: &str) -> Self {
        self.brand = brand.into();
        self
    }

    pub fn with_subtitle(mut self, subtitle: &str) -> Self {
        self.subtitle = subtitle.into();
        self
    }

    pub fn with_date(mut self, date: &str) -> Self {
        self.date = date.into();
        self
    }

    pub fn with_modules(mut self, modules: Vec<String>) -> Self {
        self.modules = modules;
        self
    }

    pub fn with_issues(mut self, total: u32, critical: u32) -> Self {
        self.total_issues = total;
        self.critical_issues = critical;
        self
    }
}

impl Component for CoverPage {
    fn component_id(&self) -> &'static str {
        "cover-page"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "divider",
    data: Divider,
    template: include_str!("../../templates/components/divider.typ"),
    description: "Horizontal rule separator with optional label",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::KeepTogether,
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

component_catalog! {
    id: "grid-component",
    data: Grid,
    template: include_str!("../../templates/components/grid_body.typ"),
    description: "Multi-column grid layout dispatching to any component type",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::Breakable,
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

component_catalog! {
    id: "flow-group",
    data: FlowGroup,
    template: include_str!("../../templates/components/flow_group_body.typ"),
    description: "Soft keep-together wrapper for heading + first content block",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::KeepTogether,
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

component_catalog! {
    id: "page-break",
    data: PageBreak,
    template: include_str!("../../templates/components/page_break.typ"),
    description: "Explicit page break",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::AlwaysNewPage,
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

component_catalog! {
    id: "table-of-contents",
    data: TableOfContents,
    template: include_str!("../../templates/components/table_of_contents.typ"),
    description: "Auto-generated table of contents from section headings",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::Breakable,
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
            title: "Table of Contents".to_string(),
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

fn default_level_two() -> u8 {
    2
}

fn default_outlined() -> bool {
    true
}

fn default_divider_below() -> bool {
    true
}

component_catalog! {
    id: "section-header-split",
    data: SectionHeaderSplit,
    template: include_str!("../../templates/components/section_header_split.typ"),
    description: "Section header with title on left and metadata on right",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::KeepWithNext,
}

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

component_catalog! {
    id: "columns",
    data: Columns,
    template: include_str!("../../templates/components/columns.typ"),
    description: "Asymmetric two-column layout with flexible width ratio",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::Breakable,
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
        self.left_width = left_width.clamp(0.2, 0.8);
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
