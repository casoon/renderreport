//! Component Showcase
//!
//! Demonstrates all standard and advanced component types in a single report.
//!
//! Run with: cargo run --example new_components_demo

use renderreport::components::advanced::*;
use renderreport::components::charts::{Chart, ChartType, Gauge, Sparkline};
use renderreport::components::text::*;
use renderreport::components::Component;
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    println!("Generating component showcase report...");

    let engine = Engine::new()?;

    let report = engine
        .report("default")
        .title("Component Showcase")
        .subtitle("All Available RenderReport Components")
        .metadata("author", "RenderReport Docs")
        // ── Text Components ─────────────────────────────────────
        .add_component(Section::new("Text Components").with_level(1))
        .add_component(Label::new("This is a Label — colored and sized")
            .with_size("14pt").with_color("#2563eb"))
        .add_component(TextBlock::new(
            "TextBlock renders multi-line paragraphs with configurable line height and alignment. \
             It supports long-form content like descriptions, explanations, and narrative sections \
             within your reports."))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "number-field",
                    "data": NumberField::new(1234567.89).with_prefix("$").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "number-field",
                    "data": NumberField::new(0.947).with_suffix("%").with_format("0.0").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "number-field",
                    "data": NumberField::new(42195.0).with_suffix(" users").to_data()
                })),
        )
        // ── Metric & Score Cards ────────────────────────────────
        .add_component(Section::new("Score Cards & Metrics").with_level(1))
        .add_component(
            Grid::new(4)
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Quality", 94).with_description("A+ rating").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Security", 87).with_description("Strong").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Speed", 72).with_description("Needs work").with_thresholds(80, 50).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Coverage", 45).with_description("Poor").with_thresholds(80, 50).to_data()
                })),
        )
        .add_component(
            SummaryBox::new("System Health")
                .add_item_with_status("API", "Operational", ScoreStatus::Good)
                .add_item_with_status("Database", "Degraded", ScoreStatus::Warning)
                .add_item_with_status("Cache", "Operational", ScoreStatus::Good)
                .add_item("Uptime", "99.95%")
                .add_item("Response Time", "142ms"),
        )
        // ── Charts ──────────────────────────────────────────────
        .add_component(Section::new("Chart Types").with_level(1))
        .add_component(
            Chart::bar("Bar Chart")
                .add_series("Sales", vec![
                    ("Mon".into(), 42.0), ("Tue".into(), 58.0), ("Wed".into(), 51.0),
                    ("Thu".into(), 67.0), ("Fri".into(), 49.0),
                ])
                .with_labels("Day", "Sales"),
        )
        .add_component(
            Chart::line("Line Chart")
                .add_series("Visitors", vec![
                    ("9am".into(), 120.0), ("10am".into(), 250.0), ("11am".into(), 380.0),
                    ("12pm".into(), 420.0), ("1pm".into(), 350.0), ("2pm".into(), 310.0),
                    ("3pm".into(), 290.0), ("4pm".into(), 270.0), ("5pm".into(), 180.0),
                ])
                .with_labels("Time", "Visitors"),
        )
        .add_component(
            Chart::pie("Pie Chart").add_series("Data", vec![
                ("Segment A".into(), 40.0), ("Segment B".into(), 30.0),
                ("Segment C".into(), 20.0), ("Segment D".into(), 10.0),
            ]),
        )
        .add_component(
            Chart::new("Area Chart", ChartType::Area)
                .add_series("Growth", vec![
                    ("Q1".into(), 100.0), ("Q2".into(), 145.0),
                    ("Q3".into(), 210.0), ("Q4".into(), 298.0),
                ])
                .with_labels("Quarter", "Value"),
        )
        // ── Gauges ──────────────────────────────────────────────
        .add_component(Section::new("Gauges & Sparklines").with_level(1))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("CPU", 65.0).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Memory", 82.0).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::thermometer("Temp", 38.5).with_range(20.0, 60.0).to_data()
                })),
        )
        .add_component(Sparkline::bar(vec![3.0, 5.0, 4.0, 7.0, 6.0, 9.0, 8.0, 11.0]).with_color("#3b82f6"))
        .add_component(Sparkline::bar(vec![12.0, 10.0, 14.0, 11.0, 16.0, 13.0, 18.0, 15.0]).with_color("#22c55e"))
        // ── Findings ────────────────────────────────────────────
        .add_component(Section::new("Finding Severities").with_level(1))
        .add_component(Finding::new("Critical Issue", Severity::Critical, "System-wide outage detected. Immediate action required.").with_category("Infra"))
        .add_component(Finding::new("High Severity", Severity::High, "Memory leak in worker process causes gradual degradation.").with_recommendation("Restart workers on 4-hour schedule until patch is deployed.").with_category("Backend"))
        .add_component(Finding::new("Medium Severity", Severity::Medium, "Deprecated API endpoint still receiving 500 requests/day.").with_category("API"))
        .add_component(Finding::new("Low Severity", Severity::Low, "Minor UI alignment issue on mobile Safari.").with_category("Frontend"))
        .add_component(Finding::new("Informational", Severity::Info, "New monitoring dashboard deployed successfully.").with_category("DevOps"))
        // ── Layout Components ───────────────────────────────────
        .add_component(Section::new("Layout & Advanced").with_level(1))
        .add_component(ProgressBar::new("Upload Complete", 100.0).with_color("#22c55e"))
        .add_component(ProgressBar::new("Processing", 67.0).with_color("#3b82f6"))
        .add_component(ProgressBar::new("Pending Review", 30.0).with_color("#f59e0b"))
        .add_component(Divider::dashed())
        .add_component(
            KeyValueList::new()
                .with_title("Configuration")
                .add("Environment", "Production")
                .add("Region", "eu-central-1")
                .add("Runtime", "Rust 1.82")
                .add_highlighted("Version", "v2.1.0"),
        )
        .add_component(
            AuditTable::new(vec![
                TableColumn::new("Component").with_width("25%"),
                TableColumn::new("Type").with_width("25%"),
                TableColumn::new("Status").with_width("25%"),
                TableColumn::new("Since").with_width("25%"),
            ])
            .with_title("Component Registry")
            .add_row(vec!["ScoreCard", "Standard", "Stable", "v0.1.0"])
            .add_row(vec!["Chart", "Visualization", "Stable", "v0.1.0"])
            .add_row(vec!["Barcode", "Encoding", "Stable", "v0.1.0"])
            .add_row(vec!["Grid", "Layout", "Stable", "v0.1.0"])
            .add_row(vec!["CoverPage", "Composite", "Stable", "v0.1.0"])
            .add_row(vec!["HeroSummary", "Composite", "Stable", "v0.1.0"]),
        )
        // ── Callouts ────────────────────────────────────────────
        .add_component(Section::new("Callout Types").with_level(1))
        .add_component(Callout::info("Informational callout for general notes.").with_title("Info"))
        .add_component(Callout::success("Success callout for positive outcomes.").with_title("Success"))
        .add_component(Callout::warning("Warning callout for potential issues.").with_title("Warning"))
        .add_component(Callout::error("Error callout for critical problems.").with_title("Error"))
        .build();

    let pdf = engine.render_pdf(&report)?;
    let output_path = "examples/output/new_components_demo.pdf";
    std::fs::write(output_path, &pdf)?;

    println!("✓ Component showcase generated: {} ({} KB)", output_path, pdf.len() / 1024);
    Ok(())
}
