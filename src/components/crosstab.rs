//! Crosstab (Pivot Table) components
//! Inspired by JasperReports Crosstab and BIRT Cross Tab

use super::Component;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Crosstab/Pivot Table component
/// Allows data aggregation in a tabular format with row/column grouping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crosstab {
    /// Crosstab title
    #[serde(default)]
    pub title: Option<String>,
    /// Row dimension
    pub row_dimension: String,
    /// Column dimension
    pub column_dimension: String,
    /// Measure field
    pub measure: String,
    /// Aggregation function (sum, avg, count, min, max)
    #[serde(default = "default_aggregation")]
    pub aggregation: String,
    /// Data rows
    pub data: Vec<HashMap<String, serde_json::Value>>,
    /// Show row totals
    #[serde(default = "default_true")]
    pub show_row_totals: bool,
    /// Show column totals
    #[serde(default = "default_true")]
    pub show_column_totals: bool,
    /// Show grand total
    #[serde(default = "default_true")]
    pub show_grand_total: bool,
}

fn default_aggregation() -> String {
    "sum".into()
}

fn default_true() -> bool {
    true
}

impl Crosstab {
    pub fn new(
        row_dimension: impl Into<String>,
        column_dimension: impl Into<String>,
        measure: impl Into<String>,
    ) -> Self {
        Self {
            title: None,
            row_dimension: row_dimension.into(),
            column_dimension: column_dimension.into(),
            measure: measure.into(),
            aggregation: "sum".into(),
            data: Vec::new(),
            show_row_totals: true,
            show_column_totals: true,
            show_grand_total: true,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_data(mut self, data: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.data = data;
        self
    }

    pub fn with_aggregation(mut self, aggregation: impl Into<String>) -> Self {
        self.aggregation = aggregation.into();
        self
    }

    pub fn hide_totals(mut self) -> Self {
        self.show_row_totals = false;
        self.show_column_totals = false;
        self.show_grand_total = false;
        self
    }
}

impl Component for Crosstab {
    fn component_id(&self) -> &'static str {
        "crosstab"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Simple pivot result for pre-aggregated data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PivotTable {
    /// Table title
    #[serde(default)]
    pub title: Option<String>,
    /// Row headers
    pub row_headers: Vec<String>,
    /// Column headers
    pub column_headers: Vec<String>,
    /// Cell values (row x column matrix)
    pub values: Vec<Vec<String>>,
    /// Show borders
    #[serde(default = "default_true")]
    pub show_borders: bool,
}

impl PivotTable {
    pub fn new(
        row_headers: Vec<String>,
        column_headers: Vec<String>,
        values: Vec<Vec<String>>,
    ) -> Self {
        Self {
            title: None,
            row_headers,
            column_headers,
            values,
            show_borders: true,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Component for PivotTable {
    fn component_id(&self) -> &'static str {
        "pivot-table"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
