#let flow-group(data) = context {
  let gap = if data.spacing != none { eval(data.spacing) } else { spacing-4 }
  let content = stack(
    spacing: gap,
    ..data.items.map(item => [#_flow-dispatch(item)]),
  )

  if data.keep_together_if_under != none {
    let threshold = eval(data.keep_together_if_under)
    let measured = measure(content)
    if measured.height <= threshold {
      block(width: 100%, breakable: false)[#content]
    } else {
      content
    }
  } else {
    content
  }
}
