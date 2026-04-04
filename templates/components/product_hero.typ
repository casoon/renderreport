// ProductHero Component
// Hero page for product/project introduction

#let product-hero(data) = {
  block(width: 100%, inset: (top: 40pt, bottom: 40pt))[
    // Title — large, bold, primary color
    #text(size: 48pt, weight: "bold", fill: color-primary)[#data.title]

    #v(spacing-2)

    // Subtitle — medium size, secondary text color
    #text(size: 18pt, fill: color-text, weight: "regular")[#data.subtitle]

    #v(spacing-4)

    // Tagline — small, muted, italic
    #if data.tagline != none [
      #text(size: font-size-sm, fill: color-text-muted, style: "italic")[#data.tagline]
      #v(spacing-5)
    ]

    // Highlights — 2-column grid with accent backgrounds
    #if data.highlights.len() > 0 [
      #{
        let cols = (1fr, 1fr)
        let items = data.highlights.map(highlight => {
          block(
            width: 100%,
            fill: color-accent-soft,
            stroke: (left: (paint: color-primary, thickness: 3pt)),
            radius: (right: 6pt),
            inset: spacing-4,
          )[
            #text(size: font-size-sm, fill: color-text, weight: "semibold")[#highlight]
          ]
        })
        grid(columns: cols, row-gutter: spacing-3, column-gutter: spacing-4, ..items)
      }
    ]
  ]
}
