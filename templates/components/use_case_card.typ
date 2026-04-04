// UseCaseCard Component
// Single use case: context + problem + solution + optional outcome

#let use-case-card(data) = {
  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
    inset: spacing-4,
  )[
    #text(size: font-size-lg, weight: "bold", fill: color-text)[#data.title]
    #v(spacing-2)
    #text(size: font-size-xs, fill: color-primary, weight: "semibold", tracking: 0.06em)[#upper(data.context)]
    #v(spacing-3)

    // Problem section
    #block(width: 100%, fill: color-bad-soft, radius: 6pt, inset: spacing-3)[
      #label-text("Problem")
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text)[#data.problem]
    ]
    #v(spacing-3)

    // Solution section
    #block(width: 100%, fill: color-ok-soft, radius: 6pt, inset: spacing-3)[
      #label-text("Solution")
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text)[#data.solution]
    ]

    #if data.outcome != none [
      #v(spacing-3)
      #block(width: 100%, fill: color-info.lighten(85%), radius: 6pt, inset: spacing-3)[
        #label-text("Outcome")
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#data.outcome]
      ]
    ]
  ]
}
