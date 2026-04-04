// BenefitStrip Component
// Horizontal benefit strip (3-5 points) for marketing, features, value props

#let benefit-strip(data) = {
  let items = range(data.titles.len()).map(i => {
    block(width: 100%)[
      #if i < data.icons.len() and data.icons.at(i) != "" [
        #text(size: 32pt)[#data.icons.at(i)]
        #v(spacing-2)
      ]
      #text(size: font-size-lg, weight: "bold", fill: color-text)[#data.titles.at(i)]
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
