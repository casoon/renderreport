// Core Web Vitals Component
// Displays LCP, FID, CLS metrics

#let vital-status(value, good-threshold, poor-threshold) = {
  if value <= good-threshold { "good" }
  else if value <= poor-threshold { "needs-improvement" }
  else { "poor" }
}

#let vital-color(status) = {
  if status == "good" { color-ok }
  else if status == "needs-improvement" { color-warn }
  else { color-bad }
}

#let vital-card(label, value, unit, status) = {
  let col = vital-color(status)
  
  box(
    width: 100%,
    inset: spacing-4,
    radius: 4pt,
    fill: color-surface,
    stroke: (left: (paint: col, thickness: 4pt)),
  )[
    #grid(
      columns: (1fr, auto),
      align: (left, right),
      [
        #text(size: font-size-sm, fill: color-text-muted)[#label]
        #v(spacing-1)
        #text(size: font-size-xl, weight: "bold", fill: color-text)[#value#text(size: font-size-sm)[#unit]]
      ],
      [
        #box(
          inset: (x: spacing-2, y: spacing-1),
          radius: 2pt,
          fill: col.lighten(80%),
        )[
          #text(size: font-size-sm, weight: "semibold", fill: col)[
            #if status == "good" [Good]
            #if status == "needs-improvement" [Needs Work]
            #if status == "poor" [Poor]
          ]
        ]
      ]
    )
  ]
}

#let core-web-vitals(data) = {
  let lcp-status = vital-status(data.lcp, 2.5, 4.0)
  let fid-status = vital-status(data.fid, 100, 300)
  let cls-status = vital-status(data.cls, 0.1, 0.25)
  
  box(width: 100%)[
    #text(weight: "semibold", size: font-size-lg)[Core Web Vitals]
    
    #v(spacing-4)
    
    #grid(
      columns: (1fr, 1fr, 1fr),
      gutter: spacing-4,
      vital-card("Largest Contentful Paint (LCP)", data.lcp, "s", lcp-status),
      vital-card("First Input Delay (FID)", data.fid, "ms", fid-status),
      vital-card("Cumulative Layout Shift (CLS)", data.cls, "", cls-status),
    )
  ]
}
