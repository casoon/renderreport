//! Benchmark Report Example
//!
//! A portfolio-level website audit comparison report using PortfolioSummary,
//! BenchmarkTable, ModuleComparison, SeverityOverview, Charts, and RoadmapBlock.
//!
//! Run with: cargo run --example benchmark_report

use renderreport::components::advanced::TableOfContents;
use renderreport::components::charts::Chart;
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    println!("Generating Website Portfolio Benchmark Report...");

    let engine = Engine::new()?;

    let report = engine
        .report("default")
        .metadata("author", "Casoon Audit Platform")
        // ── Cover Page ──────────────────────────────────────────────
        .add_component(
            CoverPage::new("Portfolio Benchmark Report", "8 Websites", 74, "C+")
                .with_brand("Casoon")
                .with_date("March 2026")
                .with_subtitle("Multi-Site Audit Comparison — Accessibility, SEO, Performance & Security")
                .with_modules(vec![
                    "Accessibility".into(),
                    "SEO".into(),
                    "Performance".into(),
                    "Security".into(),
                ])
                .with_issues(89, 12),
        )
        // ── Table of Contents ───────────────────────────────────────
        .add_component(TableOfContents::new().with_title("Contents").with_depth(2))
        // ── Portfolio Summary ───────────────────────────────────────
        .add_component(Section::new("Portfolio Overview").with_level(1))
        .add_component(
            PortfolioSummary::new(8, 74)
                .with_best("shop.example.com", 93)
                .with_worst("legacy.example.com", 41)
                .with_issues(89, 12),
        )
        // ── Severity Overview ───────────────────────────────────────
        .add_component(
            SeverityOverview::new(12, 18, 34, 25)
                .with_title("Aggregate Issue Severity"),
        )
        // ── Ranking Table ───────────────────────────────────────────
        .add_component(Section::new("Website Rankings").with_level(1))
        .add_component(
            BenchmarkTable::new(vec![
                BenchmarkRow::new(1, "shop.example.com", 93, 96, 1)
                    .with_seo(91).with_performance(90).with_security(95),
                BenchmarkRow::new(2, "docs.example.com", 88, 94, 2)
                    .with_seo(85).with_performance(88).with_security(86),
                BenchmarkRow::new(3, "blog.example.com", 82, 88, 3)
                    .with_seo(82).with_performance(78).with_security(80),
                BenchmarkRow::new(4, "www.example.com", 78, 82, 5)
                    .with_seo(76).with_performance(72).with_security(82),
                BenchmarkRow::new(5, "api.example.com", 75, 70, 4)
                    .with_seo(60).with_performance(85).with_security(88),
                BenchmarkRow::new(6, "app.example.com", 68, 72, 8)
                    .with_seo(65).with_performance(62).with_security(74),
                BenchmarkRow::new(7, "careers.example.com", 62, 58, 11)
                    .with_seo(68).with_performance(55).with_security(65),
                BenchmarkRow::new(8, "legacy.example.com", 41, 35, 18)
                    .with_seo(42).with_performance(38).with_security(48),
            ])
            .with_title("Website Ranking by Overall Score"),
        )
        // ── Score Distribution Charts ───────────────────────────────
        .add_component(Section::new("Score Analysis").with_level(1))
        .add_component(
            Chart::bar("Overall Scores by Website")
                .add_series("Score", vec![
                    ("shop".into(), 93.0),
                    ("docs".into(), 88.0),
                    ("blog".into(), 82.0),
                    ("www".into(), 78.0),
                    ("api".into(), 75.0),
                    ("app".into(), 68.0),
                    ("careers".into(), 62.0),
                    ("legacy".into(), 41.0),
                ])
                .with_labels("Website", "Score"),
        )
        .add_component(
            Chart::pie("Grade Distribution").add_series("Grades", vec![
                ("A (90-100)".into(), 1.0),
                ("B (80-89)".into(), 2.0),
                ("C (70-79)".into(), 2.0),
                ("D (60-69)".into(), 2.0),
                ("F (<60)".into(), 1.0),
            ]),
        )
        // ── Module Comparison ───────────────────────────────────────
        .add_component(Section::new("Module Averages").with_level(1))
        .add_component(
            ModuleComparison::new(vec![
                ComparisonModule::new("Accessibility", 74).with_color("#3b82f6"),
                ComparisonModule::new("SEO", 71).with_color("#22c55e"),
                ComparisonModule::new("Performance", 71).with_color("#f59e0b"),
                ComparisonModule::new("Security", 77).with_color("#8b5cf6"),
            ])
            .with_title("Average Scores Across Portfolio"),
        )
        // ── Common Findings ─────────────────────────────────────────
        .add_component(Section::new("Most Common Issues").with_level(1))
        .add_component(
            Finding::new("Missing Alt Text", Severity::High,
                "Found on 6 of 8 websites. Total: 47 images without alt text. This is the most widespread accessibility issue across the portfolio.")
                .with_recommendation("Implement an alt-text policy and add automated checks to CI/CD pipelines across all properties.")
                .with_category("Accessibility"),
        )
        .add_component(
            Finding::new("No Content-Security-Policy Header", Severity::High,
                "5 of 8 websites lack a CSP header, leaving them vulnerable to XSS attacks.")
                .with_recommendation("Deploy a shared CSP configuration via the central CDN/reverse proxy. Start in report-only mode.")
                .with_category("Security"),
        )
        .add_component(
            Finding::new("Slow Largest Contentful Paint", Severity::Medium,
                "4 websites exceed the 2.5s LCP threshold. Worst offender: legacy.example.com at 6.8 seconds.")
                .with_recommendation("Prioritize image optimization and critical CSS inlining. Consider a shared image CDN for all properties.")
                .with_category("Performance"),
        )
        .add_component(
            Finding::new("legacy.example.com Requires Rebuild", Severity::Critical,
                "Score of 41/100. The legacy site uses outdated frameworks (jQuery 1.x, Bootstrap 2), has 18 critical issues, and fails basic accessibility checks.")
                .with_recommendation("Plan a complete rebuild using the company's modern stack. In the interim, add CSP headers and fix critical accessibility violations.")
                .with_category("Strategic"),
        )
        // ── Technical Audit Table ───────────────────────────────────
        .add_component(Section::new("Cross-Site Technical Checklist").with_level(1))
        .add_component(
            AuditTable::new(vec![
                TableColumn::new("Check").with_width("25%"),
                TableColumn::new("shop").with_width("10%"),
                TableColumn::new("docs").with_width("10%"),
                TableColumn::new("blog").with_width("10%"),
                TableColumn::new("www").with_width("10%"),
                TableColumn::new("api").with_width("10%"),
                TableColumn::new("app").with_width("10%"),
                TableColumn::new("legacy").with_width("10%"),
            ])
            .with_title("Technical Compliance Matrix")
            .add_row(vec!["HTTPS", "Pass", "Pass", "Pass", "Pass", "Pass", "Pass", "Fail"])
            .add_row(vec!["CSP Header", "Pass", "Pass", "Fail", "Fail", "Pass", "Fail", "Fail"])
            .add_row(vec!["HSTS", "Pass", "Pass", "Pass", "Pass", "Pass", "Fail", "Fail"])
            .add_row(vec!["robots.txt", "Pass", "Pass", "Pass", "Pass", "N/A", "Pass", "Fail"])
            .add_row(vec!["Sitemap", "Pass", "Pass", "Pass", "Pass", "N/A", "Fail", "Fail"])
            .add_row(vec!["Mobile Ready", "Pass", "Pass", "Pass", "Pass", "N/A", "Pass", "Fail"])
            .add_row(vec!["Core Web Vitals", "Pass", "Pass", "Warn", "Warn", "Pass", "Fail", "Fail"]),
        )
        // ── Action Roadmap ──────────────────────────────────────────
        .add_component(Section::new("Portfolio Action Plan").with_level(1))
        .add_component(RoadmapBlock::new(vec![
            RoadmapColumn {
                title: "Quick Wins (Week 1-2)".into(),
                accent_color: Some("#ef4444".into()),
                items: vec![
                    ActionItem { action: "Deploy CSP headers via CDN".into(), role: "DevOps".into(), priority: "High".into(), effort: Some("4 hours".into()), benefit: "XSS protection for 5 sites".into() },
                    ActionItem { action: "Add alt text to critical images".into(), role: "Content".into(), priority: "High".into(), effort: Some("8 hours".into()), benefit: "Accessibility compliance".into() },
                    ActionItem { action: "Enable HTTPS on legacy site".into(), role: "DevOps".into(), priority: "Critical".into(), effort: Some("2 hours".into()), benefit: "Basic security compliance".into() },
                ],
            },
            RoadmapColumn {
                title: "Medium-term (Month 1-2)".into(),
                accent_color: Some("#f59e0b".into()),
                items: vec![
                    ActionItem { action: "Image CDN for all properties".into(), role: "DevOps".into(), priority: "Medium".into(), effort: Some("2 weeks".into()), benefit: "Faster LCP across portfolio".into() },
                    ActionItem { action: "CI/CD accessibility checks".into(), role: "Engineering".into(), priority: "Medium".into(), effort: Some("1 week".into()), benefit: "Prevent future regressions".into() },
                ],
            },
            RoadmapColumn {
                title: "Strategic (Quarter 2)".into(),
                accent_color: Some("#22c55e".into()),
                items: vec![
                    ActionItem { action: "Rebuild legacy.example.com".into(), role: "Engineering".into(), priority: "High".into(), effort: Some("8 weeks".into()), benefit: "Eliminate 18 critical issues".into() },
                    ActionItem { action: "Unified design system rollout".into(), role: "Design / Eng".into(), priority: "Medium".into(), effort: Some("6 weeks".into()), benefit: "Consistent UX, easier maintenance".into() },
                ],
            },
        ]))
        // ── Closing ─────────────────────────────────────────────────
        .add_component(
            Callout::info("Scores are based on automated scans using Lighthouse, axe-core, and custom security checks. Manual review recommended for critical findings.")
                .with_title("Methodology"),
        )
        .build();

    let pdf = engine.render_pdf(&report)?;
    let output_path = "examples/output/benchmark_report.pdf";
    std::fs::write(output_path, &pdf)?;

    println!(
        "✓ Benchmark report generated: {} ({} KB)",
        output_path,
        pdf.len() / 1024
    );
    Ok(())
}
