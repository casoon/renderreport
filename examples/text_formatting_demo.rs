//! Text Formatting Demo - Advanced Text and Number Field Examples
//!
//! This example demonstrates text formatting components:
//! - Label: Simple styled text for headings and captions
//! - Text: Multi-line text blocks with formatting
//! - NumberField: Formatted numbers (currency, percentages, custom)
//! - DateField: Formatted dates (European, US, ISO, custom)
//! - ResourceField: Localized strings for i18n support
//!
//! Inspired by Eclipse BIRT and Pentaho Text Field elements
//!
//! Use cases:
//! - Financial reports with currency formatting
//! - Internationalized reports with multiple locales
//! - Formatted data display and typography
//!
//! Run with: cargo run --example text_formatting_demo

use renderreport::components::text::{DateField, Label, NumberField, ResourceField, TextBlock};
use renderreport::components::{Component, Section};
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    println!("Generating text formatting demonstration report...");

    let engine = Engine::new()?;

    // Helper function to add components
    let add_comp = |component: &dyn Component| -> serde_json::Value {
        serde_json::json!({
            "type": component.component_id(),
            "data": component.to_data()
        })
    };

    let report = engine
        .report("default")
        .title("Text Formatting Showcase")
        .subtitle("Labels, Text Blocks, Numbers, and Dates")
        // ============================================
        // SECTION 1: Label Components
        // ============================================
        .add_component(Section::new("Label Components - Simple Text Styling"))
        .add_raw_component(add_comp(&TextBlock::new(
            "Labels are perfect for headings, captions, and short text that needs styling. \
             They support font size, weight, color, and alignment.",
        )))
        .add_raw_component(add_comp(&Label::new("Default Label - 10pt regular text")))
        .add_raw_component(add_comp(&Label::new("Large Heading").with_size("16pt").bold()))
        .add_raw_component(add_comp(&Label::new("Medium Subheading").with_size("14pt").bold()))
        .add_raw_component(add_comp(&Label::new("Small Caption").with_size("9pt")))
        .add_raw_component(add_comp(
            &Label::new("Bold Important Text")
                .bold()
                .with_color("#dc2626")
                .with_size("12pt"),
        ))
        .add_raw_component(add_comp(
            &Label::new("Centered Title")
                .with_size("14pt")
                .bold()
                .center(),
        ))
        .add_raw_component(add_comp(
            &Label::new("Success Message")
                .with_color("#16a34a")
                .with_size("11pt"),
        ))
        .add_raw_component(add_comp(
            &Label::new("Warning Text")
                .with_color("#ea580c")
                .with_size("11pt"),
        ))
        // ============================================
        // SECTION 2: Text Blocks
        // ============================================
        .add_component(Section::new("Text Blocks - Multi-line Content"))
        .add_raw_component(add_comp(&TextBlock::new(
            "Text components are designed for longer, multi-line content like paragraphs, \
             descriptions, and documentation. They support line height control and maximum width \
             constraints for optimal readability.",
        )))
        .add_raw_component(add_comp(&Label::new("Executive Summary:").bold().with_size("12pt")))
        .add_raw_component(add_comp(&TextBlock::new(
            "Our Q4 2024 results exceeded expectations across all key metrics. Revenue grew 23% \
             year-over-year, reaching $12.4M, while customer acquisition increased by 34%. The \
             successful launch of our Enterprise tier contributed significantly to this growth, \
             with 145 new enterprise customers onboarded during the quarter.",
        ).with_size("10pt").with_line_height("1.5")))
        .add_raw_component(add_comp(&Label::new("Technical Details:").bold().with_size("12pt")))
        .add_raw_component(add_comp(&TextBlock::new(
            "The system architecture has been optimized for high availability and scalability. \
             We implemented a microservices-based approach using Kubernetes for orchestration, \
             Redis for caching, and PostgreSQL for persistent storage. Response times improved \
             by 45% compared to the previous quarter.",
        ).with_size("9pt").with_max_width("400pt")))
        .add_raw_component(add_comp(&Label::new("Conclusion:").bold().with_size("12pt")))
        .add_raw_component(add_comp(&TextBlock::new(
            "Looking ahead to 2025, we remain focused on innovation and customer success. Our \
             product roadmap includes several exciting features that address customer feedback \
             and market demands.",
        )))
        // ============================================
        // SECTION 3: Currency Formatting
        // ============================================
        .add_component(Section::new("Currency Formatting"))
        .add_raw_component(add_comp(&TextBlock::new(
            "NumberField components provide sophisticated number formatting including currency, \
             percentages, and custom patterns. Perfect for financial reports and dashboards.",
        )))
        .add_raw_component(add_comp(&Label::new("US Dollar Amounts:").bold()))
        .add_raw_component(add_comp(&Label::new("Quarterly Revenue: ")))
        .add_raw_component(add_comp(&NumberField::currency(12456789.45, "$")))
        .add_raw_component(add_comp(&Label::new("Operating Expenses: ")))
        .add_raw_component(add_comp(&NumberField::currency(8234567.12, "$")))
        .add_raw_component(add_comp(&Label::new("Net Profit: ")))
        .add_raw_component(add_comp(&NumberField::currency(4222222.33, "$")))
        .add_raw_component(add_comp(&Label::new("Euro Amounts:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Label::new("European Sales: ")))
        .add_raw_component(add_comp(&NumberField::currency(3456789.00, "€")))
        .add_raw_component(add_comp(&Label::new("British Pound:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Label::new("UK Revenue: ")))
        .add_raw_component(add_comp(&NumberField::currency(2345678.50, "£")))
        .add_raw_component(add_comp(&Label::new("Japanese Yen:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Label::new("Japan Sales: ")))
        .add_raw_component(add_comp(&NumberField::currency(456789000.0, "¥")))
        // ============================================
        // SECTION 4: Percentage Formatting
        // ============================================
        .add_component(Section::new("Percentage Formatting"))
        .add_raw_component(add_comp(&TextBlock::new(
            "Percentage values are commonly used in reports for growth rates, market share, \
             conversion rates, and performance metrics.",
        )))
        .add_raw_component(add_comp(&Label::new("Performance Metrics:").bold()))
        .add_raw_component(add_comp(&Label::new("Year-over-Year Growth: ")))
        .add_raw_component(add_comp(&NumberField::percentage(23.5)))
        .add_raw_component(add_comp(&Label::new("Market Share: ")))
        .add_raw_component(add_comp(&NumberField::percentage(34.2)))
        .add_raw_component(add_comp(&Label::new("Customer Retention Rate: ")))
        .add_raw_component(add_comp(&NumberField::percentage(94.8)))
        .add_raw_component(add_comp(&Label::new("Conversion Rate: ")))
        .add_raw_component(add_comp(&NumberField::percentage(3.7)))
        .add_raw_component(add_comp(&Label::new("Employee Satisfaction: ")))
        .add_raw_component(add_comp(&NumberField::percentage(87.6)))
        .add_raw_component(add_comp(&Label::new("System Uptime: ")))
        .add_raw_component(add_comp(&NumberField::percentage(99.97)))
        // ============================================
        // SECTION 5: Custom Number Formatting
        // ============================================
        .add_component(Section::new("Custom Number Formatting"))
        .add_raw_component(add_comp(&Label::new("Custom Formats:").bold()))
        .add_raw_component(add_comp(&Label::new("Active Users: ")))
        .add_raw_component(add_comp(
            &NumberField::new(1234567.0)
                .with_format("#,###")
                .with_suffix(" users"),
        ))
        .add_raw_component(add_comp(&Label::new("API Requests: ")))
        .add_raw_component(add_comp(
            &NumberField::new(98765432.0)
                .with_format("#,###")
                .with_suffix(" req/day"),
        ))
        .add_raw_component(add_comp(&Label::new("Database Size: ")))
        .add_raw_component(add_comp(
            &NumberField::new(2.5)
                .with_format("0.0")
                .with_suffix(" TB"),
        ))
        .add_raw_component(add_comp(&Label::new("Response Time: ")))
        .add_raw_component(add_comp(
            &NumberField::new(142.5)
                .with_format("0.0")
                .with_suffix(" ms"),
        ))
        .add_raw_component(add_comp(&Label::new("Memory Usage: ")))
        .add_raw_component(add_comp(
            &NumberField::new(7.8)
                .with_format("0.0")
                .with_suffix(" GB"),
        ))
        // ============================================
        // SECTION 6: Date Formatting
        // ============================================
        .add_component(Section::new("Date Formatting"))
        .add_raw_component(add_comp(&TextBlock::new(
            "DateField components support multiple date formats for international audiences. \
             Common formats include ISO 8601, European (DD.MM.YYYY), and US (MM/DD/YYYY) styles.",
        )))
        .add_raw_component(add_comp(&Label::new("ISO 8601 Format (YYYY-MM-DD):").bold()))
        .add_raw_component(add_comp(&Label::new("Report Date: ")))
        .add_raw_component(add_comp(&DateField::new("2024-03-15")))
        .add_raw_component(add_comp(&Label::new("Period End: ")))
        .add_raw_component(add_comp(&DateField::new("2024-12-31")))
        .add_raw_component(add_comp(&Label::new("European Format (DD.MM.YYYY):").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Label::new("Invoice Date: ")))
        .add_raw_component(add_comp(&DateField::european("2024-03-15")))
        .add_raw_component(add_comp(&Label::new("Due Date: ")))
        .add_raw_component(add_comp(&DateField::european("2024-04-15")))
        .add_raw_component(add_comp(&Label::new("US Format (MM/DD/YYYY):").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Label::new("Contract Start: ")))
        .add_raw_component(add_comp(&DateField::us("2024-01-01")))
        .add_raw_component(add_comp(&Label::new("Contract End: ")))
        .add_raw_component(add_comp(&DateField::us("2024-12-31")))
        // ============================================
        // SECTION 7: Date Examples
        // ============================================
        .add_component(Section::new("Date Format Examples"))
        .add_raw_component(add_comp(&Label::new("Key Dates in Different Formats:").bold()))
        .add_raw_component(add_comp(&Label::new("Company Founded: ")))
        .add_raw_component(add_comp(&DateField::new("2020-06-15")))
        .add_raw_component(add_comp(&Label::new("  (European): ")))
        .add_raw_component(add_comp(&DateField::european("2020-06-15")))
        .add_raw_component(add_comp(&Label::new("  (US): ")))
        .add_raw_component(add_comp(&DateField::us("2020-06-15")))
        .add_raw_component(add_comp(&Label::new("Product Launch: ")))
        .add_raw_component(add_comp(&DateField::new("2022-11-03")))
        .add_raw_component(add_comp(&Label::new("  (European): ")))
        .add_raw_component(add_comp(&DateField::european("2022-11-03")))
        .add_raw_component(add_comp(&Label::new("  (US): ")))
        .add_raw_component(add_comp(&DateField::us("2022-11-03")))
        // ============================================
        // SECTION 8: Resource Fields (i18n)
        // ============================================
        .add_component(Section::new("Resource Fields - Internationalization"))
        .add_raw_component(add_comp(&TextBlock::new(
            "ResourceField components enable internationalization (i18n) by referencing localized \
             strings. This allows reports to be generated in multiple languages using resource bundles.",
        )))
        .add_raw_component(add_comp(&Label::new("English Resources:").bold()))
        .add_raw_component(add_comp(
            &ResourceField::new("app.title")
                .with_default("Annual Report 2024")
                .with_locale("en"),
        ))
        .add_raw_component(add_comp(
            &ResourceField::new("app.subtitle")
                .with_default("Financial Performance Overview")
                .with_locale("en"),
        ))
        .add_raw_component(add_comp(&Label::new("German Resources:").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &ResourceField::new("app.title")
                .with_default("Jahresbericht 2024")
                .with_locale("de"),
        ))
        .add_raw_component(add_comp(
            &ResourceField::new("app.subtitle")
                .with_default("Finanzielle Leistungsübersicht")
                .with_locale("de"),
        ))
        .add_raw_component(add_comp(&Label::new("French Resources:").bold().with_size("11pt")))
        .add_raw_component(add_comp(
            &ResourceField::new("app.title")
                .with_default("Rapport Annuel 2024")
                .with_locale("fr"),
        ))
        .add_raw_component(add_comp(
            &ResourceField::new("app.subtitle")
                .with_default("Aperçu de la Performance Financière")
                .with_locale("fr"),
        ))
        // ============================================
        // SECTION 9: Combined Example
        // ============================================
        .add_component(Section::new("Combined Example - Financial Summary"))
        .add_raw_component(add_comp(&Label::new("Quarterly Financial Report").bold().with_size("14pt").center()))
        .add_raw_component(add_comp(&Label::new("Q4 2024 Results").with_size("12pt").center()))
        .add_raw_component(add_comp(&Label::new("Report Date: ")))
        .add_raw_component(add_comp(&DateField::new("2024-12-31")))
        .add_raw_component(add_comp(&Label::new("Revenue Performance:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Label::new("Total Revenue: ")))
        .add_raw_component(add_comp(&NumberField::currency(12456789.45, "$")))
        .add_raw_component(add_comp(&Label::new("Growth Rate: ")))
        .add_raw_component(add_comp(&NumberField::percentage(23.5)))
        .add_raw_component(add_comp(&Label::new("Customer Metrics:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&Label::new("Active Customers: ")))
        .add_raw_component(add_comp(&NumberField::new(45678.0).with_format("#,###")))
        .add_raw_component(add_comp(&Label::new("Retention Rate: ")))
        .add_raw_component(add_comp(&NumberField::percentage(94.8)))
        .add_raw_component(add_comp(&Label::new("Summary:").bold().with_size("11pt")))
        .add_raw_component(add_comp(&TextBlock::new(
            "Strong performance across all metrics. Revenue exceeded targets by 12%, driven by \
             enterprise customer growth and successful product launches.",
        ).with_size("10pt")))
        .build();

    // Render to PDF
    let pdf = engine.render_pdf(&report)?;

    // Write to file
    let output_path = "examples/output/text_formatting_demo.pdf";
    std::fs::write(output_path, &pdf)?;

    println!("✓ Text formatting demonstration report generated successfully!");
    println!("  Output: {}", output_path);
    println!("  Size: {} KB", pdf.len() / 1024);
    println!("\nText formatting components demonstrated:");
    println!("  ✓ Label - Styled text with size, weight, color, alignment");
    println!("  ✓ Text - Multi-line blocks with line height control");
    println!("  ✓ NumberField - Currency ($, €, £, ¥)");
    println!("  ✓ NumberField - Percentages (growth, metrics)");
    println!("  ✓ NumberField - Custom formats (users, requests, sizes)");
    println!("  ✓ DateField - ISO 8601 format (YYYY-MM-DD)");
    println!("  ✓ DateField - European format (DD.MM.YYYY)");
    println!("  ✓ DateField - US format (MM/DD/YYYY)");
    println!("  ✓ ResourceField - Internationalization (en, de, fr)");

    Ok(())
}
