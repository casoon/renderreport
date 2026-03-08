// Benchmark Table Component
// Ranking table for website comparison

#let benchmark-table(data) = {
  if data.title != none {
    text(size: font-size-xl, weight: "bold")[#data.title]
    v(spacing-4)
  }

  // Header
  block(
    width: 100%,
    fill: color-surface-alt,
    radius: (top-left: 10pt, top-right: 10pt),
    inset: (x: spacing-4, y: spacing-3),
  )[
    #set text(size: font-size-xs, weight: "bold", fill: color-text-muted)
    #grid(
      columns: (8mm, 1fr, 16mm, 16mm, 16mm, 16mm, 16mm, 18mm),
      gutter: spacing-2,
      [Nr], [Domain], [Score], [A11y], [SEO], [Perf], [Sec], [Krit.],
    )
  ]

  // Rows
  for row in data.rows {
    let row-color = if row.computed_status == "good" { color-ok }
      else if row.computed_status == "warning" { color-warn }
      else { color-bad }

    block(
      width: 100%,
      inset: (x: spacing-4, y: spacing-3),
      stroke: (bottom: (paint: color-border, thickness: 0.5pt)),
    )[
      #grid(
        columns: (8mm, 1fr, 16mm, 16mm, 16mm, 16mm, 16mm, 18mm),
        gutter: spacing-2,
        text(size: font-size-sm, fill: color-text-muted)[#row.rank],
        text(size: font-size-sm, weight: "bold")[#row.domain],
        text(size: font-size-sm, weight: "bold", fill: row-color)[#row.score],
        text(size: font-size-sm)[#row.accessibility],
        {
          if row.seo != none { text(size: font-size-sm)[#row.seo] }
          else { text(size: font-size-sm, fill: color-text-muted)[—] }
        },
        {
          if row.performance != none { text(size: font-size-sm)[#row.performance] }
          else { text(size: font-size-sm, fill: color-text-muted)[—] }
        },
        {
          if row.security != none { text(size: font-size-sm)[#row.security] }
          else { text(size: font-size-sm, fill: color-text-muted)[—] }
        },
        {
          if row.critical_issues > 0 {
            text(size: font-size-sm, weight: "bold", fill: color-bad)[#row.critical_issues]
          } else {
            text(size: font-size-sm, fill: color-text-muted)[0]
          }
        },
      )
    ]
  }
}
