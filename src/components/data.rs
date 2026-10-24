//! Data and comparison components: tables, benchmarks and comparisons.

use super::catalog::{ComponentCategory, LayoutHint};
use super::Component;
use crate::component_catalog;
use serde::{Deserialize, Serialize};

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

component_catalog! {
    id: "key-value-list",
    data: KeyValueList,
    template: include_str!("../../templates/components/key_value_list.typ"),
    description: "Two-column key/value list for structured data",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
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

// ─── ComparisonBlock ─────────────────────────────────────────────────────────

fn default_label_left() -> String {
    "Before".into()
}

fn default_label_right() -> String {
    "After".into()
}

component_catalog! {
    id: "comparison-block",
    data: ComparisonBlock,
    template: include_str!("../../templates/components/comparison_block.typ"),
    description: "Side-by-side comparison with labels and scores",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::KeepTogether,
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

fn default_three() -> usize {
    3
}

component_catalog! {
    id: "comparison-cluster",
    data: ComparisonCluster,
    template: include_str!("../../templates/components/comparison_cluster.typ"),
    description: "Clustered comparison of multiple items across criteria",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

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
        Self {
            items,
            title: None,
            columns: 3,
        }
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
