// Cover Page Component
// Professional report cover with score preview

#let cover-page(data) = {
  let status-color = if data.computed_status == "good" {
    color-ok
  } else if data.computed_status == "warning" {
    color-warn
  } else {
    color-bad
  }

  // ── Accent bar at top ─────────────────────────────────────────
  place(top + left, dx: -page-margin, dy: -page-margin-top,
    rect(width: 100% + 2 * page-margin, height: 6pt, fill: color-primary)
  )

  v(12mm)

  // ── Brand + Date header ───────────────────────────────────────
  grid(
    columns: (1fr, auto),
    gutter: spacing-3,
    text(size: font-size-base, weight: "bold", fill: color-primary)[#data.brand],
    text(size: font-size-sm, fill: color-text-muted)[#data.date],
  )

  v(8mm)
  line(length: 100%, stroke: 0.5pt + color-border)
  v(16mm)

  // ── Title block ───────────────────────────────────────────────
  text(size: 32pt, weight: "bold", fill: color-text, tracking: -0.5pt)[#data.title]
  v(spacing-3)
  text(size: 18pt, weight: "semibold", fill: color-primary)[#data.domain]
  v(spacing-4)
  text(size: font-size-base, fill: color-text-muted)[#data.subtitle]

  v(16mm)

  // ── Score card ────────────────────────────────────────────────
  block(
    width: 100%,
    fill: color-surface-soft,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 8pt,
    inset: spacing-5,
  )[
    #grid(
      columns: (1fr, 1fr, 1fr),
      gutter: spacing-5,
      // Score column
      [
        #text(size: font-size-xs, weight: "bold", fill: color-text-muted, tracking: 1pt)[SCORE]
        #v(spacing-2)
        #text(size: 48pt, weight: "bold", fill: status-color)[#data.score]
        #v(spacing-2)
        #theme-progress-bar(data.score, bar-color: status-color)
      ],
      // Grade column
      [
        #text(size: font-size-xs, weight: "bold", fill: color-text-muted, tracking: 1pt)[GRADE]
        #v(spacing-2)
        #text(size: 28pt, weight: "bold", fill: status-color)[#data.grade]
        #v(spacing-3)
        #text(size: font-size-sm, fill: color-text-muted)[von 100 Punkten]
      ],
      // Issues column
      [
        #text(size: font-size-xs, weight: "bold", fill: color-text-muted, tracking: 1pt)[ISSUES]
        #v(spacing-2)
        #text(size: 28pt, weight: "bold", fill: color-text)[#data.total_issues]
        #v(spacing-3)
        #if data.critical_issues > 0 [
          #box(
            fill: color-bad-soft,
            radius: 4pt,
            inset: (x: 6pt, y: 3pt),
            text(size: font-size-xs, weight: "bold", fill: color-bad)[#data.critical_issues critical]
          )
        ] else [
          #box(
            fill: color-ok-soft,
            radius: 4pt,
            inset: (x: 6pt, y: 3pt),
            text(size: font-size-xs, weight: "bold", fill: color-ok)[none critical]
          )
        ]
      ],
    )
  ]

  v(spacing-6)

  // ── Module tags ───────────────────────────────────────────────
  if data.modules.len() > 0 {
    stack(dir: ltr, spacing: 6pt,
      ..data.modules.map(m =>
        box(
          fill: color-surface,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          inset: (x: 10pt, y: 5pt),
          text(size: font-size-xs, weight: "medium", fill: color-text-muted)[#m]
        )
      )
    )
  }

  // ── Bottom accent ─────────────────────────────────────────────
  place(bottom + left, dx: -page-margin, dy: page-margin-bottom,
    rect(width: 100% + 2 * page-margin, height: 3pt, fill: color-primary.lighten(60%))
  )

  pagebreak()
}
