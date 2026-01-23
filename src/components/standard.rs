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
