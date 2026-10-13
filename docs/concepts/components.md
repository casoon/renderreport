---
title: Components
description: The 80 built-in components, their categories and the layout hints that control page breaks.
order: 2
---

A component is a Rust struct with a builder, a Typst function and an id. In a request it is
`{ "type": "<id>", "data": { … } }`; the Rust builder writes the same shape through
`Component::to_data`.

## Categories

| Category | Components | Examples |
| --- | --- | --- |
| Layout | 9 | `section`, `cover-page`, `grid-component`, `table-of-contents`, `page-break` |
| Text & Editorial | 15 | `textblock`, `callout`, `quote-block`, `label` |
| Metrics & KPIs | 16 | `score-card`, `gauge`, `big-number`, `progress-bar` |
| Data & Comparison | 9 | `audit-table`, `crosstab`, `pivot-table`, `chart` |
| Narrative & Storytelling | 16 | `finding`, `problem-solution`, `timeline`, `process-flow` |
| Infographics | 3 | `funnel`, `word-search` |
| Marketing & Sales | 8 | `product-hero`, `feature-grid`, `pricing-card`, `cta-box` |
| Media & Assets | 4 | `image`, `barcode` |

## Layout hints

Every component carries one hint that the engine applies when it writes the Typst source:

| Hint | Effect |
| --- | --- |
| `breakable` | May split across pages (long tables, text). |
| `keep-together` | Is never split; moves to the next page as a whole. |
| `keep-with-next` | Stays on the same page as the component after it (headings). |
| `always-new-page` | Starts on a new page (cover page, explicit page break). |

## Listing them

The CLI prints the catalog of the version you have installed:

```sh
renderreport list-components
renderreport list-components --category metrics
renderreport list-components --markdown
```

The [component catalog](../../../showcase/component-catalog/) renders every component with sample
data. The table below is the output of `renderreport list-components --markdown` for 0.6.1.

## All components

