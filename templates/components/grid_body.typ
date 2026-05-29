#let grid-component(data) = {
  box(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-lg)[#data.title], spacing: spacing-4)
    ]

    #let col-widths = (1fr,) * data.columns
    #let item-min-height = if data.item_min_height != none { eval(data.item_min_height) } else { none }

    #grid(
      columns: col-widths,
      column-gutter: eval(data.column_gap),
      row-gutter: eval(data.row_gap),

      ..data.items.map(item => {
        let c = item.content
        let item-body = if type(c) == dictionary and "type" in c and "data" in c {
          [#_grid-dispatch(c)]
        } else {
          box(
            width: 100%,
            inset: spacing-3,
            fill: color-surface,
            radius: 4pt,
            stroke: (paint: color-border, thickness: 0.5pt),
          )[#_grid-dispatch(c)]
        }

        if item-min-height != none {
          box(width: 100%, height: item-min-height)[#item-body]
        } else {
          box(width: 100%)[#item-body]
        }
      })
    )
  ]
}
