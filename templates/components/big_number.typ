// BigNumber Component
// Large metric display for marketing/impact stats

#let big-number(data) = {
  block(width: 100%, fill: color-surface, stroke: (paint: color-primary, thickness: 2pt), radius: 10pt, inset: spacing-4)[
    #align(center)[
      #text(size: 48pt, weight: "bold", fill: color-primary)[#data.value]
      #v(spacing-2)
      #text(size: font-size-lg, weight: "bold", fill: color-text)[#data.label]
      #if data.context != none [
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text-muted)[#data.context]
      ]
    ]
  ]
}
