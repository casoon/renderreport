---
title: Custom components and packs
description: Add your own Typst component to the engine, or ship it in a template pack.
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

To ship components together with a page layout, themes and assets, put them in a
[template pack](../packs/): the engine renders a pack's components like built-in ones, also
inside `grid-component` and `flow-group`, and a pack component with a built-in id replaces it.
