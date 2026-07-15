// Chart component template

// Theme-aware color palette — falls back gracefully on missing tokens
#let chart-colors = (
  color-primary,
  color-ok,
  color-warn,
  color-bad,
  color-info,
  color-secondary,
  color-primary.lighten(40%),
  color-ok.lighten(40%),
)

#let chart-legend(series) = {
  v(8pt)
  grid(
    columns: (auto,) * calc.min(series.len(), 4),
    column-gutter: 12pt,
    ..series.enumerate().map(((i, s)) => [
      #box(width: 8pt, height: 8pt, fill: chart-colors.at(calc.rem(i, chart-colors.len())), radius: 1pt)
      #h(4pt)
      #text(size: 9pt, s.name)
    ])
  )
}

// A single series encodes magnitude, not identity — every bar takes the same
// hue (no legend box; the chart title already names what's plotted). Reserve
// `chart-colors` cycling for charts that render more than one series.
#let chart-bar-vertical(data) = {
  let points = if data.series.len() > 0 { data.series.at(0).data } else { () }
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
      let y_label = data.at("y_label", default: none)
      let x_label = data.at("x_label", default: none)

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
              fill: color-primary,
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
}

// Rows instead of columns: the label owns the full row width and wraps
// normally, so it stays readable regardless of how many categories there are
// or how long their text is — unlike fixed-width columns, which get narrower
// (and the label more cramped) as the category count grows.
#let chart-bar-horizontal(data) = {
  let points = if data.series.len() > 0 { data.series.at(0).data } else { () }
  let max_val = if points.len() > 0 {
    calc.max(..points.map(p => p.value), 1)
  } else { 100 }
  let bar-height = 10pt

  rect(
    width: 100%,
    stroke: 0.5pt + color-border,
    radius: 4pt,
    fill: color-surface,
    inset: 16pt,
    {
      let y_label = data.at("y_label", default: none)
      if y_label != none {
        align(left, text(size: 8pt, fill: color-text-muted, y_label))
        v(4pt)
      }

      for (i, p) in points.enumerate() {
        if i > 0 { v(spacing-3) }
        let bar_frac = calc.max(p.value / max_val, 0.03)
        text(size: 8.5pt, fill: color-text, p.label)
        v(spacing-1)
        grid(
          columns: (1fr, auto),
          column-gutter: spacing-2,
          align: (left + horizon, right + horizon),
          rect(
            width: bar_frac * 100%,
            height: bar-height,
            fill: color-primary,
            radius: (right: 3pt),
          ),
          text(size: 8pt, weight: "bold", fill: color-text, str(calc.round(p.value))),
        )
      }
    }
  )
}

#let chart-bar(data) = {
  if data.at("horizontal", default: false) {
    chart-bar-horizontal(data)
  } else {
    chart-bar-vertical(data)
  }
}

#let chart-pie(data) = {
  let points = if data.series.len() > 0 { data.series.at(0).data } else { () }
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
        let bar_width = 100
        stack(
          dir: ltr,
          ..points.enumerate().map(((i, p)) => {
            let segment_width = calc.max(p.value / total * bar_width, 2)
            rect(
              width: segment_width * 1%,
              height: 32pt,
              fill: chart-colors.at(calc.rem(i, chart-colors.len())),
            )
          })
        )
        v(12pt)
        grid(
          columns: (auto,) * calc.min(points.len(), 3),
          column-gutter: 16pt,
          row-gutter: 6pt,
          ..points.enumerate().map(((i, p)) => {
            let pct = calc.round(p.value / total * 100)
            [
              #box(width: 8pt, height: 8pt, fill: chart-colors.at(calc.rem(i, chart-colors.len())), radius: 1pt)
              #h(4pt)
              #text(size: 8pt)[#p.label: #str(calc.round(p.value)) (#str(pct)%)]
            ]
          })
        )
      }
    }
  )
}

