// CardDashboard Component

#let card-dashboard(data) = {
  if data.title != none {
    component-title(text(size: font-size-xl, weight: "bold")[#data.title])
  }

  set text(hyphenate: false)

  let boxes = data.modules.map(module => {
    let status-color = if module.computed_status == "good" {
      color-ok
    } else if module.computed_status == "warning" {
      color-warn
    } else {
      color-bad
    }

    theme-card[
      #text(size: font-size-base, weight: "bold")[#module.name]
      #v(spacing-2)
      #text(size: 22pt, weight: "bold", fill: status-color)[#module.score]
      #v(spacing-3)
      #theme-progress-bar(module.score, bar-color: status-color)
      #v(spacing-3)
      #small-text(module.interpretation)
    ]
  })

  // At most three cards per row so each stays wide enough to read without
  // hyphenating the module name; further cards wrap to the next row.
  let cols = calc.min(data.modules.len(), 3)
  grid(
    columns: (1fr,) * cols,
    column-gutter: spacing-3,
    row-gutter: spacing-3,
    ..boxes,
  )
}
