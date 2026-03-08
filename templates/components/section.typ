// Section Component
// Document section with heading

#let section(data) = {
  [
    #heading(level: data.level)[#data.title]

    #v(spacing-4)
  ]
}
