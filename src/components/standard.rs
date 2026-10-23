//! Standard component implementations

use super::catalog::{ComponentCategory, LayoutHint};
use super::{Component, ScoreStatus, Severity};
use crate::component_catalog;
use serde::{Deserialize, Serialize};

/// Visual layout variant for a score card.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ScoreCardVariant {
    /// Standard bordered card with progress bar (default)
    #[default]
    Standard,
    /// Left accent strip with larger score number; requires an explicit height
    Tall,
    /// Colored background with white text
    Inverted,
    /// No progress bar, reduced padding
    Compact,
}

component_catalog! {
    id: "score-card",
    data: ScoreCard,
    template: include_str!("../../templates/components/score_card.typ"),
    description: "Score card with gauge-style visualization and status indicator",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

/// Score card component for displaying metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreCard {
    /// Card title/label
    pub title: String,
    /// Score value (0-100)
    pub score: u32,
    /// Maximum possible score (default: 100)
    #[serde(default = "default_max_score")]
    pub max_score: u32,
    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
    /// Score status (computed from thresholds if not set)
    #[serde(default)]
    pub status: Option<ScoreStatus>,
    /// Threshold for "good" status
    #[serde(default = "default_good_threshold")]
    pub good_threshold: u32,
    /// Threshold for "warning" status
    #[serde(default = "default_warn_threshold")]
    pub warn_threshold: u32,
    /// Optional card height (e.g. "100%", "120pt")
    #[serde(default)]
    pub height: Option<String>,
    /// Inverted: colored background with white text (deprecated — use `variant`)
    #[serde(default)]
    pub inverted: bool,
    /// Explicit visual variant; takes precedence over `inverted`/`height`
    #[serde(default)]
    pub variant: ScoreCardVariant,
}

fn default_max_score() -> u32 {
    100
}
fn default_good_threshold() -> u32 {
    90
}
fn default_warn_threshold() -> u32 {
    50
}

impl ScoreCard {
    pub fn new(title: impl Into<String>, score: u32) -> Self {
        Self {
            title: title.into(),
            score,
            max_score: 100,
            description: None,
            status: None,
            good_threshold: 90,
            warn_threshold: 50,
            height: None,
            inverted: false,
            variant: ScoreCardVariant::Standard,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn with_thresholds(mut self, good: u32, warn: u32) -> Self {
        self.good_threshold = good;
        self.warn_threshold = warn;
        self
    }

    pub fn with_height(mut self, height: impl Into<String>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Set an explicit visual variant.
    pub fn variant(mut self, variant: ScoreCardVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Deprecated: use `.variant(ScoreCardVariant::Inverted)` instead.
    pub fn inverted(mut self) -> Self {
        self.inverted = true;
        self
    }

    pub fn computed_status(&self) -> ScoreStatus {
        self.status.unwrap_or_else(|| {
            ScoreStatus::from_score(self.score, self.good_threshold, self.warn_threshold)
        })
    }

    /// Resolve the effective variant, taking legacy `inverted`/`height` fields into account.
    pub fn effective_variant(&self) -> ScoreCardVariant {
        match self.variant {
            ScoreCardVariant::Standard => {
                if self.inverted {
                    ScoreCardVariant::Inverted
                } else if self.height.is_some() {
                    ScoreCardVariant::Tall
                } else {
                    ScoreCardVariant::Standard
                }
            }
            v => v,
        }
    }
}

impl Component for ScoreCard {
    fn component_id(&self) -> &'static str {
        "score-card"
    }

    fn to_data(&self) -> serde_json::Value {
        let mut data = serde_json::to_value(self).unwrap_or_default();
        if let serde_json::Value::Object(ref mut map) = data {
            map.insert(
                "computed_status".into(),
                serde_json::to_value(self.computed_status()).unwrap_or(serde_json::Value::Null),
            );
            // Always expose the resolved variant so the template dispatches correctly
            map.insert(
                "variant".into(),
                serde_json::to_value(self.effective_variant()).unwrap_or_default(),
            );
        }
        data
    }
}

component_catalog! {
    id: "finding",
    data: Finding,
    template: include_str!("../../templates/components/finding.typ"),
    description: "Audit finding with severity indicator and recommendation block",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Finding component for audit issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Finding title
    pub title: String,
    /// Severity level
    pub severity: Severity,
    /// Detailed description
    pub description: String,
    /// Recommendation for fixing
    #[serde(default)]
    pub recommendation: Option<String>,
    /// Affected URL or resource
    #[serde(default)]
    pub affected: Option<String>,
    /// Category/tag
    #[serde(default)]
    pub category: Option<String>,
    /// Label for the "Affected:" line (default: "Affected:")
    #[serde(default)]
    pub label_affected: Option<String>,
    /// Label for the "Recommendation" section header (default: "Recommendation")
    #[serde(default)]
    pub label_recommendation: Option<String>,
}

impl Finding {
    pub fn new(
        title: impl Into<String>,
        severity: Severity,
        description: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            severity,
            description: description.into(),
            recommendation: None,
            affected: None,
            category: None,
            label_affected: None,
            label_recommendation: None,
        }
    }

    pub fn with_recommendation(mut self, rec: impl Into<String>) -> Self {
        self.recommendation = Some(rec.into());
        self
    }

    pub fn with_affected(mut self, affected: impl Into<String>) -> Self {
        self.affected = Some(affected.into());
        self
    }

    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
}

impl Component for Finding {
    fn component_id(&self) -> &'static str {
        "finding"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "audit-table",
    data: AuditTable,
    template: include_str!("../../templates/components/audit_table.typ"),
    description: "Tabular data for audit results with severity columns",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

/// Audit table for tabular data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTable {
    /// Table title
    #[serde(default)]
    pub title: Option<String>,
    /// Column headers
    pub columns: Vec<TableColumn>,
    /// Row data
    pub rows: Vec<Vec<serde_json::Value>>,
    /// Show row numbers
    #[serde(default)]
    pub show_row_numbers: bool,
    /// Striped rows
    #[serde(default = "default_true")]
    pub striped: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableColumn {
    /// Column header text
    pub header: String,
    /// Column width (optional, e.g., "20%", "auto")
    #[serde(default)]
    pub width: Option<String>,
    /// Text alignment
    #[serde(default)]
    pub align: Option<String>,
}

impl TableColumn {
    pub fn new(header: impl Into<String>) -> Self {
        Self {
            header: header.into(),
            width: None,
            align: None,
        }
    }

    pub fn with_width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn with_align(mut self, align: impl Into<String>) -> Self {
        self.align = Some(align.into());
        self
    }
}

impl AuditTable {
    pub fn new(columns: Vec<TableColumn>) -> Self {
        Self {
            title: None,
            columns,
            rows: Vec::new(),
            show_row_numbers: false,
            striped: true,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn add_row(mut self, row: Vec<impl Serialize>) -> Self {
        self.rows.push(
            row.into_iter()
                .map(|v| serde_json::to_value(v).unwrap_or_default())
                .collect(),
        );
        self
    }

    pub fn with_rows(mut self, rows: Vec<Vec<impl Serialize + Clone>>) -> Self {
        self.rows = rows
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|v| serde_json::to_value(v).unwrap_or_default())
                    .collect()
            })
            .collect();
        self
    }
}

impl Component for AuditTable {
    fn component_id(&self) -> &'static str {
        "audit-table"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

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

/// Visual style variant for a callout admonition.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum CalloutVariant {
    #[default]
    Info,
    Warning,
    Error,
    Success,
    Tip,
    Neutral,
}

impl CalloutVariant {
    fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Success => "success",
            Self::Tip => "tip",
            Self::Neutral => "neutral",
        }
    }
}

