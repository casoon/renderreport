//! Charts Demo - Comprehensive Chart Components Example
//!
//! This example demonstrates all chart types available in RenderReport:
//! - Bar Charts (single and multi-series)
//! - Line Charts with trend visualization
//! - Pie Charts for distribution analysis
//! - Area Charts for cumulative data
//! - Scatter Charts for correlation analysis
//! - Radar Charts for multi-dimensional comparison
//! - Sparklines for inline trends
//! - Gauges for KPI visualization
//!
//! Inspired by JasperReports Chart Components and Pentaho Reporting
//!
//! Run with: cargo run --example charts_demo

use renderreport::components::advanced::Grid;
use renderreport::components::charts::{Chart, ChartType, Gauge, Sparkline};
use renderreport::components::{Component, Section};
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    println!("Generating comprehensive charts demonstration report...");

    let engine = Engine::new()?;

    let report = engine
        .report("default")
        .title("Data Visualization Showcase")
        .subtitle("Comprehensive Chart Components Demo")
        // ============================================
        // SECTION 1: Bar Charts
        // ============================================
        .add_component(Section::new("Bar Charts"))
        .add_component(
            Chart::bar("Quarterly Revenue 2024")
                .add_series(
                    "Revenue",
                    vec![
                        ("Q1".into(), 245000.0),
                        ("Q2".into(), 278000.0),
                        ("Q3".into(), 312000.0),
                        ("Q4".into(), 345000.0),
                    ],
                )
                .with_labels("Quarter", "Revenue ($)"),
        )
        .add_component(
            Chart::bar("Year-over-Year Comparison")
                .add_series(
                    "2023",
                    vec![
                        ("Q1".into(), 210000.0),
                        ("Q2".into(), 235000.0),
                        ("Q3".into(), 268000.0),
                        ("Q4".into(), 295000.0),
                    ],
                )
                .add_series(
                    "2024",
                    vec![
                        ("Q1".into(), 245000.0),
                        ("Q2".into(), 278000.0),
                        ("Q3".into(), 312000.0),
                        ("Q4".into(), 345000.0),
                    ],
                )
                .with_labels("Quarter", "Revenue ($)"),
        )
        // ============================================
        // SECTION 2: Line Charts
        // ============================================
        .add_component(Section::new("Line Charts"))
        .add_component(
            Chart::line("Monthly Active Users")
                .add_series(
                    "Users",
                    vec![
                        ("Jan".into(), 12450.0),
                        ("Feb".into(), 13200.0),
                        ("Mar".into(), 14800.0),
                        ("Apr".into(), 15600.0),
                        ("May".into(), 17200.0),
                        ("Jun".into(), 18900.0),
                        ("Jul".into(), 19800.0),
                        ("Aug".into(), 21500.0),
                        ("Sep".into(), 22800.0),
                        ("Oct".into(), 24100.0),
                        ("Nov".into(), 25600.0),
                        ("Dec".into(), 27300.0),
                    ],
                )
                .with_labels("Month", "Active Users"),
        )
        .add_component(
            Chart::line("Server Response Time (ms)")
                .add_series(
                    "API Gateway",
                    vec![
                        ("Mon".into(), 145.0),
                        ("Tue".into(), 132.0),
                        ("Wed".into(), 128.0),
                        ("Thu".into(), 155.0),
                        ("Fri".into(), 142.0),
                        ("Sat".into(), 118.0),
                        ("Sun".into(), 112.0),
                    ],
                )
                .add_series(
                    "Database",
                    vec![
                        ("Mon".into(), 85.0),
                        ("Tue".into(), 78.0),
                        ("Wed".into(), 82.0),
                        ("Thu".into(), 95.0),
                        ("Fri".into(), 88.0),
                        ("Sat".into(), 72.0),
                        ("Sun".into(), 68.0),
                    ],
                )
                .with_labels("Day", "Response Time (ms)"),
        )
        // ============================================
        // SECTION 3: Pie Charts
        // ============================================
        .add_component(Section::new("Pie Charts"))
        .add_component(Chart::pie("Market Share by Region").add_series(
            "Share",
            vec![
                ("North America".into(), 35.0),
                ("Europe".into(), 28.0),
                ("Asia Pacific".into(), 22.0),
                ("Latin America".into(), 10.0),
                ("Africa".into(), 5.0),
            ],
        ))
        .add_component(
            Chart::pie("Revenue by Product Category").add_series(
                "Revenue",
                vec![
                    ("Enterprise".into(), 45.0),
                    ("Professional".into(), 30.0),
                    ("Standard".into(), 18.0),
                    ("Starter".into(), 7.0),
                ],
            ),
        )
        // ============================================
        // SECTION 4: Area Charts
        // ============================================
        .add_component(Section::new("Area Charts"))
        .add_component(
            Chart::new("Cumulative User Growth", ChartType::Area)
                .add_series(
                    "Total Users",
                    vec![
                        ("Jan".into(), 10000.0),
                        ("Feb".into(), 12500.0),
                        ("Mar".into(), 15800.0),
                        ("Apr".into(), 19200.0),
                        ("May".into(), 23500.0),
                        ("Jun".into(), 28900.0),
                    ],
                )
                .with_labels("Month", "Users"),
        )
        // ============================================
        // SECTION 5: Scatter Charts
        // ============================================
        .add_component(Section::new("Scatter Charts"))
        .add_component(
            Chart::new(
                "Customer Lifetime Value vs. Acquisition Cost",
                ChartType::Scatter,
            )
            .add_series(
                "Customers",
                vec![
                    ("A".into(), 1200.0),
                    ("B".into(), 1500.0),
                    ("C".into(), 980.0),
                    ("D".into(), 2100.0),
                    ("E".into(), 1750.0),
                    ("F".into(), 1350.0),
                    ("G".into(), 1900.0),
                ],
            )
            .with_labels("Acquisition Cost", "Lifetime Value"),
        )
        // ============================================
        // SECTION 6: Radar Charts
        // ============================================
        .add_component(Section::new("Radar Charts"))
        .add_component(
            Chart::new("Product Feature Comparison", ChartType::Radar)
                .add_series(
                    "Product A",
                    vec![
                        ("Performance".into(), 85.0),
                        ("Usability".into(), 90.0),
                        ("Security".into(), 88.0),
                        ("Scalability".into(), 82.0),
                        ("Cost".into(), 75.0),
                    ],
                )
                .add_series(
                    "Product B",
                    vec![
                        ("Performance".into(), 78.0),
                        ("Usability".into(), 85.0),
                        ("Security".into(), 92.0),
                        ("Scalability".into(), 88.0),
                        ("Cost".into(), 90.0),
                    ],
                ),
        )
        // ============================================
        // SECTION 7: Sparklines (Inline Mini Charts)
        // ============================================
        .add_component(Section::new("Sparklines - Inline Trends"))
        .add_component(Sparkline::line(vec![
            10.0, 12.0, 11.0, 15.0, 14.0, 18.0, 20.0, 19.0, 22.0, 25.0,
        ]))
        .add_component(Sparkline::line(vec![
            100.0, 105.0, 98.0, 110.0, 115.0, 112.0, 120.0, 118.0, 125.0, 130.0,
        ]))
        .add_component(
            Sparkline::bar(vec![5.0, 8.0, 6.0, 12.0, 10.0, 15.0, 14.0, 18.0])
                .with_color("#3b82f6"),
        )
        .add_component(
            Sparkline::bar(vec![20.0, 25.0, 22.0, 28.0, 30.0, 27.0, 32.0, 35.0])
                .with_color("#10b981"),
        )
        // ============================================
        // SECTION 8: Gauges (KPI Meters) - in Grid layout
        // ============================================
        .add_component(Section::new("Gauges - KPI Visualization"))
        // Circular gauges in a 3-column grid
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("System Performance", 87.5).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Customer Satisfaction", 92.3).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Server Load", 45.8).with_range(0.0, 100.0).to_data()
                })),
        )
        // Thermometer gauges in a 2-column grid
        .add_component(
            Grid::new(2)
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::thermometer("CPU Temperature", 68.5).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::thermometer("Memory Usage", 72.3).with_range(0.0, 100.0).to_data()
                })),
        )
        .build();

    // Render to PDF
    let pdf = engine.render_pdf(&report)?;

    // Write to file
    let output_path = "examples/output/charts_demo.pdf";
    std::fs::write(output_path, &pdf)?;

    println!("✓ Charts demonstration report generated successfully!");
    println!("  Output: {}", output_path);
    println!("  Size: {} KB", pdf.len() / 1024);
    println!("\nChart types demonstrated:");
    println!("  ✓ Bar Charts (single and multi-series)");
    println!("  ✓ Line Charts (trend analysis)");
    println!("  ✓ Pie Charts (distribution)");
    println!("  ✓ Area Charts (cumulative growth)");
    println!("  ✓ Scatter Charts (correlation)");
    println!("  ✓ Radar Charts (multi-dimensional comparison)");
    println!("  ✓ Sparklines (inline mini-charts)");
    println!("  ✓ Gauges (KPI meters and thermometers)");

    Ok(())
}
