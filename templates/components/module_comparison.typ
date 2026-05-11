// Module Comparison Component
// Horizontal score comparison rows

#let module-comparison(data) = {
  if data.title != none {
    block(width: 100%, breakable: false, below: 0pt)[
    text(size: font-size-xl, weight: "bold")[#data.title]
        #v(spacing-4)
        #box(height: 2em, width: 0pt)[]
    ]
    v(-2em)
  }

  theme-card[
    #for (i, module) in data.modules.enumerate() {
      let bar-color = if module.accent_color != none { rgb(module.accent_color) }
        else if module.computed_status == "good" { color-ok }
        else if module.computed_status == "warning" { color-warn }
        else { color-bad }

      grid(
        columns: (28mm, 1fr, 14mm),
        gutter: spacing-3,
        align: (left, left, right),
        text(size: font-size-base)[#module.name],
        theme-progress-bar(module.score, bar-color: bar-color),
        text(size: font-size-base, weight: "bold", fill: bar-color)[#module.score],
      )
      if i < data.modules.len() - 1 {
        v(spacing-3)
      }
    }
  ]
}
