// WhyItMatters Component
// Context / relevance block for storytelling

#let why-it-matters(data) = {
  let heading = if data.title != none { data.title } else { "Why it matters" }

  block(
    width: 100%,
    fill: color-surface,
    stroke: (
      left: (paint: color-warn, thickness: 4pt),
      top: (paint: color-border, thickness: component-card-border-width),
      right: (paint: color-border, thickness: component-card-border-width),
      bottom: (paint: color-border, thickness: component-card-border-width),
    ),
    radius: (right: 8pt),
    inset: spacing-4,
  )[
    #text(weight: "bold", size: font-size-base, fill: color-warn)[#heading]
    #v(spacing-2)
    #par(justify: true)[#text(size: font-size-base, fill: color-text)[#data.body]]

    #if data.evidence != none [
      #v(spacing-3)
      #text(style: "italic", size: font-size-sm, fill: color-text-muted)[#data.evidence]
    ]

    #if data.urgency != none [
      #v(spacing-3)
      #block(
        fill: color-warn-soft,
        radius: 4pt,
        inset: (x: spacing-2, y: 3pt),
      )[
        #text(weight: "semibold", size: font-size-xs, fill: color-warn)[#data.urgency]
      ]
    ]
  ]
}
