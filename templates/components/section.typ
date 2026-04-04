// Section Component
// Document section with heading
// Uses block(breakable: false) to prevent orphaned headings at page bottom

#let section(data) = {
  block(breakable: false, below: 0pt)[
    #heading(level: data.level)[#data.title]
    #v(spacing-4)
    // Invisible anchor — forces Typst to keep heading with following content
    #box(height: 0pt)
  ]
}
