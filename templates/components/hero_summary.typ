// Hero Summary Component

#let hero-summary(data) = {
  let status-color = if data.computed_status == "good" {
    color-ok
  } else if data.computed_status == "warning" {
    color-warn
  } else {
    color-bad
  }

  // ── Score + Domain header ──────────────────────────────────
  grid(
    columns: (1.1fr, 1.4fr),
    column-gutter: spacing-5,

    // Left: Score card
    theme-card[
      #align(center)[
        #small-text(data.domain)
        #v(spacing-3)
        #text(size: 52pt, weight: "bold", fill: status-color)[#data.score]
        #v(spacing-2)
        #text(size: font-size-lg, weight: "bold", fill: status-color)[Grade #data.grade]
        #v(spacing-3)
        #theme-progress-bar(data.score, bar-color: status-color)
      ]
    ],

    // Right: Verdict
    theme-card[
      #text(size: font-size-2xl, weight: "bold")[Kurzfazit]
      #v(spacing-3)
      #text(size: font-size-base)[#data.verdict]
    ],
  )

  v(spacing-5)

  // ── KPI Metric cards ───────────────────────────────────────
  if data.metrics.len() > 0 {
    let boxes = data.metrics.map(metric => {
      let accent = if metric.accent_color != none { rgb(metric.accent_color) } else { color-primary }
      theme-card[
        #label-text(metric.title)
        #v(spacing-2)
        #text(size: font-size-2xl, weight: "bold", fill: accent)[#metric.value]
      ]
    })

    grid(
      columns: (1fr,) * data.metrics.len(),
      column-gutter: spacing-3,
      ..boxes,
    )

    v(spacing-5)
  }

  // ── Top 3 Actions ──────────────────────────────────────────
  if data.top_actions.len() > 0 {
    text(size: font-size-xl, weight: "bold")[Wichtigste Maßnahmen]
    v(spacing-3)

    for (i, action) in data.top_actions.enumerate() {
      theme-card(fill: color-surface-soft)[
        #grid(
          columns: (auto, 1fr),
          gutter: spacing-3,
          text(size: font-size-lg, weight: "bold", fill: color-primary)[#{i + 1}],
          text(size: font-size-base)[#action],
        )
      ]
      v(spacing-2)
    }

    v(spacing-3)
  }

  // ── Positive aspects ───────────────────────────────────────
  if data.positive_aspects.len() > 0 {
    text(size: font-size-xl, weight: "bold", fill: color-ok)[Stärken]
    v(spacing-3)
    for aspect in data.positive_aspects {
      text(size: font-size-base)[#text(fill: color-ok)[✓ ]#aspect]
      v(spacing-2)
    }
  }
}
