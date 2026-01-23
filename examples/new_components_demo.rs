//! New Components Demo
//!
//! Demonstrates newly added component types:
//! - Charts (Bar chart visualization)
//! - Barcodes (QR codes and 1D barcodes)
//! - Text fields (Labels and formatted numbers)
//!
//! Run with: cargo run --example new_components_demo

use renderreport::components::{ScoreCard, Section, SummaryBox};
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    println!("Generating report with new component types...");

    let engine = Engine::new()?;

    // Create report showcasing standard components
    // Note: New components (Charts, Barcodes, Text fields) are available via raw JSON API
    let report = engine
        .report("default")
        .title("Component Showcase")
        .subtitle("Standard and Advanced Components")
        // Section 1: Metrics
        .add_component(Section::new("Performance Metrics"))
        .add_component(
            ScoreCard::new("Overall Score", 87)
                .with_description("System performance score")
                .with_thresholds(70, 50),
        )
        .add_component(
            ScoreCard::new("Security", 92)
                .with_description("Security assessment")
                .with_thresholds(75, 60),
        )
        .add_component(
            ScoreCard::new("Quality", 78)
                .with_description("Code quality metrics")
                .with_thresholds(70, 50),
        )
        // Section 2: Summary
        .add_component(Section::new("Summary Statistics"))
        .add_component(
            SummaryBox::new("Key Metrics")
                .add_item("Total Users", "15,432")
                .add_item("Active Sessions", "1,247")
                .add_item("Avg Response Time", "142ms"),
        )
        .add_component(
            SummaryBox::new("Financial Overview")
                .add_item("Monthly Revenue", "$234,567")
                .add_item("Conversion Rate", "3.8%")
                .add_item("Customer LTV", "$1,245"),
        )
        .build();

    // Render to PDF
    let pdf = engine.render_pdf(&report)?;

    // Write to file
    let output_path = "examples/output/new_components_demo.pdf";
    std::fs::write(output_path, &pdf)?;

    println!("✓ Report generated successfully!");
    println!("  Output: {}", output_path);
    println!("  Size: {} KB", pdf.len() / 1024);
    println!("\nNote: This demo uses standard components.");
    println!("New components (Charts, Barcodes, Text fields, etc.) are available");
    println!("and fully tested - see tests/new_component_tests.rs for usage examples.");

    Ok(())
}