component_catalog! {
    id: "callout",
    data: Callout,
    template: include_str!("../../templates/components/callout.typ"),
    description: "Highlighted information box with icon and variant styling",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Callout/admonition component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Callout {
    /// Callout content
    pub content: String,
    /// Callout type string used by the Typst template (kept for backward compat)
    #[serde(default = "default_callout_type")]
    pub callout_type: String,
    /// Typed variant; takes precedence over `callout_type` when set
    #[serde(default)]
    pub variant: Option<CalloutVariant>,
    /// Optional title
    #[serde(default)]
    pub title: Option<String>,
    /// Inverted: filled background with white text
    #[serde(default)]
    pub inverted: bool,
}

fn default_callout_type() -> String {
    "info".into()
}

impl Callout {
    fn with_variant(content: impl Into<String>, variant: CalloutVariant) -> Self {
        Self {
            content: content.into(),
            callout_type: variant.as_str().to_string(),
            variant: Some(variant),
            title: None,
            inverted: false,
        }
    }

    pub fn info(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Info)
    }

    pub fn warning(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Warning)
    }

    pub fn error(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Error)
    }

    pub fn success(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Success)
    }

    pub fn tip(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Tip)
    }

    pub fn neutral(content: impl Into<String>) -> Self {
        Self::with_variant(content, CalloutVariant::Neutral)
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn inverted(mut self) -> Self {
        self.inverted = true;
        self
    }
}

impl Component for Callout {
    fn component_id(&self) -> &'static str {
        "callout"
    }

    fn to_data(&self) -> serde_json::Value {
        let mut data = serde_json::to_value(self).unwrap_or_default();
        // Ensure callout_type reflects the typed variant when both are set
        if let (Some(v), serde_json::Value::Object(ref mut map)) = (self.variant, &mut data) {
            map.insert(
                "callout_type".into(),
                serde_json::Value::String(v.as_str().to_string()),
            );
        }
        data
    }
}

