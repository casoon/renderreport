// Finding Component
// Displays an audit finding with severity indicator

#let severity-color(severity) = {
  if severity == "critical" or severity == "high" {
    color-bad
  } else if severity == "medium" {
    color-warn
  } else {
    color-ok
  }
}

#let severity-label(severity) = {
  if severity == "critical" { "CRITICAL" }
  else if severity == "high" { "HIGH" }
  else if severity == "medium" { "MEDIUM" }
  else if severity == "low" { "LOW" }
  else { "INFO" }
}

#let finding(data) = {
  let sev-color = severity-color(data.severity)

  block(
    width: 100%,
    inset: spacing-4,
    radius: component-finding-radius,
    stroke: (left: (paint: sev-color, thickness: 3pt), top: (paint: color-border, thickness: component-card-border-width), right: (paint: color-border, thickness: component-card-border-width), bottom: (paint: color-border, thickness: component-card-border-width)),
    fill: color-surface,
  )[
    #set text(fill: color-text)

    // Header with severity badge
    #grid(
      columns: (auto, 1fr),
      gutter: spacing-3,
      badge-for-severity(data.severity),
      text(weight: "bold", size: font-size-lg)[#data.title]
    )

    #v(spacing-3)

    // Description
    #par(justify: true)[#text(size: font-size-base)[#data.description]]

    // Affected resource
    #if data.affected != none [
      #v(spacing-2)
      #label-text([Betroffen: ])
      #mono-text(data.affected)
    ]

    // Recommendation
    #if data.recommendation != none [
      #v(spacing-3)
      #block(
        width: 100%,
        inset: spacing-3,
        radius: 8pt,
        fill: color-ok-soft,
        stroke: (paint: color-ok, thickness: 0.5pt),
      )[
        #label-text([Recommendation])
        #v(spacing-2)
        #text(size: font-size-base)[#data.recommendation]
      ]
    ]
  ]
}
