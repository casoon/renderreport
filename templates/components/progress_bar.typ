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
      height: 16pt,
      radius: 8pt,
      fill: color-surface,
      stroke: (paint: color-border, thickness: 0.5pt),
    )[
      #place(
        left + horizon,
        box(
          width: percentage * 1%,
          height: 100%,
          radius: 8pt,
          fill: bar-color,
        )
      )
      #if percentage >= 20 [
        #place(left + horizon, dx: 8pt,
          text(size: 7pt, weight: "bold", fill: white)[#calc.round(percentage, digits: 0)%]
        )
      ]
    ]
    
    #v(spacing-3)
  ]
}
