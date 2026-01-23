# @casoon/renderreport-packs

Official template packs for RenderReport.

## Available Packs

### standard
Default components, layouts, and themes. Included with renderreport.

### seo-audit
SEO audit report templates with Lighthouse scores, Core Web Vitals, and SEO-specific components.

### web-quality (planned)
Web quality audit reports with accessibility, performance, and best practices checks.

### security-audit (planned)
Security audit reports with vulnerability findings and compliance checks.

## Pack Structure

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

## Creating a Pack

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

### 3. Create Component (components/my_widget.typ)

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

```typst
#let my-report(title: none, body) = {
  if title != none {
    align(center)[
      #text(size: font-size-2xl, weight: "bold")[#title]
    ]
    v(spacing-6)
  }
  body
}
```

## Using Packs

```rust
use renderreport::Engine;

let mut engine = Engine::new()?;
engine.load_pack("my-pack")?;

let report = engine
    .report("my-report")
    .pack("my-pack")
    .title("Report Title")
    // ... add components
    .build();
```

## Pack Conventions

- Use kebab-case for IDs
- Follow token naming conventions (see CONVENTIONS.md)
- Include a README in your pack
- Version using semver
- Specify minimum engine version

## License

MIT
