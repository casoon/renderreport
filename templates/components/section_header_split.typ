// SectionHeaderSplit Component
// Full-width: optional eyebrow + heading on top, body paragraph below

#let section-header-split(data) = {
  block(width: 100%, breakable: false)[
    #if data.eyebrow != none [
      #text(
        size: font-size-xs,
        weight: "bold",
        fill: color-secondary,
        tracking: 0.12em,
        upper(data.eyebrow),
      )
      #v(spacing-2)
    ]

    // The eyebrow's v(spacing-2) above is meant to be the entire gap to the
    // heading — without this, Typst's own default heading "above" spacing
    // stacks on top of it, so the two read as detached instead of a paired
    // kicker + title.
    #show heading: set block(above: if data.eyebrow != none { 0pt } else { auto })
    #heading(level: data.level, outlined: data.outlined)[#data.title]

    #v(spacing-2)

    #par(justify: true)[
      #text(size: font-size-base, fill: color-text)[#data.body]
    ]

    #if data.divider_below [
      #v(spacing-3)
      #line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))
      #v(spacing-2)
    ] else [
      #v(spacing-3)
    ]
  ]
}
