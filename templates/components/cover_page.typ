// Cover Page Component
// Professional report cover with score preview + module gauge strip.

#let cover-page(data) = {
  let status-color = if data.computed_status == "good" {
    color-ok
  } else if data.computed_status == "warning" {
    color-warn
  } else {
    color-bad
  }

  set text(hyphenate: false)

  // ── Accent bar at top ─────────────────────────────────────────
  place(top + left, dx: -page-margin, dy: -page-margin-top,
    rect(width: 100% + 2 * page-margin, height: 6pt, fill: color-primary)
  )

  v(9mm)

  // ── Brand + Date header ───────────────────────────────────────
  // A custom logo (white-label) takes the brand slot when provided; otherwise
  // the brand name is shown as text.
  let brand-mark = if data.at("logo_src", default: "") != "" {
    image(data.logo_src, height: 9mm)
  } else {
    text(size: font-size-base, weight: "bold", fill: color-primary)[#data.brand]
  }
  grid(
    columns: (1fr, auto),
    align: (left + horizon, right + horizon),
    gutter: spacing-3,
    brand-mark,
    text(size: font-size-sm, fill: color-text-muted)[#data.date],
  )

  v(5mm)
  line(length: 100%, stroke: 0.5pt + color-border)
  v(10mm)

  // ── Title block ───────────────────────────────────────────────
  text(size: 34pt, weight: "bold", fill: color-text, tracking: -0.5pt)[#data.title]
  v(spacing-3)
  text(size: 16pt, weight: "semibold", fill: color-primary)[#data.domain]
  if data.subtitle != "" {
    v(spacing-3)
    text(size: font-size-base, fill: color-text-muted)[#data.subtitle]
  }

  v(8mm)

  // ── Score card: score + condition | findings ──────────────────
  let condition = if data.band_phrase != "" { data.band_phrase } else { data.grade }
  let score-label = if data.at("score_label", default: "") != "" { data.score_label } else { "GESAMTSCORE" }
  let findings-label = if data.at("findings_label", default: "") != "" { data.findings_label } else { "BEFUNDE" }
  let modules-label = if data.at("modules_label", default: "") != "" { data.modules_label } else { "MODULE IM ÜBERBLICK" }
  block(
    width: 100%,
    fill: color-surface-soft,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
    inset: spacing-5,
  )[
    #grid(
      columns: (1.7fr, 1fr),
      column-gutter: spacing-5,
      // Score + condition
      [
        #text(size: font-size-xs, weight: "bold", fill: color-text-muted, tracking: 1pt)[#score-label]
        #v(spacing-2)
        #grid(
          columns: (auto, 1fr),
          column-gutter: spacing-3,
          align: (bottom, bottom),
          text(size: 56pt, weight: "bold", fill: status-color)[#data.score],
          [
            #v(spacing-1)
            #text(size: font-size-lg, weight: "bold", fill: color-text)[#condition]
          ],
        )
        #v(spacing-3)
        #theme-progress-bar(data.score, bar-color: status-color)
      ],
      // Findings
      [
        #text(size: font-size-xs, weight: "bold", fill: color-text-muted, tracking: 1pt)[#findings-label]
        #v(spacing-2)
        #text(size: 32pt, weight: "bold", fill: color-text)[#data.total_issues]
        #v(spacing-3)
        #if data.critical_issues > 0 [
          #box(
            fill: color-bad-soft,
            radius: 999pt,
            inset: (x: 8pt, y: 4pt),
            text(size: font-size-xs, weight: "bold", fill: color-bad)[#data.critical_issues #{if data.at("label_critical", default: "") != "" { data.label_critical } else { "kritisch" }}]
          )
        ] else [
          #box(
            fill: color-ok-soft,
            radius: 999pt,
            inset: (x: 8pt, y: 4pt),
            text(size: font-size-xs, weight: "bold", fill: color-ok)[#{if data.at("label_no_critical", default: "") != "" { data.label_no_critical } else { "0 kritisch" }}]
          )
        ]
      ],
    )
  ]

  // ── Module gauge strip ────────────────────────────────────────
  if data.module_gauges.len() > 0 {
    v(7mm)
    text(size: font-size-xs, weight: "bold", fill: color-text-muted, tracking: 1pt)[#modules-label]
    v(spacing-3)

    let cover-ring(name, score) = {
      let gc = if score >= 75 { color-ok } else if score >= 40 { color-warn } else { color-bad }
      align(center)[
        #box(width: 62pt, height: 62pt, {
          let cx = 31pt
          let cy = 31pt
          let r = 25pt
          place(center + horizon, circle(radius: r, stroke: 4pt + color-border, fill: none))
          let frac = calc.min(1.0, calc.max(0.0, score / 100))
          if frac > 0 {
            let n = calc.max(2, int(calc.round(frac * 60)))
            let pts = range(0, n + 1).map(i => {
              let ang = -90deg + (i / n) * frac * 360deg
              (cx + r * calc.cos(ang), cy + r * calc.sin(ang))
            })
            place(top + left, path(
              stroke: (paint: gc, thickness: 4pt, cap: "round"),
              ..pts
            ))
          }
          place(center + horizon, text(weight: "bold", size: 15pt, fill: color-text)[#score])
        })
        #v(spacing-1)
        #box(width: 100%, height: 22pt,
          align(center + top, text(size: font-size-xs, weight: "medium", fill: color-text-muted)[#name])
        )
      ]
    }

    let cols = calc.min(data.module_gauges.len(), 4)
    grid(
      columns: (1fr,) * cols,
      column-gutter: spacing-3,
      row-gutter: spacing-4,
      ..data.module_gauges.map(m => cover-ring(m.name, m.score)),
    )
  }

  // ── Bottom accent ─────────────────────────────────────────────
  place(bottom + left, dx: -page-margin, dy: page-margin-bottom,
    rect(width: 100% + 2 * page-margin, height: 3pt, fill: color-primary.lighten(60%))
  )

  pagebreak()
}
