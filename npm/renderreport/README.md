# @casoon/renderreport

Data-driven report generation with Typst as embedded render engine.

Build PDF reports without learning Typst: describe the report as JSON components, and the
`renderreport` CLI renders it to PDF. This package installs that CLI.

**Docs and examples:** [casoon.github.io/renderreport](https://casoon.github.io/renderreport/)

## Installation

```bash
npm install @casoon/renderreport
```

`postinstall` downloads the binary for your platform from the GitHub release of this version and
verifies its SHA-256 checksum.

## Usage

The package provides the `renderreport` command; it has no JavaScript API.

```bash
# From a JSON file
npx renderreport render report.json -o report.pdf

# From stdin
cat report.json | npx renderreport render -o report.pdf

# All component types of the installed version
npx renderreport list-components
```

On failure the CLI prints the error, including the Typst diagnostic, to stderr and exits with
code 1.

### From Node.js

Call the CLI as a child process and pass the request on stdin:

```js
const { execFileSync } = require("node:child_process");

execFileSync("npx", ["renderreport", "render", "-o", "report.pdf"], {
  input: JSON.stringify(request),
});
```

For JavaScript runtimes without child processes, such as Cloudflare Workers, renderreport also
builds to Wasm; see the [Wasm guide](https://casoon.github.io/renderreport/docs/guides/wasm/).

## Report format

A report is a `RenderRequest`:

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

Each component is `{ "type": "<component-id>", "data": { … } }`. The
[render request reference](https://casoon.github.io/renderreport/docs/reference/render-request/)
lists every field, and the [JSON guide](https://casoon.github.io/renderreport/docs/guides/json-and-cli/)
explains how to get the complete data shape of a component.

## Supported Platforms

| Platform | Architecture |
|---|---|
| Linux | x64, ARM64 |
| macOS | ARM64 (Apple Silicon) |
| Windows | x64, ARM64 |

## Rust Library

The engine is a Rust crate: [crates.io/crates/renderreport](https://crates.io/crates/renderreport).

## License

MIT
