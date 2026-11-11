//! Component tests

use renderreport::components::*;

#[test]
fn test_score_card_creation() {
    let card = ScoreCard::new("Test Score", 85);
    assert_eq!(card.title, "Test Score");
    assert_eq!(card.score, 85);
    assert_eq!(card.max_score, 100);
}

#[test]
fn test_score_card_status() {
    let good = ScoreCard::new("Good", 95);
    assert_eq!(good.computed_status(), ScoreStatus::Good);

    let warning = ScoreCard::new("Warning", 75);
    assert_eq!(warning.computed_status(), ScoreStatus::Warning);

    let bad = ScoreCard::new("Bad", 30);
    assert_eq!(bad.computed_status(), ScoreStatus::Bad);
}

#[test]
fn test_score_card_custom_thresholds() {
    let card = ScoreCard::new("Custom", 70).with_thresholds(80, 60);
    assert_eq!(card.computed_status(), ScoreStatus::Warning);
}

#[test]
fn test_finding_creation() {
    let finding = Finding::new(
        "Test Finding",
        Severity::High,
        "This is a test finding description",
    );

    assert_eq!(finding.title, "Test Finding");
    assert_eq!(finding.severity, Severity::High);
    assert_eq!(finding.description, "This is a test finding description");
    assert!(finding.recommendation.is_none());
}

#[test]
fn test_finding_with_details() {
    let finding = Finding::new("Test", Severity::Medium, "Description")
        .with_recommendation("Fix it like this")
        .with_affected("https://example.com")
        .with_category("Security");

    assert_eq!(finding.recommendation, Some("Fix it like this".into()));
    assert_eq!(finding.affected, Some("https://example.com".into()));
    assert_eq!(finding.category, Some("Security".into()));
}

#[test]
fn test_severity_levels() {
    assert_eq!(Severity::Critical.as_str(), "critical");
    assert_eq!(Severity::High.as_str(), "high");
    assert_eq!(Severity::Medium.as_str(), "medium");
    assert_eq!(Severity::Low.as_str(), "low");
    assert_eq!(Severity::Info.as_str(), "info");
}

#[test]
fn test_audit_table_creation() {
    let table = AuditTable::new(vec![TableColumn::new("Name"), TableColumn::new("Value")]);

    assert_eq!(table.columns.len(), 2);
    assert_eq!(table.rows.len(), 0);
    assert!(table.striped);
}

#[test]
fn test_audit_table_with_data() {
    let table = AuditTable::new(vec![TableColumn::new("Check"), TableColumn::new("Status")])
        .with_title("Test Table")
        .add_row(vec!["Test 1", "Pass"])
        .add_row(vec!["Test 2", "Fail"]);

    assert_eq!(table.title, Some("Test Table".into()));
    assert_eq!(table.rows.len(), 2);
}

#[test]
fn test_section_creation() {
    let section = Section::new("Test Section");
    assert_eq!(section.title, "Test Section");
    assert_eq!(section.level, 1);
}

#[test]
fn test_section_level_clamping() {
    let section = Section::new("Test").with_level(10);
    assert_eq!(section.level, 6); // Should clamp to max 6
}

#[test]
fn test_callout_variants() {
    let info = Callout::info("Info message");
    assert_eq!(info.callout_type, "info");

    let warning = Callout::warning("Warning message");
    assert_eq!(warning.callout_type, "warning");

    let error = Callout::error("Error message");
    assert_eq!(error.callout_type, "error");

    let success = Callout::success("Success message");
    assert_eq!(success.callout_type, "success");
}

#[test]
fn test_summary_box() {
    let summary = SummaryBox::new("Test Summary")
        .add_item("Metric 1", "100")
        .add_item_with_status("Metric 2", "85", ScoreStatus::Good);

    assert_eq!(summary.title, "Test Summary");
    assert_eq!(summary.items.len(), 2);
    assert_eq!(summary.items[0].label, "Metric 1");
    assert_eq!(summary.items[1].status, Some(ScoreStatus::Good));
}

#[test]
fn test_image_component() {
    let img = Image::new("test.png")
        .with_caption("Test Image")
        .with_width("50%");

    assert_eq!(img.src, "test.png");
    assert_eq!(img.caption, Some("Test Image".into()));
    assert_eq!(img.width, Some("50%".into()));
}

#[test]
fn test_component_trait() {
    let card = ScoreCard::new("Test", 90);
    assert_eq!(card.component_id(), "score-card");

    let finding = Finding::new("Test", Severity::High, "Description");
    assert_eq!(finding.component_id(), "finding");

    let table = AuditTable::new(vec![TableColumn::new("Test")]);
    assert_eq!(table.component_id(), "audit-table");
}

#[test]
fn test_component_serialization() {
    let card = ScoreCard::new("Test", 85);
    let data = card.to_data();

    assert!(data.is_object());
    let obj = data.as_object().unwrap();
    assert_eq!(obj.get("title").and_then(|v| v.as_str()), Some("Test"));
    assert_eq!(obj.get("score").and_then(|v| v.as_u64()), Some(85));
}

fn benchmark_rows() -> Vec<BenchmarkRow> {
    vec![
        BenchmarkRow::new(1, "a.example", 93, 96, 1).with_column("SEO", 91),
        BenchmarkRow::new(2, "b.example", 41, 50, 0),
    ]
}

fn render_benchmark_pdf(table: BenchmarkTable) -> Vec<u8> {
    let engine = renderreport::Engine::new().unwrap();
    let request = engine
        .report("default")
        .title("Benchmark")
        .add_component(table)
        .build();
    engine.render_pdf(&request).unwrap()
}

#[test]
fn test_benchmark_table_default_headers_unchanged() {
    let table = BenchmarkTable::new(benchmark_rows());
    let data = table.to_data();
    let mut keys: Vec<&str> = data
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    assert_eq!(keys, ["rows", "title"]);

    let explicit_german =
        BenchmarkTable::new(benchmark_rows()).with_labels("Nr", "Domain", "Score", "A11y", "Krit.");
    assert_eq!(
        render_benchmark_pdf(table),
        render_benchmark_pdf(explicit_german)
    );
}

#[test]
fn test_benchmark_table_supplied_headers_rendered() {
    let engine = renderreport::Engine::new().unwrap();
    let request = engine
        .report("default")
        .title("Benchmark")
        .add_component(BenchmarkTable::new(benchmark_rows()).with_labels(
            "Rank",
            "Website",
            "Overall",
            "Accessibility",
            "Critical",
        ))
        .build();
    let typ = engine.render_typ(&request).unwrap();
    for (key, label) in [
        ("label_rank", "Rank"),
        ("label_domain", "Website"),
        ("label_score", "Overall"),
        ("label_accessibility", "Accessibility"),
        ("label_critical_issues", "Critical"),
    ] {
        assert!(
            typ.contains(&format!("{key}: \"{label}\"")),
            "{key} missing from Typst source"
        );
    }
    assert_ne!(
        engine.render_pdf(&request).unwrap(),
        render_benchmark_pdf(BenchmarkTable::new(benchmark_rows()))
    );
}
