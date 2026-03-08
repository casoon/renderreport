// Severity Overview Component
// Visual severity breakdown with cards and strip bar

#let severity-overview(data) = {
  if data.title != none {
    text(size: font-size-xl, weight: "bold")[#data.title]
    v(spacing-3)
  }

  // Severity cards row
  let total = data.critical + data.serious + data.moderate + data.minor

  grid(
    columns: (1fr, 1fr, 1fr, 1fr),
    column-gutter: spacing-3,

    // Critical
    block(
      width: 100%,
      fill: color-surface,
      stroke: (top: (paint: color-bad, thickness: 3pt), left: (paint: color-border, thickness: component-card-border-width), right: (paint: color-border, thickness: component-card-border-width), bottom: (paint: color-border, thickness: component-card-border-width)),
      radius: 10pt,
      inset: (x: spacing-3, y: spacing-3),
    )[
      #label-text([Kritisch])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-bad)[#data.critical]
    ],

    // Serious
    block(
      width: 100%,
      fill: color-surface,
      stroke: (top: (paint: color-bad, thickness: 3pt), left: (paint: color-border, thickness: component-card-border-width), right: (paint: color-border, thickness: component-card-border-width), bottom: (paint: color-border, thickness: component-card-border-width)),
      radius: 10pt,
      inset: (x: spacing-3, y: spacing-3),
    )[
      #label-text([Schwerwiegend])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-bad)[#data.serious]
    ],

    // Moderate
    block(
      width: 100%,
      fill: color-surface,
      stroke: (top: (paint: color-warn, thickness: 3pt), left: (paint: color-border, thickness: component-card-border-width), right: (paint: color-border, thickness: component-card-border-width), bottom: (paint: color-border, thickness: component-card-border-width)),
      radius: 10pt,
      inset: (x: spacing-3, y: spacing-3),
    )[
      #label-text([Moderat])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-warn)[#data.moderate]
    ],

    // Minor
    block(
      width: 100%,
      fill: color-surface,
      stroke: (top: (paint: color-ok, thickness: 3pt), left: (paint: color-border, thickness: component-card-border-width), right: (paint: color-border, thickness: component-card-border-width), bottom: (paint: color-border, thickness: component-card-border-width)),
      radius: 10pt,
      inset: (x: spacing-3, y: spacing-3),
    )[
      #label-text([Gering])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-ok)[#data.minor]
    ],
  )

  v(spacing-3)

  // Severity strip bar
  if total > 0 {
    let high-count = data.critical + data.serious
    theme-severity-strip(high-count, data.moderate, data.minor)
    v(spacing-2)
    small-text([#total Problemgruppen insgesamt])
  }
}
