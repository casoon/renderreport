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
    else if comp-type == "barcode" { barcode(comp-data) }
    else if comp-type == "image" { report-image(comp-data) }
    else if comp-type == "section" { section(comp-data) }
    else if comp-type == "audit-table" { audit-table(comp-data) }
    else if comp-type == "severity-overview" { severity-overview(comp-data) }
    else if comp-type == "list" { list(comp-data) }
    else if comp-type == "divider" { divider(comp-data) }
    else if comp-type == "side-label" { side-label(comp-data) }
    else if comp-type == "grid-component" { grid-component(comp-data) }
    else if comp-type == "flow-group" { flow-group(comp-data) }
    else if comp-type == "metric-strip" { metric-strip(comp-data) }
    else if comp-type == "impact-triad" { impact-triad(comp-data) }
    else if comp-type == "dominant-issue-spotlight" { dominant-issue-spotlight(comp-data) }
    else if comp-type == "phase-block" { phase-block(comp-data) }
    else if comp-type == "section-header-split" { section-header-split(comp-data) }
    else if comp-type == "wrong-right-block" { wrong-right-block(comp-data) }
    else if comp-type == "benchmark-table" { benchmark-table(comp-data) }
    else if comp-type == "benchmark-summary" { benchmark-summary(comp-data) }
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
