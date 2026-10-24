---
title: Template packs
description: Bundle templates, components, themes and assets in a folder and select them per request.
order: 4
---

A pack is a folder with a `pack.toml` manifest. A request with `pack_id` uses everything the
pack brings: its components, the template selected by `template_id`, its themes and its assets.

## Bundled packs

### standard
Templates `default` and `minimal`, themes `light`, `dark` and `print`.

### seo-audit
Template `seo-report` (cover page, table of contents) and the components
`lighthouse-scores` and `core-web-vitals`.

## Folder layout

```
pack-name/
├── pack.toml           # Manifest
├── templates/          # Report templates
│   └── *.typ
├── components/         # Custom components
│   └── *.typ
├── themes/             # Theme definitions
│   └── *.toml
└── assets/             # Fonts, images, etc.
```

## Creating a pack

### 1. Create pack.toml

```toml
[pack]
name = "my-pack"
version = "0.1.0"
description = "My custom report pack"
authors = ["Your Name <email@example.com>"]
license = "MIT"

[compatibility]
min_engine_version = "0.1.0"

[templates.my-report]
file = "my_report.typ"
description = "My custom report template"
default_theme = "default"

[themes.default]
file = "default.toml"
default = true

[components.my-widget]
file = "my_widget.typ"
description = "Custom widget component"
```

### 2. Create Theme (themes/default.toml)

```toml
id = "default"
name = "Default Theme"

[tokens]
"color.primary" = "#2563eb"
"color.text" = "#1e293b"
# ... more tokens
```

Every file the manifest lists must exist, or loading the pack fails.

### 3. Create Component (components/my_widget.typ)

The manifest key is the component id. The file must define `#let <id>(data)`. A key that
equals a built-in component id replaces that component (for `image`, define
`report-image`). `overrides` in the manifest is not evaluated.

```typst
#let my-widget(data) = {
  box(
    fill: color-surface,
    inset: spacing-4,
    radius: 4pt,
  )[
    #text(weight: "bold")[#data.title]
    #v(spacing-2)
    #data.content
  ]
}
```

### 4. Create Template (templates/my_report.typ)

A template defines `#let report(meta, body)`. The engine wraps the report content with
`#show: body => report(meta, body)`; `meta` holds `title`, `subtitle` and every `metadata`
entry of the request. The template renders its own title page; the built-in one is skipped.

```typst
#let report(meta, body) = {
  let title = meta.at("title", default: none)
  if title != none {
    align(center)[
      #text(size: font-size-2xl, weight: "bold")[#title]
    ]
    v(spacing-6)
  }
  body
}
```

### 5. Assets (optional)

Files under `assets/` are added to the Typst file system by their path inside `assets/`,
e.g. `assets/logo.svg` is `image("/logo.svg", alt: "Logo")`.

## Using a pack

```rust
use renderreport::Engine;

let mut engine = Engine::new()?;
engine.load_pack("my-pack")?;

let report = engine
    .report("my-report")
    .pack("my-pack")
    .title("Report Title")
    // ... add components, e.g. "my-widget"
    .build();
```

From the CLI: `renderreport render request.json --pack-path ./my-packs`, with `pack_id` and
`template_id` set in the request JSON.

## Manifest fields

| Field | Required | Purpose |
| --- | --- | --- |
| `pack.name`, `pack.version` | yes | Pack name and semver version |
| `pack.description`, `pack.authors`, `pack.license`, `pack.repository`, `pack.keywords`, `pack.capabilities` | no | Metadata |
| `compatibility.min_engine_version`, `max_engine_version`, `typst_version` | no | Recorded, not checked yet |
| `templates.<name>.file` | yes | File in `templates/`; `<name>` is the `template_id` |
| `templates.<name>.default_theme` | no | A theme of this pack, merged when the template is used |
| `themes.<name>.file`, `themes.<name>.default` | yes / no | File in `themes/`; `default = true` marks the pack's default theme |
| `components.<id>.file` | yes | File in `components/` defining `#let <id>(data)` |

Use kebab-case for ids and template names.
