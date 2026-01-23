// Chart component template
#let chart(data) = {
  let title = data.title
  let chart_type = data.chart_type
  let series = data.series
  let x_label = data.at("x_label", default: none)
  let y_label = data.at("y_label", default: none)
  let show_legend = data.at("show_legend", default: true)
  let width = data.at("width", default: "100%")
  let height = data.at("height", default: "200pt")
  
  block(
    width: 100%,
    breakable: false,
    {
      // Title
      if title != "" {
        align(center, text(weight: "bold", size: 12pt, title))
        v(8pt)
      }
      
      // Chart area placeholder (simplified visualization)
      // In production, this would integrate with a charting library
      rect(
        width: 100%,
        height: eval(height),
        stroke: 0.5pt + gray,
        radius: 4pt,
        fill: rgb("#f8fafc"),
        inset: 12pt,
        {
          // Y-axis label
          if y_label != none {
            rotate(-90deg, text(size: 9pt, fill: gray, y_label))
            h(8pt)
          }
          
          // Chart content area
          align(center + horizon, {
            if chart_type == "pie" {
              // Pie chart representation
              circle(radius: 60pt, stroke: 1pt + gray, fill: rgb("#3b82f6"))
              v(6pt)
              text(size: 9pt, fill: gray, [Pie Chart])
            } else if chart_type == "bar" {
              // Bar chart representation
              grid(
                columns: series.len(),
                column-gutter: 8pt,
                ..series.map(s => rect(
                  width: 30pt,
                  height: 80pt,
                  fill: rgb("#3b82f6"),
                  stroke: none
                ))
              )
              v(6pt)
              text(size: 9pt, fill: gray, [Bar Chart])
            } else if chart_type == "line" {
              // Line chart representation
              line(length: 150pt, stroke: 2pt + rgb("#3b82f6"))
              v(6pt)
              text(size: 9pt, fill: gray, [Line Chart])
            } else {
              text(size: 9pt, fill: gray, [Chart: ] + chart_type)
            }
          })
          
          // X-axis label
          if x_label != none {
            v(8pt)
            align(center, text(size: 9pt, fill: gray, x_label))
          }
        }
      )
      
      // Legend
      if show_legend and series.len() > 0 {
        v(8pt)
        grid(
          columns: (auto,) * calc.min(series.len(), 4),
          column-gutter: 12pt,
          ..series.map(s => [
            #box(width: 8pt, height: 8pt, fill: rgb("#3b82f6"), radius: 1pt)
            #h(4pt)
            #text(size: 9pt, s.name)
          ])
        )
      }
    }
  )
}
