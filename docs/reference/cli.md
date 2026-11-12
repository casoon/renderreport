---
title: CLI
description: The renderreport binary and its commands.
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
  -o, --output <OUTPUT>  Output path [default: report.pdf, or report.typ with --typ]
      --typ              Write the generated Typst source instead of compiling it to PDF
      --pack-path <DIR>  Directory to search for the pack named by the request's `pack_id`
                         (repeatable; searched before the default `./packs`)
      --data <FILE>      JSON (or .csv) data bound into the request's `{{ placeholders }}`,
                         `for` and `if`
```

Reads a [render request](../render-request/) and writes the PDF, or with `--typ` the Typst source
the engine would compile (see [the generated Typst source](../../guides/typst-source/)). Exit
code 0 on success, 1 with `Error: …` and the cause on stderr for unreadable input, invalid JSON
or a failed render.

The CLI engine uses the default configuration: packs from `./packs` and
`~/.renderreport/packs` (after any `--pack-path`), system fonts plus the bundled Fira Sans. A
request with `pack_id` loads that pack before rendering.

With `--data` the request is a template: placeholders, `for` and `if` are resolved against the
data file before rendering, see [templates and data](../../guides/templates-and-data/).

## `renderreport list-components`

```text
Usage: renderreport list-components [OPTIONS]

Options:
      --markdown             Output as Markdown instead of plain text
      --category <CATEGORY>  Filter by category (case-insensitive substring match)
```

Prints id, category, layout hint and description of every built-in component, sorted by id.
See [components](../../concepts/components/) for the output of the current version.

## `renderreport validate`

```text
Usage: renderreport validate [OPTIONS] [INPUT]

Arguments:
  [INPUT]  Path to JSON input file (reads from stdin if omitted)

Options:
      --pack-path <DIR>  Directory to search for the pack named by the request's `pack_id`
      --data <FILE>      JSON (or .csv) data bound into the request's `{{ placeholders }}`,
                         `for` and `if`
```

Checks a [render request](../render-request/) without rendering it: the request's own fields,
each component's `data` against the component, components nested in other components, and
component types that neither the catalog nor the request's pack defines. Each problem is printed
as `path: reason` on stderr, for example `components[0].data: score-card: score: invalid type:
string "hi", expected u32`. Exit code 0 when the request is valid, 1 otherwise.

`render` stays lenient and renders data that does not fit; `validate` is what reports it. With
`--data` the template is bound first and the resolved request is checked, so reported paths
refer to the resolved request.

## `renderreport schema`

```text
Usage: renderreport schema [OPTIONS]

Options:
      --component <COMPONENT>  Component ID; without it the schema of the whole request
```

Prints a JSON Schema (draft 2020-12) to stdout. Without `--component` it describes the whole
request, with every `components` entry checked against the schema of its `type`; types outside
the catalog (pack components) are accepted with any `data`. With `--component <id>` it describes
the `data` of one component. Save the schema to a file and point an editor at it for completion
and checking while writing a request:

```sh
renderreport schema > renderreport.schema.json
```
