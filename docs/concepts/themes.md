---
title: Themes and tokens
description: Named design tokens that every component reads, and the order in which themes are merged.
order: 3
---

Components never hard-code colours, fonts or spacing. They read tokens such as `color.primary`,
`font.body` or `spacing.4`, which the engine writes to the top of the Typst source as variables
(`color.primary` becomes `color-primary`). Change a token and every component that uses it
follows.

## Merge order

Per token, later themes win:

1. The engine's default theme (`Theme::default_theme()`, or one set with
   `Engine::set_default_theme`).
2. The default theme of the pack named in the request, if any.
3. The theme in the request.

A request theme only needs the tokens it changes.

## Tokens

| Group | Tokens |
| --- | --- |
| Colours | `color.primary`, `color.secondary`, `color.text`, `color.text-muted`, `color.background`, `color.surface`, `color.surface-soft`, `color.surface-alt`, `color.border` |
| Status colours | `color.ok`, `color.warn`, `color.bad`, `color.info`, each with a `-soft` variant, and `color.accent-soft` |
| Fonts | `font.body`, `font.heading`, `font.mono` |
| Spacing | `spacing.1` … `spacing.7` |
| Tables | `table.header-bg`, `table.row-alt-bg`, `table.border`, `table.border-width` |
| Page | `page.margin`, `page.margin-top`, `page.margin-bottom`, `page.header-height`, `page.footer-height` |

The default values are in `ThemeTokens::default()` in
[src/theme/tokens.rs](https://github.com/casoon/renderreport/blob/main/src/theme/tokens.rs).

## In Rust

```rust
use renderreport::theme::{Theme, TokenValue};

let mut theme = Theme::new("brand", "Brand");
theme.tokens.set("color.primary", TokenValue::Color("#0F766E".into()));
theme.tokens.set("page.margin", TokenValue::Size("22mm".into()));

let report = engine.report("default").theme(theme) /* … */ .build();
```

## In JSON

```json
"theme": {
  "id": "brand",
  "name": "Brand",
  "tokens": {
    "color.primary": "#0F766E",
    "page.margin": "22mm"
  }
}
```

A value starting with `#` becomes a colour; any other string is written to Typst as it is, so
sizes like `22mm` work directly. A font name is a Typst string and needs its own quotes:
`"font.body": "\"Inter\""`.

The [themed health check](../../../showcase/health-check-themed/) is the
[health check request](../../../showcase/health-check/) with exactly such a block added.
