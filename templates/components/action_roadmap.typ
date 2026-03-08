// Action Roadmap Component

#let action-roadmap(data) = {
  let col-boxes = data.columns.map(column => {
    let accent = if column.accent_color != none { rgb(column.accent_color) } else { color-primary }

    box(width: 100%)[
      // Column header
      #box(
        width: 100%,
        inset: (x: spacing-4, y: spacing-3),
        radius: (top-left: 10pt, top-right: 10pt),
        fill: accent.lighten(90%),
        stroke: (paint: accent, thickness: 0.8pt),
      )[
        #text(size: font-size-lg, weight: "bold", fill: accent)[#column.title]
        #h(spacing-2)
        #small-text([#column.items.len() Maßnahmen])
      ]

      #v(spacing-3)

      // Items
      #for item in column.items {
        theme-card(fill: color-surface-soft)[
          #text(size: font-size-base, weight: "bold")[#item.action]
          #v(spacing-2)
          #label-text([#item.role · #item.priority])
          #if item.benefit != "" [
            #v(spacing-2)
            #small-text(item.benefit)
          ]
        ]
        v(spacing-2)
      }
    ]
  })

  grid(
    columns: (1fr,) * data.columns.len(),
    column-gutter: spacing-4,
    ..col-boxes,
  )
}
