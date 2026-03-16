//! Text and field components
//! Inspired by BIRT Label/Text/Data and Pentaho Text/Number/Date Fields

use super::Component;
use serde::{Deserialize, Serialize};

/// Simple text label
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    /// Label text
    pub text: String,
    /// Font size
    #[serde(default)]
    pub font_size: Option<String>,
    /// Font weight (normal, bold)
    #[serde(default)]
    pub font_weight: Option<String>,
    /// Text color
    #[serde(default)]
    pub color: Option<String>,
    /// Text alignment (left, center, right)
    #[serde(default)]
    pub align: Option<String>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: None,
            font_weight: None,
            color: None,
            align: None,
        }
    }

    pub fn with_size(mut self, size: impl Into<String>) -> Self {
        self.font_size = Some(size.into());
        self
    }

    pub fn bold(mut self) -> Self {
        self.font_weight = Some("bold".into());
        self
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn center(mut self) -> Self {
        self.align = Some("center".into());
        self
    }
}

impl Component for Label {
    fn component_id(&self) -> &'static str {
        "label"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Multi-line text block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextBlock {
    /// Text content
    pub content: String,
    /// Font size
    #[serde(default)]
    pub font_size: Option<String>,
    /// Line height
    #[serde(default)]
    pub line_height: Option<String>,
    /// Text alignment
    #[serde(default)]
    pub align: Option<String>,
    /// Maximum width
    #[serde(default)]
    pub max_width: Option<String>,
}

impl TextBlock {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            font_size: None,
            line_height: None,
            align: None,
            max_width: None,
        }
    }

    pub fn with_size(mut self, size: impl Into<String>) -> Self {
        self.font_size = Some(size.into());
        self
    }

    pub fn with_line_height(mut self, height: impl Into<String>) -> Self {
        self.line_height = Some(height.into());
        self
    }

    pub fn with_max_width(mut self, width: impl Into<String>) -> Self {
        self.max_width = Some(width.into());
        self
    }
}

impl Component for TextBlock {
    fn component_id(&self) -> &'static str {
        "textblock"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Number field with formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberField {
    /// Number value
    pub value: f64,
    /// Number format pattern (e.g., "0.00", "#,###.##")
    #[serde(default)]
    pub format: Option<String>,
    /// Prefix (e.g., "$", "€")
    #[serde(default)]
    pub prefix: Option<String>,
    /// Suffix (e.g., "%", "kg")
    #[serde(default)]
    pub suffix: Option<String>,
    /// Font size
    #[serde(default)]
    pub font_size: Option<String>,
    /// Color
    #[serde(default)]
    pub color: Option<String>,
}

impl NumberField {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            format: None,
            prefix: None,
            suffix: None,
            font_size: None,
            color: None,
        }
    }

    pub fn currency(value: f64, symbol: impl Into<String>) -> Self {
        Self {
            format: Some("#,##0.00".into()),
            prefix: Some(symbol.into()),
            ..Self::new(value)
        }
    }

    pub fn percentage(value: f64) -> Self {
        Self {
            format: Some("0.0".into()),
            suffix: Some("%".into()),
            ..Self::new(value)
        }
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }
}

impl Component for NumberField {
    fn component_id(&self) -> &'static str {
        "number-field"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Date field with formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateField {
    /// ISO 8601 date string
    pub date: String,
    /// Date format pattern (e.g., "YYYY-MM-DD", "DD.MM.YYYY")
    #[serde(default = "default_date_format")]
    pub format: String,
    /// Font size
    #[serde(default)]
    pub font_size: Option<String>,
}

fn default_date_format() -> String {
    "YYYY-MM-DD".into()
}

impl DateField {
    pub fn new(date: impl Into<String>) -> Self {
        Self {
            date: date.into(),
            format: "YYYY-MM-DD".into(),
            font_size: None,
        }
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = format.into();
        self
    }

    pub fn european(date: impl Into<String>) -> Self {
        Self {
            format: "DD.MM.YYYY".into(),
            ..Self::new(date)
        }
    }

    pub fn us(date: impl Into<String>) -> Self {
        Self {
            format: "MM/DD/YYYY".into(),
            ..Self::new(date)
        }
    }
}

impl Component for DateField {
    fn component_id(&self) -> &'static str {
        "date-field"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}

/// Resource field for localized strings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceField {
    /// Resource key
    pub key: String,
    /// Default value if key not found
    #[serde(default)]
    pub default_value: Option<String>,
    /// Locale (e.g., "en", "de", "fr")
    #[serde(default)]
    pub locale: Option<String>,
}

impl ResourceField {
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            default_value: None,
            locale: None,
        }
    }

    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.default_value = Some(default.into());
        self
    }

    pub fn with_locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }
}

impl Component for ResourceField {
    fn component_id(&self) -> &'static str {
        "resource-field"
    }
    fn to_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_default()
    }
}
