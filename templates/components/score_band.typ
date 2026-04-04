// ScoreBand Component
// Three color-coded segments (bad/warn/good) with a triangle marker at value position

#let score-band(data) = {
  let val       = calc.min(100, calc.max(0, data.value))
  let good-t    = data.good_threshold
  let warn-t    = data.warn_threshold

  // Segment widths as percentages of total 100
  let bad-pct   = warn-t * 1%
  let warn-pct  = (good-t - warn-t) * 1%
  let good-pct  = (100 - good-t) * 1%

  // Marker position as fraction of total width
  let marker-pos = val * 1%

  block(width: 100%)[
    #if data.label != none [
      #label-text(data.label)
      #v(spacing-2)
    ]

    // Color bar
    #box(width: 100%, height: 28pt)[
      #grid(
        columns: (bad-pct, warn-pct, good-pct),
        gutter: 0pt,
        box(fill: color-bad,  height: 28pt, width: 100%, radius: (left: 4pt)),
        box(fill: color-warn, height: 28pt, width: 100%),
        box(fill: color-ok,   height: 28pt, width: 100%, radius: (right: 4pt)),
      )
      // Triangle marker
      #place(
        left + horizon,
        dx: marker-pos - 5pt,
        polygon(
          fill: color-text,
          (0pt, 0pt),
          (10pt, 0pt),
          (5pt, 10pt),
        )
      )
    ]

    #if data.show_value [
      #v(spacing-1)
      #align(center)[#text(size: font-size-sm, weight: "bold", fill: color-text)[#val]]
    ]
  ]
}
