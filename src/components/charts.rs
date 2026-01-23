//! Chart components
//! Inspired by JasperReports Chart Components and BIRT Charts

use super::Component;
use serde::{Deserialize, Serialize};

/// Chart types supported
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChartType {
    Bar,
    Line,
    Pie,
    Area,
    Scatter,
    Radar,
}

/// Basic chart component
/// Inspired by JasperReports: Pie, Bar, Line, Area, Scatter charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    /// Chart title
    pub title: String,
    /// Chart type
    pub chart_type: ChartType,
    /// Data series
    pub series: Vec<ChartSeries>,
    /// X-axis label
    #[serde(default)]
    pub x_label: Option<String>,
    /// Y-axis label
    #[serde(default)]
    pub y_label: Option<String>,
    /// Show legend
    #[serde(default = "default_true")]
    pub show_legend: bool,
    /// Chart width
    #[serde(default = "default_chart_width")]
    pub width: String,
    /// Chart height
    #[serde(default = "default_chart_height")]
    pub height: String,
}

fn default_true() -> bool {
    true
}
fn default_chart_width() -> String {
    "100%".into()
}
fn default_chart_height() -> String {
    "200pt".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartSeries {
    pub name: String,
    pub data: Vec<ChartDataPoint>,
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartDataPoint {
    pub label: String,
    pub value: f64,
}

impl Chart {
    pub fn new(title: impl Into<String>, chart_type: ChartType) -> Self {
        Self {
            title: title.into(),
            chart_type,
            series: Vec::new(),
            x_label: None,
            y_label: None,
            show_legend: true,
            width: "100%".into(),
            height: "200pt".into(),
        }
    }

    pub fn bar(title: impl Into<String>) -> Self {
        Self::new(title, ChartType::Bar)
    }

    pub fn line(title: impl Into<String>) -> Self {
        Self::new(title, ChartType::Line)
    }

    pub fn pie(title: impl Into<String>) -> Self {
        Self::new(title, ChartType::Pie)
    }

    pub fn add_series(mut self, name: impl Into<String>, data: Vec<(String, f64)>) -> Self {
        self.series.push(ChartSeries {
            name: name.into(),
            data: data
                .into_iter()
                .map(|(label, value)| ChartDataPoint { label, value })
                .collect(),
            color: None,
        });
        self
    }

    pub fn with_labels(mut self, x: impl Into<String>, y: impl Into<String>) -> Self {
        self.x_label = Some(x.into());
        self.y_label = Some(y.into());
        self
    }
}

impl Component for Chart {
    fn component_id(&self) -> &'static str {
        "chart"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Sparkline - Small inline chart
/// Inspired by Pentaho Sparkline elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sparkline {
    /// Sparkline type (bar, line, pie)
    pub sparkline_type: String,
    /// Data points
    pub data: Vec<f64>,
    /// Width
    #[serde(default = "default_sparkline_width")]
    pub width: String,
    /// Height
    #[serde(default = "default_sparkline_height")]
    pub height: String,
    /// Color
    #[serde(default)]
    pub color: Option<String>,
}

fn default_sparkline_width() -> String {
    "80pt".into()
}
fn default_sparkline_height() -> String {
    "20pt".into()
}

impl Sparkline {
    pub fn new(data: Vec<f64>) -> Self {
        Self {
            sparkline_type: "line".into(),
            data,
            width: "80pt".into(),
            height: "20pt".into(),
            color: None,
        }
    }

    pub fn bar(data: Vec<f64>) -> Self {
        Self {
            sparkline_type: "bar".into(),
            ..Self::new(data)
        }
    }

    pub fn line(data: Vec<f64>) -> Self {
        Self {
            sparkline_type: "line".into(),
            ..Self::new(data)
        }
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl Component for Sparkline {
    fn component_id(&self) -> &'static str {
        "sparkline"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Gauge/Meter display
/// Inspired by JasperReports Meter/Thermometer charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gauge {
    /// Gauge label
    pub label: String,
    /// Current value
    pub value: f64,
    /// Minimum value
    #[serde(default)]
    pub min: f64,
    /// Maximum value
    #[serde(default = "default_gauge_max")]
    pub max: f64,
    /// Thresholds for color zones
    #[serde(default)]
    pub thresholds: Vec<GaugeThreshold>,
    /// Display style (circular, horizontal, vertical)
    #[serde(default = "default_gauge_style")]
    pub style: String,
}

fn default_gauge_max() -> f64 {
    100.0
}
fn default_gauge_style() -> String {
    "circular".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaugeThreshold {
    pub value: f64,
    pub color: String,
}

impl Gauge {
    pub fn new(label: impl Into<String>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
            min: 0.0,
            max: 100.0,
            thresholds: vec![
                GaugeThreshold {
                    value: 33.0,
                    color: "#ef4444".into(),
                },
                GaugeThreshold {
                    value: 66.0,
                    color: "#f59e0b".into(),
                },
                GaugeThreshold {
                    value: 100.0,
                    color: "#22c55e".into(),
                },
            ],
            style: "circular".into(),
        }
    }

    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn thermometer(label: impl Into<String>, value: f64) -> Self {
        Self {
            style: "vertical".into(),
            ..Self::new(label, value)
        }
    }
}

impl Component for Gauge {
    fn component_id(&self) -> &'static str {
        "gauge"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
