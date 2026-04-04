//! Component Catalog
//!
//! A visual reference of every built-in component with realistic example content.
//! Rendered as a stable PDF artifact on every release.
//!
//! Run with: cargo run --example component_catalog

use renderreport::components::advanced::*;
use renderreport::components::charts::{Chart, Gauge, Sparkline};
use renderreport::components::text::{Label, TextBlock};
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    let engine = Engine::new()?;

    let report = engine
        .report("default")
        .title("renderreport — Component Catalog")
        .subtitle("Visual reference of all built-in components")
        .metadata("author", "renderreport")
        .metadata("date", env!("CARGO_PKG_VERSION"))
        .metadata("footer_prefix", "renderreport")
        .metadata("footer_link_url", "https://github.com/casoon/renderreport")
        // ── Cover ────────────────────────────────────────────────────────────
        .add_component(
            CoverPage::new("Component Catalog", "github.com/casoon/renderreport", 100, "A+")
                .with_brand("renderreport")
                .with_date(env!("CARGO_PKG_VERSION")),
        )
        .add_component(PageBreak::new())
        // ── 1. Layout ────────────────────────────────────────────────────────
        .add_component(Section::new("1. Layout").with_level(1))
        // Section
        .add_component(Section::new("Section").with_level(2))
        .add_component(TextBlock::new(
            "Organises content into titled blocks. Supports levels 1–3.",
        ))
        .add_component(Section::new("Level 3 Section").with_level(3))
        // Divider
        .add_component(Section::new("Divider").with_level(2))
        .add_component(TextBlock::new("Horizontal rule."))
        .add_component(Divider::new())
        // Grid
        .add_component(Section::new("Grid").with_level(2))
        .add_component(TextBlock::new(
            "Responsive multi-column layout. item_min_height enforces uniform cell heights.",
        ))
        .add_component(
            Grid::new(3)
                .with_item_min_height("80pt")
                .add_item(serde_json::json!({
                    "type": "metric-card",
                    "data": MetricCard::new("Columns", "3").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "metric-card",
                    "data": MetricCard::new("item_min_height", "80pt").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "metric-card",
                    "data": MetricCard::new("Gap", "16pt").to_data()
                })),
        )
        // FlowGroup
        .add_component(Section::new("FlowGroup").with_level(2))
        .add_component(TextBlock::new(
            "Wraps items with optional soft keep-together: if the group fits within \
             keep_together_if_under, it won't break across pages.",
        ))
        .add_component(
            FlowGroup::new()
                .with_spacing("8pt")
                .with_keep_together_if_under("200pt")
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Group Item A", 88).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Group Item B", 74).to_data()
                })),
        )
        .add_component(PageBreak::new())
        // ── 2. Score & Metrics ───────────────────────────────────────────────
        .add_component(Section::new("2. Score & Metrics").with_level(1))
        // ScoreCard
        .add_component(Section::new("ScoreCard").with_level(2))
        .add_component(TextBlock::new(
            "Score 0–100 with automatic Good / Warn / Bad status. Supports optional height.",
        ))
        .add_component(
            Grid::new(3)
                .with_item_min_height("110pt")
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Accessibility", 94)
                        .with_description("WCAG 2.1 AA compliance")
                        .to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Performance", 67)
                        .with_description("Lighthouse score")
                        .to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "score-card",
                    "data": ScoreCard::new("Security", 41)
                        .with_description("OWASP Top 10")
                        .to_data()
                })),
        )
        // MetricCard
        .add_component(Section::new("MetricCard").with_level(2))
        .add_component(TextBlock::new(
            "Single key/value highlight with optional accent colour and fixed height.",
        ))
        .add_component(
            Grid::new(4)
                .with_item_min_height("90pt")
                .add_item(serde_json::json!({
                    "type": "metric-card",
                    "data": MetricCard::new("Issues", "142")
                        .with_subtitle("total")
                        .to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "metric-card",
                    "data": MetricCard::new("Critical", "8")
                        .with_subtitle("need immediate fix")
                        .with_accent_color("#e53e3e")
                        .to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "metric-card",
                    "data": MetricCard::new("Pages", "24")
                        .with_subtitle("crawled")
                        .to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "metric-card",
                    "data": MetricCard::new("Score", "79 / 100")
                        .with_accent_color("#38a169")
                        .to_data()
                })),
        )
        // Gauge
        .add_component(Section::new("Gauge").with_level(2))
        .add_component(TextBlock::new("Arc-style progress indicator. Value 0–100."))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Overall Score", 79.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Accessibility", 94.0).to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "gauge",
                    "data": Gauge::new("Performance", 52.0).to_data()
                })),
        )
        // ProgressBar
        .add_component(Section::new("ProgressBar").with_level(2))
        .add_component(TextBlock::new(
            "Horizontal bar showing percentage completion.",
        ))
        .add_component(ProgressBar::new("Accessibility", 94.0))
        .add_component(ProgressBar::new("Performance", 67.0))
        .add_component(ProgressBar::new("SEO", 88.0))
        .add_component(ProgressBar::new("Security", 41.0))
        // ScoreBand
        .add_component(Section::new("ScoreBand").with_level(2))
        .add_component(TextBlock::new(
            "Score displayed on a colour-coded band.",
        ))
        .add_component(ScoreBand::new(82).with_label("Overall Score"))
        // ModuleComparison
        .add_component(Section::new("ModuleComparison").with_level(2))
        .add_component(TextBlock::new(
            "Side-by-side score comparison across modules.",
        ))
        .add_component(ModuleComparison::new(vec![
            ComparisonModule::new("Accessibility", 94),
            ComparisonModule::new("Performance", 67),
            ComparisonModule::new("SEO", 88),
            ComparisonModule::new("Security", 41),
        ]))
        // TrendTile
        .add_component(Section::new("TrendTile").with_level(2))
        .add_component(TextBlock::new(
            "Score with direction arrow compared to previous value.",
        ))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({
                    "type": "trend-tile",
                    "data": TrendTile::up("Accessibility", "+7").with_reference("was 87").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "trend-tile",
                    "data": TrendTile::down("Performance", "-4").with_reference("was 71").to_data()
                }))
                .add_item(serde_json::json!({
                    "type": "trend-tile",
                    "data": TrendTile::stable("SEO", "0").to_data()
                })),
        )
        .add_component(PageBreak::new())
        // ── 3. Charts ────────────────────────────────────────────────────────
        .add_component(Section::new("3. Charts").with_level(1))
        // Bar chart
        .add_component(Section::new("Chart — Bar").with_level(2))
        .add_component(TextBlock::new(
            "Supports bar, line, pie and area types. Data via add_series(name, Vec<(label, value)>).",
        ))
        .add_component(Chart::bar("Issues by Category").add_series(
            "Count",
            vec![
                ("Contrast".into(), 34.0),
                ("Alt Text".into(), 28.0),
                ("Focus".into(), 19.0),
                ("Landmarks".into(), 15.0),
                ("Forms".into(), 12.0),
            ],
        ))
        // Line chart
        .add_component(Section::new("Chart — Line").with_level(2))
        .add_component(
            Chart::line("Score Trend")
                .add_series(
                    "Score",
                    vec![
                        ("Jan".into(), 58.0),
                        ("Feb".into(), 63.0),
                        ("Mar".into(), 71.0),
                        ("Apr".into(), 79.0),
                        ("May".into(), 85.0),
                        ("Jun".into(), 91.0),
                    ],
                )
                .add_series(
                    "Target",
                    vec![
                        ("Jan".into(), 70.0),
                        ("Feb".into(), 70.0),
                        ("Mar".into(), 75.0),
                        ("Apr".into(), 80.0),
                        ("May".into(), 85.0),
                        ("Jun".into(), 90.0),
                    ],
                ),
        )
        // Sparkline
        .add_component(Section::new("Sparkline").with_level(2))
        .add_component(TextBlock::new("Compact inline trend chart."))
        .add_component(Sparkline::new(vec![62.0, 65.0, 70.0, 68.0, 74.0, 79.0, 83.0]))
        .add_component(PageBreak::new())
        // ── 4. Findings & Callouts ───────────────────────────────────────────
        .add_component(Section::new("4. Findings & Callouts").with_level(1))
        // Finding
        .add_component(Section::new("Finding").with_level(2))
        .add_component(TextBlock::new(
            "Structured audit finding with severity, description and recommendation.",
        ))
        .add_component(
            Finding::new(
                "Images missing alternative text",
                Severity::Critical,
                "14 <img> elements have no alt attribute. Screen reader users cannot \
                 perceive the image content.",
            )
            .with_recommendation(
                "Add descriptive alt text to every meaningful image. \
                 Use alt=\"\" for decorative images.",
            )
            .with_category("WCAG 1.1.1"),
        )
        .add_component(Finding::new(
            "Colour contrast ratio below 4.5:1",
            Severity::High,
            "Body text on the hero section uses #888888 on white, giving a contrast ratio of 3.5:1.",
        ))
        // Callout
        .add_component(Section::new("Callout").with_level(2))
        .add_component(TextBlock::new(
            "Highlighted note block. Types: info, warning, success, error.",
        ))
        .add_component(Callout::info(
            "All automated findings should be verified manually before remediation.",
        ))
        .add_component(Callout::warning(
            "3 critical issues require immediate attention before the next release.",
        ))
        .add_component(Callout::success(
            "No WCAG 1.3.1 violations detected across all 24 pages.",
        ))
        .add_component(Callout::error(
            "SSL certificate expires in 7 days. Renew immediately.",
        ))
        // SummaryBox
        .add_component(Section::new("SummaryBox").with_level(2))
        .add_component(TextBlock::new(
            "Named key/value pairs with optional status badges.",
        ))
        .add_component(
            SummaryBox::new("Audit Summary")
                .add_item("URL", "https://example.com")
                .add_item("Crawled Pages", "24")
                .add_item("Audit Date", "2026-04-02")
                .add_item_with_status("WCAG Level", "AA", ScoreStatus::Good)
                .add_item_with_status("Certificate", "Valid", ScoreStatus::Good)
                .add_item_with_status("Mobile Friendly", "Issues found", ScoreStatus::Warning),
        )
        // SeverityOverview
        .add_component(Section::new("SeverityOverview").with_level(2))
        .add_component(TextBlock::new("Issue count per severity level."))
        .add_component(SeverityOverview::new(8, 23, 41, 70))
        .add_component(PageBreak::new())
        // ── 5. Tables ────────────────────────────────────────────────────────
        .add_component(Section::new("5. Tables").with_level(1))
        // AuditTable
        .add_component(Section::new("AuditTable").with_level(2))
        .add_component(TextBlock::new(
            "Structured table with typed columns and row data.",
        ))
        .add_component(
            AuditTable::new(vec![
                TableColumn::new("Rule"),
                TableColumn::new("Severity"),
                TableColumn::new("Occurrences"),
                TableColumn::new("WCAG"),
            ])
            .add_row(vec!["Missing alt text", "critical", "14", "1.1.1"])
            .add_row(vec!["Low contrast", "high", "7", "1.4.3"])
            .add_row(vec!["Missing label", "medium", "3", "1.3.1"])
            .add_row(vec!["Skip link missing", "low", "1", "2.4.1"]),
        )
        // BenchmarkTable
        .add_component(Section::new("BenchmarkTable").with_level(2))
        .add_component(TextBlock::new(
            "Comparison table for benchmarking scores across multiple subjects.",
        ))
        .add_component(
            BenchmarkTable::new(vec![
                BenchmarkRow::new(1, "example.com", 79, 94, 8),
                BenchmarkRow::new(2, "competitor-a.com", 65, 71, 14),
                BenchmarkRow::new(3, "competitor-b.com", 88, 91, 2),
                BenchmarkRow::new(4, "industry-avg", 72, 78, 9),
            ])
            .with_title("Benchmark Comparison"),
        )
        .add_component(PageBreak::new())
        // ── 6. Lists & Key-Values ────────────────────────────────────────────
        .add_component(Section::new("6. Lists & Key-Values").with_level(1))
        // KeyValueList
        .add_component(Section::new("KeyValueList").with_level(2))
        .add_component(TextBlock::new(
            "Simple key/value pairs. layout: vertical or grid.",
        ))
        .add_component(
            KeyValueList::new()
                .add("Framework", "React 18")
                .add("Hosting", "Vercel")
                .add("CDN", "Cloudflare")
                .add("CMS", "Contentful")
                .add("Analytics", "Plausible"),
        )
        // List
        .add_component(Section::new("List").with_level(2))
        .add_component(TextBlock::new(
            "Ordered or unordered list. Supports numbered, grid and horizontal layouts.",
        ))
        .add_component(
            List::new()
                .with_title("Quick Wins")
                .add_item("Add alt text to 14 images — 30 min effort")
                .add_item("Increase hero text contrast from 3.5:1 to 4.5:1")
                .add_item("Associate form labels with input IDs")
                .add_item("Add visible skip-to-content link"),
        )
        // ActionRoadmap
        .add_component(Section::new("ActionRoadmap").with_level(2))
        .add_component(TextBlock::new(
            "Three-column remediation plan with prioritised action items.",
        ))
        .add_component(ActionRoadmap::new(vec![
            RoadmapColumn {
                title: "Quick Wins".into(),
                accent_color: Some("#38a169".into()),
                items: vec![
                    RoadmapItem {
                        action: "Add alt text to images".into(),
                        role: "Developer".into(),
                        priority: "high".into(),
                        effort: Some("2h".into()),
                        benefit: "Fixes 14 critical findings".into(),
                    },
                    RoadmapItem {
                        action: "Fix form labels".into(),
                        role: "Developer".into(),
                        priority: "high".into(),
                        effort: Some("1h".into()),
                        benefit: "Resolves 3 medium findings".into(),
                    },
                ],
            },
            RoadmapColumn {
                title: "Short-term".into(),
                accent_color: Some("#d69e2e".into()),
                items: vec![
                    RoadmapItem {
                        action: "Increase colour contrast".into(),
                        role: "Designer".into(),
                        priority: "medium".into(),
                        effort: Some("4h".into()),
                        benefit: "Fixes 7 high findings".into(),
                    },
                ],
            },
            RoadmapColumn {
                title: "Long-term".into(),
                accent_color: None,
                items: vec![
                    RoadmapItem {
                        action: "Redesign navigation for keyboard users".into(),
                        role: "Design + Dev".into(),
                        priority: "low".into(),
                        effort: Some("2 weeks".into()),
                        benefit: "Improves overall keyboard accessibility".into(),
                    },
                ],
            },
        ]))
        .add_component(PageBreak::new())
        // ── 7. Text Components ───────────────────────────────────────────────
        .add_component(Section::new("7. Text Components").with_level(1))
        .add_component(Section::new("Label").with_level(2))
        .add_component(TextBlock::new("Inline label, supports bold, centered, colored variants."))
        .add_component(Label::new("Default label"))
        .add_component(Label::new("Bold label").bold())
        .add_component(Label::new("Centered label").center())
        .add_component(Label::new("Coloured label").with_color("#3182ce"))
        .add_component(Section::new("TextBlock").with_level(2))
        .add_component(TextBlock::new(
            "renderreport is a Rust library for building data-driven PDF reports using \
             Typst as the embedded render engine. Components are declared in Rust, serialised \
             to JSON, and rendered via Typst templates — giving you type safety on the data \
             side and full typographic control on the layout side.",
        ))
        .add_component(Section::new("StatPair").with_level(2))
        .add_component(TextBlock::new("Two statistics shown side by side with labels."))
        .add_component(StatPair::new(
            StatPairEntry::new("Before", "58"),
            StatPairEntry::new("After", "79"),
        ))
        .build();

    let pdf = engine.render_pdf(&report)?;

    let out = "examples/output/component_catalog.pdf";
    std::fs::create_dir_all("examples/output")?;
    std::fs::write(out, &pdf)?;
    println!(
        "Component catalog written to {} ({} KB)",
        out,
        pdf.len() / 1024
    );

    Ok(())
}
