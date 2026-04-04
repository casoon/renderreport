// Testimonial Component
// Customer quote / statement

#let testimonial(data) = {
  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
    inset: spacing-5,
  )[
    // Decorative quote mark
    #text(size: 4em, fill: color-primary.transparentize(60%))["]

    #v(-spacing-4)

    // Quote text
    #par(justify: true)[
      #text(style: "italic", size: font-size-base, fill: color-text)[#data.quote]
    ]

    #v(spacing-3)

    // Attribution
    #{
      let parts = ("— " + data.author,)
      let role-company = if data.role != none and data.company != none {
        data.role + ", " + data.company
      } else if data.role != none {
        data.role
      } else if data.company != none {
        data.company
      } else {
        none
      }

      stack(
        spacing: 2pt,
        text(weight: "semibold", size: font-size-sm, fill: color-text)[— #data.author],
        if role-company != none {
          text(size: font-size-xs, fill: color-text-muted)[#role-company]
        },
      )
    }
  ]
}
