// Pivot Table component template
#let pivot-table(data) = {
  let title = data.at("title", default: none)
  let row_headers = data.row_headers
  let col_headers = data.column_headers
  let values = data.values
  let show_borders = data.at("show_borders", default: true)
  
  let stroke_val = if show_borders { 0.5pt + gray } else { none }
  let num_cols = col_headers.len() + 1
  
  block(
    width: 100%,
    breakable: true,
    {
      if title != none {
        text(weight: "bold", size: 11pt, title)
        v(6pt)
      }
      
      table(
        columns: num_cols,
        stroke: stroke_val,
        fill: (x, y) => {
          if y == 0 or x == 0 { gray.lighten(70%) }
          else { white }
        },
        
        // Build header cells
        text(weight: "bold", size: 9pt, ""),
        ..col_headers.map(h => text(weight: "bold", size: 9pt, h)),
        
        // Build data rows - must be flat array
        ..{
          let cells = ()
          for i in range(row_headers.len()) {
            // Add row header
            cells.push(text(weight: "bold", size: 9pt, row_headers.at(i)))
            // Add row values
            for v in values.at(i) {
              cells.push(text(size: 9pt, v))
            }
          }
          cells
        }
      )
    }
  )
}
