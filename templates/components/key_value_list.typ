// Key-Value List Component
// Inspired by BIRT Parameter Elements

#let key-value-list(data) = {
  box(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-base)[#data.title]
      #v(spacing-3)
    ]
    
    #if data.layout == "horizontal" [
      #for item in data.items [
        #box(
          inset: spacing-2,
          fill: if item.highlight { color-surface } else { none },
        )[
          #text(weight: "semibold", size: font-size-sm)[#item.key:]
          #h(spacing-2)
          #text(size: font-size-sm)[#item.value]
        ]
        #h(spacing-4)
      ]
    ] else [
      #grid(
        columns: (auto, 1fr),
        column-gutter: spacing-3,
        row-gutter: spacing-2,
        align: (left, left),

        ..data.items.map(item => (
          box(
            inset: (x: spacing-2, y: 2pt),
            fill: if item.highlight { color-surface } else { none },
          )[
            #text(weight: "semibold", size: font-size-sm, fill: color-text-muted)[#item.key]
          ],
          box(
            inset: (x: spacing-2, y: 2pt),
            fill: if item.highlight { color-surface } else { none },
          )[
            #text(size: font-size-sm)[#item.value]
          ],
        )).flatten()
      )
    ]
  ]
}
