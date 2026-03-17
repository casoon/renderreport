// Grid Component
// Inspired by Pentaho Row/Block Layout

// Component dispatch for nested rendering
#let _grid-dispatch(c) = {
  if type(c) == dictionary and "type" in c and "data" in c {
    let comp-type = c.at("type")
    let comp-data = c.at("data")
    // Dispatch to known component functions
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
    else {
      // Fallback for unknown types
      text(size: 9pt, fill: gray, "[" + comp-type + "]")
    }
  } else if type(c) == dictionary {
    // Raw data without type wrapper — generic display
    let inner = c
    if inner.at("title", default: none) != none {
      text(weight: "bold", size: 10pt, inner.title)
      if inner.at("score", default: none) != none {
        v(4pt)
        text(size: 20pt, weight: "bold", str(inner.score))
        if inner.at("max_score", default: none) != none {
          text(size: 10pt, fill: gray, " / " + str(inner.max_score))
        }
      }
      if inner.at("description", default: none) != none {
        v(4pt)
        text(size: 9pt, fill: gray, inner.description)
      }
    } else {
      for (key, val) in inner {
        if type(val) == str or type(val) == int or type(val) == float {
          text(size: 9pt)[#text(weight: "bold")[#key:] #str(val)]
          linebreak()
        }
      }
    }
  } else if type(c) == str {
    text(size: 10pt, c)
  } else {
    [#c]
  }
}

#let grid-component(data) = {
  box(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-lg)[#data.title]
      #v(spacing-4)
    ]

    #let col-widths = (1fr,) * data.columns

    #grid(
      columns: col-widths,
      column-gutter: eval(data.column_gap),
      row-gutter: eval(data.row_gap),

      ..data.items.map(item => {
        let c = item.content
        // Check if content has a type — if so, render without card wrapper
        if type(c) == dictionary and "type" in c and "data" in c {
          box(width: 100%, _grid-dispatch(c))
        } else {
          box(
            width: 100%,
            inset: spacing-3,
            fill: color-surface,
            radius: 4pt,
            stroke: (paint: color-border, thickness: 0.5pt),
          )[#_grid-dispatch(c)]
        }
      })
    )
  ]
}
