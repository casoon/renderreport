// Audit Table Component
// Displays tabular data for audit results

#let audit-table(data) = {
  let col-count = data.columns.len()
  
  // Build column widths
  let col-widths = data.columns.map(col => {
    if col.width != none { 
      eval(col.width) 
    } else { 
      1fr 
    }
  })
  
  block(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-lg)[#data.title]
      #v(spacing-3)
    ]
    
    #set text(size: font-size-sm)
    
    #table(
      columns: col-widths,
      inset: spacing-3,
      stroke: (paint: table-border, thickness: table-border-width),
      fill: (x, y) => {
        if y == 0 { table-header-bg }
        else if data.striped and calc.odd(y) { table-row-alt-bg }
        else { none }
      },
      
      // Header row
      ..data.columns.map(col => {
        text(weight: "semibold", fill: color-text)[#col.header]
      }),
      
      // Data rows
      ..data.rows.flatten()
    )
  ]
}
