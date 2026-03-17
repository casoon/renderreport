// Crosstab/Pivot Table component template
#let crosstab(data) = {
  let title = data.at("title", default: none)
  let row_dim = data.row_dimension
  let col_dim = data.column_dimension
  let measure = data.measure
  let aggregation = data.at("aggregation", default: "sum")
  let show_row_totals = data.at("show_row_totals", default: true)
  let show_col_totals = data.at("show_column_totals", default: true)
  let show_grand = data.at("show_grand_total", default: true)
  
  block(
    width: 100%,
    breakable: false,
    {
      if title != none {
        text(weight: "bold", size: 11pt, title)
        v(6pt)
      }
      
      // Simplified crosstab visualization
      table(
        columns: 4,
        stroke: 0.5pt + gray,
        fill: (x, y) => {
          if y == 0 or x == 0 { gray.lighten(70%) }
          else { white }
        },
        
        // Header row
        table.header(
          [],
          [#text(weight: "bold", size: 9pt, "Col 1")],
          [#text(weight: "bold", size: 9pt, "Col 2")],
          if show_row_totals [#text(weight: "bold", size: 9pt, "Total")] else []
        ),
        
        // Data rows
        [#text(weight: "bold", size: 9pt, "Row 1")], [100], [150], if show_row_totals [250] else [],
        [#text(weight: "bold", size: 9pt, "Row 2")], [200], [180], if show_row_totals [380] else [],
        
        // Totals row
        ..if show_col_totals {
          ([#text(weight: "bold", size: 9pt, "Total")], [300], [330],
           if show_grand [#text(weight: "bold", "630")] else [])
        } else { () }
      )
      
      v(6pt)
      text(size: 8pt, fill: gray, [Aggregation: #aggregation of #measure by #row_dim × #col_dim])
    }
  )
}