component_catalog! {
    id: "summary-box",
    data: SummaryBox,
    template: include_str!("../../templates/components/summary_box.typ"),
    description: "Executive summary widget with key metrics",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

/// Summary box for executive summaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryBox {
    /// Summary title
    pub title: String,
    /// Key-value pairs for summary items
    pub items: Vec<SummaryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryItem {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub status: Option<ScoreStatus>,
}

impl SummaryBox {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
        }
    }

    pub fn add_item(mut self, label: impl Into<String>, value: impl Into<String>) -> Self {
        self.items.push(SummaryItem {
            label: label.into(),
            value: value.into(),
            status: None,
        });
        self
    }

    pub fn add_item_with_status(
        mut self,
        label: impl Into<String>,
        value: impl Into<String>,
        status: ScoreStatus,
    ) -> Self {
        self.items.push(SummaryItem {
            label: label.into(),
            value: value.into(),
            status: Some(status),
        });
        self
    }
}

impl Component for SummaryBox {
    fn component_id(&self) -> &'static str {
        "summary-box"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ── Primitive Components ──────────────────────────────────────────────

component_catalog! {
    id: "status-pill",
    data: StatusPill,
    template: include_str!("../../templates/components/status_pill.typ"),
    description: "Inline status badge with color coding",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Status pill badge — colored label for status display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusPill {
    pub label: String,
    #[serde(default = "default_neutral")]
    pub status: String,
    #[serde(default)]
    pub uppercase: bool,
}

fn default_neutral() -> String {
    "neutral".into()
}

impl StatusPill {
    pub fn good(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: "good".into(),
            uppercase: false,
        }
    }
    pub fn warn(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: "warn".into(),
            uppercase: false,
        }
    }
    pub fn bad(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: "bad".into(),
            uppercase: false,
        }
    }
    pub fn neutral(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: "neutral".into(),
            uppercase: false,
        }
    }
    pub fn info(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            status: "info".into(),
            uppercase: false,
        }
    }
}

impl Component for StatusPill {
    fn component_id(&self) -> &'static str {
        "status-pill"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "tag-cloud",
    data: TagCloud,
    template: include_str!("../../templates/components/tag_cloud.typ"),
    description: "Tag cloud with weighted font sizes",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Wrapping cloud of status-pill badges (flows inline, auto-wraps)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCloud {
    pub items: Vec<TagCloudItem>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default = "default_tag_gap")]
    pub gap: String,
}

fn default_tag_gap() -> String {
    "4pt".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagCloudItem {
    pub label: String,
    #[serde(default = "default_neutral")]
    pub status: String,
}

impl TagCloud {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            title: None,
            gap: "4pt".into(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_gap(mut self, gap: impl Into<String>) -> Self {
        self.gap = gap.into();
        self
    }

    pub fn add(mut self, label: impl Into<String>, status: impl Into<String>) -> Self {
        self.items.push(TagCloudItem {
            label: label.into(),
            status: status.into(),
        });
        self
    }
}

impl Default for TagCloud {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for TagCloud {
    fn component_id(&self) -> &'static str {
        "tag-cloud"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "stat",
    data: Stat,
    template: include_str!("../../templates/components/stat.typ"),
    description: "Single statistic display with value and label",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

/// Single KPI stat with optional trend indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stat {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub trend: Option<String>,
    #[serde(default)]
    pub trend_positive: Option<bool>,
    #[serde(default)]
    pub accent_color: Option<String>,
}

impl Stat {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            unit: None,
            trend: None,
            trend_positive: None,
            accent_color: None,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn with_trend(mut self, trend: impl Into<String>, positive: bool) -> Self {
        self.trend = Some(trend.into());
        self.trend_positive = Some(positive);
        self
    }

    pub fn with_accent(mut self, color: impl Into<String>) -> Self {
        self.accent_color = Some(color.into());
        self
    }
}

impl Component for Stat {
    fn component_id(&self) -> &'static str {
        "stat"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Entry in a StatPair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatPairEntry {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub accent_color: Option<String>,
}

impl StatPairEntry {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            unit: None,
            accent_color: None,
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
}

component_catalog! {
    id: "stat-pair",
    data: StatPair,
    template: include_str!("../../templates/components/stat_pair.typ"),
    description: "Side-by-side pair of stat displays",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

/// Two-stat side-by-side layout with optional vertical divider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatPair {
    pub left: StatPairEntry,
    pub right: StatPairEntry,
    #[serde(default = "default_true")]
    pub divider: bool,
}

impl StatPair {
    pub fn new(left: StatPairEntry, right: StatPairEntry) -> Self {
        Self {
            left,
            right,
            divider: true,
        }
    }
}

impl Component for StatPair {
    fn component_id(&self) -> &'static str {
        "stat-pair"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

fn default_seventy() -> u32 {
    70
}
fn default_fifty() -> u32 {
    50
}

component_catalog! {
    id: "score-band",
    data: ScoreBand,
    template: include_str!("../../templates/components/score_band.typ"),
    description: "Score band visualization with threshold markers",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

/// Horizontal score band with color-coded segments and marker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreBand {
    pub value: u32,
    #[serde(default = "default_seventy")]
    pub good_threshold: u32,
    #[serde(default = "default_fifty")]
    pub warn_threshold: u32,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default = "default_true")]
    pub show_value: bool,
}

impl ScoreBand {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            good_threshold: 70,
            warn_threshold: 50,
            label: None,
            show_value: true,
        }
    }

    pub fn with_thresholds(mut self, good: u32, warn: u32) -> Self {
        self.good_threshold = good;
        self.warn_threshold = warn;
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Component for ScoreBand {
    fn component_id(&self) -> &'static str {
        "score-band"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "trend-tile",
    data: TrendTile,
    template: include_str!("../../templates/components/trend_tile.typ"),
    description: "Trend tile with value, delta, and sparkline",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

/// Compact trend tile with direction arrow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendTile {
    pub label: String,
    pub delta: String,
    #[serde(default = "default_stable")]
    pub direction: String,
    #[serde(default)]
    pub reference: Option<String>,
    #[serde(default = "default_true")]
    pub positive_is_up: bool,
}

fn default_stable() -> String {
    "stable".into()
}

impl TrendTile {
    pub fn up(label: impl Into<String>, delta: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            delta: delta.into(),
            direction: "up".into(),
            reference: None,
            positive_is_up: true,
        }
    }
    pub fn down(label: impl Into<String>, delta: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            delta: delta.into(),
            direction: "down".into(),
            reference: None,
            positive_is_up: true,
        }
    }
    pub fn stable(label: impl Into<String>, delta: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            delta: delta.into(),
            direction: "stable".into(),
            reference: None,
            positive_is_up: true,
        }
    }
    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }
}

