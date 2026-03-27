//! Custom Theme Example
//!
//! Shows how to create and apply custom themes with full reports.
//!
//! Run with: cargo run --example custom_theme

use renderreport::components::advanced::*;
use renderreport::components::charts::{Chart, Gauge};
use renderreport::components::Component;
use renderreport::prelude::*;
use renderreport::theme::{Theme, TokenValue};

fn main() -> renderreport::Result<()> {
    let engine = Engine::new()?;

    // ── Corporate Blue Theme ────────────────────────────────────
    let mut corporate_theme = Theme::new("corporate", "Corporate Blue");
    corporate_theme.tokens.set("color.primary", TokenValue::Color("#003366".into()));
    corporate_theme.tokens.set("color.secondary", TokenValue::Color("#336699".into()));
    corporate_theme.tokens.set("color.text", TokenValue::Color("#333333".into()));
    corporate_theme.tokens.set("color.ok", TokenValue::Color("#006633".into()));
    corporate_theme.tokens.set("color.warn", TokenValue::Color("#cc6600".into()));
    corporate_theme.tokens.set("color.bad", TokenValue::Color("#cc0000".into()));
    corporate_theme.tokens.set("font.heading", TokenValue::Font("Helvetica".into()));
    corporate_theme.tokens.set("font.body", TokenValue::Font("Helvetica".into()));

    let report = engine
        .report("default")
        .title("Quarterly Business Review")
        .subtitle("Q1 2025 — Corporate Theme Demo")
        .metadata("author", "Finance Department")
        .theme(corporate_theme)
        // Overview
        .add_component(Section::new("Executive Summary").with_level(1))
        .add_component(
            SummaryBox::new("Key Metrics")
                .add_item_with_status("Revenue", "$1.2M", ScoreStatus::Good)
                .add_item_with_status("Growth YoY", "+15%", ScoreStatus::Good)
                .add_item_with_status("Churn Rate", "2.1%", ScoreStatus::Warning)
                .add_item("Active Customers", "842")
                .add_item("NPS Score", "72"),
        )
        // Score Cards in Grid
        .add_component(Section::new("Department Scores").with_level(1))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Customer Satisfaction", 89)
                        .with_description("CSAT survey results").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Employee Engagement", 82)
                        .with_description("Quarterly pulse survey").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Operational Efficiency", 91)
                        .with_description("Process automation index").to_data()
                })),
        )
        // Chart
        .add_component(Section::new("Revenue Trend").with_level(1))
        .add_component(
            Chart::bar("Monthly Revenue ($K)")
                .add_series("2024", vec![
                    ("Jan".into(), 380.0), ("Feb".into(), 395.0), ("Mar".into(), 420.0),
                ])
                .add_series("2025", vec![
                    ("Jan".into(), 410.0), ("Feb".into(), 435.0), ("Mar".into(), 465.0),
                ])
                .with_labels("Month", "Revenue ($K)"),
        )
        .add_component(
            Chart::pie("Revenue by Segment").add_series("Revenue", vec![
                ("Enterprise".into(), 52.0),
                ("Mid-Market".into(), 28.0),
                ("SMB".into(), 15.0),
                ("Consulting".into(), 5.0),
            ]),
        )
        // KPIs
        .add_component(Section::new("Key Performance Indicators").with_level(1))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Target Achievement", 87.0).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Budget Utilization", 78.0).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Pipeline Health", 92.0).with_range(0.0, 100.0).to_data()
                })),
        )
        // Findings
        .add_component(Section::new("Highlights & Risks").with_level(1))
        .add_component(
            Finding::new("APAC Market Expansion Ahead of Schedule", Severity::Info,
                "The APAC team closed 3 enterprise deals in Q1, exceeding the annual target by 40%. Revenue from the region grew 62% YoY.")
                .with_category("Sales"),
        )
        .add_component(
            Finding::new("Rising Customer Acquisition Costs", Severity::Medium,
                "CAC increased 18% QoQ due to competitive pressure in paid channels. Current CAC: $342 (target: $290).")
                .with_recommendation("Shift 20% of paid budget to organic content marketing. Invest in referral program expansion.")
                .with_category("Marketing"),
        )
        .add_component(
            Finding::new("Infrastructure Cost Optimization Needed", Severity::High,
                "Cloud spending exceeded budget by 22% ($48K overage). Primary driver: unoptimized staging environments running 24/7.")
                .with_recommendation("Implement auto-scaling for staging environments. Schedule non-production resources to shut down outside business hours.")
                .with_category("Operations"),
        )
        // Progress
        .add_component(Section::new("OKR Progress").with_level(1))
        .add_component(ProgressBar::new("Revenue Target ($1.4M)", 86.0).with_color("#006633"))
        .add_component(ProgressBar::new("New Customers (50)", 70.0).with_color("#336699"))
        .add_component(ProgressBar::new("Product Launch", 95.0).with_color("#006633"))
        .add_component(ProgressBar::new("Hiring Plan (12 roles)", 58.0).with_color("#cc6600"))
        // Metadata
        .add_component(Divider::new())
        .add_component(
            KeyValueList::new()
                .with_title("Report Details")
                .add("Prepared by", "Finance Department")
                .add("Review Date", "April 5, 2025")
                .add("Distribution", "Executive Team, Board of Directors")
                .add_highlighted("Next Review", "July 2025"),
        )
        .add_component(
            Callout::info("All financial figures are preliminary and subject to final audit review. Corporate theme uses Helvetica and a dark blue color palette.")
                .with_title("Disclaimer"),
        )
        .build();

    let pdf = engine.render_pdf(&report)?;
    std::fs::write("examples/output/corporate_report.pdf", &pdf)?;
    println!("✓ Created corporate_report.pdf ({} KB)", pdf.len() / 1024);

    // ── Dark Theme ──────────────────────────────────────────────
    let mut dark_theme = Theme::new("dark", "Dark Mode");
    dark_theme.tokens.set("color.background", TokenValue::Color("#1a1a2e".into()));
    dark_theme.tokens.set("color.surface", TokenValue::Color("#16213e".into()));
    dark_theme.tokens.set("color.surface-soft", TokenValue::Color("#1a2744".into()));
    dark_theme.tokens.set("color.text", TokenValue::Color("#eaeaea".into()));
    dark_theme.tokens.set("color.text-muted", TokenValue::Color("#a0a0a0".into()));
    dark_theme.tokens.set("color.border", TokenValue::Color("#0f3460".into()));
    dark_theme.tokens.set("color.primary", TokenValue::Color("#e94560".into()));
    dark_theme.tokens.set("color.ok", TokenValue::Color("#06d6a0".into()));
    dark_theme.tokens.set("color.ok-soft", TokenValue::Color("#0a2e23".into()));
    dark_theme.tokens.set("color.warn", TokenValue::Color("#ffd166".into()));
    dark_theme.tokens.set("color.warn-soft", TokenValue::Color("#2e2510".into()));
    dark_theme.tokens.set("color.bad", TokenValue::Color("#ef476f".into()));
    dark_theme.tokens.set("color.bad-soft", TokenValue::Color("#2e101a".into()));
    dark_theme.tokens.set("color.info", TokenValue::Color("#4ea8de".into()));
    dark_theme.tokens.set("color.info-soft", TokenValue::Color("#0f2540".into()));
    dark_theme.tokens.set("color.accent-soft", TokenValue::Color("#1a1a3e".into()));
    // Table colors for dark theme
    dark_theme.tokens.set("table.header-bg", TokenValue::Color("#0f2040".into()));
    dark_theme.tokens.set("table.row-alt-bg", TokenValue::Color("#121d33".into()));
    dark_theme.tokens.set("table.border", TokenValue::Color("#1a3055".into()));

    let dark_report = engine
        .report("default")
        .title("System Monitoring Dashboard")
        .subtitle("Infrastructure Health Report — Dark Theme")
        .metadata("author", "DevOps Team")
        .theme(dark_theme)
        // Status Overview
        .add_component(Section::new("System Status").with_level(1))
        .add_component(
            SummaryBox::new("Infrastructure Overview")
                .add_item_with_status("API Gateway", "Healthy", ScoreStatus::Good)
                .add_item_with_status("Database Cluster", "Degraded", ScoreStatus::Warning)
                .add_item_with_status("Cache Layer", "Healthy", ScoreStatus::Good)
                .add_item_with_status("Queue Service", "Healthy", ScoreStatus::Good)
                .add_item("Uptime (30d)", "99.94%"),
        )
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("API Health", 98)
                        .with_description("Avg response <120ms").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Database", 72)
                        .with_description("Replication lag detected")
                        .with_thresholds(90, 60).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("CDN Hit Rate", 95)
                        .with_description("Global edge caching").to_data()
                })),
        )
        // Performance Charts
        .add_component(Section::new("Performance Metrics").with_level(1))
        .add_component(
            Chart::line("Request Latency (ms)")
                .add_series("P50", vec![
                    ("00:00".into(), 45.0), ("04:00".into(), 38.0), ("08:00".into(), 62.0),
                    ("12:00".into(), 85.0), ("16:00".into(), 78.0), ("20:00".into(), 52.0),
                ])
                .add_series("P99", vec![
                    ("00:00".into(), 120.0), ("04:00".into(), 95.0), ("08:00".into(), 180.0),
                    ("12:00".into(), 245.0), ("16:00".into(), 210.0), ("20:00".into(), 145.0),
                ])
                .with_labels("Time", "Latency (ms)"),
        )
        .add_component(
            Chart::bar("Requests per Service (K)")
                .add_series("Requests", vec![
                    ("Auth".into(), 245.0), ("Users".into(), 180.0), ("Orders".into(), 320.0),
                    ("Search".into(), 410.0), ("Media".into(), 95.0),
                ])
                .with_labels("Service", "Requests (K)"),
        )
        // Gauges
        .add_component(Section::new("Resource Utilization").with_level(1))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("CPU Usage", 62.0).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Memory", 78.0).with_range(0.0, 100.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Disk I/O", 45.0).with_range(0.0, 100.0).to_data()
                })),
        )
        // Incidents
        .add_component(Section::new("Active Incidents").with_level(1))
        .add_component(
            Finding::new("Database Replication Lag", Severity::High,
                "Primary-replica lag increased to 3.2 seconds (threshold: 1s). Read queries on replicas may return stale data. Started 2 hours ago.")
                .with_recommendation("Investigate slow queries on primary. Consider temporarily routing read traffic to primary until lag resolves.")
                .with_category("Database"),
        )
        .add_component(
            Finding::new("Elevated Error Rate on Search Service", Severity::Medium,
                "Error rate at 2.1% (baseline: 0.3%). Correlated with new deployment v2.4.1 rolled out at 14:30 UTC.")
                .with_recommendation("Prepare rollback to v2.4.0 if error rate exceeds 5%. Check Elasticsearch cluster health.")
                .with_category("Search"),
        )
        .add_component(
            Finding::new("SSL Certificate Renewal", Severity::Info,
                "Certificate for api.example.com expires in 14 days. Auto-renewal is configured via Let's Encrypt.")
                .with_category("Security"),
        )
        // Deployment Table
        .add_component(Section::new("Recent Deployments").with_level(1))
        .add_component(
            AuditTable::new(vec![
                TableColumn::new("Service").with_width("20%"),
                TableColumn::new("Version").with_width("15%"),
                TableColumn::new("Deployed").with_width("20%"),
                TableColumn::new("Status").with_width("15%"),
                TableColumn::new("Notes").with_width("30%"),
            ])
            .with_title("Last 24 Hours")
            .add_row(vec!["Search", "v2.4.1", "Today 14:30", "Monitoring", "New ranking algorithm"])
            .add_row(vec!["Auth", "v3.1.0", "Today 09:15", "Stable", "OAuth2 PKCE support"])
            .add_row(vec!["Orders", "v5.8.2", "Yesterday", "Stable", "Hotfix: tax calculation"])
            .add_row(vec!["Media", "v1.12.0", "Yesterday", "Stable", "WebP auto-conversion"])
            .add_row(vec!["Gateway", "v2.0.3", "2 days ago", "Stable", "Rate limit tuning"]),
        )
        .add_component(
            Callout::warning("Database replication lag is being actively monitored. On-call engineer has been notified. Dark theme demonstrates custom color tokens.")
                .with_title("Ops Status"),
        )
        .build();

    let dark_pdf = engine.render_pdf(&dark_report)?;
    std::fs::write("examples/output/dark_report.pdf", &dark_pdf)?;
    println!("✓ Created dark_report.pdf ({} KB)", dark_pdf.len() / 1024);

    Ok(())
}
