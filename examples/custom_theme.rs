//! Custom Theme Example
//!
//! Shows how to create and apply custom themes.
//!
//! Run with: cargo run --example custom_theme

use renderreport::prelude::*;
use renderreport::theme::{Theme, TokenValue};

fn main() -> renderreport::Result<()> {
    let engine = Engine::new()?;

    // Create a corporate theme
    let mut corporate_theme = Theme::new("corporate", "Corporate Blue");
    corporate_theme
        .tokens
        .set("color.primary", TokenValue::Color("#003366".into()));
    corporate_theme
        .tokens
        .set("color.secondary", TokenValue::Color("#336699".into()));
    corporate_theme
        .tokens
        .set("color.text", TokenValue::Color("#333333".into()));
    corporate_theme
        .tokens
        .set("color.ok", TokenValue::Color("#006633".into()));
    corporate_theme
        .tokens
        .set("color.warn", TokenValue::Color("#cc6600".into()));
    corporate_theme
        .tokens
        .set("color.bad", TokenValue::Color("#cc0000".into()));
    corporate_theme
        .tokens
        .set("font.heading", TokenValue::Font("Helvetica".into()));
    corporate_theme
        .tokens
        .set("font.body", TokenValue::Font("Helvetica".into()));

    // Create a dark theme variant
    let mut dark_theme = Theme::new("dark", "Dark Mode");
    dark_theme
        .tokens
        .set("color.background", TokenValue::Color("#1a1a2e".into()));
    dark_theme
        .tokens
        .set("color.surface", TokenValue::Color("#16213e".into()));
    dark_theme
        .tokens
        .set("color.text", TokenValue::Color("#eaeaea".into()));
    dark_theme
        .tokens
        .set("color.text-muted", TokenValue::Color("#a0a0a0".into()));
    dark_theme
        .tokens
        .set("color.border", TokenValue::Color("#0f3460".into()));
    dark_theme
        .tokens
        .set("color.primary", TokenValue::Color("#e94560".into()));

    // Build report with corporate theme
    let report = engine
        .report("default")
        .title("Quarterly Report")
        .subtitle("Q1 2025")
        .theme(corporate_theme)
        .add_component(
            SummaryBox::new("Key Metrics")
                .add_item_with_status("Revenue", "$1.2M", ScoreStatus::Good)
                .add_item_with_status("Growth", "+15%", ScoreStatus::Good)
                .add_item_with_status("Churn", "2.1%", ScoreStatus::Warning),
        )
        .add_component(ScoreCard::new("Customer Satisfaction", 89))
        .add_component(Callout::info("All targets met for this quarter."))
        .build();

    let pdf = engine.render_pdf(&report)?;
    std::fs::write("examples/output/corporate_report.pdf", &pdf)?;
    println!("Created corporate_report.pdf");

    // Build report with dark theme
    let dark_report = engine
        .report("default")
        .title("Dark Mode Demo")
        .theme(dark_theme)
        .add_component(ScoreCard::new("Score", 88))
        .add_component(Finding::new(
            "Sample Finding",
            Severity::Info,
            "This demonstrates the dark theme colors.",
        ))
        .build();

    let dark_pdf = engine.render_pdf(&dark_report)?;
    std::fs::write("examples/output/dark_report.pdf", &dark_pdf)?;
    println!("Created dark_report.pdf");

    Ok(())
}
