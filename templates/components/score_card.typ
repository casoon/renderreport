// Score Card Component
// Displays a metric with score visualization

#let score-card(data) = {
  let status-color = if data.computed_status == "good" {
    color-ok
  } else if data.computed_status == "warning" {
    color-warn
  } else {
    color-bad
  }

  box(
    width: 100%,
    inset: spacing-4,
    radius: component-score-card-radius,
    fill: color-surface,
    stroke: (paint: color-border, thickness: 0.5pt),
  )[
    #set text(fill: color-text)
    
    #text(size: font-size-sm, fill: color-text-muted)[#data.title]
    
    #v(spacing-2)
    
    #text(size: font-size-2xl, weight: "bold", fill: status-color)[
      #data.score#text(size: font-size-lg, weight: "regular")[/#data.max_score]
    ]
    
    #if data.description != none [
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text-muted)[#data.description]
    ]
  ]
}
