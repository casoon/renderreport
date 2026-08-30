// Benchmark Table Component
// Ranking table for website comparison. The fixed columns are Nr/Domain/
// Score/A11y/Krit. — every extra scored-dimension column (SEO, Security, ...)
// is caller-defined via BenchmarkRow.extra_columns, not hardcoded here. The
// header list is the union of labels seen across all rows, in first-seen
// order, so a new caller-side dimension needs no change in this template.

#let benchmark-table(data) = {
  if data.title != none {
    component-title(text(size: font-size-xl, weight: "bold")[#data.title], spacing: spacing-4)
  }

  let extra-headers = ()
  for row in data.rows {
    for col in row.extra_columns {
      if not extra-headers.contains(col.label) {
        extra-headers.push(col.label)
      }
    }
  }

  let col-widths = (8mm, 1fr, 16mm, 16mm) + extra-headers.map(_ => 16mm) + (18mm,)

  // Header
  block(
    width: 100%,
    fill: color-surface-alt,
    radius: (top-left: 10pt, top-right: 10pt),
    inset: (x: spacing-4, y: spacing-3),
  )[
    #set text(size: font-size-xs, weight: "bold", fill: color-text-muted)
    #grid(
      columns: col-widths,
      gutter: spacing-2,
      [Nr], [Domain], [Score], [A11y],
      ..extra-headers.map(h => [#h]),
      [Krit.],
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
        columns: col-widths,
        gutter: spacing-2,
        text(size: font-size-sm, fill: color-text-muted)[#row.rank],
        text(size: font-size-sm, weight: "bold")[#row.domain],
        text(size: font-size-sm, weight: "bold", fill: row-color)[#row.score],
        text(size: font-size-sm)[#row.accessibility],
        ..extra-headers.map(header => {
          let matching = row.extra_columns.filter(col => col.label == header)
          if matching.len() > 0 and matching.at(0).score != none {
            text(size: font-size-sm)[#matching.at(0).score]
          } else {
            text(size: font-size-sm, fill: color-text-muted)[—]
          }
        }),
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
