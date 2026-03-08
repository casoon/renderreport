// Benchmark Summary Component
// Portfolio-level KPI cards for batch reports

#let benchmark-summary(data) = {
  let avg-color = if data.average_score >= 75 { color-ok }
    else if data.average_score >= 50 { color-warn }
    else { color-bad }

  grid(
    columns: (1fr, 1fr, 1fr, 1fr),
    column-gutter: spacing-3,

    // Total sites
    theme-card[
      #label-text([Geprüfte Websites])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold")[#data.total_sites]
    ],

    // Average score
    theme-card[
      #label-text([Durchschnitt])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: avg-color)[#data.average_score]
      #v(spacing-2)
      #theme-progress-bar(data.average_score, bar-color: avg-color)
    ],

    // Best
    theme-card[
      #label-text([Beste Website])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-ok)[#data.best_score]
      #v(spacing-2)
      #small-text(data.best_domain)
    ],

    // Worst
    theme-card[
      #label-text([Schwächste Website])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-bad)[#data.worst_score]
      #v(spacing-2)
      #small-text(data.worst_domain)
    ],
  )

  v(spacing-4)

  // Issue summary row
  grid(
    columns: (1fr, 1fr),
    column-gutter: spacing-3,

    theme-card[
      #label-text([Verstöße gesamt])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold")[#data.total_issues]
    ],

    theme-card[
      #label-text([Kritische Verstöße])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-bad)[#data.critical_issues]
    ],
  )
}
