// StepCardRow Component
// Numbered steps displayed horizontally (2-4 steps)

#let step-card-row(data) = {
  let items = range(data.titles.len()).map(i => {
    block(width: 100%)[
      #block(
        width: 40pt,
        height: 40pt,
        fill: color-primary,
        stroke: (paint: color-primary, thickness: 2pt),
        radius: 50%,
        align(center + horizon)[
          #text(size: font-size-lg, weight: "bold", fill: white)[#{i + 1}]
        ],
      )
      #v(spacing-3)
      #text(size: font-size-base, weight: "bold", fill: color-text)[#data.titles.at(i)]
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text-muted)[#data.descriptions.at(i)]
    ]
  })

  block(width: 100%)[
    #grid(
      columns: (1fr,) * data.columns,
      column-gutter: spacing-4,
      ..items,
    )
  ]
}