impl Component for TrendTile {
    fn component_id(&self) -> &'static str {
        "trend-tile"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ── New Composite Components ──────────────────────────────────────────

component_catalog! {
    id: "metric-card",
    data: MetricCard,
    template: include_str!("../../templates/components/metric_card.typ"),
    description: "Compact metric card with value, unit, and trend indicator",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

/// Compact metric card for KPI displays (flexible string values, unlike ScoreCard)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCard {
    pub title: String,
    pub value: String,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub accent_color: Option<String>,
    #[serde(default)]
    pub height: Option<String>,
}

impl MetricCard {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            subtitle: None,
            accent_color: None,
            height: None,
        }
    }

    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn with_accent_color(mut self, color: impl Into<String>) -> Self {
        self.accent_color = Some(color.into());
        self
    }

    pub fn with_height(mut self, height: impl Into<String>) -> Self {
        self.height = Some(height.into());
        self
    }
}

impl Component for MetricCard {
    fn component_id(&self) -> &'static str {
        "metric-card"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "hero-summary",
    data: HeroSummary,
    template: include_str!("../../templates/components/hero_summary.typ"),
    description: "Full-width hero summary with title, subtitle, and key metrics",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Hero summary for the first content page after TOC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroSummary {
    pub score: u32,
    pub grade: String,
    pub domain: String,
    pub date: String,
    pub verdict: String,
    pub metrics: Vec<HeroMetric>,
    #[serde(default)]
    pub top_actions: Vec<String>,
    #[serde(default)]
    pub positive_aspects: Vec<String>,
    #[serde(default = "default_hero_good")]
    pub good_threshold: u32,
    #[serde(default = "default_hero_warn")]
    pub warn_threshold: u32,
}

fn default_hero_good() -> u32 {
    70
}
fn default_hero_warn() -> u32 {
    50
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeroMetric {
    pub title: String,
    pub value: String,
    #[serde(default)]
    pub accent_color: Option<String>,
}

impl HeroSummary {
    pub fn new(score: u32, grade: impl Into<String>, domain: impl Into<String>) -> Self {
        Self {
            score,
            grade: grade.into(),
            domain: domain.into(),
            date: String::new(),
            verdict: String::new(),
            metrics: Vec::new(),
            top_actions: Vec::new(),
            positive_aspects: Vec::new(),
            good_threshold: 70,
            warn_threshold: 50,
        }
    }

    pub fn with_date(mut self, date: impl Into<String>) -> Self {
        self.date = date.into();
        self
    }

    pub fn with_verdict(mut self, verdict: impl Into<String>) -> Self {
        self.verdict = verdict.into();
        self
    }

    pub fn add_metric(mut self, metric: HeroMetric) -> Self {
        self.metrics.push(metric);
        self
    }

    pub fn with_top_actions(mut self, actions: Vec<String>) -> Self {
        self.top_actions = actions;
        self
    }

    pub fn with_positive_aspects(mut self, aspects: Vec<String>) -> Self {
        self.positive_aspects = aspects;
        self
    }

    pub fn with_thresholds(mut self, good: u32, warn: u32) -> Self {
        self.good_threshold = good;
        self.warn_threshold = warn;
        self
    }
}

impl Component for HeroSummary {
    fn component_id(&self) -> &'static str {
        "hero-summary"
    }

    fn to_data(&self) -> serde_json::Value {
        let mut data = serde_json::to_value(self).unwrap_or_default();
        if let serde_json::Value::Object(ref mut map) = data {
            let status = if self.score >= self.good_threshold {
                "good"
            } else if self.score >= self.warn_threshold {
                "warning"
            } else {
                "bad"
            };
            map.insert(
                "computed_status".into(),
                serde_json::Value::String(status.into()),
            );
        }
        data
    }
}

component_catalog! {
    id: "card-dashboard",
    data: CardDashboard,
    template: include_str!("../../templates/components/card_dashboard.typ"),
    description: "Multi-card dashboard layout with configurable columns",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::Breakable,
}

/// Card dashboard showing scores as a horizontal card strip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDashboard {
    #[serde(default)]
    pub title: Option<String>,
    pub modules: Vec<DashboardCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardCard {
    pub name: String,
    pub score: u32,
    pub interpretation: String,
    #[serde(default = "default_good_threshold")]
    pub good_threshold: u32,
    #[serde(default = "default_warn_threshold")]
    pub warn_threshold: u32,
}

impl CardDashboard {
    pub fn new(modules: Vec<DashboardCard>) -> Self {
        Self {
            title: None,
            modules,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for CardDashboard {
    fn component_id(&self) -> &'static str {
        "card-dashboard"
    }

    fn to_data(&self) -> serde_json::Value {
        let mut data = serde_json::to_value(self).unwrap_or_default();
        // Compute status for each module, reading scores from the typed source
        // (self.modules) rather than round-tripping through the serialized JSON.
        if let serde_json::Value::Object(ref mut map) = data {
            if let Some(serde_json::Value::Array(ref mut modules)) = map.get_mut("modules") {
                for (module, source) in modules.iter_mut().zip(&self.modules) {
                    if let serde_json::Value::Object(ref mut m) = module {
                        let status = if source.score >= source.good_threshold {
                            "good"
                        } else if source.score >= source.warn_threshold {
                            "warning"
                        } else {
                            "bad"
                        };
                        m.insert(
                            "computed_status".into(),
                            serde_json::Value::String(status.into()),
                        );
                    }
                }
            }
        }
        data
    }
}

component_catalog! {
    id: "roadmap-block",
    data: RoadmapBlock,
    template: include_str!("../../templates/components/roadmap_block.typ"),
    description: "Project roadmap block with phases and milestones",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::Breakable,
}

/// Roadmap block with categorized columns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadmapBlock {
    pub columns: Vec<RoadmapColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadmapColumn {
    pub title: String,
    #[serde(default)]
    pub accent_color: Option<String>,
    pub items: Vec<ActionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub action: String,
    pub role: String,
    pub priority: String,
    #[serde(default)]
    pub effort: Option<String>,
    pub benefit: String,
}

impl RoadmapBlock {
    pub fn new(columns: Vec<RoadmapColumn>) -> Self {
        Self { columns }
    }
}

impl Component for RoadmapBlock {
    fn component_id(&self) -> &'static str {
        "roadmap-block"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Severity Overview ──────────────────────────────────────────────────────

component_catalog! {
    id: "severity-overview",
    data: SeverityOverview,
    template: include_str!("../../templates/components/severity_overview.typ"),
    description: "Severity distribution overview with count badges",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

/// Visual severity breakdown with cards and severity strip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityOverview {
    #[serde(default)]
    pub title: Option<String>,
    pub critical: u32,
    pub serious: u32,
    pub moderate: u32,
    pub minor: u32,
    /// Card labels (critical/serious/moderate/minor) and the strip-bar total
    /// suffix. Each defaults to German in the template when not set — pass
    /// these explicitly for a non-German report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_critical: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_serious: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_moderate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_minor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_total_suffix: Option<String>,
}

impl SeverityOverview {
    pub fn new(critical: u32, serious: u32, moderate: u32, minor: u32) -> Self {
        Self {
            title: Some("Issue Overview".into()),
            critical,
            serious,
            moderate,
            minor,
            label_critical: None,
            label_serious: None,
            label_moderate: None,
            label_minor: None,
            label_total_suffix: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the card/strip-bar labels — required for a non-German report,
    /// since the template's own defaults are German.
    pub fn with_labels(
        mut self,
        label_critical: impl Into<String>,
        label_serious: impl Into<String>,
        label_moderate: impl Into<String>,
        label_minor: impl Into<String>,
        label_total_suffix: impl Into<String>,
    ) -> Self {
        self.label_critical = Some(label_critical.into());
        self.label_serious = Some(label_serious.into());
        self.label_moderate = Some(label_moderate.into());
        self.label_minor = Some(label_minor.into());
        self.label_total_suffix = Some(label_total_suffix.into());
        self
    }
}

impl Component for SeverityOverview {
    fn component_id(&self) -> &'static str {
        "severity-overview"
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

// ─── Module Comparison ──────────────────────────────────────────────────────

component_catalog! {
    id: "module-comparison",
    data: ModuleComparison,
    template: include_str!("../../templates/components/module_comparison.typ"),
    description: "Side-by-side module comparison with feature matrix",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

/// Horizontal score comparison rows for modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleComparison {
    #[serde(default)]
    pub title: Option<String>,
    pub modules: Vec<ComparisonModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonModule {
    pub name: String,
    pub score: u32,
    #[serde(default)]
    pub accent_color: Option<String>,
    /// Computed from thresholds
    #[serde(default)]
    computed_status: String,
}

impl ModuleComparison {
    pub fn new(modules: Vec<ComparisonModule>) -> Self {
        Self {
            title: Some("Module Comparison".into()),
            modules,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl ComparisonModule {
    pub fn new(name: &str, score: u32) -> Self {
        let computed_status = if score >= 75 {
            "good"
        } else if score >= 50 {
            "warning"
        } else {
            "bad"
        }
        .to_string();

        Self {
            name: name.into(),
            score,
            accent_color: None,
            computed_status,
        }
    }

    pub fn with_color(mut self, color: &str) -> Self {
        self.accent_color = Some(color.into());
        self
    }
}

impl Component for ModuleComparison {
    fn component_id(&self) -> &'static str {
        "module-comparison"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Portfolio Summary ──────────────────────────────────────────────────────

component_catalog! {
    id: "portfolio-summary",
    data: PortfolioSummary,
    template: include_str!("../../templates/components/portfolio_summary.typ"),
    description: "Portfolio summary with aggregate scores across items",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

/// Portfolio-level summary cards for batch reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioSummary {
    pub total_sites: u32,
    pub average_score: u32,
    pub best_score: u32,
    pub best_domain: String,
    pub worst_score: u32,
    pub worst_domain: String,
    pub total_issues: u32,
    pub critical_issues: u32,
}

impl PortfolioSummary {
    pub fn new(total_sites: u32, average_score: u32) -> Self {
        Self {
            total_sites,
            average_score,
            best_score: 0,
            best_domain: String::new(),
            worst_score: 0,
            worst_domain: String::new(),
            total_issues: 0,
            critical_issues: 0,
        }
    }

    pub fn with_best(mut self, domain: &str, score: u32) -> Self {
        self.best_domain = domain.into();
        self.best_score = score;
        self
    }

    pub fn with_worst(mut self, domain: &str, score: u32) -> Self {
        self.worst_domain = domain.into();
        self.worst_score = score;
        self
    }

    pub fn with_issues(mut self, total: u32, critical: u32) -> Self {
        self.total_issues = total;
        self.critical_issues = critical;
        self
    }
}

impl Component for PortfolioSummary {
    fn component_id(&self) -> &'static str {
        "portfolio-summary"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Benchmark Table ────────────────────────────────────────────────────────

component_catalog! {
    id: "benchmark-table",
    data: BenchmarkTable,
    template: include_str!("../../templates/components/benchmark_table.typ"),
    description: "Benchmark comparison table with scored rows",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

/// Ranking table for batch report website comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTable {
    #[serde(default)]
    pub title: Option<String>,
    pub rows: Vec<BenchmarkRow>,
}

/// One caller-defined extra ranking column (e.g. a scored audit dimension).
///
/// Deliberately generic — callers add whatever named dimensions their domain
/// has (`with_column("SEO", 91)`, `with_column("Security", 95)`, ...) instead
/// of `BenchmarkRow` hardcoding a fixed, closed set of field names. This
/// keeps `renderreport` domain-agnostic: a new caller-side dimension never
/// requires a new field/method/template branch here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkColumn {
    pub label: String,
    #[serde(default)]
    pub score: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRow {
    pub rank: u32,
    pub domain: String,
    pub score: u32,
    pub accessibility: u32,
    /// Additional scored dimensions, in the order added. The rendered table
    /// derives its extra column headers from the union of labels seen across
    /// all rows (first-seen order) — rows that omit a given row don't need
    /// to break column alignment for the rest of the table.
    #[serde(default)]
    pub extra_columns: Vec<BenchmarkColumn>,
    pub critical_issues: u32,
    /// Computed from score
    #[serde(default)]
    computed_status: String,
}

impl BenchmarkTable {
    pub fn new(rows: Vec<BenchmarkRow>) -> Self {
        Self {
            title: Some("Website-Ranking".into()),
            rows,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl BenchmarkRow {
    pub fn new(
        rank: u32,
        domain: &str,
        score: u32,
        accessibility: u32,
        critical_issues: u32,
    ) -> Self {
        let computed_status = if score >= 75 {
            "good"
        } else if score >= 50 {
            "warning"
        } else {
            "bad"
        }
        .to_string();

        Self {
            rank,
            domain: domain.into(),
            score,
            accessibility,
            extra_columns: Vec::new(),
            critical_issues,
            computed_status,
        }
    }

    /// Add a scored dimension column (e.g. `with_column("SEO", 91)`).
    pub fn with_column(mut self, label: impl Into<String>, score: u32) -> Self {
        self.extra_columns.push(BenchmarkColumn {
            label: label.into(),
            score: Some(score),
        });
        self
    }
}

impl Component for BenchmarkTable {
    fn component_id(&self) -> &'static str {
        "benchmark-table"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Phase 3: Marketing / Narrative Components ──────────────────────────────

/// Marketing feature/benefit grid
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureGridItem {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

component_catalog! {
    id: "feature-grid",
    data: FeatureGrid,
    template: include_str!("../../templates/components/feature_grid.typ"),
    description: "Grid of feature items with icon, title, and description",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::Breakable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureGrid {
    pub items: Vec<FeatureGridItem>,
    #[serde(default)]
    pub columns: Option<usize>,
    #[serde(default)]
    pub title: Option<String>,
}

impl FeatureGrid {
    pub fn new(items: Vec<FeatureGridItem>) -> Self {
        Self {
            items,
            columns: None,
            title: None,
        }
    }

    pub fn with_columns(mut self, columns: usize) -> Self {
        self.columns = Some(columns);
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for FeatureGrid {
    fn component_id(&self) -> &'static str {
        "feature-grid"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "cta-box",
    data: CTABox,
    template: include_str!("../../templates/components/cta_box.typ"),
    description: "Call-to-action box with headline, body, and button label",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Call to action block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CTABox {
    pub headline: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub action_label: Option<String>,
    #[serde(default)]
    pub action_url: Option<String>,
    #[serde(default)]
    pub tone: Option<String>,
}

impl CTABox {
    pub fn new(headline: impl Into<String>) -> Self {
        Self {
            headline: headline.into(),
            body: None,
            action_label: None,
            action_url: None,
            tone: None,
        }
    }

    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn with_action(mut self, label: impl Into<String>, url: impl Into<String>) -> Self {
        self.action_label = Some(label.into());
        self.action_url = Some(url.into());
        self
    }

    pub fn with_tone(mut self, tone: impl Into<String>) -> Self {
        self.tone = Some(tone.into());
        self
    }
}

impl Component for CTABox {
    fn component_id(&self) -> &'static str {
        "cta-box"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "testimonial",
    data: Testimonial,
    template: include_str!("../../templates/components/testimonial.typ"),
    description: "Customer testimonial quote with attribution",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Customer quote / testimonial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Testimonial {
    pub quote: String,
    pub author: String,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub company: Option<String>,
}

impl Testimonial {
    pub fn new(quote: impl Into<String>, author: impl Into<String>) -> Self {
        Self {
            quote: quote.into(),
            author: author.into(),
            role: None,
            company: None,
        }
    }

    pub fn with_role(mut self, role: impl Into<String>) -> Self {
        self.role = Some(role.into());
        self
    }

    pub fn with_company(mut self, company: impl Into<String>) -> Self {
        self.company = Some(company.into());
        self
    }
}

impl Component for Testimonial {
    fn component_id(&self) -> &'static str {
        "testimonial"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// A step in a process flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStep {
    pub label: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
}

component_catalog! {
    id: "process-flow",
    data: ProcessFlow,
    template: include_str!("../../templates/components/process_flow.typ"),
    description: "Horizontal or vertical process flow with numbered steps",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Linear process visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessFlow {
    pub steps: Vec<ProcessStep>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub direction: Option<String>,
}

impl ProcessFlow {
    pub fn new(steps: Vec<ProcessStep>) -> Self {
        Self {
            steps,
            title: None,
            direction: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn vertical(mut self) -> Self {
        self.direction = Some("vertical".into());
        self
    }
}

impl Component for ProcessFlow {
    fn component_id(&self) -> &'static str {
        "process-flow"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// A timeline milestone item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineItem {
    pub date: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
}

component_catalog! {
    id: "timeline",
    data: Timeline,
    template: include_str!("../../templates/components/timeline.typ"),
    description: "Project timeline with milestones and status indicators",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::Breakable,
}

/// Project phases / milestone timeline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub items: Vec<TimelineItem>,
    #[serde(default)]
    pub title: Option<String>,
}

impl Timeline {
    pub fn new(items: Vec<TimelineItem>) -> Self {
        Self { items, title: None }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for Timeline {
    fn component_id(&self) -> &'static str {
        "timeline"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// A step in a conversion funnel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelStep {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}

component_catalog! {
    id: "funnel",
    data: Funnel,
    template: include_str!("../../templates/components/funnel.typ"),
    description: "Conversion funnel visualization with stage counts",
    category: ComponentCategory::Infographics,
    layout_hint: LayoutHint::KeepTogether,
}

/// Conversion funnel visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Funnel {
    pub steps: Vec<FunnelStep>,
    #[serde(default)]
    pub title: Option<String>,
}

impl Funnel {
    pub fn new(steps: Vec<FunnelStep>) -> Self {
        Self { steps, title: None }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for Funnel {
    fn component_id(&self) -> &'static str {
        "funnel"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "problem-solution",
    data: ProblemSolution,
    template: include_str!("../../templates/components/problem_solution.typ"),
    description: "Two-column problem/solution comparison block",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Two-part problem/solution block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemSolution {
    pub problem: String,
    pub solution: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub problem_label: Option<String>,
    #[serde(default)]
    pub solution_label: Option<String>,
    #[serde(default)]
    pub impact: Option<String>,
    #[serde(default)]
    pub effort: Option<String>,
}

impl ProblemSolution {
    pub fn new(problem: impl Into<String>, solution: impl Into<String>) -> Self {
        Self {
            problem: problem.into(),
            solution: solution.into(),
            title: None,
            problem_label: None,
            solution_label: None,
            impact: None,
            effort: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_labels(mut self, problem: impl Into<String>, solution: impl Into<String>) -> Self {
        self.problem_label = Some(problem.into());
        self.solution_label = Some(solution.into());
        self
    }

    pub fn with_impact(mut self, impact: impl Into<String>) -> Self {
        self.impact = Some(impact.into());
        self
    }

    pub fn with_effort(mut self, effort: impl Into<String>) -> Self {
        self.effort = Some(effort.into());
        self
    }
}

impl Component for ProblemSolution {
    fn component_id(&self) -> &'static str {
        "problem-solution"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "before-after",
    data: BeforeAfter,
    template: include_str!("../../templates/components/before_after.typ"),
    description: "Before/after comparison with labeled panels",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Before/after comparison block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeforeAfter {
    pub before: String,
    pub after: String,
    #[serde(default)]
    pub label_before: Option<String>,
    #[serde(default)]
    pub label_after: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

impl BeforeAfter {
    pub fn new(before: impl Into<String>, after: impl Into<String>) -> Self {
        Self {
            before: before.into(),
            after: after.into(),
            label_before: None,
            label_after: None,
            title: None,
            note: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_labels(mut self, before: impl Into<String>, after: impl Into<String>) -> Self {
        self.label_before = Some(before.into());
        self.label_after = Some(after.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

impl Component for BeforeAfter {
    fn component_id(&self) -> &'static str {
        "before-after"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "why-it-matters",
    data: WhyItMatters,
    template: include_str!("../../templates/components/why_it_matters.typ"),
    description: "Highlighted explanation block for key insights",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

/// Context / relevance block for storytelling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhyItMatters {
    pub body: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub evidence: Option<String>,
    #[serde(default)]
    pub urgency: Option<String>,
}

impl WhyItMatters {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            title: None,
            evidence: None,
            urgency: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_evidence(mut self, evidence: impl Into<String>) -> Self {
        self.evidence = Some(evidence.into());
        self
    }

    pub fn with_urgency(mut self, urgency: impl Into<String>) -> Self {
        self.urgency = Some(urgency.into());
        self
    }
}

impl Component for WhyItMatters {
    fn component_id(&self) -> &'static str {
        "why-it-matters"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "fact-box",
    data: FactBox,
    template: include_str!("../../templates/components/fact_box.typ"),
    description: "Fact box with a prominent statistic and supporting text",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Inline info/fact box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactBox {
    pub body: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub variant: Option<String>,
}

impl FactBox {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            label: None,
            variant: None,
        }
    }

    pub fn info(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            label: None,
            variant: Some("info".into()),
        }
    }

    pub fn tip(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            label: None,
            variant: Some("tip".into()),
        }
    }

    pub fn warning(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            label: None,
            variant: Some("warning".into()),
        }
    }

    pub fn stat(body: impl Into<String>) -> Self {
        Self {
            body: body.into(),
            label: None,
            variant: Some("stat".into()),
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Component for FactBox {
    fn component_id(&self) -> &'static str {
        "fact-box"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "quote-block",
    data: QuoteBlock,
    template: include_str!("../../templates/components/quote_block.typ"),
    description: "Styled block quote with attribution",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

/// Quote / pull quote block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteBlock {
    pub quote: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub emphasis: Option<bool>,
}

impl QuoteBlock {
    pub fn new(quote: impl Into<String>) -> Self {
        Self {
            quote: quote.into(),
            author: None,
            emphasis: None,
        }
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn emphasized(mut self) -> Self {
        self.emphasis = Some(true);
        self
    }
}

impl Component for QuoteBlock {
    fn component_id(&self) -> &'static str {
        "quote-block"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

component_catalog! {
    id: "product-hero",
    data: ProductHero,
    template: include_str!("../../templates/components/product_hero.typ"),
    description: "Product hero block with image, headline, and description",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

/// Product/Project hero page — for introducing a product or project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductHero {
    pub title: String,
    pub subtitle: String,
    #[serde(default)]
    pub tagline: Option<String>,
    #[serde(default)]
    pub highlights: Vec<String>,
    #[serde(default)]
    pub cta_label: Option<String>,
    #[serde(default)]
    pub cta_url: Option<String>,
}

impl ProductHero {
    pub fn new(title: impl Into<String>, subtitle: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: subtitle.into(),
            tagline: None,
            highlights: vec![],
            cta_label: None,
            cta_url: None,
        }
    }

    pub fn with_tagline(mut self, tagline: impl Into<String>) -> Self {
        self.tagline = Some(tagline.into());
        self
    }

    pub fn add_highlight(mut self, highlight: impl Into<String>) -> Self {
        self.highlights.push(highlight.into());
        self
    }

    pub fn with_highlights(mut self, highlights: Vec<String>) -> Self {
        self.highlights = highlights;
        self
    }

    pub fn with_cta(mut self, label: impl Into<String>, url: impl Into<String>) -> Self {
        self.cta_label = Some(label.into());
        self.cta_url = Some(url.into());
        self
    }
}

impl Component for ProductHero {
    fn component_id(&self) -> &'static str {
        "product-hero"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// SpotlightCard is defined and exported from advanced.rs
