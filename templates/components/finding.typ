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
  
  box(
    width: 100%,
    inset: spacing-4,
    radius: component-finding-radius,
    stroke: (left: (paint: sev-color, thickness: 3pt)),
    fill: color-surface,
  )[
    #set text(fill: color-text)
    
    // Header with severity badge
    #grid(
      columns: (auto, 1fr),
      gutter: spacing-3,
      box(
        inset: (x: spacing-2, y: spacing-1),
        radius: 2pt,
        fill: sev-color,
        text(size: font-size-sm, weight: "bold", fill: white)[#severity-label(data.severity)]
      ),
      text(weight: "semibold", size: font-size-lg)[#data.title]
    )
    
    #v(spacing-3)
    
    // Description
    #text(size: font-size-base)[#data.description]
    
    // Affected resource
    #if data.affected != none [
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text-muted)[
        *Affected:* #raw(data.affected)
      ]
    ]
    
    // Recommendation
    #if data.recommendation != none [
      #v(spacing-3)
      #box(
        width: 100%,
        inset: spacing-3,
        radius: 2pt,
        fill: rgb("#f0fdf4"),
        stroke: (paint: color-ok, thickness: 0.5pt),
      )[
        #text(size: font-size-sm, weight: "semibold", fill: color-ok)[Recommendation]
        #v(spacing-1)
        #text(size: font-size-sm)[#data.recommendation]
      ]
    ]
  ]
}
