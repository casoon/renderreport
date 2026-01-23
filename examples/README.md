# RenderReport Examples

This directory contains comprehensive examples demonstrating all features of the RenderReport library.

## Quick Start Examples

### Simple Report
**File:** `simple_report.rs`  
**Run:** `cargo run --example simple_report`  
**Output:** `simple_report.pdf` (10 KB)

Basic example showing how to create a simple report with standard components.

### SEO Audit Report
**File:** `seo_audit.rs`  
**Run:** `cargo run --example seo_audit`  
**Output:** `seo_audit_report.pdf` (44 KB)

Complete SEO audit report demonstrating:
- Custom theme configuration
- ScoreCard components with thresholds
- Finding components with severity levels
- AuditTable for structured data
- Section headings and organization

### Custom Themes
**File:** `custom_theme.rs`  
**Run:** `cargo run --example custom_theme`  
**Output:** `corporate_report.pdf`, `dark_report.pdf` (21 KB, 9.6 KB)

Demonstrates theme customization:
- Corporate branding theme
- Dark mode theme
- Token-based styling (colors, fonts, spacing)

## Advanced Components

### Advanced Components Demo
**File:** `advanced_components.rs`  
**Run:** `cargo run --example advanced_components`  
**Output:** Generated PDF

Showcases advanced layout components inspired by JasperReports, BIRT, and Pentaho:
- **List** - Iterative lists with nested items
- **Grid** - Multi-column grid layouts
- **ProgressBar** - Visual progress indicators
- **KeyValueList** - Key-value pair displays
- **Divider** - Horizontal separators
- **Watermark** - Background watermark text
- **PageBreak** - Force page breaks

## New Component Demonstrations

### Charts Demo ✓
**File:** `charts_demo.rs`  
**Run:** `cargo run --example charts_demo`  
**Output:** `charts_demo.pdf` (21 KB)

**Comprehensive chart visualization showcase:**

**Bar Charts**
- Single and multi-series bar charts
- Quarterly revenue visualization
- Year-over-year comparisons

**Line Charts**
- Trend analysis for time-series data
- Multi-metric performance tracking
- Monthly active users growth

**Pie Charts**
- Market share distribution
- Revenue by product category
- Proportional data visualization

**Area Charts**
- Cumulative growth visualization
- Stacked area charts

**Scatter Charts**
- Correlation analysis
- Customer lifetime value analysis

**Radar Charts**
- Multi-dimensional comparisons
- Product feature evaluation

**Sparklines**
- Inline mini-charts for trends
- Bar and line sparklines
- Compact data visualization

**Gauges**
- KPI visualization with thresholds
- Circular gauges (performance meters)
- Thermometer style gauges (vertical)
- System load and health indicators

### Barcode Demo ✓
**File:** `barcode_demo.rs`  
**Run:** `cargo run --example barcode_demo`  
**Output:** `barcode_demo.pdf` (56 KB)

**Complete barcode generation guide:**

**1D Barcodes:**
- **Code 128** - Universal alphanumeric barcode for shipping and inventory
- **Code 39** - Industrial standard for automotive and healthcare
- **EAN-13** - European Article Number for retail products
- **EAN-8** - Compact version for small products
- **UPC-A** - Universal Product Code for North American retail
- **UPC-E** - Compressed UPC for small packages
- **ITF** - Interleaved 2 of 5 for shipping containers
- **Codabar** - Medical and library applications

**2D Barcodes:**
- **QR Code** - High capacity with error correction (URLs, vCards, WiFi config)
- **Data Matrix** - Compact 2D code for electronics and pharmaceuticals
- **PDF417** - High capacity stacked barcode for IDs and shipping labels

### Data Analysis Demo
**File:** `data_analysis_demo.rs`  
**Status:** ⚠️ Template optimization in progress

Demonstrates data analysis components:
- **PivotTable** - Pre-aggregated pivot displays
- **Crosstab** - Dynamic pivot with row/column aggregation

### Text Formatting Demo
**File:** `text_formatting_demo.rs`  
**Status:** ⚠️ Template optimization in progress

Demonstrates text and number formatting:
- **Label** - Styled text with size, weight, color, alignment
- **Text** - Multi-line text blocks with line height control
- **NumberField** - Currency formatting ($, €, £, ¥)
- **NumberField** - Percentage formatting
- **NumberField** - Custom formats with prefixes/suffixes
- **DateField** - Date formatting (ISO, European, US)
- **ResourceField** - Internationalization support

### New Components Demo ✓
**File:** `new_components_demo.rs`  
**Run:** `cargo run --example new_components_demo`  
**Output:** `new_components_demo.pdf` (12 KB)

Simple demonstration using standard components to verify the new component system works correctly.

## Running Examples

Run any example with:
```bash
cargo run --example <example_name>
```

For example:
```bash
cargo run --example charts_demo
cargo run --example barcode_demo
cargo run --example seo_audit
```

## Example Output

All examples generate PDF files in `examples/output/`:
- `examples/output/simple_report.pdf`
- `examples/output/seo_audit_report.pdf`
- `examples/output/corporate_report.pdf`
- `examples/output/dark_report.pdf`
- `examples/output/charts_demo.pdf`
- `examples/output/barcode_demo.pdf`
- `examples/output/new_components_demo.pdf`

Pre-generated PDFs are included in the repository for easy viewing. See [examples/output/README.md](output/README.md) for details.

## Component Categories

### Standard Components (7)
- ScoreCard, Finding, AuditTable, Section, Image, Callout, SummaryBox

### Advanced Components (7)
- List, Grid, ProgressBar, KeyValueList, Divider, Watermark, PageBreak

### Chart Components (3)
- Chart (6 types: Bar, Line, Pie, Area, Scatter, Radar)
- Sparkline (inline mini-charts)
- Gauge (KPI meters and thermometers)

### Data Analysis Components (2)
- Crosstab (dynamic pivot with aggregation)
- PivotTable (pre-aggregated display)

### Barcode Components (1)
- Barcode (11 formats: Code128, Code39, EAN-13, EAN-8, UPC-A, UPC-E, QR, DataMatrix, PDF417, ITF, Codabar)

### Text Components (5)
- Label (styled text)
- Text (multi-line blocks)
- NumberField (formatted numbers)
- DateField (formatted dates)
- ResourceField (localized strings)

## Total: 25 Components

For more information, see the main [README.md](../README.md) and [API documentation](../docs/API.md).
