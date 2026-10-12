---
title: Quickstart
description: A first report in Rust, then the same idea as JSON through the CLI.
order: 2
---

## From Rust

```rust
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    let engine = Engine::new()?;

    let report = engine
        .report("default")
        .title("Website Health Check")
        .metadata("author", "Web Team")
        .add_component(Section::new("Summary").with_level(1))
        .add_component(ScoreCard::new("Accessibility", 91).with_description("axe, 42 pages"))
        .add_component(
            Finding::new(
                "212 URLs return a redirect chain",
                Severity::Medium,
                "Old product URLs redirect twice before reaching the current page.",
            )
            .with_recommendation("Point each old URL directly at its final target."),
        )
        .build();

    std::fs::write("report.pdf", engine.render_pdf(&report)?)?;
    Ok(())
}
```

`engine.report("default")` starts a report on the built-in `default` template. Each
`add_component` appends one component; `build()` returns a `RenderRequest`, and `render_pdf`
compiles it to PDF bytes.

## From JSON

The same request as JSON, for the CLI or any language that can write a file:

```json
{
  "template_id": "default",
  "title": "Website Health Check",
  "metadata": { "author": "Web Team" },
  "components": [
    { "type": "section", "data": { "title": "Summary", "level": 1, "content": [] } },
    {
      "type": "callout",
      "data": { "callout_type": "info", "title": "Note", "content": "Generated from JSON." }
    }
  ]
}
```

```sh
renderreport render report.json -o report.pdf
```

Each component is `{ "type": "<component-id>", "data": { … } }`. The fields in `data` are the
ones the component's Rust builder writes, so the easiest way to get a complete shape is to build a
request in Rust once and print it with `serde_json::to_string_pretty(&request)`. The
[health check example](../../../showcase/health-check/) shows a full request next to its PDF.

## Next steps

- [How a report is rendered](../../concepts/how-it-works/)
- [All components](../../concepts/components/), or `renderreport list-components`
- [Themes and tokens](../../concepts/themes/)
