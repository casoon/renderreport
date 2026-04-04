# renderreport Components — Complete Reference

**51+ production-ready components** organized into **8 semantic categories** for easy discovery and composition.

---

## 1. Layout & Structure

Foundation components for organizing content flow and spatial arrangement.

| Component | Type | Purpose |
|-----------|------|---------|
| **Section** | `section` | Semantic heading for content organization (levels 1–3) |
| **SectionHeaderSplit** | `section-header-split` | Two-column header: title + body text |
| **Divider** | `divider` | Visual horizontal separator |
| **Grid** | `grid-component` | Multi-column layout dispatcher (routes data to components) |
| **Columns** | `columns` | Asymmetric 2-column layout with flexible width ratio (60/40, 70/30, etc.) |
| **FlowGroup** | `flow-group` | Groups multiple components with optional title |
| **PageBreak** | `page-break` | Explicit page break for multi-page documents |

---

## 2. Text & Editorial

Basic text and typographic components.

| Component | Type | Purpose |
|-----------|------|---------|
| **TextBlock** | `textblock` | Paragraph text with optional alignment |
| **Label** | `label` | Small semantic label for metadata/status/categories |
| **Eyebrow** | `eyebrow` | Tiny uppercase label (context above headings) |

---

## 3. Metrics & KPIs

Components for displaying numbers, scores, and key metrics.

| Component | Type | Purpose |
|-----------|------|---------|
| **ScoreCard** | `score-card` | Metric display with score visualization (0-100) |
| **MetricCard** | `metric-card` | Key metric with value + optional accent color |
| **TrendTile** | `trend-tile` | Metric with trend indicator (up/down/flat) + change percentage |
| **Stat** | `stat` | Number + label pair (simple format) |
| **StatPair** | `stat-pair` | Two stats side-by-side |
| **Gauge** | `gauge` | Circular progress visualization (0-100 arc) |
| **ProgressBar** | `progress-bar` | Linear progress visualization |
| **BigNumber** | `big-number` | Large, prominent metric for impact stats ("+23%", "10x") |
| **Watermark** | `watermark` | Background text watermark (e.g. "DRAFT", "CONFIDENTIAL") |

---

## 4. Data & Comparison

Tables and comparison visualizations.

| Component | Type | Purpose |
|-----------|------|---------|
| **AuditTable** | `audit-table` | Specialized table for audit findings/issues |
| **ComparisonBlock** | `comparison-block` | Side-by-side before/after comparison |
| **ComparisonCluster** | `comparison-cluster` | Grid of comparison items with values + status |
| **Crosstab** | `crosstab` | Aggregated tabular data (rows × columns × measure) |
| **PivotTable** | `pivot-table` | Pivot aggregation with customizable dimensions |

---

## 5. Narrative & Storytelling

Components for building narrative flow and context.

| Component | Type | Purpose |
|-----------|------|---------|
| **Finding** | `finding` | Audit finding with severity, description, recommendation |
| **SpotlightCard** | `spotlight-card` | Important element with left severity stripe + metric |
| **QuoteBlock** | `quote-block` | Inspirational or expert quote |
| **PullQuote** | `pull-quote` | Large, visually prominent full-width quote |
| **WhyItMatters** | `why-it-matters` | Context block: why this is relevant + urgency |
| **ProblemSolution** | `problem-solution` | Two-part block: problem (red) ↔ solution (green) |
| **BeforeAfter** | `before-after` | Before/after comparison (Is/Should, Old/New) |
| **FactBox** | `fact-box` | Compact inline fact or statistic |
| **CoverPage** | `cover-page` | Title page for audit/report documents |

---

## 6. Infographics

Visual storytelling with processes, flows, and timelines.

| Component | Type | Purpose |
|-----------|------|---------|
| **ProcessFlow** | `process-flow` | Numbered process steps with arrows (horizontal/vertical) |
| **Timeline** | `timeline` | Vertical timeline with dates, titles, status |
| **Funnel** | `funnel` | Conversion funnel with decreasing stages |
| **PhaseBlock** | `phase-block` | Numbered phase in multi-step remediation plan |
| **RoadmapBlock** | `roadmap-block` | Multi-column roadmap with phases + actions |
| **ImpactGrid** | `impact-grid` | 3-card impact assessment (User/Risk/Conversion) |
| **CardDashboard** | `card-dashboard` | Grid of feature/module cards |

---

## 7. Marketing & Sales

Components for product showcases, features, and conversions.

