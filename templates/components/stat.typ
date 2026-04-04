// Stat Component
// Large value KPI with label on top, optional trend badge below

#let stat(data) = {
  let accent = if data.accent_color != none { rgb(data.accent_color) } else { color-primary }

  theme-card[
    #label-text(data.label)
    #v(spacing-2)
    #stack(
      dir: ltr,
      spacing: 4pt,
      text(size: font-size-2xl, weight: "bold", fill: accent)[#data.value],
      if data.unit != none [
        #pad(top: 4pt)[#small-text(data.unit)]
      ],
    )
    #if data.trend != none [
      #v(spacing-2)
      #{
        let is-pos = data.trend_positive
        let trend-color = if is-pos == true { color-ok } else if is-pos == false { color-bad } else { color-text-muted }
        let trend-bg   = if is-pos == true { color-ok-soft } else if is-pos == false { color-bad-soft } else { color-surface-alt }
        theme-badge(data.trend, trend-color, trend-bg)
      }
    ]
  ]
}
