// DiagnosisPanel Component
// Card with label–diagnosis rows and optional status indicators

#let diagnosis-panel(data) = {
  theme-card[
    #if data.at("title", default: none) != none [
      #text(weight: "bold", size: font-size-sm, fill: color-text)[#data.title]
      #v(spacing-3)
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
          if row.at("status", default: none) != none {
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