| Component | Type | Purpose |
|-----------|------|---------|
| **ProductHero** | `product-hero` | Full-page product intro with highlights + CTA |
| **HeroSummary** | `hero-summary` | Executive summary: score/grade + key metrics + top actions |
| **FeatureGrid** | `feature-grid` | Features/benefits grid with optional icons + status |
| **BenefitStrip** | `benefit-strip` | Horizontal benefit strip (3-5 points) |
| **CTABox** | `cta-box` | Call-to-action block with headline + body + link |
| **PricingCard** | `pricing-card` | Single pricing/plan card with features + highlight |
| **Testimonial** | `testimonial` | Customer/partner testimonial with quote + attribution |
| **UseCaseCard** | `use-case-card` | Use case: context + problem + solution + outcome |
| **LogoStrip** | `logo-strip` | Logos of customers, partners, certifications |
| **RecommendationCard** | `recommendation-card` | Lightweight recommendation with impact/effort/priority badges |
| **ChecklistPanel** | `checklist-panel` | Checklist with items + completion status |

---

## 8. Media & Assets

Media, data visualization, and reference components.

| Component | Type | Purpose |
|-----------|------|---------|
| **Image** | `image` | Image with optional caption |
| **Barcode** | `barcode` | Barcode/QR code generation |
| **Sparkline** | `sparkline` | Tiny inline chart (trend visualization) |
| **Chart** | `chart` | Data visualization (line, bar, pie, combo) |
| **List** | `list` | Itemized list with optional nesting |
| **KeyValueList** | `key-value-list` | Key-value pairs (metadata, specs, config) |
| **FaqList** | `faq-list` | Question-answer pairs for FAQs |
| **GlossaryList** | `glossary-list` | Term-definition pairs for reference |

---

## Patterns — Pre-configured Report Structures

Pre-configured component sequences for common report types. Each pattern defines narrative flow and sensible defaults.

### AuditPattern

For **security/compliance audits**.

```
Hero (HeroSummary: score/grade/verdict)
  ↓
Context (Why this audit matters)
  ↓
Analysis (Findings, impact assessment, checkpoints)
  ↓
Solution (Remediation roadmap, timeline)
  ↓
Actions (Implementation steps)
  ↓
CTA (Schedule review, contact security team)
```

**Components used:**
- `hero-summary`, `finding`, `impact-grid`, `checklist-panel`, `roadmap-block`, `timeline`, `cta-box`

### MarketingPattern

For **product/feature showcases and brochures**.

```
Hero (ProductHero: full-page intro with highlights)
  ↓
Context (Problem statement, why it matters)
  ↓
Analysis (Features, benefits, proof points)
  ↓
Solution (How it works, differentiation)
  ↓
Actions (Testimonials, case studies, use cases)
  ↓
CTA (Pricing, demo request, signup)
```

**Components used:**
- `product-hero`, `feature-grid`, `benefit-strip`, `comparison-block`, `process-flow`, `testimonial`, `use-case-card`, `pricing-card`, `cta-box`

### ExecutivePattern

For **C-level summaries and board reports**.

```
Hero (HeroSummary: key metrics, status, verdict)
  ↓
Context (Executive summary, key facts)
  ↓
Analysis (Top findings, impact, risk assessment)
  ↓
Solution (Strategic recommendation, approach)
  ↓
Actions (Ownership, timeline, budget)
  ↓
CTA (Approve, schedule meeting, next steps)
```

**Components used:**
- `hero-summary`, `big-number`, `fact-box`, `spotlight-card`, `impact-grid`, `phase-block`, `timeline`, `cta-box`

---

## Variant Consistency

Components supporting consistent styling APIs:

| Component | Supports |
|-----------|----------|
| **CTABox** | `.tone()` — primary, urgent, neutral |
| **SpotlightCard** | `.variant()` — critical, info, feature, opportunity |
| **Finding** | `.severity()` — critical, high, medium, low |
| **ProcessFlow** | `.direction()` — horizontal, vertical |
| **Timeline** | `.status()` — done, active, planned |
| **PhaseBlock** | `.accent_color()` — custom phase color |

---

## Recommended Component Combinations

### Quick Wins (single page)
- `hero-summary` + `finding` (2-3) + `checklist-panel` = Minimal audit report

### Standard Audit (2-3 pages)
- `hero-summary` + `finding` (5-10) + `impact-grid` + `roadmap-block` + `timeline` = Complete audit

### Executive Briefing (1 page)
- `hero-summary` + `spotlight-card` (2-3) + `cta-box` = Board-ready summary

### Product Launch (3-5 pages)
- `product-hero` + `feature-grid` + `comparison-block` + `testimonial` + `pricing-card` = Marketing site

### Blog/Article (variable)
- `section` + `textblock` + `pullquote` + `image` + `chart` = Editorial content

---

## Total Count

- **Layout & Structure:** 7
- **Text & Editorial:** 3
- **Metrics & KPIs:** 9
- **Data & Comparison:** 5
- **Narrative & Storytelling:** 9
- **Infographics:** 7
- **Marketing & Sales:** 11
- **Media & Assets:** 8

**Total: 59+ components**
