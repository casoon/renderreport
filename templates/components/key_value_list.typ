// Key-Value List Component
// Inspired by BIRT Parameter Elements

#let key-value-list(data) = {
  box(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-base)[#data.title])
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
      // Key column is a fixed fraction, not `auto`: `auto` sizes to the
      // widest *unbroken* run in any key, uncapped. A short label (the
      // component's intended use) is unaffected, but an unbounded-length
      // key (a CSS selector, a URL, a filename) forces the column to claim
      // nearly the full row width, squeezing the value into a
      // one-word-per-line sliver spanning dozens of rows and corrupting the
      // following page break. 32% comfortably fits every key seen in
      // practice (single words, short phrases like "Unminifizierte
      // Dateien") while still bounding the pathological case.
      #grid(
        columns: (32%, 1fr),
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
