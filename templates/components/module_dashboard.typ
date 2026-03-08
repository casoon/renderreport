// Module Dashboard Component

#let module-dashboard(data) = {
  if data.title != none {
    text(size: font-size-xl, weight: "bold")[#data.title]
    v(spacing-3)
  }

  let boxes = data.modules.map(module => {
    let status-color = if module.computed_status == "good" {
      color-ok
    } else if module.computed_status == "warning" {
      color-warn
    } else {
      color-bad
    }

    theme-card[
      #text(size: font-size-lg, weight: "bold")[#module.name]
      #v(spacing-2)
      #text(size: 22pt, weight: "bold", fill: status-color)[#module.score#text(size: font-size-sm, weight: "regular", fill: color-text-muted)[\/100]]
      #v(spacing-3)
      #theme-progress-bar(module.score, bar-color: status-color)
      #v(spacing-3)
      #small-text(module.interpretation)
    ]
  })

  grid(
    columns: (1fr,) * data.modules.len(),
    column-gutter: spacing-3,
    ..boxes,
  )
}
