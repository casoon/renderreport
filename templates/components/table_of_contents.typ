#let table-of-contents(data) = {
  // Not listed in its own outline; still a PDF bookmark.
  heading(level: 1, outlined: false, bookmarked: true)[#data.title]
  v(spacing-3)
  {
    let font-size = if "font_size" in data and data.font_size != none { eval(data.font_size) } else { font-size-sm }
    set text(size: font-size)
    set par(leading: 1.35em)
    show outline.entry: set text(size: font-size)
    outline(
      title: none,
      indent: 1.2em,
      depth: data.depth,
    )
  }
  pagebreak()
}
