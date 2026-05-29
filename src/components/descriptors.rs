//! Central catalog registration for all built-in component types.
//!
//! Each `component_catalog!` call registers a [`ComponentDescriptor`] with the
//! global [`ComponentCatalog`] at program startup.  The registry can then
//! derive its template list directly from the catalog.

use crate::component_catalog;
use crate::components::catalog::{ComponentCategory, LayoutHint};

// ── Standard Components ──────────────────────────────────────────────────────

component_catalog! {
    id: "score-card",
    template: include_str!("../../templates/components/score_card.typ"),
    description: "Score card with gauge-style visualization and status indicator",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "finding",
    template: include_str!("../../templates/components/finding.typ"),
    description: "Audit finding with severity indicator and recommendation block",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "audit-table",
    template: include_str!("../../templates/components/audit_table.typ"),
    description: "Tabular data for audit results with severity columns",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "section",
    template: include_str!("../../templates/components/section.typ"),
    description: "Document section heading with optional subtitle",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::KeepWithNext,
}

component_catalog! {
    id: "image",
    template: include_str!("../../templates/components/image.typ"),
    description: "Image with optional caption and alignment",
    category: ComponentCategory::MediaAssets,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "callout",
    template: include_str!("../../templates/components/callout.typ"),
    description: "Highlighted information box with icon and variant styling",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "summary-box",
    template: include_str!("../../templates/components/summary_box.typ"),
    description: "Executive summary widget with key metrics",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

// ── Advanced Components ───────────────────────────────────────────────────────

component_catalog! {
    id: "list",
    template: include_str!("../../templates/components/list.typ"),
    description: "Bullet or numbered list with optional icons",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "divider",
    template: include_str!("../../templates/components/divider.typ"),
    description: "Horizontal rule separator with optional label",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "progress-bar",
    template: include_str!("../../templates/components/progress_bar.typ"),
    description: "Horizontal progress bar with percentage label",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "key-value-list",
    template: include_str!("../../templates/components/key_value_list.typ"),
    description: "Two-column key/value list for structured data",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "watermark",
    template: include_str!("../../templates/components/watermark.typ"),
    description: "Page-level watermark overlay",
    category: ComponentCategory::MediaAssets,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "page-break",
    template: include_str!("../../templates/components/page_break.typ"),
    description: "Explicit page break",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::AlwaysNewPage,
}

// ── Chart Components ──────────────────────────────────────────────────────────

component_catalog! {
    id: "chart",
    template: include_str!("../../templates/components/chart.typ"),
    description: "Bar, line, or area chart rendered from data arrays",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "sparkline",
    template: include_str!("../../templates/components/sparkline.typ"),
    description: "Compact inline sparkline trend chart",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "gauge",
    template: include_str!("../../templates/components/gauge.typ"),
    description: "Radial gauge visualization for a single metric",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

// ── Crosstab Components ───────────────────────────────────────────────────────

component_catalog! {
    id: "crosstab",
    template: include_str!("../../templates/components/crosstab.typ"),
    description: "Cross-tabulation table with row/column summaries",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "pivot-table",
    template: include_str!("../../templates/components/pivot_table.typ"),
    description: "Pivot table with aggregated multi-dimensional data",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

// ── Barcode Components ────────────────────────────────────────────────────────

component_catalog! {
    id: "barcode",
    template: include_str!("../../templates/components/barcode.typ"),
    description: "1D/2D barcode (EAN-13, QR, DataMatrix, etc.)",
    category: ComponentCategory::MediaAssets,
    layout_hint: LayoutHint::KeepTogether,
}

// ── Text Components ───────────────────────────────────────────────────────────

component_catalog! {
    id: "label",
    template: include_str!("../../templates/components/label.typ"),
    description: "Styled text label with optional icon",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "textblock",
    template: include_str!("../../templates/components/text.typ"),
    description: "Rich text block with heading, body, and optional list",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "number-field",
    template: include_str!("../../templates/components/number_field.typ"),
    description: "Formatted numeric field with unit and label",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "date-field",
    template: include_str!("../../templates/components/date_field.typ"),
    description: "Formatted date field with label",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "resource-field",
    template: include_str!("../../templates/components/resource_field.typ"),
    description: "Labeled resource reference with URL",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "side-label",
    template: include_str!("../../templates/components/side_label.typ"),
    description: "Vertical side label aligned alongside content",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "table-of-contents",
    template: include_str!("../../templates/components/table_of_contents.typ"),
    description: "Auto-generated table of contents from section headings",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::Breakable,
}

// ── Composite Components ──────────────────────────────────────────────────────

component_catalog! {
    id: "metric-card",
    template: include_str!("../../templates/components/metric_card.typ"),
    description: "Compact metric card with value, unit, and trend indicator",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "hero-summary",
    template: include_str!("../../templates/components/hero_summary.typ"),
    description: "Full-width hero summary with title, subtitle, and key metrics",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "product-hero",
    template: include_str!("../../templates/components/product_hero.typ"),
    description: "Product hero block with image, headline, and description",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "card-dashboard",
    template: include_str!("../../templates/components/card_dashboard.typ"),
    description: "Multi-card dashboard layout with configurable columns",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "roadmap-block",
    template: include_str!("../../templates/components/roadmap_block.typ"),
    description: "Project roadmap block with phases and milestones",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "severity-overview",
    template: include_str!("../../templates/components/severity_overview.typ"),
    description: "Severity distribution overview with count badges",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "cover-page",
    template: include_str!("../../templates/components/cover_page.typ"),
    description: "Full-page cover with title, logo, and metadata",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::AlwaysNewPage,
}

component_catalog! {
    id: "module-comparison",
    template: include_str!("../../templates/components/module_comparison.typ"),
    description: "Side-by-side module comparison with feature matrix",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "portfolio-summary",
    template: include_str!("../../templates/components/portfolio_summary.typ"),
    description: "Portfolio summary with aggregate scores across items",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "benchmark-table",
    template: include_str!("../../templates/components/benchmark_table.typ"),
    description: "Benchmark comparison table with scored rows",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

// ── Phase 1: Primitives ───────────────────────────────────────────────────────

component_catalog! {
    id: "eyebrow",
    template: include_str!("../../templates/components/eyebrow.typ"),
    description: "Small eyebrow label above a heading",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepWithNext,
}

component_catalog! {
    id: "status-pill",
    template: include_str!("../../templates/components/status_pill.typ"),
    description: "Inline status badge with color coding",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "tag-cloud",
    template: include_str!("../../templates/components/tag_cloud.typ"),
    description: "Tag cloud with weighted font sizes",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "stat",
    template: include_str!("../../templates/components/stat.typ"),
    description: "Single statistic display with value and label",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "stat-pair",
    template: include_str!("../../templates/components/stat_pair.typ"),
    description: "Side-by-side pair of stat displays",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "score-band",
    template: include_str!("../../templates/components/score_band.typ"),
    description: "Score band visualization with threshold markers",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "trend-tile",
    template: include_str!("../../templates/components/trend_tile.typ"),
    description: "Trend tile with value, delta, and sparkline",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

// ── Phase 2: Composite Blocks ─────────────────────────────────────────────────

component_catalog! {
    id: "section-header-split",
    template: include_str!("../../templates/components/section_header_split.typ"),
    description: "Section header with title on left and metadata on right",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::KeepWithNext,
}

component_catalog! {
    id: "phase-block",
    template: include_str!("../../templates/components/phase_block.typ"),
    description: "Project phase block with status, owner, and date range",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "checklist-panel",
    template: include_str!("../../templates/components/checklist_panel.typ"),
    description: "Checklist panel with pass/fail/warning rows",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "metric-strip",
    template: include_str!("../../templates/components/metric_strip.typ"),
    description: "Horizontal strip of compact metric items",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "impact-grid",
    template: include_str!("../../templates/components/impact_grid.typ"),
    description: "Grid of impact cards with title, score, and description",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "spotlight-card",
    template: include_str!("../../templates/components/spotlight_card.typ"),
    description: "Spotlight card with colored severity stripe and body",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "comparison-block",
    template: include_str!("../../templates/components/comparison_block.typ"),
    description: "Side-by-side comparison with labels and scores",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "comparison-cluster",
    template: include_str!("../../templates/components/comparison_cluster.typ"),
    description: "Clustered comparison of multiple items across criteria",
    category: ComponentCategory::DataComparison,
    layout_hint: LayoutHint::Breakable,
}

// ── Phase 3: Marketing / Narrative Components ─────────────────────────────────

component_catalog! {
    id: "feature-grid",
    template: include_str!("../../templates/components/feature_grid.typ"),
    description: "Grid of feature items with icon, title, and description",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "cta-box",
    template: include_str!("../../templates/components/cta_box.typ"),
    description: "Call-to-action box with headline, body, and button label",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "testimonial",
    template: include_str!("../../templates/components/testimonial.typ"),
    description: "Customer testimonial quote with attribution",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "process-flow",
    template: include_str!("../../templates/components/process_flow.typ"),
    description: "Horizontal or vertical process flow with numbered steps",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "timeline",
    template: include_str!("../../templates/components/timeline.typ"),
    description: "Project timeline with milestones and status indicators",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "funnel",
    template: include_str!("../../templates/components/funnel.typ"),
    description: "Conversion funnel visualization with stage counts",
    category: ComponentCategory::Infographics,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "problem-solution",
    template: include_str!("../../templates/components/problem_solution.typ"),
    description: "Two-column problem/solution comparison block",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "before-after",
    template: include_str!("../../templates/components/before_after.typ"),
    description: "Before/after comparison with labeled panels",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "why-it-matters",
    template: include_str!("../../templates/components/why_it_matters.typ"),
    description: "Highlighted explanation block for key insights",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "fact-box",
    template: include_str!("../../templates/components/fact_box.typ"),
    description: "Fact box with a prominent statistic and supporting text",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "quote-block",
    template: include_str!("../../templates/components/quote_block.typ"),
    description: "Styled block quote with attribution",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

// ── Phase 2 (Marketing) Components ───────────────────────────────────────────

component_catalog! {
    id: "benefit-strip",
    template: include_str!("../../templates/components/benefit_strip.typ"),
    description: "Horizontal strip of benefit items with icons",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "pricing-card",
    template: include_str!("../../templates/components/pricing_card.typ"),
    description: "Pricing tier card with features and CTA",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "recommendation-card",
    template: include_str!("../../templates/components/recommendation_card.typ"),
    description: "Recommendation card with impact, effort, and priority badges",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "step-card-row",
    template: include_str!("../../templates/components/step_card_row.typ"),
    description: "Horizontal row of numbered step cards",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "columns",
    template: include_str!("../../templates/components/columns.typ"),
    description: "Asymmetric two-column layout with flexible width ratio",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "device-preview",
    template: include_str!("../../templates/components/device_preview.typ"),
    description: "Device frame (desktop/mobile) with screenshot",
    category: ComponentCategory::Infographics,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "faq-list",
    template: include_str!("../../templates/components/faq_list.typ"),
    description: "Expandable FAQ list with question/answer pairs",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "use-case-card",
    template: include_str!("../../templates/components/use_case_card.typ"),
    description: "Use case card with scenario title, user, and outcome",
    category: ComponentCategory::MarketingSales,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "logo-strip",
    template: include_str!("../../templates/components/logo_strip.typ"),
    description: "Horizontal strip of client/partner logos",
    category: ComponentCategory::MediaAssets,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "pull-quote",
    template: include_str!("../../templates/components/pull_quote.typ"),
    description: "Large pull quote with decorative quotation marks",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "big-number",
    template: include_str!("../../templates/components/big_number.typ"),
    description: "Oversized numeric display for key statistics",
    category: ComponentCategory::MetricsKpis,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "glossary-list",
    template: include_str!("../../templates/components/glossary_list.typ"),
    description: "Definition list for glossary terms",
    category: ComponentCategory::TextEditorial,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "diagnosis-panel",
    template: include_str!("../../templates/components/diagnosis_panel.typ"),
    description: "Diagnosis panel with findings grouped by severity",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "dominant-issue-spotlight",
    template: include_str!("../../templates/components/dominant_issue_spotlight.typ"),
    description: "Full-width spotlight for the most critical finding",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

component_catalog! {
    id: "wrong-right-block",
    template: include_str!("../../templates/components/wrong_right_block.typ"),
    description: "Wrong vs. right comparison with annotated examples",
    category: ComponentCategory::NarrativeStorytelling,
    layout_hint: LayoutHint::KeepTogether,
}

// ── Dispatcher Templates (dispatch code auto-generated by build.rs) ───────────

component_catalog! {
    id: "grid-component",
    template: include_str!(concat!(env!("OUT_DIR"), "/grid.typ")),
    description: "Multi-column grid layout dispatching to any component type",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::Breakable,
}

component_catalog! {
    id: "flow-group",
    template: include_str!(concat!(env!("OUT_DIR"), "/flow_group.typ")),
    description: "Soft keep-together wrapper for heading + first content block",
    category: ComponentCategory::Layout,
    layout_hint: LayoutHint::KeepTogether,
}
