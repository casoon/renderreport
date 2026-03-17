//! SEO Audit Report Example
//!
//! Demonstrates a professional, multi-page SEO audit report using
//! composite components: CoverPage, TableOfContents, HeroSummary,
//! ModuleDashboard, SeverityOverview, Findings, ActionRoadmap, and ModuleComparison.
//!
//! Run with: cargo run --example seo_audit

use renderreport::components::advanced::TableOfContents;
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    println!("Generating SEO Audit report...");

    let engine = Engine::new()?;

    let report = engine
        .report("seo-audit")
        .metadata("author", "Casoon")
        // ── Cover Page ──────────────────────────────────────────────
        .add_component(
            CoverPage::new("Website Audit Report", "example.com", 78, "C+")
                .with_brand("Casoon")
                .with_date("March 2026")
                .with_subtitle("SEO, Accessibility, Performance & Security Analysis")
                .with_modules(vec![
                    "SEO".into(),
                    "Accessibility".into(),
                    "Performance".into(),
                    "Security".into(),
                ])
                .with_issues(15, 3),
        )
        // ── Table of Contents ───────────────────────────────────────
        .add_component(
            TableOfContents::new()
                .with_title("Table of Contents")
                .with_depth(2),
        )
        // ── Hero Summary ────────────────────────────────────────────
        .add_component(
            HeroSummary::new(78, "C+", "example.com")
                .with_date("March 2026")
                .with_verdict("The website shows solid fundamentals but has significant SEO and performance issues that need attention.")
                .add_metric(HeroMetric { title: "Critical Issues".into(), value: "3".into(), accent_color: Some("#ef4444".into()) })
                .add_metric(HeroMetric { title: "Warnings".into(), value: "7".into(), accent_color: Some("#f59e0b".into()) })
                .add_metric(HeroMetric { title: "Passed Checks".into(), value: "45".into(), accent_color: Some("#22c55e".into()) })
                .add_metric(HeroMetric { title: "Overall Score".into(), value: "78 / 100".into(), accent_color: Some("#3b82f6".into()) })
                .with_top_actions(vec![
                    "Fix missing meta descriptions on 12 pages".into(),
                    "Optimize Largest Contentful Paint (currently 3.2s)".into(),
                    "Add alt text to 8 images".into(),
                ])
                .with_positive_aspects(vec![
                    "Valid SSL certificate with HSTS enabled".into(),
                    "Responsive design passes all mobile tests".into(),
                    "robots.txt and XML sitemap correctly configured".into(),
                    "Structured data (JSON-LD) present on key pages".into(),
                ]),
        )
        // ── Module Dashboard ────────────────────────────────────────
        .add_component(Section::new("Module Overview").with_level(1))
        .add_component(
            ModuleDashboard::new(vec![
                DashboardModule { name: "SEO".into(), score: 72, interpretation: "Several meta tags missing, thin content on subpages".into(), good_threshold: 80, warn_threshold: 50 },
                DashboardModule { name: "Accessibility".into(), score: 92, interpretation: "Excellent WCAG 2.1 AA compliance".into(), good_threshold: 80, warn_threshold: 50 },
                DashboardModule { name: "Performance".into(), score: 68, interpretation: "LCP too slow, render-blocking resources".into(), good_threshold: 80, warn_threshold: 50 },
                DashboardModule { name: "Security".into(), score: 85, interpretation: "Strong HTTPS setup, minor header improvements needed".into(), good_threshold: 80, warn_threshold: 50 },
            ])
            .with_title("Module Scores"),
        )
        // ── Severity Overview ───────────────────────────────────────
        .add_component(
            SeverityOverview::new(3, 4, 5, 3)
                .with_title("Issue Severity Breakdown"),
        )
        // ── SEO Findings ────────────────────────────────────────────
        .add_component(Section::new("SEO Analysis").with_level(1))
        .add_component(
            Finding::new("Missing Meta Descriptions", Severity::High,
                "12 pages lack meta descriptions, reducing click-through rates from search results. Pages without meta descriptions see 20-30% lower CTR on average.")
                .with_recommendation("Add unique, compelling meta descriptions (150-160 characters) to all pages. Prioritize high-traffic landing pages first.")
                .with_affected("example.com/products, example.com/about, example.com/blog/*")
                .with_category("On-Page SEO"),
        )
        .add_component(
            Finding::new("Duplicate Title Tags", Severity::Medium,
                "5 pages share the same title tag. This confuses search engines and dilutes ranking potential.")
                .with_recommendation("Create unique, descriptive title tags for each page. Include primary keywords and keep titles under 60 characters.")
                .with_affected("example.com/services/*, example.com/solutions/*")
                .with_category("On-Page SEO"),
        )
        .add_component(
            Finding::new("Missing Canonical Tags", Severity::Medium,
                "URL parameters create duplicate content (e.g., ?sort=price, ?page=2). Without canonical tags, search engines may index multiple versions.")
                .with_recommendation("Implement self-referencing canonical tags on all pages. Add canonical tags pointing to the main version for parameterized URLs.")
                .with_category("Technical SEO"),
        )
        // ── Performance Findings ────────────────────────────────────
        .add_component(Section::new("Performance Analysis").with_level(1))
        .add_component(
            Finding::new("Largest Contentful Paint Too Slow", Severity::High,
                "LCP measured at 3.2 seconds (target: <2.5s). The hero image on the homepage is 2.4 MB and loads without optimization.")
                .with_recommendation("Compress hero image to WebP format (saves ~60%), implement lazy loading for below-fold images, and add width/height attributes.")
                .with_affected("example.com/ (homepage)")
                .with_category("Core Web Vitals"),
        )
        .add_component(
            Finding::new("Render-Blocking Resources", Severity::Medium,
                "3 CSS files and 2 JavaScript files block initial rendering. Total blocking time: 850ms.")
                .with_recommendation("Inline critical CSS, defer non-essential JavaScript, and use async loading for third-party scripts.")
                .with_category("Loading Performance"),
        )
        // ── Accessibility Findings ──────────────────────────────────
        .add_component(Section::new("Accessibility Analysis").with_level(1))
        .add_component(
            Finding::new("Missing Alt Text on Images", Severity::High,
                "8 images lack alternative text, making content inaccessible to screen reader users. This also impacts image SEO.")
                .with_recommendation("Add descriptive alt text to all informational images. Use empty alt attributes (alt=\"\") only for decorative images.")
                .with_affected("example.com/products (5 images), example.com/team (3 images)")
                .with_category("WCAG 2.1 AA"),
        )
        .add_component(
            Finding::new("Keyboard Navigation Complete", Severity::Info,
                "All interactive elements are reachable via keyboard. Focus indicators are visible and follow a logical tab order.")
                .with_category("WCAG 2.1 AA"),
        )
        // ── Security Findings ───────────────────────────────────────
        .add_component(Section::new("Security Analysis").with_level(1))
        .add_component(
            Finding::new("Missing Content-Security-Policy Header", Severity::Medium,
                "No CSP header detected. This leaves the site vulnerable to cross-site scripting (XSS) attacks.")
                .with_recommendation("Implement a Content-Security-Policy header. Start with report-only mode to identify issues, then enforce.")
                .with_category("HTTP Headers"),
        )
        .add_component(
            Finding::new("SSL/TLS Configuration Strong", Severity::Info,
                "TLS 1.3 supported, HSTS enabled with 1-year max-age, certificate valid until December 2026. No mixed content detected.")
                .with_category("Transport Security"),
        )
        // ── Technical SEO Checklist ─────────────────────────────────
        .add_component(Section::new("Technical SEO Checklist").with_level(1))
        .add_component(
            AuditTable::new(vec![
                TableColumn::new("Check").with_width("35%"),
                TableColumn::new("Status").with_width("15%"),
                TableColumn::new("Details").with_width("50%"),
            ])
            .with_title("Technical Audit Results")
            .add_row(vec!["robots.txt", "Pass", "Correctly configured, allows search engine crawling"])
            .add_row(vec!["XML Sitemap", "Pass", "Found at /sitemap.xml, 156 URLs indexed"])
            .add_row(vec!["HTTPS", "Pass", "TLS 1.3, valid certificate, no mixed content"])
            .add_row(vec!["Canonical Tags", "Fail", "Missing on parameterized URLs"])
            .add_row(vec!["Structured Data", "Pass", "JSON-LD schema on product and organization pages"])
            .add_row(vec!["Mobile Friendly", "Pass", "Responsive design, passes mobile-friendly test"])
            .add_row(vec!["Core Web Vitals", "Warning", "LCP: 3.2s (needs improvement), FID: 45ms, CLS: 0.05"])
            .add_row(vec!["Open Graph Tags", "Pass", "Present on all main pages"]),
        )
        // ── Module Comparison ───────────────────────────────────────
        .add_component(Section::new("Score Comparison").with_level(1))
        .add_component(
            ModuleComparison::new(vec![
                ComparisonModule::new("SEO", 72).with_color("#3b82f6"),
                ComparisonModule::new("Accessibility", 92).with_color("#22c55e"),
                ComparisonModule::new("Performance", 68).with_color("#f59e0b"),
                ComparisonModule::new("Security", 85).with_color("#8b5cf6"),
            ])
            .with_title("Module Scores Compared"),
        )
        // ── Action Roadmap ──────────────────────────────────────────
        .add_component(Section::new("Recommended Actions").with_level(1))
        .add_component(ActionRoadmap::new(vec![
            RoadmapColumn {
                title: "Immediate (Week 1-2)".into(),
                accent_color: Some("#ef4444".into()),
                items: vec![
                    RoadmapItem { action: "Add missing meta descriptions".into(), role: "Content Team".into(), priority: "High".into(), effort: Some("4 hours".into()), benefit: "Higher CTR from search results".into() },
                    RoadmapItem { action: "Add alt text to images".into(), role: "Content Team".into(), priority: "High".into(), effort: Some("2 hours".into()), benefit: "Accessibility compliance, image SEO".into() },
                    RoadmapItem { action: "Compress hero image".into(), role: "Developer".into(), priority: "High".into(), effort: Some("1 hour".into()), benefit: "Faster LCP, better Core Web Vitals".into() },
                ],
            },
            RoadmapColumn {
                title: "Short-term (Week 3-4)".into(),
                accent_color: Some("#f59e0b".into()),
                items: vec![
                    RoadmapItem { action: "Implement canonical tags".into(), role: "Developer".into(), priority: "Medium".into(), effort: Some("3 hours".into()), benefit: "Eliminate duplicate content issues".into() },
                    RoadmapItem { action: "Defer render-blocking resources".into(), role: "Developer".into(), priority: "Medium".into(), effort: Some("4 hours".into()), benefit: "Faster initial page load".into() },
                    RoadmapItem { action: "Add CSP header".into(), role: "DevOps".into(), priority: "Medium".into(), effort: Some("2 hours".into()), benefit: "XSS protection".into() },
                ],
            },
            RoadmapColumn {
                title: "Long-term (Month 2-3)".into(),
                accent_color: Some("#22c55e".into()),
                items: vec![
                    RoadmapItem { action: "Fix duplicate title tags".into(), role: "Content Team".into(), priority: "Medium".into(), effort: Some("3 hours".into()), benefit: "Better search differentiation".into() },
                    RoadmapItem { action: "Implement image CDN".into(), role: "DevOps".into(), priority: "Low".into(), effort: Some("8 hours".into()), benefit: "Faster global image delivery".into() },
                ],
            },
        ]))
        // ── Closing Note ────────────────────────────────────────────
        .add_component(
            Callout::info("This report was generated automatically. Scores are based on automated checks and may not capture all aspects of your website. A manual review is recommended for critical findings.")
                .with_title("Note"),
        )
        .build();

    let pdf = engine.render_pdf(&report)?;
    let output_path = "examples/output/seo_audit_report.pdf";
    std::fs::write(output_path, &pdf)?;

    println!("✓ SEO Audit report generated successfully!");
    println!("  Output: {}", output_path);
    println!("  Size: {} KB", pdf.len() / 1024);

    Ok(())
}
