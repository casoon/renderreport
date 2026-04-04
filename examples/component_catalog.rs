//! Component Catalog
//!
//! A visual reference of every registered component with realistic example content.
//! This file is the living documentation — a CI test verifies that every component
//! registered in the engine appears here.
//!
//! Run with: cargo run --example component_catalog
//!
//! Stable download URL (latest release):
//! https://github.com/casoon/renderreport/releases/latest/download/component-catalog.pdf

use renderreport::components::advanced::*;
use renderreport::components::barcode::Barcode;
use renderreport::components::charts::{Chart, Gauge, Sparkline};
use renderreport::components::crosstab::{Crosstab, PivotTable};
use renderreport::components::text::{DateField, Eyebrow, Label, NumberField, ResourceField, TextBlock};
use renderreport::prelude::*;
use std::collections::HashMap;

fn main() -> renderreport::Result<()> {
    let engine = Engine::new()?;

    let report = engine
        .report("default")
        .title("renderreport — Component Catalog")
        .subtitle("Visual reference of all registered components")
        .metadata("author", "renderreport")
        .metadata("date", env!("CARGO_PKG_VERSION"))
        .metadata("footer_prefix", "renderreport")
        .metadata("footer_link_url", "https://github.com/casoon/renderreport")
        // ── Cover ────────────────────────────────────────────────────────────
        // @id: cover-page (not rendered in this catalog, but component exists)
        // @id: product-hero
        .add_component(
            ProductHero::new("renderreport", "Type-safe PDF reports from Rust")
                .with_tagline("Declare components as Rust structs. Render to PDF in milliseconds.")
                .with_highlights(vec![
                    "Full type safety — catch errors at compile time".into(),
                    "Theme system — customize every visual token".into(),
                    "Component-driven — build complex layouts from reusable blocks".into(),
                    "Fast rendering — 20-page PDFs in under 500ms".into(),
                    "Open source — MIT licensed, embedded-friendly".into(),
                    "51 built-in components — from metrics to infographics".into(),
                ])
                .with_cta("View on GitHub", "https://github.com/casoon/renderreport"),
        )
        // @id: table-of-contents
        .add_component(TableOfContents::new().with_title("Contents").with_depth(2))
        // @id: page-break
        .add_component(PageBreak::new())

        // ── 1. Layout ────────────────────────────────────────────────────────
        .add_component(Section::new("1. Layout").with_level(1)) // @id: section

        .add_component(Section::new("1.1 Section").with_level(2))
        .add_component(TextBlock::new(
            "Organises content into titled blocks. Supports levels 1–3.",
        )) // @id: textblock
        .add_component(Section::new("Level 3").with_level(3))

        .add_component(Section::new("1.2 SectionHeaderSplit").with_level(2))
        .add_component(TextBlock::new(
            "Two-column header: title on the left, body text on the right.",
        ))
        .add_component(
            // @id: section-header-split
            SectionHeaderSplit::new("Performance Analysis", "Covers load time, Core Web Vitals, and resource budget across all crawled pages.")
                .with_eyebrow("Module Report")
                .with_level(2),
        )

        .add_component(Section::new("1.3 Divider").with_level(2))
        .add_component(TextBlock::new("Horizontal rule."))
        .add_component(Divider::new()) // @id: divider

        .add_component(Section::new("1.4 Grid").with_level(2))
        .add_component(TextBlock::new(
            "Multi-column layout. item_min_height for uniform cell heights.",
        ))
        .add_component(
            Grid::new(3) // @id: grid-component
                .with_item_min_height("80pt")
                .add_item(serde_json::json!({ "type": "metric-card",
                    "data": MetricCard::new("Columns", "3").to_data() }))
                .add_item(serde_json::json!({ "type": "metric-card",
                    "data": MetricCard::new("Min-Height", "80pt").to_data() }))
                .add_item(serde_json::json!({ "type": "metric-card",
                    "data": MetricCard::new("Gap", "16pt").to_data() })),
        )

        .add_component(Section::new("1.5 FlowGroup").with_level(2))
        .add_component(TextBlock::new(
            "Soft keep-together wrapper: stays on one page if it fits within keep_together_if_under.",
        ))
        .add_component(
            FlowGroup::new() // @id: flow-group
                .with_spacing("8pt")
                .with_keep_together_if_under("200pt")
                .add_item(serde_json::json!({ "type": "score-card",
                    "data": ScoreCard::new("Group Item A", 88).to_data() }))
                .add_item(serde_json::json!({ "type": "score-card",
                    "data": ScoreCard::new("Group Item B", 74).to_data() })),
        )

        .add_component(Section::new("1.6 Watermark").with_level(2))
        .add_component(TextBlock::new(
            "Diagonal background text. Built-in presets: draft(), confidential().",
        ))
        .add_component(Watermark::draft()) // @id: watermark
        .add_component(PageBreak::new())

        // ── 2. Score & Metrics ───────────────────────────────────────────────
        .add_component(Section::new("2. Score & Metrics").with_level(1))

        .add_component(Section::new("2.1 ScoreCard").with_level(2))
        .add_component(TextBlock::new(
            "Score 0–100 with automatic Good/Warn/Bad status. Optional fixed height.",
        ))
        .add_component(
            Grid::new(3).with_item_min_height("110pt")
                .add_item(serde_json::json!({ "type": "score-card",
                    "data": ScoreCard::new("Accessibility", 94).with_description("WCAG 2.1 AA").to_data() }))
                .add_item(serde_json::json!({ "type": "score-card",
                    "data": ScoreCard::new("Performance", 67).with_description("Lighthouse").to_data() }))
                .add_item(serde_json::json!({ "type": "score-card",
                    "data": ScoreCard::new("Security", 41).with_description("OWASP Top 10").to_data() })),
        )

        .add_component(Section::new("2.2 MetricCard").with_level(2))
        .add_component(TextBlock::new(
            "Key/value highlight with optional accent colour and fixed height.",
        ))
        .add_component(
            Grid::new(4).with_item_min_height("90pt")
                .add_item(serde_json::json!({ "type": "metric-card",
                    "data": MetricCard::new("Issues", "142").with_subtitle("total").to_data() }))
                .add_item(serde_json::json!({ "type": "metric-card",
                    "data": MetricCard::new("Critical", "8").with_accent_color("#e53e3e").to_data() }))
                .add_item(serde_json::json!({ "type": "metric-card",
                    "data": MetricCard::new("Pages", "24").with_subtitle("crawled").to_data() }))
                .add_item(serde_json::json!({ "type": "metric-card",
                    "data": MetricCard::new("Score", "79/100").with_accent_color("#38a169").to_data() })),
        )

        .add_component(Section::new("2.3 Gauge").with_level(2))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({ "type": "gauge",
                    "data": Gauge::new("Overall", 79.0).to_data() }))
                .add_item(serde_json::json!({ "type": "gauge",
                    "data": Gauge::new("Accessibility", 94.0).to_data() }))
                .add_item(serde_json::json!({ "type": "gauge",
                    "data": Gauge::new("Performance", 52.0).to_data() })),
        )

        .add_component(Section::new("2.4 ProgressBar").with_level(2))
        .add_component(ProgressBar::new("Accessibility", 94.0)) // @id: progress-bar
        .add_component(ProgressBar::new("Performance", 67.0))
        .add_component(ProgressBar::new("Security", 41.0))

        .add_component(Section::new("2.5 ScoreBand").with_level(2))
        .add_component(ScoreBand::new(82).with_label("Overall Score")) // @id: score-band

        .add_component(Section::new("2.6 Stat").with_level(2))
        .add_component(TextBlock::new("Single stat with optional unit, trend and accent."))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({ "type": "stat",
                    "data": Stat::new("Total Issues", "142").with_unit("findings").to_data() }))
                .add_item(serde_json::json!({ "type": "stat",
                    "data": Stat::new("Score Delta", "+21").with_trend("up", true).to_data() }))
                .add_item(serde_json::json!({ "type": "stat",
                    "data": Stat::new("Revenue", "€2.4M").with_accent("#22c55e").to_data() })),
        )

        .add_component(Section::new("2.7 StatPair").with_level(2))
        .add_component(TextBlock::new("Two stats side by side."))
        .add_component(
            Grid::new(2)
                .add_item(serde_json::json!({ "type": "stat-pair", // @id: stat-pair
                    "data": StatPair::new(
                        StatPairEntry::new("Before", "58"),
                        StatPairEntry::new("After", "79"),
                    ).to_data() }))
                .add_item(serde_json::json!({ "type": "stat-pair",
                    "data": StatPair::new(
                        StatPairEntry::new("Score", "79").with_unit("/100"),
                        StatPairEntry::new("Delta", "+21").with_accent("#22c55e"),
                    ).to_data() })),
        )

        .add_component(Section::new("2.8 MetricStrip").with_level(2))
        .add_component(TextBlock::new(
            "Compact horizontal strip of key metrics.",
        ))
        .add_component(MetricStrip::new(vec![ // @id: metric-strip
            MetricStripItem::new("Score", "79").with_unit("/100"),
            MetricStripItem::new("Issues", "142").with_status("bad"),
            MetricStripItem::new("Critical", "8").with_accent("#e53e3e"),
            MetricStripItem::new("Pages", "24").with_unit("crawled"),
        ]))

        .add_component(Section::new("2.9 TrendTile").with_level(2))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({ "type": "trend-tile",
                    "data": TrendTile::up("Accessibility", "+7").with_reference("was 87").to_data() }))
                .add_item(serde_json::json!({ "type": "trend-tile",
                    "data": TrendTile::down("Performance", "-4").with_reference("was 71").to_data() }))
                .add_item(serde_json::json!({ "type": "trend-tile",
                    "data": TrendTile::stable("SEO", "0").to_data() })),
        )

        .add_component(Section::new("2.10 ModuleComparison").with_level(2))
        .add_component(ModuleComparison::new(vec![ // @id: module-comparison
            ComparisonModule::new("Accessibility", 94),
            ComparisonModule::new("Performance", 67),
            ComparisonModule::new("SEO", 88),
            ComparisonModule::new("Security", 41),
        ]))

        .add_component(Section::new("2.11 PortfolioSummary").with_level(2))
        .add_component(TextBlock::new(
            "Portfolio-level overview card for batch benchmark reports.",
        ))
        .add_component( // @id: portfolio-summary
            PortfolioSummary::new(8, 74)
                .with_best("shop.example.com", 93)
                .with_worst("legacy.example.com", 41)
                .with_issues(89, 12),
        )
        .add_component(PageBreak::new())

        // ── 3. Narrative & Hero ──────────────────────────────────────────────
        .add_component(Section::new("3. Narrative & Hero").with_level(1))

        .add_component(Section::new("3.1 HeroSummary").with_level(2))
        .add_component(TextBlock::new(
            "Executive summary block with score, grade, verdict and key metrics.",
        ))
        .add_component( // @id: hero-summary
            HeroSummary::new(79, "B", "example.com")
                .with_date("2026-04-02")
                .with_verdict(
                    "The site performs well in accessibility but needs work on performance \
                     and security to reach the next tier.",
                )
                .add_metric(HeroMetric {
                    title: "Issues".into(),
                    value: "142".into(),
                    accent_color: Some("#e53e3e".into()),
                })
                .add_metric(HeroMetric {
                    title: "Critical".into(),
                    value: "8".into(),
                    accent_color: None,
                })
                .with_top_actions(vec![
                    "Add alt text to 14 images".into(),
                    "Fix colour contrast on hero section".into(),
                ])
                .with_positive_aspects(vec![
                    "No keyboard trap violations".into(),
                    "Valid SSL certificate".into(),
                ]),
        )

        .add_component(Section::new("3.2 CardDashboard").with_level(2))
        .add_component(TextBlock::new(
            "Grid of module score cards with interpretation text.",
        ))
        .add_component(CardDashboard::new(vec![ // @id: card-dashboard
            DashboardCard {
                name: "Accessibility".into(),
                score: 94,
                interpretation: "Excellent. Only minor contrast issues remain.".into(),
                good_threshold: 80,
                warn_threshold: 60,
            },
            DashboardCard {
                name: "Performance".into(),
                score: 67,
                interpretation: "Below target. LCP and CLS need improvement.".into(),
                good_threshold: 80,
                warn_threshold: 60,
            },
            DashboardCard {
                name: "Security".into(),
                score: 41,
                interpretation: "Critical: outdated headers, open redirects.".into(),
                good_threshold: 75,
                warn_threshold: 50,
            },
        ]))

        .add_component(Section::new("3.3 StatusPill").with_level(2))
        .add_component(TextBlock::new(
            "Compact colour-coded status badge. Built-in presets: good, warn, bad, neutral, info.",
        ))
        .add_component(
            Grid::new(5)
                .add_item(serde_json::json!({ "type": "status-pill",
                    "data": StatusPill::good("Passed").to_data() }))
                .add_item(serde_json::json!({ "type": "status-pill",
                    "data": StatusPill::warn("Review").to_data() }))
                .add_item(serde_json::json!({ "type": "status-pill",
                    "data": StatusPill::bad("Failed").to_data() }))
                .add_item(serde_json::json!({ "type": "status-pill",
                    "data": StatusPill::info("Info").to_data() }))
                .add_item(serde_json::json!({ "type": "status-pill",
                    "data": StatusPill::neutral("Pending").to_data() })),
        )

        .add_component(Section::new("3.4 Eyebrow").with_level(2))
        .add_component(TextBlock::new(
            "Small uppercase label above a heading, typical in editorial design.",
        ))
        .add_component(Eyebrow::new("Module Report")) // @id: eyebrow
        .add_component(Eyebrow::new("Confidential").with_color("#e53e3e"))
        .add_component(PageBreak::new())

        // ── 4. Charts ────────────────────────────────────────────────────────
        .add_component(Section::new("4. Charts").with_level(1))

        .add_component(Section::new("4.1 Chart — Bar").with_level(2))
        .add_component(TextBlock::new("Types: bar, line, pie, area. Data via add_series(name, Vec<(label, value)>).")) // @id: chart
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

        .add_component(Section::new("4.2 Chart — Line (multi-series)").with_level(2))
        .add_component(
            Chart::line("Score Trend")
                .add_series(
                    "Score",
                    vec![
                        ("Jan".into(), 58.0), ("Feb".into(), 63.0), ("Mar".into(), 71.0),
                        ("Apr".into(), 79.0), ("May".into(), 85.0), ("Jun".into(), 91.0),
                    ],
                )
                .add_series(
                    "Target",
                    vec![
                        ("Jan".into(), 70.0), ("Feb".into(), 72.0), ("Mar".into(), 75.0),
                        ("Apr".into(), 80.0), ("May".into(), 85.0), ("Jun".into(), 90.0),
                    ],
                ),
        )

        .add_component(Section::new("4.3 Chart — Pie").with_level(2))
        .add_component(Chart::pie("Issues by Severity").add_series(
            "Count",
            vec![
                ("Critical".into(), 8.0),
                ("High".into(), 23.0),
                ("Medium".into(), 41.0),
                ("Low".into(), 70.0),
            ],
        ))

        .add_component(Section::new("4.4 Sparkline").with_level(2))
        .add_component(TextBlock::new("Compact inline trend chart.")) // @id: sparkline
        .add_component(Sparkline::new(vec![62.0, 65.0, 70.0, 68.0, 74.0, 79.0, 83.0]))
        .add_component(PageBreak::new())

        // ── 5. Findings & Callouts ───────────────────────────────────────────
        .add_component(Section::new("5. Findings & Callouts").with_level(1))

        .add_component(Section::new("5.1 Finding").with_level(2))
        .add_component(TextBlock::new(
            "Structured audit finding with severity, description and recommendation.",
        ))
        .add_component( // @id: finding
            Finding::new(
                "Images missing alternative text",
                Severity::Critical,
                "14 <img> elements have no alt attribute.",
            )
            .with_recommendation("Add descriptive alt text to every meaningful image.")
            .with_category("WCAG 1.1.1"),
        )
        .add_component(Finding::new(
            "Colour contrast ratio below 4.5:1",
            Severity::High,
            "Hero text uses #888888 on white — contrast ratio 3.5:1.",
        ))

        .add_component(Section::new("5.2 SpotlightCard").with_level(2))
        .add_component(TextBlock::new(
            "Full-width spotlight block for the single most impactful finding.",
        ))
        .add_component( // @id: spotlight-card
            SpotlightCard::new(
                "Missing alternative text",
                "14 images across 8 pages have no alt attribute, blocking screen reader users \
                 from perceiving visual content.",
            )
            .with_variant("critical")
            .with_eyebrow("Top Finding")
            .with_metric("14")
            .with_detail("Affects all screen reader users — estimated 15% of visitors.")
            .with_action("Add descriptive alt attributes to all meaningful images"),
        )

        .add_component(Section::new("5.3 ImpactGrid").with_level(2))
        .add_component(TextBlock::new(
            "Three-panel impact overview: user, risk, conversion.",
        ))
        .add_component( // @id: impact-grid
            ImpactGrid::new(
                ImpactGridCard::new("User Impact", "Screen readers blocked", "14 images with no alt text prevent assistive technology users from accessing content.").with_status("bad"),
                ImpactGridCard::new("Risk Level", "Compliance risk", "WCAG 2.1 AA non-compliance may trigger legal obligations in the EU and US.").with_status("warn"),
                ImpactGridCard::new("Conversion", "Trust signal", "Accessibility issues correlate with higher bounce rates on mobile.").with_status("warn"),
            )
            .with_title("Issue Impact"),
        )

        .add_component(Section::new("5.4 ChecklistPanel").with_level(2))
        .add_component(TextBlock::new(
            "Checklist-style diagnosis rows with status indicators.",
        ))
        .add_component( // @id: checklist-panel
            ChecklistPanel::new(vec![
                ChecklistRow::new("Alt text", "14 violations found").with_status("bad"),
                ChecklistRow::new("Colour contrast", "7 violations found").with_status("warn"),
                ChecklistRow::new("Keyboard focus", "All interactive elements reachable").with_status("good"),
                ChecklistRow::new("ARIA labels", "No violations detected").with_status("good"),
                ChecklistRow::new("Skip links", "Missing on 3 pages").with_status("warn"),
            ])
            .with_title("Accessibility Diagnosis"),
        )

        .add_component(Section::new("5.5 ComparisonBlock").with_level(2))
        .add_component(TextBlock::new(
            "Before/after comparison showing incorrect vs correct implementation.",
        ))
        .add_component( // @id: comparison-block
            ComparisonBlock::new(
                r#"<img src="hero.jpg">"#,
                r#"<img src="hero.jpg" alt="Team collaborating in a modern office">"#,
            )
            .code()
            .with_note("Always provide meaningful alt text for informative images."),
        )

        .add_component(Section::new("5.6 Callout").with_level(2))
        .add_component(TextBlock::new("Highlighted note block. Types: info, warning, success, error."))
        .add_component(Callout::info("All automated findings should be verified manually.")) // @id: callout
        .add_component(Callout::warning("3 critical issues require attention before release."))
        .add_component(Callout::success("No WCAG 1.3.1 violations detected."))
        .add_component(Callout::error("SSL certificate expires in 7 days."))

        .add_component(Section::new("5.7 SummaryBox").with_level(2))
        .add_component( // @id: summary-box
            SummaryBox::new("Audit Summary")
                .add_item("URL", "https://example.com")
                .add_item("Crawled Pages", "24")
                .add_item_with_status("WCAG Level", "AA", ScoreStatus::Good)
                .add_item_with_status("Mobile Friendly", "Issues found", ScoreStatus::Warning),
        )

        .add_component(Section::new("5.8 SeverityOverview").with_level(2))
        .add_component(SeverityOverview::new(8, 23, 41, 70)) // @id: severity-overview
        .add_component(PageBreak::new())

        // ── 6. Tables ────────────────────────────────────────────────────────
        .add_component(Section::new("6. Tables").with_level(1))

        .add_component(Section::new("6.1 AuditTable").with_level(2))
        .add_component( // @id: audit-table
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

        .add_component(Section::new("6.2 BenchmarkTable").with_level(2))
        .add_component( // @id: benchmark-table
            BenchmarkTable::new(vec![
                BenchmarkRow::new(1, "example.com", 79, 94, 8),
                BenchmarkRow::new(2, "competitor-a.com", 65, 71, 14),
                BenchmarkRow::new(3, "competitor-b.com", 88, 91, 2),
                BenchmarkRow::new(4, "industry-avg", 72, 78, 9),
            ])
            .with_title("Benchmark Comparison"),
        )

        .add_component(Section::new("6.3 PivotTable").with_level(2))
        .add_component(TextBlock::new("Pre-aggregated matrix of row × column values."))
        .add_component( // @id: pivot-table
            PivotTable::new(
                vec!["Accessibility".into(), "Performance".into(), "SEO".into()],
                vec!["Q1".into(), "Q2".into(), "Q3".into(), "Q4".into()],
                vec![
                    vec!["72".into(), "79".into(), "85".into(), "94".into()],
                    vec!["61".into(), "63".into(), "65".into(), "67".into()],
                    vec!["80".into(), "82".into(), "85".into(), "88".into()],
                ],
            )
            .with_title("Module Scores by Quarter"),
        )

        .add_component(Section::new("6.4 Crosstab").with_level(2))
        .add_component(TextBlock::new(
            "Aggregates raw row data across row and column dimensions.",
        ))
        .add_component({ // @id: crosstab
            let mut rows: Vec<HashMap<String, serde_json::Value>> = Vec::new();
            for (region, module, score) in [
                ("North", "Accessibility", 92), ("North", "Performance", 71),
                ("South", "Accessibility", 88), ("South", "Performance", 64),
                ("East",  "Accessibility", 95), ("East",  "Performance", 73),
            ] {
                let mut row = HashMap::new();
                row.insert("Region".into(), serde_json::json!(region));
                row.insert("Module".into(), serde_json::json!(module));
                row.insert("Score".into(), serde_json::json!(score));
                rows.push(row);
            }
            Crosstab::new("Region", "Module", "Score")
                .with_title("Scores by Region × Module")
                .with_aggregation("avg")
                .with_data(rows)
        })
        .add_component(PageBreak::new())

        // ── 7. Lists & Structure ─────────────────────────────────────────────
        .add_component(Section::new("7. Lists & Structure").with_level(1))

        .add_component(Section::new("7.1 List").with_level(2))
        .add_component( // @id: list
            List::new()
                .with_title("Quick Wins")
                .add_item("Add alt text to 14 images — 30 min effort")
                .add_item("Increase hero text contrast from 3.5:1 to 4.5:1")
                .add_item("Associate form labels with input IDs")
                .add_item("Add visible skip-to-content link"),
        )

        .add_component(Section::new("7.2 KeyValueList").with_level(2))
        .add_component( // @id: key-value-list
            KeyValueList::new()
                .add("Framework", "React 18")
                .add("Hosting", "Vercel")
                .add("CDN", "Cloudflare")
                .add("Analytics", "Plausible"),
        )

        .add_component(Section::new("7.3 RoadmapBlock").with_level(2))
        .add_component(RoadmapBlock::new(vec![ // @id: roadmap-block
            RoadmapColumn {
                title: "Quick Wins".into(),
                accent_color: Some("#38a169".into()),
                items: vec![
                    ActionItem {
                        action: "Add alt text to images".into(),
                        role: "Developer".into(),
                        priority: "high".into(),
                        effort: Some("2h".into()),
                        benefit: "Fixes 14 critical findings".into(),
                    },
                ],
            },
            RoadmapColumn {
                title: "Short-term".into(),
                accent_color: Some("#d69e2e".into()),
                items: vec![
                    ActionItem {
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
                    ActionItem {
                        action: "Redesign navigation for keyboard users".into(),
                        role: "Design + Dev".into(),
                        priority: "low".into(),
                        effort: Some("2w".into()),
                        benefit: "Full keyboard accessibility".into(),
                    },
                ],
            },
        ]))

        .add_component(Section::new("7.4 ComparisonCluster").with_level(2))
        .add_component(TextBlock::new("Grid of comparison items with values and status."))
        .add_component( // @id: comparison-cluster
            ComparisonCluster::new(vec![
                ComparisonClusterItem::new("example.com", "79").with_status("warn"),
                ComparisonClusterItem::new("competitor-a.com", "65").with_status("bad"),
                ComparisonClusterItem::new("competitor-b.com", "88").with_status("good"),
                ComparisonClusterItem::new("industry avg", "72").with_sub("120 sites"),
            ])
            .with_title("Score Comparison")
            .with_columns(4),
        )

        .add_component(Section::new("7.5 SideLabel").with_level(2))
        .add_component(TextBlock::new(
            "Sidebar heading with optional subheading and bullet items.",
        ))
        .add_component( // @id: side-label
            SideLabel::new("What we checked")
                .with_subheading("Scope of this audit")
                .add_item("All public pages (24 crawled)")
                .add_item("WCAG 2.1 Level AA criteria")
                .add_item("Keyboard and screen reader behaviour")
                .with_divider(),
        )

        .add_component(Section::new("7.6 PhaseBlock").with_level(2))
        .add_component(TextBlock::new(
            "Numbered phase in a multi-step remediation plan.",
        ))
        .add_component( // @id: phase-block
            PhaseBlock::new(1, "Triage", "Fix all critical and high findings.")
                .with_items(vec![
                    "Add alt text to 14 images".into(),
                    "Fix colour contrast on hero section".into(),
                    "Associate form labels with inputs".into(),
                ])
                .with_total(3)
                .with_color("#e53e3e"),
        )
        .add_component(PageBreak::new())

        // ── 8. Text & Labels ─────────────────────────────────────────────────
        .add_component(Section::new("8. Text & Labels").with_level(1))

        .add_component(Section::new("8.1 Label").with_level(2))
        .add_component(Label::new("Default label")) // @id: label
        .add_component(Label::new("Bold label").bold())
        .add_component(Label::new("Centred label").center())
        .add_component(Label::new("Coloured label").with_color("#3182ce"))

        .add_component(Section::new("8.2 TextBlock").with_level(2))
        .add_component(TextBlock::new(
            "renderreport is a Rust library for building data-driven PDF reports using \
             Typst as the embedded render engine. Components are declared in Rust, serialised \
             to JSON, and rendered via Typst templates.",
        ))

        .add_component(Section::new("8.3 StatusPill (inline)").with_level(2))
        .add_component(TextBlock::new("Already shown in section 3.3 — referenced for completeness."))

        // ── 9. Numbers, Dates & Barcodes ─────────────────────────────────────
        .add_component(PageBreak::new())
        .add_component(Section::new("9. Numbers, Dates & Barcodes").with_level(1))

        .add_component(Section::new("9.1 NumberField").with_level(2))
        .add_component(TextBlock::new(
            "Formatted number with optional prefix, suffix and locale.",
        ))
        .add_component(
            Grid::new(4)
                .add_item(serde_json::json!({ "type": "number-field",
                    "data": NumberField::new(42000.0).to_data() }))
                .add_item(serde_json::json!({ "type": "number-field",
                    "data": NumberField::currency(2400.0, "€").to_data() }))
                .add_item(serde_json::json!({ "type": "number-field",
                    "data": NumberField::percentage(79.0).to_data() }))
                .add_item(serde_json::json!({ "type": "number-field",
                    "data": NumberField::new(1234.56).with_prefix("Score: ").to_data() })),
        )

        .add_component(Section::new("9.2 DateField").with_level(2))
        .add_component(
            Grid::new(3)
                .add_item(serde_json::json!({ "type": "date-field",
                    "data": DateField::new("2026-04-02").to_data() }))
                .add_item(serde_json::json!({ "type": "date-field",
                    "data": DateField::european("2026-04-02").to_data() }))
                .add_item(serde_json::json!({ "type": "date-field",
                    "data": DateField::us("2026-04-02").to_data() })),
        )

        .add_component(Section::new("9.3 ResourceField").with_level(2))
        .add_component(TextBlock::new("i18n resource lookup by key with locale."))
        .add_component( // @id: resource-field
            ResourceField::new("report.title")
                .with_default("Audit Report")
                .with_locale("de"),
        )

        .add_component(Section::new("9.4 Barcode — 1D").with_level(2))
        .add_component(TextBlock::new("Code128, EAN-13, QR, Data Matrix and more."))
        .add_component(
            Grid::new(2)
                .add_item(serde_json::json!({ "type": "barcode",
                    "data": Barcode::code128("PROD-2026-XYZ-001").to_data() }))
                .add_item(serde_json::json!({ "type": "barcode",
                    "data": Barcode::ean13("5901234123457").to_data() })),
        )

        .add_component(Section::new("9.5 Barcode — 2D").with_level(2))
        .add_component(
            Grid::new(2)
                .add_item(serde_json::json!({ "type": "barcode",
                    "data": Barcode::qr_code("https://github.com/casoon/renderreport")
                        .with_size("80pt", "80pt").to_data() }))
                .add_item(serde_json::json!({ "type": "barcode",
                    "data": Barcode::data_matrix("ASSET-2026-001")
                        .with_size("80pt", "80pt").to_data() })),
        )

        // ── 10. Image ────────────────────────────────────────────────────────
        .add_component(Section::new("10. Image").with_level(1))
        .add_component(TextBlock::new(
            "Embeds an image from the assets map. Centred by default. \
             Supports caption and width.",
        ))
        .asset("placeholder.svg", "examples/assets/placeholder.svg")
        .add_component( // @id: image
            Image::new("placeholder.svg")
                .with_caption("Example image with caption")
                .with_width("60%"),
        )
        .add_component(PageBreak::new())

        // ── 11. Marketing & Narrative ─────────────────────────────────────────
        .add_component(Section::new("11. Marketing & Narrative").with_level(1))

        .add_component(Section::new("11.1 FeatureGrid").with_level(2))
        .add_component(TextBlock::new("Marketing feature/benefit grid with optional icons."))
        .add_component( // @id: feature-grid
            FeatureGrid::new(vec![
                FeatureGridItem { title: "Fast Rendering".into(), description: Some("PDF output in under a second.".into()), icon: Some("⚡".into()), status: Some("highlight".into()) },
                FeatureGridItem { title: "Type-safe".into(), description: Some("All components are Rust structs.".into()), icon: Some("🦀".into()), status: Some("positive".into()) },
                FeatureGridItem { title: "Themeable".into(), description: Some("Custom tokens per report.".into()), icon: Some("🎨".into()), status: None },
                FeatureGridItem { title: "Extensible".into(), description: Some("Add custom Typst templates.".into()), icon: Some("🔧".into()), status: None },
            ])
            .with_columns(2)
            .with_title("Why renderreport?"),
        )

        .add_component(Section::new("11.2 CTABox").with_level(2))
        .add_component(TextBlock::new("Call-to-action block. Tones: primary, urgent, neutral."))
        .add_component( // @id: cta-box
            CTABox::new("Start your first report today")
                .with_body("renderreport is open source and ready to embed in your pipeline.")
                .with_action("View on GitHub", "https://github.com/casoon/renderreport")
                .with_tone("primary"),
        )

        .add_component(Section::new("11.3 Testimonial").with_level(2))
        .add_component( // @id: testimonial
            Testimonial::new(
                "renderreport saved us hours every sprint — our PDF reports now build in CI automatically.",
                "Engineering Lead",
            )
            .with_company("Example Corp"),
        )

        .add_component(Section::new("11.4 ProcessFlow").with_level(2))
        .add_component(TextBlock::new("Linear process with numbered steps. Direction: horizontal or vertical."))
        .add_component( // @id: process-flow
            ProcessFlow::new(vec![
                ProcessStep { label: "Collect Data".into(), description: Some("Pull metrics from APIs.".into()), icon: None },
                ProcessStep { label: "Run renderreport".into(), description: Some("Render Rust structs to PDF.".into()), icon: None },
                ProcessStep { label: "Deliver PDF".into(), description: Some("Email or S3 upload.".into()), icon: None },
            ])
            .with_title("Report Pipeline"),
        )

        .add_component(Section::new("11.5 Timeline").with_level(2))
        .add_component( // @id: timeline
            Timeline::new(vec![
                TimelineItem { date: "Q1 2026".into(), title: "v0.1 alpha".into(), description: Some("Core engine + standard components.".into()), status: Some("good".into()) },
                TimelineItem { date: "Q2 2026".into(), title: "v0.2 beta".into(), description: Some("Pack system + theming.".into()), status: Some("warn".into()) },
                TimelineItem { date: "Q3 2026".into(), title: "v1.0 stable".into(), description: None, status: None },
            ])
            .with_title("Release Timeline"),
        )

        .add_component(Section::new("11.6 Funnel").with_level(2))
        .add_component( // @id: funnel
            Funnel::new(vec![
                FunnelStep { label: "Visitors".into(), value: "10,000".into(), unit: Some("sessions".into()), color: None },
                FunnelStep { label: "Signups".into(), value: "1,200".into(), unit: Some("users".into()), color: None },
                FunnelStep { label: "Activated".into(), value: "430".into(), unit: Some("users".into()), color: None },
                FunnelStep { label: "Paying".into(), value: "88".into(), unit: Some("users".into()), color: None },
            ])
            .with_title("Conversion Funnel"),
        )

        .add_component(Section::new("11.7 ProblemSolution").with_level(2))
        .add_component( // @id: problem-solution
            ProblemSolution::new(
                "Generating PDFs manually is slow, error-prone, and hard to maintain.",
                "renderreport lets you declare report components as Rust structs and render to PDF automatically.",
            )
            .with_labels("The Problem", "The Solution"),
        )

        .add_component(Section::new("11.8 BeforeAfter").with_level(2))
        .add_component( // @id: before-after
            BeforeAfter::new(
                "Hand-written PDF layout in LaTeX — 300 lines of fragile code.",
                "renderreport: 30 lines of Rust, fully type-safe, CI-friendly.",
            )
            .with_labels("Before", "After"),
        )

        .add_component(Section::new("11.9 WhyItMatters").with_level(2))
        .add_component( // @id: why-it-matters
            WhyItMatters::new(
                "Automated, reproducible reports eliminate human error and free engineers \
                 to focus on insights rather than formatting.",
            )
            .with_title("Why automation matters"),
        )

        .add_component(Section::new("11.10 FactBox").with_level(2))
        .add_component( // @id: fact-box
            FactBox::new("renderreport renders a 20-page PDF in under 500 ms on a standard CI runner.")
                .with_label("Performance fact"),
        )

        .add_component(Section::new("11.11 QuoteBlock").with_level(2))
        .add_component( // @id: quote-block
            QuoteBlock::new(
                "Good tools shape good reports. renderreport gives your data the presentation it deserves.",
            )
            .with_author("renderreport team"),
        )

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
