# @casoon/renderreport

Data-driven report generation with Typst as embedded render engine.

Build professional PDF reports without learning Typst. Use components, themes, and template packs to create reports from structured data.

## Installation

```bash
npm install @casoon/renderreport
```

The correct binary for your platform is installed automatically.

## Usage

### Node.js API

```js
const { render, renderSync } = require("@casoon/renderreport");

// Async
const pdf = await render({
  template_id: "default",
  title: "My Report",
  subtitle: "Generated with RenderReport",
  metadata: {
    author: "Casoon",
    date: "2025-01-15"
  },
  components: [
    {
      type: "score-card",
      data: { title: "Performance", score: 92 }
    },
    {
      type: "callout",
      data: { variant: "info", content: "Everything looks great!" }
    }
  ]
});

require("fs").writeFileSync("report.pdf", pdf);

// Sync
const pdfSync = renderSync(reportDefinition);

// Write directly to file
await render(reportDefinition, { output: "report.pdf" });
```

### CLI

```bash
# From JSON file
npx renderreport render input.json -o report.pdf

# From stdin
cat input.json | npx renderreport render -o report.pdf
```

## JSON Schema

The report definition follows the `RenderRequest` format:

```json
{
  "template_id": "default",
  "title": "Report Title",
  "subtitle": "Optional Subtitle",
  "metadata": {
    "author": "Author Name",
    "date": "2025-01-15"
  },
  "components": [
    {
      "type": "component-id",
      "data": { }
    }
  ]
}
```

## Available Components

| Component | ID | Description |
|---|---|---|
| ScoreCard | `score-card` | Metric with score visualization |
| Finding | `finding` | Audit finding with severity |
| AuditTable | `audit-table` | Tabular data |
| Section | `section` | Document section heading |
| Callout | `callout` | Highlighted info box |
| SummaryBox | `summary-box` | Executive summary widget |
| MetricCard | `metric-card` | Key metric display |
| HeroSummary | `hero-summary` | Hero section with metrics |
| Chart | `chart` | Bar, Line, Pie, Area, Scatter, Radar |
| Sparkline | `sparkline` | Inline sparkline chart |
| Gauge | `gauge` | Gauge/thermometer visualization |
| Barcode | `barcode` | Code128, QR, EAN-13, and more |
| List | `list` | Ordered/unordered list |
| ProgressBar | `progress-bar` | Progress visualization |
| KeyValueList | `key-value-list` | Key-value pairs |
| CoverPage | `cover-page` | Report cover page |
| BenchmarkTable | `benchmark-table` | Benchmark comparison |
| Crosstab | `crosstab` | Cross-tabulation |
| TextBlock | `textblock` | Multi-line text block |
| Label | `label` | Styled label |

## Supported Platforms

| Platform | Package |
|---|---|
| Linux x64 | `@casoon/renderreport-cli-linux-x64` |
| Linux ARM64 | `@casoon/renderreport-cli-linux-arm64` |
| macOS x64 (Intel) | `@casoon/renderreport-cli-darwin-x64` |
| macOS ARM64 (Apple Silicon) | `@casoon/renderreport-cli-darwin-arm64` |
| Windows x64 | `@casoon/renderreport-cli-win32-x64` |

## Rust Library

This is the Node.js wrapper. For the Rust library, see [crates.io/crates/renderreport](https://crates.io/crates/renderreport).

## License

MIT OR Apache-2.0
