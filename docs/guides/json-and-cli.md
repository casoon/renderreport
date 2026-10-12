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
`report.pdf`. On success the CLI prints the number of bytes written; on failure it prints an
error to stderr and exits with code 1. To see the Typst diagnostic of a failed render, render the
same request from Rust, where the error carries it.

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

Component data has no published JSON Schema yet. The fields are the ones each component's Rust
builder writes, and a template may expect fields that the builder computes, such as a score
card's `computed_status`. The reliable way to get a shape:

1. Build the component once in Rust.
2. Print the request with `serde_json::to_string_pretty(&request)`.
3. Use that as the template for the JSON your program writes.

[examples/requests/health-check.json](https://github.com/casoon/renderreport/blob/main/examples/requests/health-check.json)
was made this way.

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
