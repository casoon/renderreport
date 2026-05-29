// Flow Group Component
// Soft keep-together wrapper for headings + first follow-up blocks.

#let _flow-dispatch(c) = {
  if type(c) == dictionary and "type" in c and "data" in c {
    let comp-type = c.at("type")
    let comp-data = c.at("data")
    if comp-type == "score-card" { score-card(comp-data) }
    else if comp-type == "gauge" { gauge(comp-data) }
    else if comp-type == "chart" { chart(comp-data) }
    else if comp-type == "sparkline" { sparkline(comp-data) }
    else if comp-type == "finding" { finding(comp-data) }
    else if comp-type == "callout" { callout(comp-data) }
    else if comp-type == "progress-bar" { progress-bar(comp-data) }
    else if comp-type == "summary-box" { summary-box(comp-data) }
    else if comp-type == "key-value-list" { key-value-list(comp-data) }
    else if comp-type == "metric-card" { metric-card(comp-data) }
    else if comp-type == "label" { label(comp-data) }
    else if comp-type == "textblock" { textblock(comp-data) }
    else if comp-type == "number-field" { number-field(comp-data) }
    else if comp-type == "date-field" { date-field(comp-data) }
    else if comp-type == "resource-field" { resource-field(comp-data) }
    else if comp-type == "barcode" { barcode(comp-data) }
    else if comp-type == "image" { report-image(comp-data) }
    else if comp-type == "section" { section(comp-data) }
    else if comp-type == "audit-table" { audit-table(comp-data) }
    else if comp-type == "severity-overview" { severity-overview(comp-data) }
    else if comp-type == "crosstab" { crosstab(comp-data) }
    else if comp-type == "pivot-table" { pivot-table(comp-data) }
    else if comp-type == "list" { list(comp-data) }
    else if comp-type == "divider" { divider(comp-data) }
    else if comp-type == "watermark" { watermark(comp-data) }
    else if comp-type == "page-break" { page-break(comp-data) }
    else if comp-type == "tag-cloud" { tag-cloud(comp-data) }
    else if comp-type == "side-label" { side-label(comp-data) }
    else if comp-type == "table-of-contents" { table-of-contents(comp-data) }
    else if comp-type == "eyebrow" { eyebrow(comp-data) }
    else if comp-type == "status-pill" { status-pill(comp-data) }
    else if comp-type == "stat" { stat(comp-data) }
    else if comp-type == "stat-pair" { stat-pair(comp-data) }
    else if comp-type == "score-band" { score-band(comp-data) }
    else if comp-type == "trend-tile" { trend-tile(comp-data) }
    else if comp-type == "hero-summary" { hero-summary(comp-data) }
    else if comp-type == "product-hero" { product-hero(comp-data) }
    else if comp-type == "card-dashboard" { card-dashboard(comp-data) }
    else if comp-type == "roadmap-block" { roadmap-block(comp-data) }
    else if comp-type == "cover-page" { cover-page(comp-data) }
    else if comp-type == "module-comparison" { module-comparison(comp-data) }
    else if comp-type == "portfolio-summary" { portfolio-summary(comp-data) }
    else if comp-type == "benchmark-table" { benchmark-table(comp-data) }
    else if comp-type == "checklist-panel" { checklist-panel(comp-data) }
    else if comp-type == "metric-strip" { metric-strip(comp-data) }
    else if comp-type == "impact-grid" { impact-grid(comp-data) }
    else if comp-type == "spotlight-card" { spotlight-card(comp-data) }
    else if comp-type == "phase-block" { phase-block(comp-data) }
    else if comp-type == "section-header-split" { section-header-split(comp-data) }
    else if comp-type == "comparison-block" { comparison-block(comp-data) }
    else if comp-type == "comparison-cluster" { comparison-cluster(comp-data) }
    else if comp-type == "feature-grid" { feature-grid(comp-data) }
    else if comp-type == "cta-box" { cta-box(comp-data) }
    else if comp-type == "testimonial" { testimonial(comp-data) }
    else if comp-type == "process-flow" { process-flow(comp-data) }
    else if comp-type == "timeline" { timeline(comp-data) }
    else if comp-type == "funnel" { funnel(comp-data) }
    else if comp-type == "problem-solution" { problem-solution(comp-data) }
    else if comp-type == "before-after" { before-after(comp-data) }
    else if comp-type == "why-it-matters" { why-it-matters(comp-data) }
    else if comp-type == "fact-box" { fact-box(comp-data) }
    else if comp-type == "quote-block" { quote-block(comp-data) }
    else if comp-type == "benefit-strip" { benefit-strip(comp-data) }
    else if comp-type == "pricing-card" { pricing-card(comp-data) }
    else if comp-type == "recommendation-card" { recommendation-card(comp-data) }
    else if comp-type == "step-card-row" { step-card-row(comp-data) }
    else if comp-type == "columns" { columns(comp-data) }
    else if comp-type == "device-preview" { device-preview(comp-data) }
    else if comp-type == "faq-list" { faq-list(comp-data) }
    else if comp-type == "use-case-card" { use-case-card(comp-data) }
    else if comp-type == "logo-strip" { logo-strip(comp-data) }
    else if comp-type == "pull-quote" { pull-quote(comp-data) }
    else if comp-type == "big-number" { big-number(comp-data) }
    else if comp-type == "glossary-list" { glossary-list(comp-data) }
    else if comp-type == "diagnosis-panel" { diagnosis-panel(comp-data) }
    else if comp-type == "dominant-issue-spotlight" { dominant-issue-spotlight(comp-data) }
    else if comp-type == "wrong-right-block" { wrong-right-block(comp-data) }
    else if comp-type == "grid-component" { grid-component(comp-data) }
    else if comp-type == "flow-group" { flow-group(comp-data) }
    else {
      text(size: 9pt, fill: gray, "[" + comp-type + "]")
    }
  } else if type(c) == str {
    text(size: 10pt, c)
  } else {
    [#c]
  }
}

#let flow-group(data) = context {
  let gap = if data.spacing != none { eval(data.spacing) } else { spacing-4 }
  let content = stack(
    spacing: gap,
    ..data.items.map(item => [#_flow-dispatch(item)]),
  )

  if data.keep_together_if_under != none {
    let threshold = eval(data.keep_together_if_under)
    let measured = measure(content)
    if measured.height <= threshold {
      block(width: 100%, breakable: false)[#content]
    } else {
      content
    }
  } else {
    content
  }
}
