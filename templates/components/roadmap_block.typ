// RoadmapBlock Component

#let roadmap-block(data) = {
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
        #small-text([#column.items.len() actions])
      ]

      #v(spacing-2)

      // Items
      #for item in column.items {
        theme-card(fill: color-surface-soft)[
          #text(size: font-size-base, weight: "bold")[#item.action]
          #v(spacing-2)
          #grid(
            columns: (auto, 1fr),
            row-gutter: spacing-1,
            column-gutter: spacing-3,
            label-text([Impact]), text(size: font-size-sm)[#item.priority],
            label-text([Effort]), text(size: font-size-sm)[#{if item.effort != none { item.effort } else { "—" }}],
            label-text([Role]), text(size: font-size-sm)[#item.role],
          )
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
