---
title: Overview
description: What renderreport does, where it stops, and how this documentation is organised.
order: 0
---

renderreport turns structured data into PDF reports. You describe a report as a list of
components (score cards, findings, tables, timelines, pricing cards and more), pick a theme, and
the engine writes the Typst source and compiles it to PDF in-process. You never write Typst
yourself, and no `typst` binary is involved.

## What it covers

- 80 built-in components in 8 categories, from layout and text to metrics, narrative and
  marketing blocks.
- Theme tokens (`color.primary`, `spacing.4`, `page.margin` …) that every component reads, merged
  from the default theme, a template pack and the request.
- Template packs: your own Typst templates, components and themes, loaded from disk.
- Accessible output: every PDF is validated against PDF/UA-1 while it is written.
- Four ways in: the Rust library, the `renderreport` CLI, the npm package (which installs the CLI)
  and a Wasm build for JavaScript hosts such as Cloudflare Workers.

## Where it stops

renderreport lays out data you already have. It does not collect metrics, run audits or fetch
anything; that is the job of the program that builds the report. It is not a Typst editor
either: to compile hand-written `.typ` files, use Typst or
[typstgen](https://github.com/casoon/typstgen).

## How this documentation is organised

- **Getting started**: [installation](getting-started/installation/) and a
  [first report](getting-started/quickstart/) from Rust and from JSON.
- **Concepts**: [how a report is rendered](concepts/how-it-works/),
  [components](concepts/components/) and [themes](concepts/themes/).
- **Guides**: [the CLI and JSON](guides/json-and-cli/), [Wasm](guides/wasm/),
  [custom components and packs](guides/custom-components/) and [the generated Typst source](guides/typst-source/).
- **Reference**: [the render request](reference/render-request/), [the CLI](reference/cli/) and
  [the library API](reference/library/).

The project is in early development: component data shapes can still change between minor
versions. The [changelog](../changelog/) lists every breaking change.
