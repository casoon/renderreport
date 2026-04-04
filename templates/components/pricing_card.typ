// PricingCard Component
// Single pricing/plan card with features and CTA

#let pricing-card(data) = {
  let border-width = if data.highlighted { 2pt } else { component-card-border-width }
  let border-color = if data.highlighted { color-primary } else { color-border }
  let bg-color = if data.highlighted { color-accent-soft } else { color-surface }

  block(
    width: 100%,
    fill: bg-color,
    stroke: (paint: border-color, thickness: border-width),
    radius: 10pt,
    inset: spacing-4,
  )[
    #if data.highlighted [
      #text(size: font-size-xs, weight: "bold", fill: color-primary, tracking: 0.08em)[RECOMMENDED]
      #v(spacing-2)
    ]

    #text(size: font-size-lg, weight: "bold", fill: color-text)[#data.name]
    #v(spacing-3)

    #text(size: font-size-3xl, weight: "bold", fill: color-primary)[#data.price]
    #if data.billing_period != none [
      #text(size: font-size-sm, fill: color-text-muted)[#data.billing_period]
    ]

    #if data.description != none [
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text-muted)[#data.description]
    ]

    #v(spacing-3)

    #if data.features.len() > 0 [
      #{
        for feature in data.features [
          #grid(
            columns: (10pt, 1fr),
            gutter: spacing-2,
            text(size: font-size-sm, fill: color-ok)[✓],
            text(size: font-size-sm, fill: color-text)[#feature],
          )
          #v(spacing-1)
        ]
      }
      #v(spacing-2)
    ]

    #if data.cta_label != none [
      #text(size: font-size-sm, weight: "semibold", fill: color-primary)[#data.cta_label →]
    ]
  ]
}
