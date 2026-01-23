// Progress Bar Component
// Inspired by JasperReports Chart Components

#let progress-bar(data) = {
  let bar-color = if data.color != none {
    rgb(data.color)
  } else {
    color-primary
  }
  
  let percentage = data.percentage
  
  box(width: 100%)[
    // Label and percentage
    #grid(
      columns: (1fr, auto),
      text(size: font-size-sm)[#data.label],
      if data.show_percentage [
        #text(size: font-size-sm, weight: "semibold")[#calc.round(percentage, digits: 1)%]
      ]
    )
    
    #v(spacing-2)
    
    // Progress bar
    #box(
      width: 100%,
      height: 12pt,
      radius: 6pt,
      fill: color-surface,
      stroke: (paint: color-border, thickness: 0.5pt),
    )[
      #place(
        left + horizon,
        box(
          width: percentage * 1%,
          height: 100%,
          radius: 6pt,
          fill: bar-color,
        )
      )
    ]
    
    #v(spacing-3)
  ]
}
