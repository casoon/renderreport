# @casoon/renderreport

[![CI](https://github.com/casoon/renderreport/actions/workflows/ci.yml/badge.svg)](https://github.com/casoon/renderreport/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/renderreport.svg)](https://crates.io/crates/renderreport)
[![Documentation](https://docs.rs/renderreport/badge.svg)](https://docs.rs/renderreport)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.92%2B-blue.svg)](https://www.rust-lang.org)

> ⚠️ **Early Development Stage** - This project is in active early development. APIs may change, and some features are still being implemented and optimized.

Data-driven report generation with Typst as embedded render engine.

Build professional PDF reports without learning Typst. Use components, themes, and template packs to create reports from structured data.

<p align="center">
  <img src="https://raw.githubusercontent.com/casoon/renderreport/main/examples/terminal/render.svg" width="528" height="188" alt="Terminal: renderreport render health-check.json -o health-check.pdf prints Written 35217 bytes to health-check.pdf; renderreport list-components | tail -1 prints 80 component(s).">
</p>

**Website:** [renderreport.casoon.de](https://renderreport.casoon.de/en/) · **Docs and examples:** [casoon.github.io/renderreport](https://casoon.github.io/renderreport/)

## Features

- **80 components** — score cards, findings, tables, charts, timelines, pricing cards, barcodes
  and more, each with a Rust builder and a JSON form.
- **Theme tokens** — colours, fonts, spacing and page layout as named tokens, merged per request.
- **Template packs** — page templates, components, themes and assets in a folder.
- **Embedded Typst** — no Typst CLI; the engine compiles PDF/UA-1 tagged PDFs as a library.
- **CLI, Rust and Wasm** — render from JSON on the command line, from Rust, or in a
  Cloudflare Worker.

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

### Library

Add to your `Cargo.toml`:

```toml
[dependencies]
renderreport = "0.6"
```

### CLI

```sh
curl -fsSL https://raw.githubusercontent.com/casoon/renderreport/main/install.sh | sh
```

Downloads the archive for your platform from the latest release, verifies it against the
published `.sha256` file and installs the binary to `/usr/local/bin`. Set
`RENDERREPORT_INSTALL_DIR` to install elsewhere, or `VERSION` to pin a release. On Windows, take
the `.zip` from the [releases page](https://github.com/casoon/renderreport/releases).

## Documentation

The [documentation](https://casoon.github.io/renderreport/) covers everything beyond the
quick start:

- [Installation](https://casoon.github.io/renderreport/docs/getting-started/installation/) — library, CLI binaries, npm
- [Components](https://casoon.github.io/renderreport/docs/concepts/components/) — all components, categories and page-break hints
- [Themes and tokens](https://casoon.github.io/renderreport/docs/concepts/themes/)
- [The CLI and JSON](https://casoon.github.io/renderreport/docs/guides/json-and-cli/)
- [Render request](https://casoon.github.io/renderreport/docs/reference/render-request/)
- [Template packs](https://casoon.github.io/renderreport/docs/guides/packs/)
- [Report patterns](https://casoon.github.io/renderreport/docs/guides/patterns/)
- [The generated Typst source](https://casoon.github.io/renderreport/docs/guides/typst-source/)
- [Wasm and Cloudflare Workers](https://casoon.github.io/renderreport/docs/guides/wasm/)
- [Library API](https://casoon.github.io/renderreport/docs/reference/library/) — full API on [docs.rs](https://docs.rs/renderreport)

Contributing: [CONTRIBUTING.md](CONTRIBUTING.md). Changes: [CHANGELOG.md](CHANGELOG.md).

## License

MIT
