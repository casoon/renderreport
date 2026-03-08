// Cover Page Component
// Professional audit report cover with score preview

#let cover-page(data) = {
  let status-color = if data.computed_status == "good" {
    color-ok
  } else if data.computed_status == "warning" {
    color-warn
  } else {
    color-bad
  }

  // Top bar: Brand + Date
  v(18mm)
  grid(
    columns: (1fr, auto),
    gutter: spacing-3,
    text(size: font-size-sm, weight: "bold", fill: color-text)[#data.brand],
    text(size: font-size-sm, fill: color-text-muted)[#data.date],
  )

  v(24mm)

  // Main title area
  text(size: 28pt, weight: "bold", fill: color-text)[#data.title]
  v(spacing-2)
  text(size: font-size-xl, fill: color-primary)[#data.domain]
  v(spacing-4)
  text(size: font-size-base, fill: color-text-muted)[#data.subtitle]

  v(30mm)

  // Score card
  block(
    width: 100%,
    fill: color-surface-soft,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 12pt,
    inset: spacing-5,
  )[
    #grid(
      columns: (1fr, 1fr, 1fr),
      gutter: spacing-5,
      [
        #label-text([Gesamtscore])
        #v(spacing-2)
        #text(size: 42pt, weight: "bold", fill: status-color)[#data.score]
        #v(spacing-2)
        #theme-progress-bar(data.score, bar-color: status-color)
      ],
      [
        #label-text([Bewertung])
        #v(spacing-2)
        #text(size: 22pt, weight: "bold", fill: status-color)[Grade #data.grade]
        #v(spacing-3)
        #text(size: font-size-sm, fill: color-text-muted)[von 100 Punkten]
      ],
      [
        #label-text([Probleme])
        #v(spacing-2)
        #text(size: 22pt, weight: "bold")[#data.total_issues]
        #v(spacing-3)
        #if data.critical_issues > 0 [
          #text(size: font-size-sm, weight: "bold", fill: color-bad)[#data.critical_issues kritisch]
        ] else [
          #text(size: font-size-sm, fill: color-text-muted)[keine kritischen]
        ]
      ],
    )
  ]

  v(spacing-6)

  // Module scope line
  if data.modules.len() > 0 {
    text(size: font-size-sm, fill: color-text-muted)[
      #data.modules.join(" · ")
    ]
  }

  pagebreak()
}
