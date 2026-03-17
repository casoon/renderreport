//! Simple Report Example
//!
//! A beginner-friendly example showcasing the most common RenderReport components.
//! Great starting point to understand the API.
//!
//! Run with: cargo run --example simple_report

use renderreport::components::advanced::*;
use renderreport::components::charts::{Chart, Gauge};
use renderreport::components::Component;
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    let engine = Engine::new()?;

    let report = engine
        .report("default")
        .title("Project Status Report")
        .subtitle("Sprint 24 — March 2026")
        .metadata("author", "Engineering Team")
        // ── Overview Section ────────────────────────────────────────
        .add_component(Section::new("Overview").with_level(1))
        .add_component(
            SummaryBox::new("Sprint Summary")
                .add_item_with_status("Sprint Goal", "Feature Complete", ScoreStatus::Good)
                .add_item("Story Points", "47 / 52")
                .add_item("Velocity", "48 avg")
                .add_item_with_status("Release Readiness", "On Track", ScoreStatus::Good),
        )
        // ── Score Cards in Grid ─────────────────────────────────────
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Code Quality", 92)
                        .with_description("SonarQube analysis")
                        .to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Test Coverage", 87)
                        .with_description("Unit + Integration")
                        .with_thresholds(80, 60)
                        .to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Performance", 95)
                        .with_description("Lighthouse score")
                        .to_data()
                })),
        )
        // ── Progress ────────────────────────────────────────────────
        .add_component(Section::new("Sprint Progress").with_level(1))
        .add_component(ProgressBar::new("Backend API", 95.0).with_color("#22c55e"))
        .add_component(ProgressBar::new("Frontend UI", 82.0).with_color("#3b82f6"))
        .add_component(ProgressBar::new("Documentation", 60.0).with_color("#f59e0b"))
        .add_component(ProgressBar::new("QA Testing", 70.0).with_color("#8b5cf6"))
        // ── Chart ───────────────────────────────────────────────────
        .add_component(Section::new("Velocity Trend").with_level(1))
        .add_component(
            Chart::bar("Story Points per Sprint")
                .add_series("Completed", vec![
                    ("S20".into(), 38.0),
                    ("S21".into(), 42.0),
                    ("S22".into(), 45.0),
                    ("S23".into(), 51.0),
                    ("S24".into(), 47.0),
                ])
                .with_labels("Sprint", "Story Points"),
        )
        // ── KPI Gauges ──────────────────────────────────────────────
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Uptime", 99.8).with_range(95.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Response Time", 72.0).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Error Rate", 95.0).with_range(0.0, 100.0).to_data()
                })),
        )
        // ── Findings ────────────────────────────────────────────────
        .add_component(Section::new("Open Issues").with_level(1))
        .add_component(
            Finding::new("Database Connection Pool Exhaustion", Severity::High,
                "Under peak load (>500 concurrent users), the connection pool is exhausted causing 503 errors.")
                .with_recommendation("Increase pool size from 20 to 50 and implement connection timeout of 5s.")
                .with_category("Infrastructure"),
        )
        .add_component(
            Finding::new("Deprecated API Endpoints", Severity::Medium,
                "3 API endpoints marked for deprecation in v2.0 are still being consumed by mobile clients.")
                .with_recommendation("Coordinate with mobile team to migrate to v2 endpoints before the April release.")
                .with_category("API"),
        )
        // ── Metadata ────────────────────────────────────────────────
        .add_component(Divider::new())
        .add_component(
            KeyValueList::new()
                .with_title("Project Details")
                .add("Project", "RenderReport")
                .add("Version", "0.1.0-alpha")
                .add("Sprint", "24 (March 3 – March 17)")
                .add_highlighted("Next Release", "v0.2.0 — April 1, 2026"),
        )
        // ── Closing ─────────────────────────────────────────────────
        .add_component(
            Callout::info("This report was auto-generated from sprint data. Review detailed metrics in Jira.")
                .with_title("Note"),
        )
        .build();

    let pdf = engine.render_pdf(&report)?;
    std::fs::write("examples/output/simple_report.pdf", &pdf)?;

    println!("✓ Simple report generated: examples/output/simple_report.pdf ({} KB)", pdf.len() / 1024);
    Ok(())
}
