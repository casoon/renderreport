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
          stroke: 0.5pt + rgb("#e2e8f0"),
          radius: 4pt,
          fill: rgb("#f8fafc"),
          inset: 16pt,
          {
            // Y-axis label
            if y_label != none {
              align(left, text(size: 8pt, fill: rgb("#94a3b8"), y_label))
              v(4pt)
            }

            // Scale lines
            let chart_height = 140
            grid(
              columns: (1fr,) * points.len(),
              column-gutter: 12pt,
              ..points.enumerate().map(((i, p)) => {
                let bar_height = calc.max(p.value / max_val * chart_height, 4)
                align(center, {
                  // Value on top
                  text(size: 8pt, weight: "bold", fill: rgb("#334155"), str(calc.round(p.value)))
                  v(4pt)
                  // Bar
                  rect(
                    width: 100%,
                    height: bar_height * 1pt,
                    fill: colors.at(calc.rem(i, colors.len())),
                    radius: (top: 3pt),
                  )
                  v(6pt)
                  // Label
                  text(size: 7pt, fill: rgb("#64748b"), p.label)
                })
              })
            )

            // X-axis label
            if x_label != none {
              v(8pt)
              align(center, text(size: 8pt, fill: rgb("#94a3b8"), x_label))
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
          stroke: 0.5pt + rgb("#e2e8f0"),
          radius: 4pt,
          fill: rgb("#f8fafc"),
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
      } else if chart_type == "line" {
        // Line chart representation
        let points = if series.len() > 0 { series.at(0).data } else { () }

        rect(
          width: 100%,
          height: eval(height),
          stroke: 0.5pt + rgb("#e2e8f0"),
          radius: 4pt,
          fill: rgb("#f8fafc"),
          inset: 12pt,
          {
            if y_label != none {
              rotate(-90deg, text(size: 8pt, fill: rgb("#94a3b8"), y_label))
              h(8pt)
            }
            align(center + horizon, {
              line(length: 150pt, stroke: 2pt + rgb("#3b82f6"))
              v(6pt)
              text(size: 9pt, fill: rgb("#94a3b8"), [Line Chart])
            })
            if x_label != none {
              v(8pt)
              align(center, text(size: 8pt, fill: rgb("#94a3b8"), x_label))
            }
          }
        )
      } else {
        rect(
          width: 100%,
          height: eval(height),
          stroke: 0.5pt + gray,
          radius: 4pt,
          fill: rgb("#f8fafc"),
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
