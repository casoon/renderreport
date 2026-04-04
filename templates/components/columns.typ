// Columns Layout Component
// Asymmetric two-column layout with flexible width ratio

#let columns(data) = {
  let gap = if data.gap != none { data.gap } else { spacing-4 }
  let right_width = 1.0 - data.left_width

  block(width: 100%)[
    #grid(
      columns: (data.left_width * 100% - gap * 0.5, right_width * 100% - gap * 0.5),
      column-gutter: gap,

      block(width: 100%)[#data.left],
      block(width: 100%)[#data.right],
    )
  ]
}
