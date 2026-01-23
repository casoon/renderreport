# RenderReport API Reference

## Core Types

### Engine

The main entry point for rendering reports.

```rust
use renderreport::Engine;

// Create with default configuration
let engine = Engine::new()?;

// Or with custom configuration
let config = EngineConfig::builder()
    .pack_path("./my-packs")
    .use_system_fonts(true)
    .build();
let engine = Engine::with_config(config)?;
```

#### Methods

- `new() -> Result<Engine>` - Create engine with defaults
- `with_config(config: EngineConfig) -> Result<Engine>` - Create with custom config
- `report(template_id: impl Into<String>) -> ReportBuilder` - Start building a report
- `render_pdf(request: &RenderRequest) -> Result<Vec<u8>>` - Render to PDF
- `load_pack(pack_id: &str) -> Result<()>` - Load a template pack
- `set_default_theme(theme: Theme)` - Set default theme

### ReportBuilder

Fluent API for building reports.

```rust
let report = engine.report("default")
    .title("My Report")
    .subtitle("Subtitle")
    .theme(my_theme)
    .add_component(ScoreCard::new("Score", 85))
    .add_component(Finding::new("Issue", Severity::High, "Description"))
    .metadata("author", "John Doe")
    .build();
```

#### Methods

- `title(title: impl Into<String>) -> Self` - Set title
- `subtitle(subtitle: impl Into<String>) -> Self` - Set subtitle
- `theme(theme: Theme) -> Self` - Set theme
- `add_component(component: impl Component) -> Self` - Add component
- `metadata(key: impl Into<String>, value: impl Into<String>) -> Self` - Add metadata
- `asset(name: impl Into<String>, path: impl Into<PathBuf>) -> Self` - Register asset
- `build() -> RenderRequest` - Build the request

## Components

### ScoreCard

Display a metric with score visualization.

```rust
use renderreport::components::ScoreCard;

let card = ScoreCard::new("Performance", 85)
    .with_description("Good performance")
    .with_thresholds(90, 50);
```

**Fields:**
- `title: String` - Card title
- `score: u32` - Score value (0-100 typically)
- `max_score: u32` - Maximum score (default: 100)
- `description: Option<String>` - Optional description
- `good_threshold: u32` - Threshold for "good" (default: 90)
- `warn_threshold: u32` - Threshold for "warning" (default: 50)

### Finding

Audit finding with severity indicator.

```rust
use renderreport::components::{Finding, Severity};

let finding = Finding::new(
    "Missing Meta Description",
    Severity::High,
    "The homepage lacks a meta description tag."
)
.with_recommendation("Add a compelling meta description.")
.with_affected("https://example.com/")
.with_category("SEO");
```

**Fields:**
- `title: String` - Finding title
- `severity: Severity` - Severity level
- `description: String` - Detailed description
- `recommendation: Option<String>` - How to fix
- `affected: Option<String>` - Affected resource
- `category: Option<String>` - Category/tag

**Severity Levels:**
- `Severity::Critical`
- `Severity::High`
- `Severity::Medium`
- `Severity::Low`
- `Severity::Info`

### AuditTable

Data table for audit results.

```rust
use renderreport::components::{AuditTable, TableColumn};

let table = AuditTable::new(vec![
    TableColumn::new("Check"),
    TableColumn::new("Status"),
    TableColumn::new("Details"),
])
.with_title("Technical Checklist")
.add_row(vec!["HTTPS", "✓ Pass", "Valid certificate"])
.add_row(vec!["Sitemap", "✓ Pass", "Found at /sitemap.xml"]);
```

### Section

Document section with heading.

```rust
use renderreport::components::Section;

let section = Section::new("Performance Metrics")
    .with_level(1); // H1, H2, etc. (1-6)
```

### SummaryBox

Executive summary widget.

```rust
use renderreport::components::{SummaryBox, ScoreStatus};

let summary = SummaryBox::new("Executive Summary")
    .add_item("Total Issues", "15")
    .add_item_with_status("Overall Score", "78/100", ScoreStatus::Warning);
```

### Callout

Information callout box.

```rust
use renderreport::components::Callout;

let callout = Callout::info("This is an informational note.");
let warning = Callout::warning("Be careful with this!");
let error = Callout::error("Something went wrong.");
let success = Callout::success("Operation completed!");
```

### Image

Image with optional caption.

```rust
use renderreport::components::Image;

let img = Image::new("screenshot.png")
    .with_caption("Screenshot of the homepage")
    .with_width("80%");
```

## Theming

### Theme

A complete theme definition.

```rust
use renderreport::theme::{Theme, TokenValue};

let mut theme = Theme::new("brand", "Brand Theme");
theme.tokens.set("color.primary", TokenValue::Color("#1a56db".into()));
theme.tokens.set("color.ok", TokenValue::Color("#059669".into()));
theme.tokens.set("font.body", TokenValue::Font("Helvetica".into()));
```

### Token Naming Convention

**Colors:**
- `color.primary` - Primary brand color
- `color.secondary` - Secondary accent
- `color.text` - Main text color
- `color.text-muted` - Muted/secondary text
- `color.ok` - Success/good (green)
- `color.warn` - Warning (orange/yellow)
- `color.bad` - Error/bad (red)
- `color.background` - Page background
- `color.surface` - Card/box background
- `color.border` - Border color

**Typography:**
- `font.body` - Body text font
- `font.heading` - Heading font
- `font.mono` - Monospace font
- `font.size.base` - Base font size
- `font.size.sm` - Small
- `font.size.lg` - Large
- `font.size.xl` - Extra large
- `font.size.2xl` - 2x large

**Spacing:**
- `spacing.1` - 4pt
- `spacing.2` - 8pt
- `spacing.3` - 12pt
- `spacing.4` - 16pt
- `spacing.5` - 24pt
- `spacing.6` - 32pt

See [CONVENTIONS.md](CONVENTIONS.md) for complete token reference.

## Template Packs

### Loading Packs

```rust
engine.load_pack("seo-audit")?;

let report = engine.report("seo-report")
    .pack("seo-audit")
    .title("SEO Audit")
    .build();
```

### Creating Custom Packs

See [docs/PACKS_README.md](PACKS_README.md) for pack creation guide.

## Error Handling

All fallible operations return `Result<T, renderreport::Error>`.

```rust
use renderreport::Error;

match engine.render_pdf(&report) {
    Ok(pdf_bytes) => { /* success */ },
    Err(Error::Pack(e)) => { /* pack error */ },
    Err(Error::Render(e)) => { /* render error */ },
    Err(e) => { /* other error */ },
}
```

## Examples

See the `examples/` directory for complete examples:
- `simple_report.rs` - Minimal example
- `seo_audit.rs` - Full SEO audit report
- `custom_theme.rs` - Theme customization