#let chart-line-area(data, is-area) = {
  let all_values = data.series.map(s => s.data.map(p => p.value)).flatten()
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
      let y_label = data.at("y_label", default: none)
      let x_label = data.at("x_label", default: none)

      if y_label != none {
        align(left, text(size: 8pt, fill: color-text-muted, y_label))
        v(4pt)
      }

      for (si, s) in data.series.enumerate() {
        let color = chart-colors.at(calc.rem(si, chart-colors.len()))
        let points_data = s.data

        if points_data.len() > 1 {
          grid(
            columns: (1fr,) * points_data.len(),
            column-gutter: 0pt,
            ..points_data.enumerate().map(((i, p)) => {
              let bar_height = calc.max((p.value - min_val) / val_range * chart_h, 2)
              align(center, {
                text(size: 7pt, weight: "bold", fill: color-text, str(calc.round(p.value)))
                v(2pt)
                v((chart_h - bar_height) * 1pt)
                if is-area {
                  rect(
                    width: 100%,
                    height: bar_height * 1pt,
                    fill: color.lighten(60%),
                    stroke: (top: 2.5pt + color),
                  )
                } else {
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
}

#let chart-scatter(data) = {
  let all_values = data.series.map(s => s.data.map(p => p.value)).flatten()
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
      let y_label = data.at("y_label", default: none)
      let x_label = data.at("x_label", default: none)

      if y_label != none {
        align(left, text(size: 8pt, fill: color-text-muted, y_label))
        v(4pt)
      }

      for (si, s) in data.series.enumerate() {
        let color = chart-colors.at(calc.rem(si, chart-colors.len()))
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
}

// Real spider / radar chart for the first series (values assumed 0..100).
#let chart-radar(data) = {
  let points = if data.series.len() > 0 { data.series.at(0).data } else { () }
  let n = points.len()
  if n >= 3 {
    let size = 240pt
    let cx = size / 2
    let cy = size / 2
    let r-max = size / 2 - 36pt
    let coord(i, r) = {
      let a = -90deg + i * (360deg / n)
      (cx + r * calc.cos(a), cy + r * calc.sin(a))
    }
    align(center, box(width: size, height: size, {
      // Concentric grid rings at 25/50/75/100 %
      for g in (0.25, 0.5, 0.75, 1.0) {
        let rp = range(0, n).map(i => coord(i, r-max * g))
        place(top + left, polygon(stroke: (paint: color-border, thickness: 0.5pt), ..rp))
      }
      // Spokes
      for i in range(0, n) {
        place(top + left, line(
          start: (cx, cy),
          end: coord(i, r-max),
          stroke: (paint: color-border, thickness: 0.5pt),
        ))
      }
      // Value polygon
      let dp = range(0, n).map(i => coord(i, r-max * calc.min(points.at(i).value, 100) / 100))
      place(top + left, polygon(
        fill: color-primary.transparentize(82%),
        stroke: (paint: color-primary, thickness: 1.4pt),
        ..dp,
      ))
      // Value dots
      for i in range(0, n) {
        let (dx, dy) = coord(i, r-max * calc.min(points.at(i).value, 100) / 100)
        place(top + left, dx: dx - 2pt, dy: dy - 2pt, circle(radius: 2pt, fill: color-primary, stroke: none))
      }
      // Axis labels + value
      for i in range(0, n) {
        let (lx, ly) = coord(i, r-max + 17pt)
        place(top + left, dx: lx - 38pt, dy: ly - 10pt, box(width: 76pt, align(center, {
          text(size: 7pt, weight: "medium", fill: color-text-muted)[#points.at(i).label]
          linebreak()
          text(size: 8.5pt, weight: "bold", fill: color-text)[#str(calc.round(points.at(i).value))]
        })))
      }
    }))
  }
}

#let chart(data) = {
  let title = data.title
  let chart_type = data.chart_type
  let series = data.series
  let show_legend = data.at("show_legend", default: true)
  let height = data.at("height", default: "200pt")

  block(
    width: 100%,
    breakable: false,
    {
      if title != "" {
        align(center, text(weight: "bold", size: 12pt, title))
        v(8pt)
      }

      if chart_type == "bar"          { chart-bar(data) }
      else if chart_type == "pie"     { chart-pie(data) }
      else if chart_type == "line"    { chart-line-area(data, false) }
      else if chart_type == "area"    { chart-line-area(data, true) }
      else if chart_type == "scatter" { chart-scatter(data) }
      else if chart_type == "radar"   { chart-radar(data) }
      else {
        rect(
          width: 100%,
          height: eval(height),
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 12pt,
          align(center + horizon, text(size: 9pt, fill: color-text-muted, [Chart: ] + chart_type))
        )
      }

      if show_legend and series.len() > 1 {
        chart-legend(series)
      }
    }
  )
}
