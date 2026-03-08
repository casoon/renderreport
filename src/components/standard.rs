//! Standard component implementations

use super::{Component, ScoreStatus, Severity};
use serde::{Deserialize, Serialize};

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

    pub fn computed_status(&self) -> ScoreStatus {
        self.status.unwrap_or_else(|| {
            ScoreStatus::from_score(self.score, self.good_threshold, self.warn_threshold)
        })
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
                serde_json::to_value(self.computed_status()).unwrap(),
            );
        }
        data
    }
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
        self.content.push(serde_json::json!({
            "type": component.component_id(),
            "data": component.to_data()
        }));
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
}

impl Component for Image {
    fn component_id(&self) -> &'static str {
        "image"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Callout/admonition component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Callout {
    /// Callout content
    pub content: String,
    /// Callout type (info, warning, error, success, tip)
    #[serde(default = "default_callout_type")]
    pub callout_type: String,
    /// Optional title
    #[serde(default)]
    pub title: Option<String>,
}

fn default_callout_type() -> String {
    "info".into()
}

impl Callout {
    pub fn info(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            callout_type: "info".into(),
            title: None,
        }
    }

    pub fn warning(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            callout_type: "warning".into(),
            title: None,
        }
    }

    pub fn error(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            callout_type: "error".into(),
            title: None,
        }
    }

    pub fn success(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            callout_type: "success".into(),
            title: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for Callout {
    fn component_id(&self) -> &'static str {
        "callout"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
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

// ── New Composite Components ──────────────────────────────────────────

/// Compact metric card for KPI displays (flexible string values, unlike ScoreCard)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCard {
    pub title: String,
    pub value: String,
    #[serde(default)]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub accent_color: Option<String>,
}

impl MetricCard {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            subtitle: None,
            accent_color: None,
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
}

impl Component for MetricCard {
    fn component_id(&self) -> &'static str {
        "metric-card"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
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
            map.insert("computed_status".into(), serde_json::Value::String(status.into()));
        }
        data
    }
}

/// Module dashboard showing scores as a horizontal card strip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDashboard {
    #[serde(default)]
    pub title: Option<String>,
    pub modules: Vec<DashboardModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardModule {
    pub name: String,
    pub score: u32,
    pub interpretation: String,
    #[serde(default = "default_good_threshold")]
    pub good_threshold: u32,
    #[serde(default = "default_warn_threshold")]
    pub warn_threshold: u32,
}

impl ModuleDashboard {
    pub fn new(modules: Vec<DashboardModule>) -> Self {
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

impl Component for ModuleDashboard {
    fn component_id(&self) -> &'static str {
        "module-dashboard"
    }

    fn to_data(&self) -> serde_json::Value {
        let mut data = serde_json::to_value(self).unwrap_or_default();
        // Compute status for each module
        if let serde_json::Value::Object(ref mut map) = data {
            if let Some(serde_json::Value::Array(ref mut modules)) = map.get_mut("modules") {
                for module in modules.iter_mut() {
                    if let serde_json::Value::Object(ref mut m) = module {
                        let score = m.get("score").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                        let good = m.get("good_threshold").and_then(|v| v.as_u64()).unwrap_or(90) as u32;
                        let warn = m.get("warn_threshold").and_then(|v| v.as_u64()).unwrap_or(50) as u32;
                        let status = if score >= good { "good" } else if score >= warn { "warning" } else { "bad" };
                        m.insert("computed_status".into(), serde_json::Value::String(status.into()));
                    }
                }
            }
        }
        data
    }
}

/// Action roadmap with categorized columns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRoadmap {
    pub columns: Vec<RoadmapColumn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadmapColumn {
    pub title: String,
    #[serde(default)]
    pub accent_color: Option<String>,
    pub items: Vec<RoadmapItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoadmapItem {
    pub action: String,
    pub role: String,
    pub priority: String,
    #[serde(default)]
    pub effort: Option<String>,
    pub benefit: String,
}

impl ActionRoadmap {
    pub fn new(columns: Vec<RoadmapColumn>) -> Self {
        Self { columns }
    }
}

impl Component for ActionRoadmap {
    fn component_id(&self) -> &'static str {
        "action-roadmap"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Severity Overview ──────────────────────────────────────────────────────

/// Visual severity breakdown with cards and severity strip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeverityOverview {
    #[serde(default)]
    pub title: Option<String>,
    pub critical: u32,
    pub serious: u32,
    pub moderate: u32,
    pub minor: u32,
}

impl SeverityOverview {
    pub fn new(critical: u32, serious: u32, moderate: u32, minor: u32) -> Self {
        Self {
            title: Some("Problemübersicht".into()),
            critical,
            serious,
            moderate,
            minor,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.into());
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
    #[serde(default)]
    pub total_issues: u32,
    #[serde(default)]
    pub critical_issues: u32,
    #[serde(default)]
    pub modules: Vec<String>,
    /// Computed from score thresholds
    #[serde(default)]
    computed_status: String,
}

impl CoverPage {
    pub fn new(title: &str, domain: &str, score: u32, grade: &str) -> Self {
        let computed_status = if score >= 70 {
            "good"
        } else if score >= 50 {
            "warning"
        } else {
            "bad"
        }
        .to_string();

        Self {
            brand: "AuditMySite".into(),
            title: title.into(),
            domain: domain.into(),
            subtitle: "Automatisierte Analyse zu Accessibility, Performance, SEO, Sicherheit und Mobile.".into(),
            date: String::new(),
            score,
            grade: grade.into(),
            total_issues: 0,
            critical_issues: 0,
            modules: vec![],
            computed_status,
        }
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
            title: Some("Modul-Scores im Vergleich".into()),
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

// ─── Benchmark Summary ─────────────────────────────────────────────────────

/// Portfolio-level summary cards for batch reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub total_sites: u32,
    pub average_score: u32,
    pub best_score: u32,
    pub best_domain: String,
    pub worst_score: u32,
    pub worst_domain: String,
    pub total_issues: u32,
    pub critical_issues: u32,
}

impl BenchmarkSummary {
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

impl Component for BenchmarkSummary {
    fn component_id(&self) -> &'static str {
        "benchmark-summary"
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

// ─── Benchmark Table ────────────────────────────────────────────────────────

/// Ranking table for batch report website comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTable {
    #[serde(default)]
    pub title: Option<String>,
    pub rows: Vec<BenchmarkRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkRow {
    pub rank: u32,
    pub domain: String,
    pub score: u32,
    pub accessibility: u32,
    #[serde(default)]
    pub seo: Option<u32>,
    #[serde(default)]
    pub performance: Option<u32>,
    #[serde(default)]
    pub security: Option<u32>,
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
    pub fn new(rank: u32, domain: &str, score: u32, accessibility: u32, critical_issues: u32) -> Self {
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
            seo: None,
            performance: None,
            security: None,
            critical_issues,
            computed_status,
        }
    }

    pub fn with_seo(mut self, score: u32) -> Self {
        self.seo = Some(score);
        self
    }

    pub fn with_performance(mut self, score: u32) -> Self {
        self.performance = Some(score);
        self
    }

    pub fn with_security(mut self, score: u32) -> Self {
        self.security = Some(score);
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
