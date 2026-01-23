// Grid Component
// Inspired by Pentaho Row/Block Layout

#let grid-component(data) = {
  box(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-lg)[#data.title]
      #v(spacing-4)
    ]
    
    // Build column template based on items and colspans
    #let col-widths = (1fr,) * data.columns
    
    #grid(
      columns: col-widths,
      column-gutter: eval(data.column_gap),
      row-gutter: eval(data.row_gap),
      
      ..data.items.map(item => {
        // Note: colspan handling would require more complex logic
        // For now, each item takes one column
        box(
          width: 100%,
          inset: spacing-3,
          fill: color-surface,
          radius: 4pt,
          stroke: (paint: color-border, thickness: 0.5pt),
        )[
          // Render nested component
          // This is a simplified version
          #raw(str(item.content))
        ]
      })
    )
  ]
}
