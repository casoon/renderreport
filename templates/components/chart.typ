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

  // Color palette for multiple series/data points
  let colors = (
    rgb("#3b82f6"),
    rgb("#22c55e"),
    rgb("#f59e0b"),
    rgb("#ef4444"),
    rgb("#8b5cf6"),
    rgb("#06b6d4"),
    rgb("#ec4899"),
    rgb("#f97316"),
  )

  block(
    width: 100%,
    breakable: false,
    {
      // Title
      if title != "" {
        align(center, text(weight: "bold", size: 12pt, title))
        v(8pt)
      }

      if chart_type == "bar" {
        // Bar chart — render actual data points from first series
        let points = if series.len() > 0 { series.at(0).data } else { () }
        let max_val = if points.len() > 0 {
          calc.max(..points.map(p => p.value), 1)
        } else { 100 }

        rect(
          width: 100%,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 16pt,
          {
            if y_label != none {
              align(left, text(size: 8pt, fill: color-text-muted, y_label))
              v(4pt)
            }

            let chart_height = 140
            grid(
              columns: (1fr,) * points.len(),
              column-gutter: 12pt,
              ..points.enumerate().map(((i, p)) => {
                let bar_height = calc.max(p.value / max_val * chart_height, 4)
                align(center, {
                  text(size: 8pt, weight: "bold", fill: color-text, str(calc.round(p.value)))
                  v(4pt)
                  rect(
                    width: 100%,
                    height: bar_height * 1pt,
                    fill: colors.at(calc.rem(i, colors.len())),
                    radius: (top: 3pt),
                  )
                  v(6pt)
                  text(size: 7pt, fill: color-text-muted, p.label)
                })
              })
            )

            if x_label != none {
              v(8pt)
              align(center, text(size: 8pt, fill: color-text-muted, x_label))
            }
          }
        )
      } else if chart_type == "pie" {
        // Pie chart — show data as colored segments legend (actual circle drawing is limited in Typst)
        let points = if series.len() > 0 { series.at(0).data } else { () }
        let total = if points.len() > 0 {
          points.map(p => p.value).sum()
        } else { 1 }

        rect(
          width: 100%,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 16pt,
          {
            if points.len() > 0 {
              // Stacked horizontal bar as pie chart approximation
              let bar_width = 100
              stack(
                dir: ltr,
                ..points.enumerate().map(((i, p)) => {
                  let segment_width = calc.max(p.value / total * bar_width, 2)
                  rect(
                    width: segment_width * 1%,
                    height: 32pt,
                    fill: colors.at(calc.rem(i, colors.len())),
                  )
                })
              )
              v(12pt)
              // Legend with values
              grid(
                columns: (auto,) * calc.min(points.len(), 3),
                column-gutter: 16pt,
                row-gutter: 6pt,
                ..points.enumerate().map(((i, p)) => {
                  let pct = calc.round(p.value / total * 100)
                  [
                    #box(width: 8pt, height: 8pt, fill: colors.at(calc.rem(i, colors.len())), radius: 1pt)
                    #h(4pt)
                    #text(size: 8pt)[#p.label: #str(calc.round(p.value)) (#str(pct)%)]
                  ]
                })
              )
            }
          }
        )
      } else if chart_type == "line" or chart_type == "area" {
        // Line/Area chart — data-driven with dots and connecting lines
        let all_values = series.map(s => s.data.map(p => p.value)).flatten()
        let max_val = if all_values.len() > 0 { calc.max(..all_values, 1) } else { 100 }
        let min_val = if all_values.len() > 0 { calc.min(..all_values, 0) } else { 0 }
        let val_range = if max_val > min_val { max_val - min_val } else { 1 }
        let labels = if series.len() > 0 { series.at(0).data.map(p => p.label) } else { () }
        let chart_h = 140
        let chart_w_pt = 100 // percentage

        rect(
          width: 100%,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 16pt,
          {
            if y_label != none {
              align(left, text(size: 8pt, fill: color-text-muted, y_label))
              v(4pt)
            }

            // Chart area with placed elements
            let n_points = if labels.len() > 0 { labels.len() } else { 1 }

            for (si, s) in series.enumerate() {
              let color = colors.at(calc.rem(si, colors.len()))
              let points_data = s.data

              if points_data.len() > 1 {
                // Render columns with bars for area, dots for line
                grid(
                  columns: (1fr,) * points_data.len(),
                  column-gutter: 0pt,
                  ..points_data.enumerate().map(((i, p)) => {
                    let bar_height = calc.max((p.value - min_val) / val_range * chart_h, 2)
                    align(center, {
                      // Value label
                      text(size: 7pt, weight: "bold", fill: color-text, str(calc.round(p.value)))
                      v(2pt)
                      // Spacer to push down (inverse height)
                      v((chart_h - bar_height) * 1pt)
                      if chart_type == "area" {
                        // Filled area bar
                        rect(
                          width: 100%,
                          height: bar_height * 1pt,
                          fill: color.lighten(60%),
                          stroke: (top: 2.5pt + color),
                        )
                      } else {
                        // Line chart: thin bar with dot on top
                        rect(
                          width: 100%,
                          height: bar_height * 1pt,
                          fill: none,
                          stroke: none,
                          {
                            place(center + top, circle(radius: 4pt, fill: color, stroke: 1pt + color-background))
                            place(center + top, line(
                              length: 100%,
                              stroke: 2pt + color,
                            ))
                          }
                        )
                      }
                    })
                  })
                )

                // X-axis labels
                grid(
                  columns: (1fr,) * points_data.len(),
                  column-gutter: 0pt,
                  ..points_data.map(p => {
                    align(center, text(size: 7pt, fill: color-text-muted, p.label))
                  })
                )
              }
            }

            if x_label != none {
              v(8pt)
              align(center, text(size: 8pt, fill: color-text-muted, x_label))
            }
          }
        )
      } else if chart_type == "scatter" {
        // Scatter chart — dots positioned by value
        let all_values = series.map(s => s.data.map(p => p.value)).flatten()
        let max_val = if all_values.len() > 0 { calc.max(..all_values, 1) } else { 100 }
        let min_val = if all_values.len() > 0 { calc.min(..all_values, 0) } else { 0 }
        let val_range = if max_val > min_val { max_val - min_val } else { 1 }
        let chart_h = 140

        rect(
          width: 100%,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 16pt,
          {
            if y_label != none {
              align(left, text(size: 8pt, fill: color-text-muted, y_label))
              v(4pt)
            }

            for (si, s) in series.enumerate() {
              let color = colors.at(calc.rem(si, colors.len()))
              let points_data = s.data

              if points_data.len() > 0 {
                grid(
                  columns: (1fr,) * points_data.len(),
                  column-gutter: 4pt,
                  ..points_data.enumerate().map(((i, p)) => {
                    let dot_y = calc.max((p.value - min_val) / val_range * chart_h, 4)
                    align(center, {
                      text(size: 6pt, fill: color-text-muted, str(calc.round(p.value)))
                      v(2pt)
                      v((chart_h - dot_y) * 1pt)
                      circle(radius: 5pt, fill: color, stroke: 1.5pt + color-background)
                      v(dot_y * 1pt - 10pt)
                    })
                  })
                )

                // X-axis labels
                grid(
                  columns: (1fr,) * points_data.len(),
                  column-gutter: 4pt,
                  ..points_data.map(p => {
                    align(center, text(size: 6pt, fill: color-text-muted, p.label))
                  })
                )
              }
            }

            if x_label != none {
              v(8pt)
              align(center, text(size: 8pt, fill: color-text-muted, x_label))
            }
          }
        )
      } else if chart_type == "radar" {
        // Radar chart — horizontal bar comparison per axis
        let labels = if series.len() > 0 { series.at(0).data.map(p => p.label) } else { () }
        let all_values = series.map(s => s.data.map(p => p.value)).flatten()
        let max_val = if all_values.len() > 0 { calc.max(..all_values, 1) } else { 100 }

        rect(
          width: 100%,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 16pt,
          {
            for (li, label) in labels.enumerate() {
              // Axis label + value summary on same line
              for (si, s) in series.enumerate() {
                let color = colors.at(calc.rem(si, colors.len()))
                let val = if li < s.data.len() { s.data.at(li).value } else { 0 }
                let pct = calc.min(val / max_val * 100, 100)
                let val-color = if val >= 85 { rgb("#22c55e") } else if val >= 70 { rgb("#3b82f6") } else if val >= 50 { rgb("#f59e0b") } else { rgb("#ef4444") }

                grid(
                  columns: (120pt, 1fr, 36pt),
                  gutter: 8pt,
                  align: (left + horizon, left + horizon, right + horizon),
                  text(size: 9pt, weight: "semibold", fill: color-text, label),
                  box(
                    width: 100%,
                    height: 18pt,
                    radius: 4pt,
                    fill: color-surface-alt,
                    stroke: 0.5pt + color-border,
                  )[
                    #place(left + horizon,
                      box(width: pct * 1%, height: 100%, radius: 4pt, fill: val-color.lighten(20%))
                    )
                    #if pct >= 15 [
                      #place(left + horizon, dx: 6pt,
                        text(size: 7pt, weight: "bold", fill: val-color.darken(30%))[#str(calc.round(val))]
                      )
                    ]
                  ],
                  text(size: 9pt, weight: "bold", fill: val-color)[#str(calc.round(val))],
                )
                v(5pt)
              }
            }
          }
        )
      } else {
        rect(
          width: 100%,
          height: eval(height),
          stroke: 0.5pt + gray,
          radius: 4pt,
          fill: color-surface,
          inset: 12pt,
          align(center + horizon, text(size: 9pt, fill: gray, [Chart: ] + chart_type))
        )
      }

      // Legend for multi-series
      if show_legend and series.len() > 1 {
        v(8pt)
        grid(
          columns: (auto,) * calc.min(series.len(), 4),
          column-gutter: 12pt,
          ..series.enumerate().map(((i, s)) => [
            #box(width: 8pt, height: 8pt, fill: colors.at(calc.rem(i, colors.len())), radius: 1pt)
            #h(4pt)
            #text(size: 9pt, s.name)
          ])
        )
      }
    }
  )
}