| ID | Category | Layout Hint | Description |
|---|---|---|---|
| `audit-table` | Data & Comparison | Breakable | Tabular data for audit results with severity columns |
| `barcode` | Media & Assets | KeepTogether | 1D/2D barcode (EAN-13, QR, DataMatrix, etc.) |
| `before-after` | Narrative & Storytelling | KeepTogether | Before/after comparison with labeled panels |
| `benchmark-table` | Data & Comparison | Breakable | Benchmark comparison table with scored rows |
| `benefit-strip` | Marketing & Sales | KeepTogether | Horizontal strip of benefit items with icons |
| `big-number` | Metrics & KPIs | KeepTogether | Oversized numeric display for key statistics |
| `callout` | Text & Editorial | KeepTogether | Highlighted information box with icon and variant styling |
| `card-dashboard` | Metrics & KPIs | Breakable | Multi-card dashboard layout with configurable columns |
| `chart` | Data & Comparison | KeepTogether | Bar, line, or area chart rendered from data arrays |
| `checklist-panel` | Narrative & Storytelling | Breakable | Checklist panel with pass/fail/warning rows |
| `columns` | Layout | Breakable | Asymmetric two-column layout with flexible width ratio |
| `comparison-block` | Data & Comparison | KeepTogether | Side-by-side comparison with labels and scores |
| `comparison-cluster` | Data & Comparison | Breakable | Clustered comparison of multiple items across criteria |
| `cover-page` | Layout | KeepTogether | Full-page cover with title, logo, and metadata |
| `crosstab` | Data & Comparison | Breakable | Cross-tabulation table with row/column summaries |
| `cta-box` | Marketing & Sales | KeepTogether | Call-to-action box with headline, body, and button label |
| `date-field` | Text & Editorial | KeepTogether | Formatted date field with label |
| `device-preview` | Infographics | KeepTogether | Device frame (desktop/mobile) with screenshot |
| `diagnosis-panel` | Narrative & Storytelling | Breakable | Diagnosis panel with findings grouped by severity |
| `divider` | Layout | KeepTogether | Horizontal rule separator with optional label |
| `dominant-issue-spotlight` | Narrative & Storytelling | KeepTogether | Full-width spotlight for the most critical finding |
| `eyebrow` | Text & Editorial | KeepWithNext | Small eyebrow label above a heading |
| `fact-box` | Text & Editorial | KeepTogether | Fact box with a prominent statistic and supporting text |
| `faq-list` | Text & Editorial | Breakable | Expandable FAQ list with question/answer pairs |
| `feature-grid` | Marketing & Sales | Breakable | Grid of feature items with icon, title, and description |
| `finding` | Narrative & Storytelling | KeepTogether | Audit finding with severity indicator and recommendation block |
| `flow-group` | Layout | KeepTogether | Soft keep-together wrapper for heading + first content block |
| `funnel` | Infographics | KeepTogether | Conversion funnel visualization with stage counts |
| `gauge` | Metrics & KPIs | KeepTogether | Radial gauge visualization for a single metric |
| `glossary-list` | Text & Editorial | Breakable | Definition list for glossary terms |
| `grid-component` | Layout | Breakable | Multi-column grid layout dispatching to any component type |
| `hero-summary` | Marketing & Sales | KeepTogether | Full-width hero summary with title, subtitle, and key metrics |
| `image` | Media & Assets | KeepTogether | Image with optional caption and alignment |
| `impact-grid` | Narrative & Storytelling | KeepTogether | Grid of impact cards with title, score, and description |
| `key-value-list` | Data & Comparison | Breakable | Two-column key/value list for structured data |
| `label` | Text & Editorial | KeepTogether | Styled text label with optional icon |
| `list` | Text & Editorial | Breakable | Bullet or numbered list with optional icons |
| `logo-strip` | Media & Assets | KeepTogether | Horizontal strip of client/partner logos |
| `metric-card` | Metrics & KPIs | KeepTogether | Compact metric card with value, unit, and trend indicator |
| `metric-strip` | Metrics & KPIs | KeepTogether | Horizontal strip of compact metric items |
| `module-comparison` | Data & Comparison | Breakable | Side-by-side module comparison with feature matrix |
| `number-field` | Metrics & KPIs | KeepTogether | Formatted numeric field with unit and label |
| `page-break` | Layout | AlwaysNewPage | Explicit page break |
| `phase-block` | Narrative & Storytelling | KeepTogether | Project phase block with status, owner, and date range |
| `pivot-table` | Data & Comparison | Breakable | Pivot table with aggregated multi-dimensional data |
| `portfolio-summary` | Metrics & KPIs | KeepTogether | Portfolio summary with aggregate scores across items |
| `pricing-card` | Marketing & Sales | KeepTogether | Pricing tier card with features and CTA |
| `problem-solution` | Narrative & Storytelling | KeepTogether | Two-column problem/solution comparison block |
| `process-flow` | Narrative & Storytelling | KeepTogether | Horizontal or vertical process flow with numbered steps |
| `product-hero` | Marketing & Sales | KeepTogether | Product hero block with image, headline, and description |
| `progress-bar` | Metrics & KPIs | KeepTogether | Horizontal progress bar with percentage label |
| `pull-quote` | Text & Editorial | KeepTogether | Large pull quote with decorative quotation marks |
| `quote-block` | Text & Editorial | KeepTogether | Styled block quote with attribution |
| `recommendation-card` | Narrative & Storytelling | KeepTogether | Recommendation card with impact, effort, and priority badges |
| `resource-field` | Text & Editorial | KeepTogether | Labeled resource reference with URL |
| `roadmap-block` | Narrative & Storytelling | Breakable | Project roadmap block with phases and milestones |
| `score-band` | Metrics & KPIs | KeepTogether | Score band visualization with threshold markers |
| `score-card` | Metrics & KPIs | KeepTogether | Score card with gauge-style visualization and status indicator |
| `section` | Layout | KeepWithNext | Document section heading with optional subtitle |
| `section-header-split` | Layout | KeepWithNext | Section header with title on left and metadata on right |
| `severity-overview` | Metrics & KPIs | KeepTogether | Severity distribution overview with count badges |
| `side-label` | Text & Editorial | KeepTogether | Vertical side label aligned alongside content |
| `sparkline` | Metrics & KPIs | KeepTogether | Compact inline sparkline trend chart |
| `spotlight-card` | Narrative & Storytelling | KeepTogether | Spotlight card with colored severity stripe and body |
| `stat` | Metrics & KPIs | KeepTogether | Single statistic display with value and label |
| `stat-pair` | Metrics & KPIs | KeepTogether | Side-by-side pair of stat displays |
| `status-pill` | Text & Editorial | KeepTogether | Inline status badge with color coding |
| `step-card-row` | Narrative & Storytelling | KeepTogether | Horizontal row of numbered step cards |
| `summary-box` | Metrics & KPIs | KeepTogether | Executive summary widget with key metrics |
| `table-of-contents` | Layout | Breakable | Auto-generated table of contents from section headings |
| `tag-cloud` | Text & Editorial | KeepTogether | Tag cloud with weighted font sizes |
| `testimonial` | Marketing & Sales | KeepTogether | Customer testimonial quote with attribution |
| `textblock` | Text & Editorial | Breakable | Rich text block with heading, body, and optional list |
| `timeline` | Narrative & Storytelling | Breakable | Project timeline with milestones and status indicators |
| `trend-tile` | Metrics & KPIs | KeepTogether | Trend tile with value, delta, and sparkline |
| `use-case-card` | Marketing & Sales | KeepTogether | Use case card with scenario title, user, and outcome |
| `watermark` | Media & Assets | KeepTogether | Page-level watermark overlay |
| `why-it-matters` | Narrative & Storytelling | KeepTogether | Highlighted explanation block for key insights |
| `word-search` | Infographics | KeepTogether | Parameter-driven word search puzzle generator with word list and solution highlighting |
| `wrong-right-block` | Narrative & Storytelling | KeepTogether | Wrong vs. right comparison with annotated examples |
