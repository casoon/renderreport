---
title: Custom components and packs
description: Add your own Typst component to the engine, and what template packs contribute today.
order: 3
---

## Your own component

A component is a Typst function that takes one dictionary. Register it under an id, and every
request can use that id:

```rust
use renderreport::components::ComponentId;
use renderreport::prelude::*;

let mut engine = Engine::new()?;
engine.components_mut().register(
    ComponentId::new("note-strip"),
    r#"#let note-strip(data) = block(
  width: 100%,
  inset: spacing-3,
  fill: color-accent-soft,
  radius: 4pt,
)[#text(weight: "bold")[#data.label] #data.text]"#
        .to_string(),
);
```

```json
{ "type": "note-strip", "data": { "label": "Note", "text": "Prices exclude VAT." } }
```

The function name must equal the id, and the template can use every
[theme token](../../concepts/themes/) as a variable. The engine includes the function only in
reports that use it. Components that are not in the built-in catalog are laid out
`keep-together`.

From Rust, implement the `Component` trait (`component_id` and `to_data`) for your struct and
pass it to `add_component` like a built-in one.

## Template packs

A pack is a folder with a `pack.toml` manifest, Typst templates, components and themes. The
engine looks for packs in `./packs` and `~/.renderreport/packs`; add folders with
`EngineConfig::add_pack_path`.

```rust
engine.load_pack("seo-audit")?;

let report = engine.report("seo-report").pack("seo-audit") /* … */ .build();
```

What rendering uses from a pack today: its **default theme**, merged between the engine's
default theme and the request theme. The templates and components of a pack are loaded and
available through `Engine::get_pack`, but the engine does not yet add them to the Typst
source. To use a pack component now, register its template as shown above.

The manifest format and folder layout are described in
[docs/PACKS_README.md](https://github.com/casoon/renderreport/blob/main/docs/PACKS_README.md).
