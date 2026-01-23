//! Advanced Components Example
//!
//! Demonstrates advanced components inspired by JasperReports, BIRT, and Pentaho.
//!
//! Run with: cargo run --example advanced_components

use renderreport::components::advanced::*;
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    let engine = Engine::new()?;

    let report = engine
        .report("default")
        .title("Advanced Components Demo")
        .subtitle("Inspired by JasperReports, Eclipse BIRT & Pentaho")
        // List Component (JasperReports-style)
        .add_component(Section::new("List Component").with_level(1))
        .add_component(
            List::new()
                .with_title("Project Tasks")
                .add_item_with_icon("Design database schema", "✓")
                .add_item_with_icon("Implement API endpoints", "✓")
                .add_item_with_icon("Create frontend components", "⏳")
                .add_item("Write documentation")
                .add_item("Deploy to production"),
        )
        // Grid Layout (Pentaho-style)
        .add_component(Section::new("Grid Layout").with_level(1))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Performance", 92).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Security", 88).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Quality", 95).to_data()
                })),
        )
        // Progress Bars
        .add_component(Section::new("Progress Indicators").with_level(1))
        .add_component(ProgressBar::new("Backend Development", 85.0).with_color("#22c55e"))
        .add_component(ProgressBar::new("Frontend Development", 62.0).with_color("#f59e0b"))
        .add_component(ProgressBar::new("Documentation", 40.0).with_color("#ef4444"))
        // Dividers (BIRT-style)
        .add_component(Divider::new())
        // Key-Value List (BIRT Parameters-style)
        .add_component(Section::new("Project Metadata").with_level(1))
        .add_component(
            KeyValueList::new()
                .with_title("Project Information")
                .add("Project Name", "RenderReport")
                .add("Version", "0.1.0")
                .add("Language", "Rust")
                .add_highlighted("Status", "Active Development")
                .add("License", "MIT OR Apache-2.0"),
        )
        .add_component(Divider::dashed())
        // Numbered List
        .add_component(Section::new("Deployment Steps").with_level(1))
        .add_component(
            List::new()
                .numbered()
                .add_item("Run tests: cargo test")
                .add_item("Build release: cargo build --release")
                .add_item("Run benchmarks: cargo bench")
                .add_item("Update documentation: cargo doc")
                .add_item("Publish to crates.io: cargo publish"),
        )
        .add_component(Divider::thick().with_color("#2563eb"))
        // Watermark
        .add_component(Watermark::draft())
        .build();

    // Render to PDF
    let pdf_bytes = engine.render_pdf(&report)?;
    std::fs::write("examples/output/advanced_components_report.pdf", &pdf_bytes)?;

    println!(
        "Report saved to advanced_components_report.pdf ({} bytes)",
        pdf_bytes.len()
    );

    Ok(())
}
