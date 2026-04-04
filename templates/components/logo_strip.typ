// LogoStrip Component
// Display logos/names of customers, partners, certifications

#let logo-strip(data) = {
  block(width: 100%)[
    #if data.title != none [
      #text(size: font-size-sm, weight: "bold", fill: color-text-muted, tracking: 0.06em)[#upper(data.title)]
      #v(spacing-3)
    ]

    #grid(
      columns: (1fr,) * data.columns,
      column-gutter: spacing-3,
      row-gutter: spacing-3,
      ..data.labels.map(label => {
        block(
          width: 100%,
          fill: color-surface,
          stroke: (paint: color-border, thickness: 0.5pt),
          radius: 6pt,
          inset: spacing-3,
        )[
          #align(center + horizon)[
            #text(size: font-size-sm, weight: "semibold", fill: color-text)[#label]
          ]
        ]
      }),
    )
  ]
}
