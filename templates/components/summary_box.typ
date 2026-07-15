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
      // The value column is 1fr (not auto): an `auto` column sizes itself to
      // its content's natural width, so a long value (an HTML snippet, a DOM
      // path) would claim unbounded width and squeeze the label column to
      // near-zero — the label's own text then has nowhere to go but overlap
      // the value next to it. 1fr wraps long values inside their own space.
      #grid(
        columns: (auto, auto, 1fr),
        gutter: spacing-2,
        align: (left, left, left),

        status-indicator(item.status),
        text(fill: color-text-muted, size: font-size-base)[#item.label],
        text(weight: "bold", size: font-size-base)[#item.value],
      )
      #v(spacing-2)
    ]
  ]
}
