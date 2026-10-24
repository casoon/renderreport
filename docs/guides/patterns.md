---
title: Report patterns
description: Assemble an audit, executive or marketing report from a fixed narrative order instead of placing every component by hand.
order: 6
---

A pattern collects component data in a fixed narrative order — hero, context, analysis,
solution, actions, call to action — and turns it into a component list with section headings
in between. Three patterns are built in:

| Pattern | Builder methods |
| --- | --- |
| `AuditPattern` | `with_hero_summary`, `add_context`, `add_findings`, `add_impact_grid`, `add_checklist`, `add_roadmap`, `add_timeline`, `with_cta` |
| `ExecutivePattern` | `with_hero_summary`, `add_summary`, `add_key_metrics`, `add_top_findings`, `add_risk_overview`, `add_recommendation`, `add_implementation_plan`, `add_timeline`, `with_cta` |
| `MarketingPattern` | `with_product_hero`, `add_problem_section`, `add_features`, `add_benefits`, `add_comparison`, `add_process_flow`, `add_testimonial`, `add_use_case`, `add_pull_quote`, `add_pricing_cards`, `with_cta` |

Each method takes the component data as JSON (`serde_json::Value`), the same `data` object a
[render request](../../reference/render-request/) carries. `to_components()` returns the list
of `{ "type", "data" }` items, which goes into a request like any other components:

```rust
use renderreport::{patterns::AuditPattern, Engine, RenderRequest};
use serde_json::json;

let components = AuditPattern::new()
    .with_hero_summary(json!({ "domain": "example.com", "score": 68, "grade": "C+" }))
    .add_findings(vec![json!({
        "title": "Missing CSP header",
        "severity": "high",
        "description": "No Content-Security-Policy is sent.",
    })])
    .to_components();

let mut request = RenderRequest::new("default");
request.title = Some("Security Audit".into());
request.components = components;
let pdf = Engine::new()?.render_pdf(&request)?;
```

`BasePattern` holds the six slots and their order; the full audit example is
[examples/audit_pattern_example.rs](https://github.com/casoon/renderreport/blob/main/examples/audit_pattern_example.rs).
