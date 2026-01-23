//! Token definitions and values
//!
//! Tokens follow a hierarchical naming convention:
//! - `color.*` - Colors (primary, text, muted, ok, warn, bad, etc.)
//! - `font.*` - Typography (body, heading, mono)
//! - `spacing.*` - Spacing scale (1, 2, 3, 4, etc.)
//! - `table.*` - Table-specific tokens
//! - `page.*` - Page layout tokens
//! - `component.*` - Component-specific tokens

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single token value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TokenValue {
    /// Color value (hex, rgb, or named)
    Color(String),
    /// Size/spacing value with unit
    Size(String),
    /// Font family
    Font(String),
    /// Numeric value
    Number(f64),
    /// Boolean flag
    Bool(bool),
    /// Raw string value
    String(String),
}

impl TokenValue {
    /// Convert to Typst code representation
    pub fn to_typst(&self) -> String {
        match self {
            TokenValue::Color(c) => {
                if c.starts_with('#') {
                    format!("rgb(\"{}\")", c)
                } else {
                    c.clone()
                }
            }
            TokenValue::Size(s) => s.clone(),
            TokenValue::Font(f) => format!("\"{}\"", f),
            TokenValue::Number(n) => n.to_string(),
            TokenValue::Bool(b) => b.to_string(),
            TokenValue::String(s) => format!("\"{}\"", s),
        }
    }
}

/// Collection of theme tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeTokens {
    #[serde(flatten)]
    tokens: HashMap<String, TokenValue>,
}

impl ThemeTokens {
    /// Create empty token collection
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }

    /// Get a token value
    pub fn get(&self, key: &str) -> Option<&TokenValue> {
        self.tokens.get(key)
    }

    /// Set a token value
    pub fn set(&mut self, key: impl Into<String>, value: TokenValue) {
        self.tokens.insert(key.into(), value);
    }

    /// Merge another token set (other takes precedence)
    pub fn merge(&mut self, other: &ThemeTokens) {
        for (key, value) in &other.tokens {
            self.tokens.insert(key.clone(), value.clone());
        }
    }

    /// Iterate over all tokens
    pub fn iter(&self) -> impl Iterator<Item = (&String, &TokenValue)> {
        self.tokens.iter()
    }

    /// Generate Typst variable definitions
    pub fn to_typst_definitions(&self) -> String {
        let mut lines = Vec::new();
        for (key, value) in &self.tokens {
            let var_name = key.replace('.', "-");
            lines.push(format!("#let {} = {}", var_name, value.to_typst()));
        }
        lines.sort();
        lines.join("\n")
    }
}

