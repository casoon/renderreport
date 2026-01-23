//! Theme and token tests

use renderreport::theme::{Theme, ThemeTokens, TokenValue};

#[test]
fn test_token_value_to_typst() {
    let color = TokenValue::Color("#ff0000".into());
    assert_eq!(color.to_typst(), "rgb(\"#ff0000\")");

    let size = TokenValue::Size("12pt".into());
    assert_eq!(size.to_typst(), "12pt");

    let font = TokenValue::Font("Arial".into());
    assert_eq!(font.to_typst(), "\"Arial\"");

    let number = TokenValue::Number(42.0);
    assert_eq!(number.to_typst(), "42");

    let boolean = TokenValue::Bool(true);
    assert_eq!(boolean.to_typst(), "true");
}

#[test]
fn test_theme_tokens_default() {
    let tokens = ThemeTokens::default();

    // Check colors
    assert!(tokens.get("color.primary").is_some());
    assert!(tokens.get("color.text").is_some());
    assert!(tokens.get("color.ok").is_some());

    // Check fonts
    assert!(tokens.get("font.body").is_some());
    assert!(tokens.get("font.size.base").is_some());

    // Check spacing
    assert!(tokens.get("spacing.1").is_some());
    assert!(tokens.get("spacing.4").is_some());
}

#[test]
fn test_theme_tokens_set_get() {
    let mut tokens = ThemeTokens::new();

    tokens.set("custom.token", TokenValue::Color("#123456".into()));

    assert_eq!(
        tokens.get("custom.token"),
        Some(&TokenValue::Color("#123456".into()))
    );
}

#[test]
fn test_theme_tokens_merge() {
    let mut base = ThemeTokens::new();
    base.set("color.primary", TokenValue::Color("#000000".into()));
    base.set("color.secondary", TokenValue::Color("#111111".into()));

    let mut override_tokens = ThemeTokens::new();
    override_tokens.set("color.primary", TokenValue::Color("#ffffff".into()));

    base.merge(&override_tokens);

    // Primary should be overridden
    assert_eq!(
        base.get("color.primary"),
        Some(&TokenValue::Color("#ffffff".into()))
    );

    // Secondary should remain
    assert_eq!(
        base.get("color.secondary"),
        Some(&TokenValue::Color("#111111".into()))
    );
}

#[test]
fn test_theme_tokens_to_typst_definitions() {
    let mut tokens = ThemeTokens::new();
    tokens.set("color.primary", TokenValue::Color("#2563eb".into()));
    tokens.set("font.body", TokenValue::Font("Inter".into()));

    let typst = tokens.to_typst_definitions();

    assert!(typst.contains("#let color-primary = rgb(\"#2563eb\")"));
    assert!(typst.contains("#let font-body = \"Inter\""));
}

#[test]
fn test_theme_creation() {
    let theme = Theme::new("custom", "Custom Theme");

    assert_eq!(theme.id, "custom");
    assert_eq!(theme.name, "Custom Theme");
    assert!(theme.description.is_none());
}

#[test]
fn test_theme_default() {
    let theme = Theme::default_theme();

    assert_eq!(theme.id, "default");
    assert_eq!(theme.name, "Default Theme");
    assert!(theme.tokens.get("color.primary").is_some());
}

#[test]
fn test_theme_merge() {
    let mut base = Theme::default_theme();
    let original_secondary = base.tokens.get("color.secondary").cloned();

    let mut custom = Theme::new("custom", "Custom");
    custom
        .tokens
        .set("color.primary", TokenValue::Color("#ffffff".into()));

    base.merge(&custom);

    // Primary should be overridden
    assert_eq!(
        base.tokens.get("color.primary"),
        Some(&TokenValue::Color("#ffffff".into()))
    );

    // Secondary should remain unchanged
    assert_eq!(
        base.tokens.get("color.secondary"),
        original_secondary.as_ref()
    );
}

#[test]
fn test_theme_with_overrides() {
    let theme = Theme::default_theme().with_overrides(
        vec![
            ("color.primary".into(), TokenValue::Color("#ff0000".into())),
            ("font.body".into(), TokenValue::Font("Helvetica".into())),
        ]
        .into_iter()
        .collect(),
    );

    assert_eq!(
        theme.tokens.get("color.primary"),
        Some(&TokenValue::Color("#ff0000".into()))
    );
    assert_eq!(
        theme.tokens.get("font.body"),
        Some(&TokenValue::Font("Helvetica".into()))
    );
}

#[test]
fn test_token_iteration() {
    let mut tokens = ThemeTokens::new();
    tokens.set("test.1", TokenValue::Number(1.0));
    tokens.set("test.2", TokenValue::Number(2.0));

    let count = tokens.iter().count();
    assert_eq!(count, 2);
}

#[test]
fn test_typst_variable_naming() {
    let mut tokens = ThemeTokens::new();
    tokens.set("color.text-muted", TokenValue::Color("#666666".into()));

    let typst = tokens.to_typst_definitions();

    // Dots should be replaced with hyphens
    assert!(typst.contains("color-text-muted"));
}
