// Metric Card Component
// Compact KPI display with accent color

#let metric-card(data) = {
  let accent = if data.accent_color != none { rgb(data.accent_color) } else { color-primary }

  theme-card[
    #label-text(data.title)
    #v(spacing-2)
    #text(size: font-size-2xl, weight: "bold", fill: accent)[#data.value]
    #if data.subtitle != none [
      #v(spacing-2)
      #small-text(data.subtitle)
    ]
  ]
}
