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
  theme-card[
    #text(weight: "bold", size: font-size-xl, fill: color-text)[#data.title]

    #v(spacing-4)

    #for item in data.items [
      #grid(
        columns: (auto, 1fr, auto),
        gutter: spacing-2,
        align: (left, left, right),

        status-indicator(item.status),
        text(fill: color-text-muted, size: font-size-base)[#item.label],
        text(weight: "bold", size: font-size-base)[#item.value],
      )
      #v(spacing-2)
    ]
  ]
}
