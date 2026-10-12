---
title: CLI
description: The renderreport binary and its two commands.
order: 2
---

The binary is built with the `cli` feature and ships prebuilt with every release, see
[installation](../../getting-started/installation/).

## `renderreport render`

```text
Usage: renderreport render [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Path to JSON input file (reads from stdin if omitted)

Options:
  -o, --output <OUTPUT>  Output PDF path [default: report.pdf]
```

Reads a [render request](../render-request/) and writes the PDF. Exit code 0 on success, 1 with
`Error: …` on stderr for unreadable input, invalid JSON or a failed render.

The CLI engine uses the default configuration: packs from `./packs` and
`~/.renderreport/packs`, system fonts plus the bundled Fira Sans.

## `renderreport list-components`

```text
Usage: renderreport list-components [OPTIONS]

Options:
      --markdown             Output as Markdown instead of plain text
      --category <CATEGORY>  Filter by category (case-insensitive substring match)
```

Prints id, category, layout hint and description of every built-in component, sorted by id.
See [components](../../concepts/components/) for the output of the current version.
