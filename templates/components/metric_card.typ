// Metric Card Component
// Compact KPI display with accent color

#let metric-card(data) = {
  let accent = if data.accent_color != none { rgb(data.accent_color) } else { color-primary }
  let body = [
    #label-text(data.title)
    #v(spacing-2)
    #text(size: font-size-2xl, weight: "bold", fill: accent)[#data.value]
    #if data.subtitle != none [
      #v(spacing-2)
      #small-text(data.subtitle)
    ]
  ]

  if data.height != none {
    block(
      width: 100%,
      height: eval(data.height),
      fill: color-surface,
      stroke: (paint: color-border, thickness: component-card-border-width),
      radius: 10pt,
      inset: (x: spacing-4, y: spacing-4),
    )[ #body ]
  } else {
    theme-card[#body]
  }
}
