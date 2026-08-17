// ChecklistPanel Component
// Card with label–diagnosis rows and optional status indicators

#let checklist-panel(data) = {
  // Long checklists (e.g. an "all violations" list with dozens of rows) can
  // exceed a single page. theme-card() is breakable: false (right for small
  // cards), which would force the whole card onto one page and overflow past
  // the page boundary instead of paginating — the same breakable body/
  // non-breakable header split used by phase-block() for the same reason.
  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
    inset: (x: spacing-4, y: spacing-4),
    breakable: true,
  )[
    #if data.title != none [
      #component-title(text(weight: "bold", size: font-size-sm, fill: color-text)[#data.title])
    ]

    #for (i, row) in data.rows.enumerate() [
      #grid(
        columns: (1fr, 2fr),
        column-gutter: spacing-4,
        align: (top, top),

        // Label + optional status dot
        stack(
          dir: ltr,
          spacing: 4pt,
          if row.status != none {
            let dot-color = if row.status == "good" { color-ok }
              else if row.status == "warn" { color-warn }
              else if row.status == "bad"  { color-bad }
              else { color-text-muted }
            box(
              width: 7pt,
              height: 7pt,
              radius: 999pt,
              fill: dot-color,
            )
          },
          text(size: font-size-sm, weight: "bold", fill: color-text)[#row.label],
        ),

        text(size: font-size-sm, fill: color-text)[#row.diagnosis],
      )

      #if i < data.rows.len() - 1 [
        #v(spacing-2)
        #line(length: 100%, stroke: 0.5pt + color-border)
        #v(spacing-2)
      ]
    ]
  ]
}
