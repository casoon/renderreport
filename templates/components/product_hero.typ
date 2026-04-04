// ProductHero Component
// Hero page for product/project introduction — occupies full page

#let product-hero(data) = {
  // Full-page hero with vertical centering
  block(width: 100%, height: 100%, breakable: false)[
    #v(1fr)

    // Decorative accent line
    #block(width: 60pt, height: 4pt, fill: color-primary, radius: 2pt)

    #v(spacing-5)

    // Title — large, bold
    #text(size: 52pt, weight: "bold", fill: color-text, tracking: -0.02em)[#data.title]

    #v(spacing-3)

    // Subtitle — lighter, generous size
    #text(size: 20pt, fill: color-text-muted, weight: "regular")[#data.subtitle]

    // Tagline — small, muted, italic
    #if data.tagline != none [
      #v(spacing-4)
      #block(
        width: 75%,
      )[
        #text(size: font-size-base, fill: color-text-muted, style: "italic")[#data.tagline]
      ]
    ]

    #v(spacing-7)

    // Highlights — 2-column grid with left accent bar
    #if data.highlights.len() > 0 [
      #{
        let items = data.highlights.map(highlight => {
          block(
            width: 100%,
            fill: color-accent-soft,
            stroke: (left: (paint: color-primary, thickness: 3pt)),
            radius: (right: 5pt),
            inset: (x: spacing-4, y: spacing-3),
          )[
            #text(size: font-size-sm, fill: color-text)[#highlight]
          ]
        })
        grid(
          columns: (1fr, 1fr),
          row-gutter: spacing-3,
          column-gutter: spacing-4,
          ..items,
        )
      }
    ]

    #v(1fr)

    // Footer line with optional CTA
    #line(length: 100%, stroke: 0.5pt + color-border)
    #v(spacing-3)
    #{
      if data.cta_url != none and data.cta_label != none {
        text(size: font-size-xs, fill: color-text-muted)[
          #link(data.cta_url)[#text(fill: color-primary, weight: "semibold")[#data.cta_label]]
        ]
      }
    }
  ]
}
