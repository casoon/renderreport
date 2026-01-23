# @casoon/renderreport

[![Crates.io](https://img.shields.io/crates/v/renderreport.svg)](https://crates.io/crates/renderreport)
[![Documentation](https://docs.rs/renderreport/badge.svg)](https://docs.rs/renderreport)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

> ⚠️ **Early Development Stage** - This project is in active early development. APIs may change, and some features are still being implemented and optimized.

Data-driven report generation with Typst as embedded render engine.

Build professional PDF reports without learning Typst. Use components, themes, and template packs to create reports from structured data.

## Features

- **Component-based**: Build reports using pre-built components (ScoreCard, Finding, Table, etc.)
- **Theme Tokens**: CSS-variable-like theming system for consistent styling
- **Template Packs**: Extend with custom templates and components
- **Embedded Typst**: No CLI dependency, Typst runs as a library
- **Type-safe**: Full Rust API with compile-time guarantees

## Quick Start

```rust
use renderreport::prelude::*;

fn main() -> renderreport::Result<()> {
    let engine = Engine::new()?;

    let report = engine
        .report("default")
        .title("My Report")
        .add_component(ScoreCard::new("Quality", 95))
        .add_component(Finding::new(
            "Issue Found",
            Severity::Medium,
            "Description of the issue"
        ))
        .build();

    let pdf = engine.render_pdf(&report)?;
    std::fs::write("report.pdf", pdf)?;
    Ok(())
}
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
renderreport = "0.1"
```

## Components

### Standard Components

| Component | Description |
|-----------|-------------|
| `ScoreCard` | Metric with score visualization |
| `Finding` | Audit finding with severity |
| `AuditTable` | Data table for results |
| `Section` | Document section with heading |
| `SummaryBox` | Executive summary widget |
| `Callout` | Information callout box |
| `Image` | Image with caption |

### Advanced Components

Inspired by [JasperReports](https://jasperreports.sourceforge.net/), [Eclipse BIRT](https://eclipse-birt.github.io/birt-website/), and [Pentaho Reporting](https://help.pentaho.com/):

| Component | Description | Inspired By |
|-----------|-------------|-------------|
| `List` | Iterative list with nested items | JasperReports List Component |
| `Grid` | Multi-column grid layout | Pentaho Row/Block Layout |
| `ProgressBar` | Visual progress indicator | JasperReports Charts |
| `KeyValueList` | Key-value pairs display | BIRT Parameters |
| `Divider` | Horizontal separator line | BIRT Band Elements |
| `Watermark` | Background watermark text | Pentaho Watermark |
| `PageBreak` | Force page break | BIRT Page Setup |

### Chart Components

Visualize data with comprehensive chart types inspired by JasperReports Chart Components:

| Component | Description | Chart Types |
|-----------|-------------|-------------|
| `Chart` | Full-featured charts | Bar, Line, Pie, Area, Scatter, Radar |
| `Sparkline` | Inline mini-charts | Line, Bar |
| `Gauge` | Progress/metric meters | Circular, Thermometer, Horizontal |

### Data Analysis Components

Complex data aggregation inspired by BIRT Cross Tabs and JasperReports Crosstabs:

| Component | Description | Use Case |
|-----------|-------------|----------|
| `Crosstab` | Dynamic pivot table with aggregation | Sales by Region × Product |
| `PivotTable` | Pre-aggregated pivot display | Summary reports |

### Barcode Components

Generate barcodes in multiple formats (inspired by JasperReports Barcode and Pentaho):

| Component | Formats Supported |
|-----------|------------------|
| `Barcode` | Code128, Code39, EAN-13, EAN-8, UPC-A, UPC-E, QR Code, Data Matrix, PDF417, ITF, Codabar |

### Text & Field Components

Simple text display inspired by BIRT and Pentaho field elements:

| Component | Description | Example Use |
|-----------|-------------|-------------|
| `Label` | Simple styled text | Headings, captions |
| `Text` | Multi-line text block | Paragraphs, descriptions |
| `NumberField` | Formatted numbers | Currency: $1,234.56, Percentage: 87.5% |
| `DateField` | Formatted dates | 2024-03-15, 15.03.2024, 03/15/2024 |
| `ResourceField` | Localized strings | i18n support |

## Theming

```rust
use renderreport::theme::{Theme, TokenValue};

let mut theme = Theme::new("brand", "Brand Theme");
theme.tokens.set("color.primary", TokenValue::Color("#1a56db".into()));
theme.tokens.set("color.ok", TokenValue::Color("#059669".into()));

let report = engine.report("default")
    .theme(theme)
    // ...
```

See [docs/CONVENTIONS.md](docs/CONVENTIONS.md) for all available tokens.

## Template Packs

Load external packs for specialized reports:

```rust
engine.load_pack("seo-audit")?;

let report = engine
    .report("seo-audit")
    .pack("seo-audit")
    // ...
```

Create your own packs with custom templates and components. See [@casoon/renderreport-packs](https://github.com/casoon/renderreport-packs).

## Project Structure

```
renderreport/
├── src/
│   ├── lib.rs           # Main library entry
│   ├── engine/          # Core rendering engine
│   ├── components/      # Standard components
│   ├── theme/           # Theme & token system
│   ├── pack/            # Pack loading system
│   ├── render/          # Typst compilation
│   └── vfs/             # Virtual filesystem
├── templates/           # Built-in Typst templates
├── packs/               # Bundled template packs
├── examples/            # Usage examples
└── docs/                # Documentation
```

## Roadmap

### Phase 1: MVP (Current)
- [x] Core engine with Typst integration
- [x] Standard component library
- [x] Theme token system
- [x] Basic pack loading
- [ ] Full test coverage

### Phase 2: Pack System
- [ ] Pack validation & versioning
- [ ] Remote pack loading
- [ ] Pack registry

### Phase 3: Ecosystem
- [ ] Preview server for development
- [ ] Visual regression tests
- [ ] HTML output (experimental)
- [ ] WASM support

## Related Projects

- [@casoon/renderreport-packs](https://github.com/casoon/renderreport-packs) - Official template packs
- [@casoon/renderreport-examples](https://github.com/casoon/renderreport-examples) - Demo applications

## License

MIT OR Apache-2.0
