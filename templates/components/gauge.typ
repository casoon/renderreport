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
        // Circular gauge — a progress arc that fills proportionally to the
        // value over a light full-circle track, with the value centred.
        align(center, box(width: 84pt, height: 84pt, {
          let cx = 42pt
          let cy = 42pt
          let r = 33pt
          // Full track ring (light grey)
          place(center + horizon, circle(
            radius: r,
            stroke: 5pt + color-border,
            fill: none
          ))
          // Progress arc: polyline from 12 o'clock, clockwise, pct of 360°.
          let frac = calc.min(1.0, calc.max(0.0, pct / 100))
          if frac > 0 {
            let n = calc.max(2, int(calc.round(frac * 60)))
            let pts = range(0, n + 1).map(i => {
              let ang = -90deg + (i / n) * frac * 360deg
              (cx + r * calc.cos(ang), cy + r * calc.sin(ang))
            })
            place(top + left, path(
              stroke: (paint: gauge_color, thickness: 5pt, cap: "round"),
              ..pts
            ))
          }
          place(center + horizon,
            text(weight: "bold", size: 18pt, fill: color-text, str(calc.round(value)))
          )
        }))
      } else if style == "vertical" {
        // Thermometer style
        align(center, {
          rect(
            width: 30pt,
            height: 120pt,
            stroke: 1pt + color-border,
            fill: color-surface,
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
          stroke: 1pt + color-border,
          fill: color-surface,
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

      v(7pt)
      // Fixed-height label area (room for up to two lines) so every gauge cell
      // is the same height — wrapping labels no longer push the value into the
      // row below and all values stay aligned.
      align(center, box(width: 100%, height: 24pt,
        align(center + top, text(size: 9pt, weight: "medium", fill: color-text, label))
      ))
    }
  )
}
