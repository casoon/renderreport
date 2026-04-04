// StatPair Component
// Two stats side-by-side with optional vertical divider

#let stat-entry(entry) = {
  let accent = if entry.accent_color != none { rgb(entry.accent_color) } else { color-primary }
  stack(
    spacing: spacing-2,
    label-text(entry.label),
    stack(
      dir: ltr,
      spacing: 4pt,
      text(size: font-size-2xl, weight: "bold", fill: accent)[#entry.value],
      if entry.unit != none [
        #pad(top: 4pt)[#small-text(entry.unit)]
      ],
    ),
  )
}

#let stat-pair(data) = {
  theme-card[
    #grid(
      columns: if data.divider { (1fr, 0pt, 1fr) } else { (1fr, 1fr) },
      column-gutter: spacing-4,
      align: (top, center, top),

      stat-entry(data.left),

      if data.divider [
        #line(length: 100%, angle: 90deg, stroke: 0.5pt + color-border)
      ],

      stat-entry(data.right),
    )
  ]
}
