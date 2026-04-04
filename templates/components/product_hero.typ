// ProductHero Component
// Hero page for product/project introduction

#let product-hero(data) = {
  block(width: 100%, height: 100%)[
    #set text(fill: color-text)

    // Title
    #text(size: font-size-3xl, weight: "bold", fill: color-primary)[#data.title]

    #v(spacing-3)

    // Subtitle
    #text(size: font-size-lg, fill: color-text)[#data.subtitle]

    #if data.tagline != none [
      #v(spacing-2)
      #text(size: font-size-base, fill: color-text-muted, style: "italic")[#data.tagline]
    ]

    #v(spacing-6)

    // Highlights
    #if data.highlights.len() > 0 [
      #{
        let cols = if data.highlights.len() <= 2 { (1fr, 1fr) } else { (1fr, 1fr, 1fr) }
        let items = data.highlights.map(highlight => {
          block(
            width: 100%,
            fill: color-surface,
            stroke: (paint: color-primary, thickness: 0.5pt),
            radius: 6pt,
            inset: spacing-3,
          )[
            #text(size: font-size-sm, fill: color-text)[✓ #highlight]
          ]
        })
        grid(columns: cols, gutter: spacing-4, ..items)
      }

      #v(spacing-6)
    ]

    // CTA (if provided)
    #if data.cta_label != none and data.cta_url != none [
      #link(data.cta_url)[
        #block(
          width: auto,
          fill: color-primary,
          radius: 6pt,
          inset: (x: spacing-4, y: spacing-3),
        )[
          #text(size: font-size-base, weight: "bold", fill: white)[#data.cta_label →]
        ]
      ]
    ]
  ]
}