impl Default for ThemeTokens {
    fn default() -> Self {
        let mut tokens = Self::new();

        // Colors
        tokens.set("color.primary", TokenValue::Color("#2563eb".into()));
        tokens.set("color.secondary", TokenValue::Color("#64748b".into()));
        tokens.set("color.text", TokenValue::Color("#1e293b".into()));
        tokens.set("color.text-muted", TokenValue::Color("#64748b".into()));
        tokens.set("color.background", TokenValue::Color("#ffffff".into()));
        tokens.set("color.surface", TokenValue::Color("#f8fafc".into()));
        tokens.set("color.border", TokenValue::Color("#e2e8f0".into()));
        tokens.set("color.ok", TokenValue::Color("#22c55e".into()));
        tokens.set("color.warn", TokenValue::Color("#f59e0b".into()));
        tokens.set("color.bad", TokenValue::Color("#ef4444".into()));

        // Typography
        tokens.set("font.body", TokenValue::Font("Inter".into()));
        tokens.set("font.heading", TokenValue::Font("Inter".into()));
        tokens.set("font.mono", TokenValue::Font("JetBrains Mono".into()));
        tokens.set("font.size.base", TokenValue::Size("11pt".into()));
        tokens.set("font.size.sm", TokenValue::Size("9pt".into()));
        tokens.set("font.size.lg", TokenValue::Size("14pt".into()));
        tokens.set("font.size.xl", TokenValue::Size("18pt".into()));
        tokens.set("font.size.2xl", TokenValue::Size("24pt".into()));

        // Spacing
        tokens.set("spacing.1", TokenValue::Size("4pt".into()));
        tokens.set("spacing.2", TokenValue::Size("8pt".into()));
        tokens.set("spacing.3", TokenValue::Size("12pt".into()));
        tokens.set("spacing.4", TokenValue::Size("16pt".into()));
        tokens.set("spacing.5", TokenValue::Size("24pt".into()));
        tokens.set("spacing.6", TokenValue::Size("32pt".into()));

        // Table
        tokens.set("table.header-bg", TokenValue::Color("#f1f5f9".into()));
        tokens.set("table.row-alt-bg", TokenValue::Color("#f8fafc".into()));
        tokens.set("table.border", TokenValue::Color("#e2e8f0".into()));
        tokens.set("table.border-width", TokenValue::Size("0.5pt".into()));

        // Page
        tokens.set("page.margin", TokenValue::Size("2.5cm".into()));
        tokens.set("page.margin-top", TokenValue::Size("3cm".into()));
        tokens.set("page.margin-bottom", TokenValue::Size("2.5cm".into()));
        tokens.set("page.header-height", TokenValue::Size("1.5cm".into()));
        tokens.set("page.footer-height", TokenValue::Size("1cm".into()));

        // Components
        tokens.set(
            "component.score-card.radius",
            TokenValue::Size("8pt".into()),
        );
        tokens.set("component.finding.radius", TokenValue::Size("4pt".into()));
        tokens.set("component.callout.radius", TokenValue::Size("4pt".into()));

        tokens
    }
}

/// Standard token names (for documentation and validation)
pub mod token_names {
    // Colors
    pub const COLOR_PRIMARY: &str = "color.primary";
    pub const COLOR_SECONDARY: &str = "color.secondary";
    pub const COLOR_TEXT: &str = "color.text";
    pub const COLOR_TEXT_MUTED: &str = "color.text-muted";
    pub const COLOR_BACKGROUND: &str = "color.background";
    pub const COLOR_SURFACE: &str = "color.surface";
    pub const COLOR_BORDER: &str = "color.border";
    pub const COLOR_OK: &str = "color.ok";
    pub const COLOR_WARN: &str = "color.warn";
    pub const COLOR_BAD: &str = "color.bad";

    // Typography
    pub const FONT_BODY: &str = "font.body";
    pub const FONT_HEADING: &str = "font.heading";
    pub const FONT_MONO: &str = "font.mono";
    pub const FONT_SIZE_BASE: &str = "font.size.base";
    pub const FONT_SIZE_SM: &str = "font.size.sm";
    pub const FONT_SIZE_LG: &str = "font.size.lg";
    pub const FONT_SIZE_XL: &str = "font.size.xl";
    pub const FONT_SIZE_2XL: &str = "font.size.2xl";

    // Spacing
    pub const SPACING_1: &str = "spacing.1";
    pub const SPACING_2: &str = "spacing.2";
    pub const SPACING_3: &str = "spacing.3";
    pub const SPACING_4: &str = "spacing.4";
    pub const SPACING_5: &str = "spacing.5";
    pub const SPACING_6: &str = "spacing.6";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_to_typst() {
        assert_eq!(
            TokenValue::Color("#ff0000".into()).to_typst(),
            "rgb(\"#ff0000\")"
        );
        assert_eq!(TokenValue::Size("12pt".into()).to_typst(), "12pt");
        assert_eq!(TokenValue::Number(42.0).to_typst(), "42");
    }

    #[test]
    fn test_default_tokens() {
        let tokens = ThemeTokens::default();
        assert!(tokens.get("color.primary").is_some());
        assert!(tokens.get("font.body").is_some());
        assert!(tokens.get("spacing.4").is_some());
    }
}
