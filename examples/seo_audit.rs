//! SEO Audit Report Example
//!
//! Demonstrates creating a complete SEO audit report using RenderReport.
//!
//! Run with: cargo run --example seo_audit

use renderreport::prelude::*;
use renderreport::theme::{Theme, TokenValue};

fn main() -> renderreport::Result<()> {
    // Initialize the engine
    let engine = Engine::new()?;

    // Create a custom theme (brand colors)
    let mut custom_theme = Theme::new("brand", "Brand Theme");
    custom_theme
        .tokens
        .set("color.primary", TokenValue::Color("#1a56db".into()));
    custom_theme
        .tokens
        .set("color.ok", TokenValue::Color("#059669".into()));

    // Build the report
    let report = engine
        .report("seo-audit")
        .title("SEO Audit Report")
        .subtitle("example.com - January 2025")
        .theme(custom_theme)
        .metadata("url", "https://example.com")
        .metadata("auditor", "Casoon")
        // Executive Summary
        .add_component(SummaryBox::new("Executive Summary")
            .add_item_with_status("Overall Score", "78/100", ScoreStatus::Warning)
            .add_item_with_status("Performance", "85/100", ScoreStatus::Good)
            .add_item_with_status("SEO Score", "72/100", ScoreStatus::Warning)
            .add_item("Critical Issues", "3")
            .add_item("Warnings", "12")
            .add_item("Passed Checks", "45"))
        // Score Cards
        .add_component(Section::new("Lighthouse Scores").with_level(1))
        .add_component(ScoreCard::new("Performance", 85)
            .with_description("Good performance overall"))
        .add_component(ScoreCard::new("Accessibility", 92)
            .with_description("Excellent accessibility"))
        .add_component(ScoreCard::new("Best Practices", 83)
            .with_description("Room for improvement"))
        .add_component(ScoreCard::new("SEO", 72)
            .with_description("Needs attention")
            .with_thresholds(80, 50))
        // Findings Section
        .add_component(Section::new("Critical Findings").with_level(1))
        .add_component(Finding::new(
            "Missing Meta Description",
            Severity::High,
            "The homepage is missing a meta description tag. This affects click-through rates in search results."
        )
            .with_recommendation("Add a compelling meta description between 150-160 characters.")
            .with_affected("https://example.com/")
            .with_category("On-Page SEO"))
        .add_component(Finding::new(
            "Large Contentful Paint (LCP) Too Slow",
            Severity::Medium,
            "LCP is 3.2 seconds, which exceeds the recommended 2.5 second threshold."
        )
            .with_recommendation("Optimize the hero image and consider lazy loading below-fold images.")
            .with_affected("Homepage"))
        .add_component(Finding::new(
            "Missing Alt Text on Images",
            Severity::Medium,
            "15 images are missing alt text attributes."
        )
            .with_recommendation("Add descriptive alt text to all images for better accessibility and SEO.")
            .with_category("Accessibility"))
        // Technical SEO Table
        .add_component(Section::new("Technical SEO Checklist").with_level(1))
        .add_component(
            AuditTable::new(vec![
                TableColumn::new("Check").with_width("40%"),
                TableColumn::new("Status").with_width("15%"),
                TableColumn::new("Details").with_width("45%"),
            ])
            .with_title("Technical SEO Audit")
            .add_row(vec!["robots.txt", "✓ Pass", "Properly configured"])
            .add_row(vec!["XML Sitemap", "✓ Pass", "Found at /sitemap.xml"])
            .add_row(vec!["HTTPS", "✓ Pass", "Valid SSL certificate"])
            .add_row(vec!["Canonical Tags", "⚠ Warning", "Missing on 3 pages"])
            .add_row(vec!["Structured Data", "✗ Fail", "No schema markup found"])
            .add_row(vec!["Mobile Friendly", "✓ Pass", "Responsive design detected"])
            .add_row(vec!["Page Speed", "⚠ Warning", "Could be improved"])
        )
        // Callout
        .add_component(Callout::info(
            "This report was generated automatically. For detailed recommendations, \
             please consult with an SEO specialist."
        ).with_title("Note"))
        .build();

    // Render to PDF
    let pdf_bytes = engine.render_pdf(&report)?;

    // Save the PDF
    std::fs::write("examples/output/seo_audit_report.pdf", &pdf_bytes)?;
    println!(
        "Report saved to seo_audit_report.pdf ({} bytes)",
        pdf_bytes.len()
    );

    Ok(())
}
