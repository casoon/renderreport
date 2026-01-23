// Lighthouse Scores Component
// Displays Google Lighthouse scores in a visual grid

#let score-color(score) = {
  if score >= 90 { color-ok }
  else if score >= 50 { color-warn }
  else { color-bad }
}

#let score-circle(score, label) = {
  let col = score-color(score)
  
  box(
    width: 80pt,
    inset: spacing-3,
  )[
    #align(center)[
      // Score circle
      #box(
        width: 60pt,
        height: 60pt,
        radius: 50%,
        stroke: (paint: col, thickness: 4pt),
        inset: spacing-2,
      )[
        #align(center + horizon)[
          #text(size: font-size-xl, weight: "bold", fill: col)[#score]
        ]
      ]
      
      #v(spacing-2)
      
      // Label
      #text(size: font-size-sm, fill: color-text-muted)[#label]
    ]
  ]
}

#let lighthouse-scores(data) = {
  box(
    width: 100%,
    inset: spacing-4,
    radius: 8pt,
    fill: color-surface,
    stroke: color-border,
  )[
    #text(weight: "semibold", size: font-size-lg)[Lighthouse Scores]
    
    #v(spacing-4)
    
    #align(center)[
      #grid(
        columns: (1fr, 1fr, 1fr, 1fr),
        gutter: spacing-4,
        score-circle(data.performance, "Performance"),
        score-circle(data.accessibility, "Accessibility"),
        score-circle(data.best_practices, "Best Practices"),
        score-circle(data.seo, "SEO"),
      )
    ]
  ]
}
