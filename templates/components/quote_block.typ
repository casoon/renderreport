// QuoteBlock Component
// Quote / core statement / pull quote

#let quote-block(data) = {
  let is-emphasis = data.at("emphasis", default: false) == true
  let quote-size  = if is-emphasis { font-size-xl } else { font-size-base }

  block(
    width: 100%,
    fill: color-surface,
    stroke: (left: (paint: color-primary, thickness: 4pt)),
    radius: (right: 8pt),
    inset: spacing-4,
  )[
    // Decorative opening quote glyph
    #text(size: 3em, fill: color-primary.transparentize(55%), weight: "bold")["]

    #v(-spacing-3)

    // Quote text
    #par(justify: true)[
      #text(style: "italic", size: quote-size, fill: color-text)[#data.quote]
    ]

    #if data.author != none [
      #v(spacing-3)
      #text(weight: "semibold", size: font-size-sm, fill: color-text-muted)[— #data.author]
    ]
  ]
}
