// ComparisonCluster Component
// Grid of comparable items with value, label, optional sub-text and status dot

#let comparison-cluster(data) = {
  let n-cols = data.columns
  let cols = (1fr,) * n-cols

  block(width: 100%)[
    #if data.title != none [
      #text(weight: "bold", size: font-size-sm, fill: color-text)[#data.title]
      #v(spacing-3)
    ]

    #grid(
      columns: cols,
      column-gutter: spacing-3,
      row-gutter: spacing-3,
      ..data.items.map(item => {
        let dot-color = if item.status == "good"    { color-ok }
          else if item.status == "warn"             { color-warn }
          else if item.status == "bad"              { color-bad }
          else { none }

        theme-card[
          #if dot-color != none [
            #stack(
              dir: ltr,
              spacing: 4pt,
              box(width: 8pt, height: 8pt, radius: 999pt, fill: dot-color),
              label-text(item.label),
            )
          ] else [
            #label-text(item.label)
          ]
          #v(spacing-2)
          #text(size: font-size-xl, weight: "bold", fill: color-text)[#item.value]
          #if item.sub != none [
            #v(spacing-1)
            #small-text(item.sub)
          ]
        ]
      })
    )
  ]
}
