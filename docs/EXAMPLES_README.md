# @casoon/renderreport-examples

Demo applications and reference implementations for RenderReport.

## Examples

### seo_audit.rs
Complete SEO audit report with Lighthouse scores, findings, and technical checklist.

```bash
cargo run --example seo_audit
```

### simple_report.rs
Minimal example showing basic usage.

```bash
cargo run --example simple_report
```

### custom_theme.rs
Demonstrates theme customization with brand colors.

```bash
cargo run --example custom_theme
```

## Running Examples

```bash
# Clone the repository
git clone https://github.com/casoon/renderreport
cd renderreport

# Run an example
cargo run --example seo_audit

# Output: seo_audit_report.pdf
```

## Example: SEO Audit Report

```rust
use renderreport::prelude::*;

let engine = Engine::new()?;

let report = engine
    .report("seo-audit")
    .title("SEO Audit Report")
    .subtitle("example.com")
    // Summary
    .add_component(SummaryBox::new("Executive Summary")
        .add_item_with_status("Score", "78/100", ScoreStatus::Warning)
        .add_item("Issues", "15"))
    // Scores
    .add_component(ScoreCard::new("Performance", 85))
    .add_component(ScoreCard::new("SEO", 72))
    // Findings
    .add_component(Finding::new(
        "Missing Meta Description",
        Severity::High,
        "The homepage lacks a meta description."
    ).with_recommendation("Add a 150-160 character description."))
    // Technical Audit
    .add_component(AuditTable::new(vec![
        TableColumn::new("Check"),
        TableColumn::new("Status"),
    ])
    .add_row(vec!["robots.txt", "✓ Pass"])
    .add_row(vec!["Sitemap", "✓ Pass"])
    .add_row(vec!["HTTPS", "✓ Pass"]))
    .build();

let pdf = engine.render_pdf(&report)?;
std::fs::write("report.pdf", pdf)?;
```

## License

MIT
