---
title: The generated Typst source
description: Get the .typ source the engine would compile, for snapshot tests, linting or your own post-processing.
order: 4
---

`Engine::render_typ` runs every step up to compilation and returns the Typst source as a string:

```rust
let source: String = engine.render_typ(&report)?;
std::fs::write("report.typ", &source)?;
```

From the CLI, `--typ` writes the same source for a JSON request:

```sh
renderreport render report.json --typ -o report.typ
```

The source is self-contained apart from images and fonts: theme tokens, helpers, page setup, the
functions of the components in use and one call per component, in that order (see
[how a report is rendered](../../concepts/how-it-works/)).

## Uses

- **Snapshot tests.** The output is deterministic for a given request and engine version:
  tokens are sorted and component functions are emitted in a fixed order. renderreport's own
  test suite compares against `fixtures/snapshots/minimal_report.typ`.
- **Debugging a template.** When a render fails with a Typst diagnostic, the source shows the
  exact call and data that caused it.
- **Lint and format pipelines,** for example `typstyle`, before compiling with your own Typst
  setup.

## Data as Typst literals

Component data is turned into Typst literals by `to_typst_dict`: `null` becomes `none`, strings
become escaped string literals, arrays become arrays and objects become dictionaries.
