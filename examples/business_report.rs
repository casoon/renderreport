//! Business Report Example
//!
//! A comprehensive quarterly business performance report demonstrating
//! CoverPage, TableOfContents, HeroSummary, CardDashboard, Charts,
//! Gauges, Findings, RoadmapBlock, and ModuleComparison.
//!
//! Run with: cargo run --example business_report

use renderreport::components::advanced::*;
use renderreport::components::charts::{Chart, ChartType, Gauge};
use renderreport::components::Component;
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    println!("Generating Q4 2025 Business Performance Report...");

    let engine = Engine::new()?;

    let report = engine
        .report("default")
        .metadata("author", "Casoon Analytics")
        // ── Cover Page ──────────────────────────────────────────────
        .add_component(
            CoverPage::new("Q4 2025 Business Report", "Casoon GmbH", 82, "B")
                .with_brand("Casoon")
                .with_date("January 2026")
                .with_subtitle("Quarterly Performance Review — Revenue, Operations & Strategy")
                .with_modules(vec![
                    "Revenue".into(),
                    "Operations".into(),
                    "Customer Success".into(),
                    "Engineering".into(),
                ])
                .with_issues(8, 2),
        )
        // ── Table of Contents ───────────────────────────────────────
        .add_component(TableOfContents::new().with_title("Contents").with_depth(2))
        // ── Executive Summary ───────────────────────────────────────
        .add_component(
            HeroSummary::new(82, "B", "Casoon GmbH")
                .with_date("Q4 2025")
                .with_verdict("Strong quarter with 18% YoY revenue growth. Customer retention improved to 94%. Two critical operational issues require immediate attention.")
                .add_metric(HeroMetric { title: "Revenue".into(), value: "€2.4M".into(), accent_color: Some("#22c55e".into()) })
                .add_metric(HeroMetric { title: "Growth YoY".into(), value: "+18%".into(), accent_color: Some("#3b82f6".into()) })
                .add_metric(HeroMetric { title: "Customers".into(), value: "342".into(), accent_color: Some("#8b5cf6".into()) })
                .add_metric(HeroMetric { title: "NPS Score".into(), value: "67".into(), accent_color: Some("#f59e0b".into()) })
                .with_top_actions(vec![
                    "Resolve scaling issues in payment processing".into(),
                    "Hire 3 senior engineers for Q1 roadmap".into(),
                    "Launch enterprise tier by February".into(),
                ])
                .with_positive_aspects(vec![
                    "Revenue exceeded forecast by 12%".into(),
                    "Customer churn reduced from 8% to 6%".into(),
                    "99.95% platform uptime achieved".into(),
                    "Employee satisfaction score: 4.2/5".into(),
                ]),
        )
        // ── Department Dashboard ────────────────────────────────────
        .add_component(Section::new("Department Performance").with_level(1))
        .add_component(
            CardDashboard::new(vec![
                DashboardCard { name: "Sales".into(), score: 88, interpretation: "Exceeded Q4 target by 12%, strong enterprise pipeline".into(), good_threshold: 80, warn_threshold: 60 },
                DashboardCard { name: "Marketing".into(), score: 75, interpretation: "Lead quality improved, CAC slightly above target".into(), good_threshold: 80, warn_threshold: 60 },
                DashboardCard { name: "Engineering".into(), score: 90, interpretation: "All roadmap items delivered, 99.95% uptime".into(), good_threshold: 80, warn_threshold: 60 },
                DashboardCard { name: "Support".into(), score: 72, interpretation: "Response time improved, ticket backlog growing".into(), good_threshold: 80, warn_threshold: 60 },
            ]),
        )
        // ── Financial Overview ──────────────────────────────────────
        .add_component(Section::new("Financial Overview").with_level(1))
        .add_component(
            Chart::bar("Quarterly Revenue (€)")
                .add_series("2024", vec![
                    ("Q1".into(), 480000.0),
                    ("Q2".into(), 520000.0),
                    ("Q3".into(), 560000.0),
                    ("Q4".into(), 610000.0),
                ])
                .add_series("2025", vec![
                    ("Q1".into(), 580000.0),
                    ("Q2".into(), 640000.0),
                    ("Q3".into(), 720000.0),
                    ("Q4".into(), 800000.0),
                ])
                .with_labels("Quarter", "Revenue (€)"),
        )
        .add_component(
            Chart::pie("Revenue by Product Line").add_series("Revenue", vec![
                ("Enterprise Platform".into(), 45.0),
                ("Professional Plan".into(), 28.0),
                ("Starter Plan".into(), 15.0),
                ("Consulting Services".into(), 12.0),
            ]),
        )
        .add_component(
            Chart::new("Monthly Recurring Revenue Trend", ChartType::Area)
                .add_series("MRR", vec![
                    ("Jul".into(), 195000.0),
                    ("Aug".into(), 205000.0),
                    ("Sep".into(), 218000.0),
                    ("Oct".into(), 230000.0),
                    ("Nov".into(), 245000.0),
                    ("Dec".into(), 262000.0),
                ])
                .with_labels("Month", "MRR (€)"),
        )
        // ── KPI Gauges ──────────────────────────────────────────────
        .add_component(Section::new("Key Performance Indicators").with_level(1))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Customer Retention", 94.0).with_range(80.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("NPS Score", 67.0).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Platform Uptime", 99.9).with_range(99.0, 100.0).to_data()
                })),
        )
        .add_component(
            AuditTable::new(vec![
                TableColumn::new("Metric").with_width("30%"),
                TableColumn::new("Q3 2025").with_width("17%"),
                TableColumn::new("Q4 2025").with_width("17%"),
                TableColumn::new("Change").with_width("17%"),
                TableColumn::new("Target").with_width("19%"),
            ])
            .with_title("Quarterly KPI Comparison")
            .add_row(vec!["Revenue", "€720K", "€800K", "+11%", "€750K"])
            .add_row(vec!["New Customers", "28", "35", "+25%", "30"])
            .add_row(vec!["Churn Rate", "7.2%", "6.0%", "-1.2pp", "<7%"])
            .add_row(vec!["Avg. Deal Size", "€18.5K", "€21.2K", "+15%", "€20K"])
            .add_row(vec!["Support CSAT", "4.1/5", "4.3/5", "+0.2", "4.0/5"])
            .add_row(vec!["Deploy Frequency", "8/month", "12/month", "+50%", "10/month"]),
        )
        // ── Key Findings ────────────────────────────────────────────
        .add_component(Section::new("Key Findings").with_level(1))
        .add_component(
            Finding::new("Payment Processing Scaling Issues", Severity::Critical,
                "During Black Friday peak (Nov 29), the payment gateway experienced 45-second timeouts affecting 3.2% of transactions. Estimated revenue loss: €12,400.")
                .with_recommendation("Migrate to horizontally-scaled payment microservice. Implement circuit breaker pattern and retry logic with exponential backoff.")
                .with_category("Infrastructure"),
        )
        .add_component(
            Finding::new("Growing Support Ticket Backlog", Severity::High,
                "Unresolved tickets increased from 45 to 78 (+73%). Average first-response time: 4.2 hours (target: 2 hours). Enterprise customers affected.")
                .with_recommendation("Hire 2 additional support engineers. Implement AI-powered ticket triage to route issues faster. Prioritize enterprise accounts.")
                .with_category("Customer Success"),
        )
        .add_component(
            Finding::new("Enterprise Pipeline Strong", Severity::Info,
                "Q1 2026 pipeline: €1.2M across 8 enterprise prospects. 3 deals in final negotiation stage. Conversion probability: 65%.")
                .with_category("Sales"),
        )
        // ── Score Comparison ────────────────────────────────────────
        .add_component(Section::new("Department Score Comparison").with_level(1))
        .add_component(
            ModuleComparison::new(vec![
                ComparisonModule::new("Sales", 88).with_color("#22c55e"),
                ComparisonModule::new("Marketing", 75).with_color("#3b82f6"),
                ComparisonModule::new("Engineering", 90).with_color("#8b5cf6"),
                ComparisonModule::new("Support", 72).with_color("#f59e0b"),
            ])
            .with_title("Department Scores Q4 2025"),
        )
        // ── Strategic Roadmap ───────────────────────────────────────
        .add_component(Section::new("Q1 2026 Strategic Roadmap").with_level(1))
        .add_component(RoadmapBlock::new(vec![
            RoadmapColumn {
                title: "January".into(),
                accent_color: Some("#ef4444".into()),
                items: vec![
                    ActionItem { action: "Fix payment scaling".into(), role: "Engineering".into(), priority: "Critical".into(), effort: Some("3 weeks".into()), benefit: "Eliminate revenue loss during peaks".into() },
                    ActionItem { action: "Hire 2 support engineers".into(), role: "HR / Support".into(), priority: "High".into(), effort: Some("4 weeks".into()), benefit: "Reduce response time to <2h".into() },
                ],
            },
            RoadmapColumn {
                title: "February".into(),
                accent_color: Some("#f59e0b".into()),
                items: vec![
                    ActionItem { action: "Launch Enterprise Tier".into(), role: "Product / Sales".into(), priority: "High".into(), effort: Some("6 weeks".into()), benefit: "Capture €1.2M pipeline".into() },
                    ActionItem { action: "Implement AI ticket triage".into(), role: "Engineering".into(), priority: "Medium".into(), effort: Some("3 weeks".into()), benefit: "40% faster ticket routing".into() },
                ],
            },
            RoadmapColumn {
                title: "March".into(),
                accent_color: Some("#22c55e".into()),
                items: vec![
                    ActionItem { action: "Marketing automation rollout".into(), role: "Marketing".into(), priority: "Medium".into(), effort: Some("4 weeks".into()), benefit: "Reduce CAC by 20%".into() },
                    ActionItem { action: "SOC 2 compliance audit".into(), role: "Security / Legal".into(), priority: "Medium".into(), effort: Some("8 weeks".into()), benefit: "Enterprise sales enablement".into() },
                ],
            },
        ]))
        // ── Closing ─────────────────────────────────────────────────
        .add_component(
            Callout::info("This report was compiled from data across Jira, HubSpot, Stripe, and Datadog. Next review scheduled for April 7, 2026.")
                .with_title("Data Sources"),
        )
        .build();

    let pdf = engine.render_pdf(&report)?;
    let output_path = "examples/output/business_report.pdf";
    std::fs::write(output_path, &pdf)?;

    println!(
        "✓ Business report generated: {} ({} KB)",
        output_path,
        pdf.len() / 1024
    );
    Ok(())
}
