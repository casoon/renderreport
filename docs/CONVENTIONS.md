# RenderReport Conventions

This document defines the naming conventions, ID formats, and design patterns used throughout RenderReport.

## Component IDs

Component identifiers use **kebab-case** (lowercase with hyphens).

### Standard Components

| ID | Description |
|---|---|
| `score-card` | Metric display with score visualization |
| `finding` | Audit finding with severity indicator |
| `audit-table` | Data table for audit results |
| `section` | Document section with heading |
| `image` | Image with optional caption |
| `callout` | Information callout/admonition box |
| `summary-box` | Executive summary widget |
| `chart` | Data visualization (bar, pie, etc.) |

### Custom Component IDs

When creating custom components:
- Use kebab-case: `my-custom-component`
- Prefix with pack name for pack-specific: `seo-audit.lighthouse-scores`
- Avoid generic names that might conflict

## Token Names

Theme tokens use **dot-notation** with hierarchical namespaces.

### Color Tokens

```
color.primary       # Primary brand color
color.secondary     # Secondary accent color
color.text          # Main text color
color.text-muted    # Secondary/muted text
color.background    # Page background
color.surface       # Card/box background
color.border        # Border color
color.ok            # Success/good status (green)
color.warn          # Warning status (yellow/orange)
color.bad           # Error/bad status (red)
```

### Typography Tokens

```
font.body           # Body text font family
font.heading        # Heading font family
font.mono           # Monospace font family
font.size.base      # Base font size (default: 11pt)
font.size.sm        # Small text
font.size.lg        # Large text
font.size.xl        # Extra large
font.size.2xl       # 2x large (titles)
```

### Spacing Tokens

```
spacing.1           # 4pt
spacing.2           # 8pt
spacing.3           # 12pt
spacing.4           # 16pt (base)
spacing.5           # 24pt
spacing.6           # 32pt
```

### Table Tokens

```
table.header-bg     # Header row background
table.row-alt-bg    # Alternating row background
table.border        # Table border color
table.border-width  # Border thickness
```

### Page Tokens

```
page.margin         # Default margin (all sides)
page.margin-top     # Top margin
page.margin-bottom  # Bottom margin
page.header-height  # Header area height
page.footer-height  # Footer area height
```

### Component-Specific Tokens

```
component.<id>.<property>

# Examples:
component.score-card.radius
component.finding.radius
component.callout.radius
```

## Pack Manifest Fields

### Required Fields

```toml
[pack]
name = "pack-name"      # kebab-case identifier
version = "0.1.0"       # Semantic versioning
```

### Optional Fields

```toml
[pack]
description = "..."
authors = ["Name <email>"]
license = "MIT"
repository = "https://..."
keywords = ["tag1", "tag2"]

[compatibility]
min_engine_version = "0.1.0"
max_engine_version = "1.0.0"
typst_version = "0.13"
```

### Template Entry

```toml
[templates.template-id]
file = "path/to/template.typ"
description = "Human-readable description"
default_theme = "theme-id"
schema = "path/to/schema.json"  # Optional JSON Schema
```

### Theme Entry

```toml
[themes.theme-id]
file = "path/to/theme.toml"
description = "Human-readable description"
default = true  # Mark as default theme
```

### Component Entry

```toml
[components.component-id]
file = "path/to/component.typ"
description = "Human-readable description"
overrides = "standard-component-id"  # If overriding standard
```

## Data Conventions

### Severity Levels

Used in findings and issues:

| Level | Color Token | Usage |
|---|---|---|
| `critical` | `color.bad` | Requires immediate attention |
| `high` | `color.bad` | Significant impact |
| `medium` | `color.warn` | Should be addressed |
| `low` | `color.ok` | Minor improvement |
| `info` | `color.ok` | Informational only |

### Score Status

Used in score cards and summaries:

| Status | Color Token | Typical Range |
|---|---|---|
| `good` | `color.ok` | 90-100 |
| `warning` | `color.warn` | 50-89 |
| `bad` | `color.bad` | 0-49 |

Thresholds are configurable per component.

## File Structure Conventions

### Pack Structure

```
pack-name/
├── pack.toml           # Manifest (required)
├── templates/          # Typst templates
│   └── *.typ
├── components/         # Component templates
│   └── *.typ
├── themes/             # Theme definitions
│   └── *.toml
└── assets/             # Static assets
    ├── fonts/
    └── images/
```

### Report Data Structure

```json
{
  "template_id": "template-name",
  "pack_id": "pack-name",
  "title": "Report Title",
  "subtitle": "Optional Subtitle",
  "theme": { /* optional overrides */ },
  "components": [
    {
      "type": "component-id",
      "data": { /* component data */ }
    }
  ],
  "assets": {
    "logo": "path/to/logo.png"
  },
  "metadata": {
    "author": "Name",
    "date": "2025-01-01"
  }
}
```

## Typst Template Conventions

### Variable Naming in Templates

- Theme tokens become Typst variables with dots replaced by hyphens
- `color.primary` → `color-primary`
- `font.size.base` → `font-size-base`

### Component Function Signature

```typst
#let component-name(data) = {
  // data is a dictionary with component properties
  // Access via: data.property_name
}
```

### Required Token Variables

All templates must work with these standard tokens available:

```typst
// Colors
#let color-primary = rgb("#2563eb")
#let color-text = rgb("#1e293b")
// ... etc

// Typography
#let font-body = "Inter"
#let font-size-base = 11pt
// ... etc

// Spacing
#let spacing-4 = 16pt
// ... etc
```
