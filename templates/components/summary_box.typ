// Summary Box Component
// Executive summary widget with key-value pairs

#let status-indicator(status) = {
  if status == none { none }
  else {
    let color = if status == "good" { color-ok }
      else if status == "warning" { color-warn }
      else { color-bad }
    box(
      width: 8pt,
      height: 8pt,
      radius: 50%,
      fill: color,
    )
  }
}

#let summary-box(data) = {
  box(
    width: 100%,
    inset: spacing-4,
    radius: 4pt,
    fill: color-surface,
    stroke: (paint: color-border, thickness: 0.5pt),
  )[
    #text(weight: "bold", size: font-size-lg, fill: color-text)[#data.title]
    
    #v(spacing-4)
    
    #for item in data.items [
      #grid(
        columns: (auto, 1fr, auto),
        gutter: spacing-2,
        align: (left, left, right),
        
        status-indicator(item.status),
        text(fill: color-text-muted, size: font-size-sm)[#item.label],
        text(weight: "semibold", size: font-size-sm)[#item.value],
      )
      #v(spacing-2)
    ]
  ]
}
