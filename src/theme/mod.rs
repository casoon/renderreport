//! Theme system with CSS-variable-like tokens
//!
//! Themes define visual appearance through tokens (color, typography, spacing).
//! Tokens can be overridden at multiple levels:
//! 1. Pack default theme
//! 2. User-defined theme
//! 3. Per-request overrides

mod tokens;

pub use tokens::{ThemeTokens, TokenValue};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A complete theme definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
    /// Token values
    pub tokens: ThemeTokens,
}

impl Theme {
    /// Create a new theme with the given ID
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            tokens: ThemeTokens::default(),
        }
    }

    /// Create the default theme
    pub fn default_theme() -> Self {
        Self {
            id: "default".into(),
            name: "Default Theme".into(),
            description: Some("Standard RenderReport theme".into()),
            tokens: ThemeTokens::default(),
        }
    }

    /// Merge another theme's tokens (other takes precedence)
    pub fn merge(&mut self, other: &Theme) {
        self.tokens.merge(&other.tokens);
    }

    /// Override specific tokens
    pub fn with_overrides(mut self, overrides: HashMap<String, TokenValue>) -> Self {
        for (key, value) in overrides {
            self.tokens.set(&key, value);
        }
        self
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_theme()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::new("custom", "Custom Theme");
        assert_eq!(theme.id, "custom");
        assert_eq!(theme.name, "Custom Theme");
    }

    #[test]
    fn test_theme_merge() {
        let mut base = Theme::default_theme();
        let mut custom = Theme::new("custom", "Custom");
        custom
            .tokens
            .set("color.primary", TokenValue::Color("#ff0000".into()));

        base.merge(&custom);
        assert_eq!(
            base.tokens.get("color.primary"),
            Some(&TokenValue::Color("#ff0000".into()))
        );
    }
}
