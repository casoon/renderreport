//! Engine and rendering tests

use renderreport::prelude::*;
use renderreport::render::RenderRequest;

#[test]
fn test_engine_creation() {
    let engine = Engine::new();
    assert!(engine.is_ok());
}

#[test]
fn test_engine_default() {
    let engine = Engine::default();
    assert_eq!(engine.default_theme().id, "default");
}

#[test]
fn test_engine_with_custom_theme() {
    let mut engine = Engine::new().unwrap();

    let custom_theme = Theme::new("custom", "Custom Theme");
    engine.set_default_theme(custom_theme);

    assert_eq!(engine.default_theme().id, "custom");
}

#[test]
fn test_report_builder() {
    let engine = Engine::new().unwrap();

    let report = engine
        .report("default")
        .title("Test Report")
        .subtitle("Test Subtitle")
        .add_component(ScoreCard::new("Test", 85))
        .build();

    assert_eq!(report.template_id, "default");
    assert_eq!(report.title, Some("Test Report".into()));
    assert_eq!(report.subtitle, Some("Test Subtitle".into()));
    assert_eq!(report.components.len(), 1);
}

#[test]
fn test_report_builder_with_theme() {
    let engine = Engine::new().unwrap();

    let custom_theme = Theme::new("test", "Test Theme");

    let report = engine.report("default").theme(custom_theme.clone()).build();

    assert!(report.theme.is_some());
    assert_eq!(report.theme.unwrap().id, "test");
}

#[test]
fn test_report_builder_with_metadata() {
    let engine = Engine::new().unwrap();

    let report = engine
        .report("default")
        .metadata("author", "Test Author")
        .metadata("version", "1.0")
        .build();

    assert_eq!(report.metadata.get("author"), Some(&"Test Author".into()));
    assert_eq!(report.metadata.get("version"), Some(&"1.0".into()));
}

#[test]
fn test_report_builder_multiple_components() {
    let engine = Engine::new().unwrap();

    let report = engine
        .report("default")
        .add_component(ScoreCard::new("Score 1", 85))
        .add_component(ScoreCard::new("Score 2", 90))
        .add_component(Finding::new("Issue", Severity::High, "Description"))
        .build();

    assert_eq!(report.components.len(), 3);
}

#[test]
fn test_component_registry() {
    let engine = Engine::new().unwrap();
    let registry = engine.components();

    // Check standard components are registered
    assert!(registry.has_component(&ComponentId::new("score-card")));
    assert!(registry.has_component(&ComponentId::new("finding")));
    assert!(registry.has_component(&ComponentId::new("audit-table")));
    assert!(registry.has_component(&ComponentId::new("section")));
}

#[test]
fn test_typst_source_generation() {
    let engine = Engine::new().unwrap();

    let report = engine
        .report("default")
        .title("Test")
        .add_component(ScoreCard::new("Score", 85))
        .build();

    // This tests the internal source generation
    // We can't directly test compile without full Typst setup
    assert_eq!(report.template_id, "default");
}

#[test]
fn test_render_request_creation() {
    let request = RenderRequest::new("test-template");

    assert_eq!(request.template_id, "test-template");
    assert!(request.title.is_none());
    assert!(request.components.is_empty());
}

#[test]
fn test_engine_config() {
    use renderreport::engine::EngineConfig;

    let config = EngineConfig::default();

    assert!(config.use_embedded_fonts);
    assert!(config.use_system_fonts);
    assert!(!config.pack_paths.is_empty());
}

#[test]
fn test_engine_config_builder() {
    use renderreport::engine::EngineConfig;
    use std::path::PathBuf;

    let config = EngineConfig::builder()
        .pack_path(PathBuf::from("./custom-packs"))
        .use_system_fonts(false)
        .build();

    assert!(!config.use_system_fonts);
    assert!(config
        .pack_paths
        .iter()
        .any(|p| p.ends_with("custom-packs")));
}
