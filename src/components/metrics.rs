//! Metric and KPI components: score cards, stats, progress and trends.

use super::catalog::{ComponentCategory, LayoutHint};
use super::{Component, ScoreStatus};
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

fn default_true() -> bool {
    true
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

component_catalog! {
    id: "progress-bar",
    data: ProgressBar,
    template: include_str!("../../templates/components/progress_bar.typ"),
    description: "Horizontal progress bar with percentage label",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
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

component_catalog! {
    id: "metric-strip",
    data: MetricStrip,
    template: include_str!("../../templates/components/metric_strip.typ"),
    description: "Horizontal strip of compact metric items",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
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
        Self {
            items,
            compact: false,
        }
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

component_catalog! {
    id: "big-number",
    data: BigNumber,
    template: include_str!("../../templates/components/big_number.typ"),
    description: "Oversized numeric display for key statistics",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
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
