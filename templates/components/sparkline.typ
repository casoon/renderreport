// Sparkline component template
#let sparkline(data) = {
  let sparkline_type = data.sparkline_type
  let values = data.data
  let width = data.at("width", default: "80pt")
  let height = data.at("height", default: "20pt")
  let color = data.at("color", default: none)
  let stroke_color = if color != none { rgb(color) } else { rgb("#3b82f6") }
  
  box(
    width: eval(width),
    height: eval(height),
    baseline: 25%,
    {
      if sparkline_type == "bar" {
        // Mini bar chart
        let max_val = calc.max(..values)
        let bar_width = eval(width) / values.len()
        
        grid(
          columns: values.len(),
          column-gutter: 0pt,
          ..values.map(v => {
            let bar_height = if max_val > 0 { (v / max_val) * eval(height) } else { 0pt }
            align(bottom, rect(
              width: bar_width - 1pt,
              height: bar_height,
              fill: stroke_color,
              stroke: none
            ))
          })
        )
      } else {
        // Line sparkline (simplified)
        rect(
          width: 100%,
          height: 100%,
          stroke: 1pt + stroke_color,
          fill: none,
          radius: 2pt
        )
      }
    }
  )
}
