//! Simple Report Example
//!
//! Minimal example showing basic RenderReport usage.
//!
//! Run with: cargo run --example simple_report

use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    // Initialize engine with defaults
    let engine = Engine::new()?;

    // Build a simple report
    let report = engine
        .report("default")
        .title("My First Report")
        .subtitle("A simple demonstration")
        .add_component(ScoreCard::new("Quality Score", 95))
        .add_component(Callout::success("Everything looks great!"))
        .add_component(
            AuditTable::new(vec![TableColumn::new("Item"), TableColumn::new("Value")])
                .add_row(vec!["Tests Passed", "42"])
                .add_row(vec!["Coverage", "87%"])
                .add_row(vec!["Issues", "0"]),
        )
        .build();

    // Render and save
    let pdf = engine.render_pdf(&report)?;
    std::fs::write("examples/output/simple_report.pdf", &pdf)?;

    println!("Created simple_report.pdf");
    Ok(())
}
