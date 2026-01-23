// Gauge component template
#let gauge(data) = {
  let label = data.label
  let value = data.value
  let min_val = data.at("min", default: 0.0)
  let max_val = data.at("max", default: 100.0)
  let thresholds = data.at("thresholds", default: ())
  let style = data.at("style", default: "circular")
  
  // Calculate percentage
  let pct = if max_val > min_val {
    ((value - min_val) / (max_val - min_val)) * 100
  } else {
    0
  }
  
  // Determine color based on thresholds
  let gauge_color = rgb("#3b82f6")
  for threshold in thresholds {
    if value >= threshold.value {
      gauge_color = rgb(threshold.color)
    }
  }
  
  block(
    width: 100%,
    breakable: false,
    {
      if style == "circular" {
        // Circular gauge
        align(center, {
          circle(
            radius: 40pt,
            stroke: 3pt + gray.lighten(50%),
            fill: none
          )
          place(
            center + horizon,
            circle(
              radius: 35pt,
              stroke: 5pt + gauge_color,
              fill: none
            )
          )
          place(
            center + horizon,
            text(weight: "bold", size: 16pt, str(calc.round(value, digits: 1)))
          )
        })
      } else if style == "vertical" {
        // Thermometer style
        align(center, {
          rect(
            width: 30pt,
            height: 120pt,
            stroke: 1pt + gray,
            fill: gray.lighten(80%),
            radius: 15pt,
            {
              place(
                bottom,
                rect(
                  width: 100%,
                  height: pct * 1%,
                  fill: gauge_color,
                  stroke: none,
                  radius: (bottom: 15pt)
                )
              )
            }
          )
        })
      } else {
        // Horizontal bar
        rect(
          width: 200pt,
          height: 20pt,
          stroke: 1pt + gray,
          fill: gray.lighten(80%),
          radius: 10pt,
          {
            place(
              left,
              rect(
                width: pct * 1%,
                height: 100%,
                fill: gauge_color,
                stroke: none,
                radius: (left: 10pt)
              )
            )
          }
        )
      }
      
      v(8pt)
      align(center, text(size: 10pt, weight: "medium", label))
      align(center, text(size: 9pt, fill: gray, str(value) + " / " + str(max_val)))
    }
  )
}
