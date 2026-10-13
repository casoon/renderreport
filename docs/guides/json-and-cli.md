---
title: The CLI and JSON
description: Render reports from JSON files or stdin, from any language that can write JSON.
order: 1
---

The `renderreport` binary takes a render request as JSON and writes a PDF. It is the way in for
programs that are not written in Rust: build the JSON, call the CLI, read the PDF.

```sh
renderreport render report.json -o report.pdf
cat report.json | renderreport render -o report.pdf
```

Without an input path the request is read from stdin. Without `-o` the PDF is written to
`report.pdf`. On success the CLI prints the number of bytes written; on failure it prints the
error with its cause, down to the Typst diagnostic, to stderr and exits with code 1:

```text
Error: failed to render PDF: Render error: Typst compilation failed: … dictionary does not contain key "callout_type"
```

## Shape of the request

```json
{
  "template_id": "default",
  "title": "Website Health Check",
  "subtitle": "shop.example.com — October 2026",
  "metadata": { "author": "Web Team", "date": "2026-10-12" },
  "components": [
    { "type": "section", "data": { "title": "Summary", "level": 1, "content": [] } }
  ]
}
```

Only `template_id` and `components` are required. The [render request](../../reference/render-request/)
reference lists every field.

## Component data

The fields of a component are the fields of its Rust struct on
[docs.rs](https://docs.rs/renderreport), in snake case. Only the fields without a default are
required; for a score card that is `title` and `score`:

```json
{ "type": "score-card", "data": { "title": "Accessibility", "score": 91 } }
```

Before rendering, the engine runs each built-in component's data through its struct, the same way
the Rust builder does: defaults are filled in, and derived fields such as a score card's
`computed_status` or a word search's grid are computed. Fields the struct does not know are kept.
Data that does not fit the struct (a required field missing, a wrong type) is passed on unchanged,
and the render fails with the Typst error, naming the component.

To check what a request turns into, write the Typst source instead of the PDF:

```sh
renderreport render report.json --typ -o report.typ
```

## From Node

The npm package installs the CLI; call it like any other binary:

```js
import { execFileSync } from 'node:child_process';

execFileSync('npx', ['renderreport', 'render', '-o', 'report.pdf'], {
  input: JSON.stringify(request),
});
```

For a JavaScript runtime without processes, such as Cloudflare Workers, use the
[Wasm build](../wasm/).
