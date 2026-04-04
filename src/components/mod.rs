//! Component system for building reports
//!
//! Components are the building blocks of reports. Each component has:
//! - A unique ID (e.g., "score-card", "finding", "audit-table")
//! - Data requirements (what data it expects)
//! - A Typst template that renders it
//!
//! ## Standard Components
//!
//! - `ScoreCard` - Display a metric with score visualization
//! - `Finding` - An audit finding with severity
//! - `AuditTable` - Tabular data for audit results
//! - `Section` - Document section with heading
//! - `Image` - Image with optional caption
//! - `Callout` - Highlighted information box
//! - `SummaryBox` - Executive summary widget

pub mod advanced;
pub mod barcode;
pub mod charts;
pub mod crosstab;
mod registry;
mod standard;
pub mod text;

pub use registry::{ComponentId, ComponentRegistry};
pub use standard::*;

// Re-export new primitive and composite types
pub use advanced::{
    ChecklistPanel, ChecklistRow, ComparisonBlock, ComparisonCluster, ComparisonClusterItem,
    ImpactGrid, ImpactGridCard, MetricStrip, MetricStripItem, PhaseBlock, SectionHeaderSplit,
    SpotlightCard,
};
pub use text::Eyebrow;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for all report components
pub trait Component: Send + Sync {
    /// Get the component type ID
    fn component_id(&self) -> &'static str;

    /// Serialize component data for template rendering
    fn to_data(&self) -> serde_json::Value;

    /// Validate the component data
    fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}

/// Dynamic component wrapper for heterogeneous collections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicComponent {
    /// Component type ID
    #[serde(rename = "type")]
    pub component_type: String,
    /// Component data as JSON
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}

impl DynamicComponent {
    pub fn new(component_type: impl Into<String>) -> Self {
        Self {
            component_type: component_type.into(),
            data: HashMap::new(),
        }
    }

    pub fn with_field(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.data.insert(
            key.into(),
            serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        );
        self
    }
}

impl Component for DynamicComponent {
    fn component_id(&self) -> &'static str {
        // This is a bit of a hack, but necessary for dynamic components
        Box::leak(self.component_type.clone().into_boxed_str())
    }

    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap_or(serde_json::Value::Null)
    }
}

/// Severity levels for findings and issues
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "critical",
            Severity::High => "high",
            Severity::Medium => "medium",
            Severity::Low => "low",
            Severity::Info => "info",
        }
    }

    pub fn color_token(&self) -> &'static str {
        match self {
            Severity::Critical | Severity::High => "color.bad",
            Severity::Medium => "color.warn",
            Severity::Low | Severity::Info => "color.ok",
        }
    }
}

/// Score status based on thresholds
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScoreStatus {
    Good,
    Warning,
    Bad,
}

impl ScoreStatus {
    pub fn from_score(score: u32, warn_threshold: u32, bad_threshold: u32) -> Self {
        if score >= warn_threshold {
            ScoreStatus::Good
        } else if score >= bad_threshold {
            ScoreStatus::Warning
        } else {
            ScoreStatus::Bad
        }
    }

    pub fn color_token(&self) -> &'static str {
        match self {
            ScoreStatus::Good => "color.ok",
            ScoreStatus::Warning => "color.warn",
            ScoreStatus::Bad => "color.bad",
        }
    }
}
