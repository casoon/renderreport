// Audit Table Component
// Modern, borderless data table: no cell grid, thin horizontal row dividers,
// a stronger header underline, and generous vertical padding. Avoids the
// "form/DIN" look of fully-boxed tables.

#let audit-table(data) = {
  let col-count = data.columns.len()

  // Build column widths
  let col-widths = data.columns.map(col => {
    if col.width != none {
      eval(col.width)
    } else {
      1fr
    }
  })

  block(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-lg)[#data.title])
    ]

    #set text(size: font-size-sm)

    #table(
      columns: col-widths,
      inset: (x: spacing-3, y: 8pt),
      align: left + horizon,
      // Horizontal rules only — a strong header underline and light hairlines
      // between rows. No vertical lines, no outer box.
      stroke: (x, y) => (
        bottom: if y == 0 {
          (paint: color-text-muted, thickness: 1pt)
        } else {
          (paint: table-border, thickness: 0.5pt)
        },
      ),
      fill: (x, y) => {
        if data.striped and y > 0 and calc.even(y) { table-row-alt-bg } else { none }
      },

      // Header row — compact muted label, like a section eyebrow
      ..data.columns.map(col => {
        text(weight: "bold", fill: color-text-muted, size: font-size-xs)[#col.header]
      }),

      // Data rows
      ..data.rows.flatten()
    )
  ]
}
