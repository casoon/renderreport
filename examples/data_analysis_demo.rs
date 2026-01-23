//! Data Analysis Demo - Crosstab and Pivot Table Examples
//!
//! This example demonstrates data analysis components:
//! - PivotTable: Pre-aggregated pivot display for summary reports
//! - Crosstab: Dynamic pivot table with row/column aggregation
//!
//! Inspired by JasperReports Crosstab and Eclipse BIRT Cross Tab components
//!
//! Use cases:
//! - Sales analysis by region and product
//! - Performance metrics across multiple dimensions
//! - Financial reporting and data summarization
//!
//! Run with: cargo run --example data_analysis_demo

use renderreport::components::crosstab::{Crosstab, PivotTable};
use renderreport::components::text::{Label, Text};
use renderreport::components::{Component, Section};
use renderreport::prelude::*;
use std::collections::HashMap;

fn main() -> renderreport::Result<()> {
    println!("Generating data analysis demonstration report...");

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
        .title("Data Analysis Showcase")
        .subtitle("Pivot Tables and Crosstab Reports")
        // ============================================
        // SECTION 1: Introduction
        // ============================================
        .add_component(Section::new("Introduction to Data Analysis Components"))
        .add_raw_component(add_comp(&Text::new(
            "This report demonstrates powerful data analysis components inspired by enterprise \
             reporting tools like JasperReports and Eclipse BIRT. These components allow you to \
             create sophisticated pivot tables and crosstab reports for multi-dimensional data analysis.",
        )))
        // ============================================
        // SECTION 2: Sales by Region and Product
        // ============================================
        .add_component(Section::new("Sales Analysis - Regional Performance"))
        .add_raw_component(add_comp(&Label::new(
            "Quarterly sales broken down by region and product category. \
             This pivot table shows how different products perform across geographic regions.",
        )))
        .add_raw_component(add_comp(
            &PivotTable::new(
                vec![
                    "North America".into(),
                    "Europe".into(),
                    "Asia Pacific".into(),
                    "Latin America".into(),
                    "Africa".into(),
                ],
                vec![
                    "Electronics".into(),
                    "Software".into(),
                    "Services".into(),
                    "Hardware".into(),
                ],
                vec![
                    vec![
                        "$2.4M".into(),
                        "$1.8M".into(),
                        "$3.2M".into(),
                        "$1.5M".into(),
                    ],
                    vec![
                        "$1.9M".into(),
                        "$2.1M".into(),
                        "$2.8M".into(),
                        "$1.2M".into(),
                    ],
                    vec![
                        "$3.1M".into(),
                        "$1.5M".into(),
                        "$2.6M".into(),
                        "$2.0M".into(),
                    ],
                    vec![
                        "$0.8M".into(),
                        "$0.6M".into(),
                        "$1.2M".into(),
                        "$0.4M".into(),
                    ],
                    vec![
                        "$0.5M".into(),
                        "$0.3M".into(),
                        "$0.7M".into(),
                        "$0.2M".into(),
                    ],
                ],
            )
            .with_title("Q4 2024 Sales by Region and Product ($)"),
        ))
        // ============================================
        // SECTION 3: Customer Metrics
        // ============================================
        .add_component(Section::new("Customer Analytics"))
        .add_raw_component(add_comp(&Label::new(
            "Customer acquisition and retention metrics across different channels and time periods.",
        )))
        .add_raw_component(add_comp(
            &PivotTable::new(
                vec![
                    "Direct Sales".into(),
                    "Online Marketing".into(),
                    "Partner Channel".into(),
                    "Referral Program".into(),
                ],
                vec!["Q1 2024".into(), "Q2 2024".into(), "Q3 2024".into(), "Q4 2024".into()],
                vec![
                    vec!["245".into(), "278".into(), "312".into(), "345".into()],
                    vec!["892".into(), "1,045".into(), "1,234".into(), "1,456".into()],
                    vec!["156".into(), "189".into(), "223".into(), "267".into()],
                    vec!["423".into(), "512".into(), "634".into(), "789".into()],
                ],
            )
            .with_title("New Customer Acquisitions by Channel"),
        ))
        // ============================================
        // SECTION 4: Product Performance Matrix
        // ============================================
        .add_component(Section::new("Product Performance Matrix"))
        .add_raw_component(add_comp(&Label::new(
            "Comprehensive product performance metrics showing units sold across different market segments.",
        )))
        .add_raw_component(add_comp(
            &PivotTable::new(
                vec![
                    "Enterprise Suite".into(),
                    "Professional Plan".into(),
                    "Standard Edition".into(),
                    "Starter Package".into(),
                    "Free Tier".into(),
                ],
                vec![
                    "Enterprise".into(),
                    "SMB".into(),
                    "Startup".into(),
                    "Individual".into(),
                ],
                vec![
                    vec!["456".into(), "123".into(), "45".into(), "12".into()],
                    vec!["234".into(), "567".into(), "234".into(), "89".into()],
                    vec!["89".into(), "345".into(), "678".into(), "234".into()],
                    vec!["23".into(), "156".into(), "456".into(), "567".into()],
                    vec!["12".into(), "45".into(), "234".into(), "1,234".into()],
                ],
            )
            .with_title("Units Sold by Product and Customer Segment"),
        ))
        // ============================================
        // SECTION 5: Revenue by Sales Rep
        // ============================================
        .add_component(Section::new("Sales Team Performance"))
        .add_raw_component(add_comp(&Label::new(
            "Individual sales representative performance across different product lines.",
        )))
        .add_raw_component(add_comp(
            &PivotTable::new(
                vec![
                    "Sarah Johnson".into(),
                    "Michael Chen".into(),
                    "Emily Rodriguez".into(),
                    "David Kim".into(),
                    "Lisa Anderson".into(),
                ],
                vec![
                    "Cloud Services".into(),
                    "On-Premise".into(),
                    "Consulting".into(),
                    "Support".into(),
                ],
                vec![
                    vec!["$456K".into(), "$234K".into(), "$567K".into(), "$123K".into()],
                    vec!["$523K".into(), "$312K".into(), "$445K".into(), "$156K".into()],
                    vec!["$389K".into(), "$278K".into(), "$623K".into(), "$189K".into()],
                    vec!["$412K".into(), "$245K".into(), "$534K".into(), "$145K".into()],
                    vec!["$478K".into(), "$298K".into(), "$589K".into(), "$167K".into()],
                ],
            )
            .with_title("Sales Representative Revenue by Product Line"),
        ))
        // ============================================
        // SECTION 6: Monthly Revenue Trends
        // ============================================
        .add_component(Section::new("Monthly Revenue Analysis"))
        .add_raw_component(add_comp(&Label::new(
            "Month-over-month revenue comparison across different business units.",
        )))
        .add_raw_component(add_comp(
            &PivotTable::new(
                vec![
                    "January".into(),
                    "February".into(),
                    "March".into(),
                    "April".into(),
                    "May".into(),
                    "June".into(),
                ],
                vec![
                    "Product Sales".into(),
                    "Services".into(),
                    "Subscriptions".into(),
                    "Licenses".into(),
                ],
                vec![
                    vec!["$1.2M".into(), "$0.8M".into(), "$2.1M".into(), "$0.5M".into()],
                    vec!["$1.3M".into(), "$0.9M".into(), "$2.3M".into(), "$0.6M".into()],
                    vec!["$1.5M".into(), "$1.0M".into(), "$2.5M".into(), "$0.7M".into()],
                    vec!["$1.4M".into(), "$1.1M".into(), "$2.6M".into(), "$0.7M".into()],
                    vec!["$1.6M".into(), "$1.2M".into(), "$2.8M".into(), "$0.8M".into()],
                    vec!["$1.7M".into(), "$1.3M".into(), "$3.0M".into(), "$0.9M".into()],
                ],
            )
            .with_title("First Half 2024 Revenue Breakdown"),
        ))
        // ============================================
        // SECTION 7: Support Ticket Analysis
        // ============================================
        .add_component(Section::new("Customer Support Metrics"))
        .add_raw_component(add_comp(&Label::new(
            "Support ticket volume and resolution metrics by priority and product category.",
        )))
        .add_raw_component(add_comp(
            &PivotTable::new(
                vec![
                    "Critical".into(),
                    "High".into(),
                    "Medium".into(),
                    "Low".into(),
                ],
                vec![
                    "Platform".into(),
                    "API".into(),
                    "Mobile App".into(),
                    "Integration".into(),
                ],
                vec![
                    vec!["12".into(), "8".into(), "15".into(), "6".into()],
                    vec!["45".into(), "34".into(), "52".into(), "23".into()],
                    vec!["123".into(), "89".into(), "145".into(), "67".into()],
                    vec!["234".into(), "178".into(), "267".into(), "112".into()],
                ],
            )
            .with_title("Support Tickets by Priority and Category (Last Month)"),
        ))
        // ============================================
        // SECTION 8: Dynamic Crosstab Example
        // ============================================
        .add_component(Section::new("Dynamic Crosstab - Advanced Aggregation"))
        .add_raw_component(add_comp(&Text::new(
            "Crosstab components allow dynamic data aggregation with row and column totals. \
             Perfect for ad-hoc analysis and reporting where data structure may vary.",
        )))
        // Create sample data for crosstab
        .add_raw_component({
            let mut data_rows = Vec::new();

            // Sample sales data
            let regions = vec!["North", "South", "East", "West"];
            let products = vec!["Widget A", "Widget B", "Widget C"];
            let sales = vec![
                vec![15000, 12000, 18000],
                vec![22000, 19000, 24000],
                vec![16000, 14000, 21000],
                vec![19000, 17000, 23000],
            ];

            for (i, region) in regions.iter().enumerate() {
                for (j, product) in products.iter().enumerate() {
                    let mut row = HashMap::new();
                    row.insert("region".into(), serde_json::json!(region));
                    row.insert("product".into(), serde_json::json!(product));
                    row.insert("sales".into(), serde_json::json!(sales[i][j]));
                    data_rows.push(row);
                }
            }

            add_comp(
                &Crosstab::new("region", "product", "sales")
                    .with_title("Sales by Region and Product (Dynamic Aggregation)")
                    .with_data(data_rows)
                    .with_aggregation("sum"),
            )
        })
        // ============================================
        // SECTION 9: Market Share Analysis
        // ============================================
        .add_component(Section::new("Market Share Distribution"))
        .add_raw_component(add_comp(&Label::new(
            "Competitive market share analysis across different geographic regions.",
        )))
        .add_raw_component(add_comp(
            &PivotTable::new(
                vec![
                    "Our Company".into(),
                    "Competitor A".into(),
                    "Competitor B".into(),
                    "Competitor C".into(),
                    "Others".into(),
                ],
                vec![
                    "North America".into(),
                    "Europe".into(),
                    "Asia".into(),
                    "Other".into(),
                ],
                vec![
                    vec!["34.2%".into(), "28.5%".into(), "22.3%".into(), "15.0%".into()],
                    vec!["28.5%".into(), "32.1%".into(), "25.8%".into(), "13.6%".into()],
                    vec!["22.3%".into(), "24.7%".into(), "31.2%".into(), "21.8%".into()],
                    vec!["10.5%".into(), "9.8%".into(), "15.4%".into(), "24.3%".into()],
                    vec!["4.5%".into(), "4.9%".into(), "5.3%".into(), "25.3%".into()],
                ],
            )
            .with_title("Market Share by Region (%)"),
        ))
        .build();

    // Render to PDF
    let pdf = engine.render_pdf(&report)?;

    // Write to file
    let output_path = "examples/output/data_analysis_demo.pdf";
    std::fs::write(output_path, &pdf)?;

    println!("✓ Data analysis demonstration report generated successfully!");
    println!("  Output: {}", output_path);
    println!("  Size: {} KB", pdf.len() / 1024);
    println!("\nData analysis components demonstrated:");
    println!("  ✓ PivotTable - Pre-aggregated data display");
    println!("    • Sales by region and product");
    println!("    • Customer acquisition metrics");
    println!("    • Product performance matrix");
    println!("    • Sales team performance");
    println!("    • Monthly revenue trends");
    println!("    • Support ticket analysis");
    println!("    • Market share distribution");
    println!("  ✓ Crosstab - Dynamic aggregation");
    println!("    • Row and column totals");
    println!("    • Grand total calculation");
    println!("    • Sum aggregation function");

    Ok(())
}
