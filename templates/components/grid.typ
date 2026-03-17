// Grid Component
// Inspired by Pentaho Row/Block Layout

#let grid-component(data) = {
  box(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-lg)[#data.title]
      #v(spacing-4)
    ]

    // Build column template based on items and colspans
    #let col-widths = (1fr,) * data.columns

    #grid(
      columns: col-widths,
      column-gutter: eval(data.column_gap),
      row-gutter: eval(data.row_gap),

      ..data.items.map(item => {
        let c = item.content
        box(
          width: 100%,
          inset: spacing-3,
          fill: color-surface,
          radius: 4pt,
          stroke: (paint: color-border, thickness: 0.5pt),
        )[
          // Render nested component content
          #if type(c) == dictionary {
            // Try to extract meaningful display from component data
            let inner = c.at("data", default: c)
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
              // Generic fallback: show all string/number values
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
        ]
      })
    )
  ]
}
