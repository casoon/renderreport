// SpotlightCard Component
// General-purpose spotlight card with left severity stripe

#let spotlight-card-color(variant) = {
  if variant == "critical"         { color-bad }
  else if variant == "info"        { color-info }
  else if variant == "feature"     { color-ok }
  else if variant == "opportunity" { color-warn }
  else { color-primary }
}

#let spotlight-card(data) = {
  let variant      = if data.variant != none { data.variant } else { "info" }
  let stripe-color = spotlight-card-color(variant)

  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
  )[
    #grid(
      columns: (4pt, 1fr),
      gutter: 0pt,

      // Left severity stripe
      block(
        width: 4pt,
        height: 100%,
        fill: stripe-color,
        radius: (left: 10pt),
      ),

      // Main content
      block(inset: (x: spacing-4, y: spacing-4))[
        #grid(
          columns: if data.metric != none { (1fr, auto) } else { (1fr,) },
          column-gutter: spacing-4,
          align: (top, right),

          // Left: eyebrow, title, body, detail, action
          stack(
            spacing: spacing-2,

            if data.eyebrow != none {
              text(
                size: font-size-xs,
                weight: "bold",
                fill: stripe-color,
                tracking: 0.08em,
                upper(data.eyebrow),
              )
            },

            text(size: font-size-lg, weight: "bold", fill: color-text)[#data.title],

            par(justify: true)[#text(size: font-size-sm, fill: color-text)[#data.body]],

            if data.detail != none {
              text(size: font-size-xs, fill: color-text-muted)[#data.detail]
            },

            if data.action != none {
              text(weight: "semibold", size: font-size-sm, fill: stripe-color)[#data.action →]
            },
          ),

          // Right: big metric (only if present)
          ..if data.metric != none {(
            align(right + top)[
              #text(size: font-size-3xl, weight: "bold", fill: stripe-color)[#data.metric]
            ],
          )},
        )
      ],
    )
  ]
}
