// Section Component
// Per-level visual accent + orphan protection via phantom height.
//
// Level 2 — major section: primary left bar + soft background
// Level 3 — sub-section: thin primary left bar, no background
// Level 4+ — plain bold, muted color

#let section(data) = {
  let lv = data.level

  let heading-block = if lv == 2 {
    block(
      width: 100%,
      fill: color-surface-soft,
      stroke: (left: 3pt + color-primary),
      inset: (left: 10pt, right: 8pt, y: 6pt),
      radius: (right: 4pt),
    )[#heading(level: lv)[#data.title]]
  } else if lv == 3 {
    block(
      width: 100%,
      stroke: (left: 2pt + color-primary),
      inset: (left: 8pt, y: 3pt),
    )[#heading(level: lv)[#data.title]]
  } else {
    block(width: 100%)[#heading(level: lv)[#data.title]]
  }

  // Orphan protection: ghost height forces Typst to reserve ~2 lines of space
  // after the heading on the same page. Cancelled by v(-2em) below so the gap
  // is not visible in the rendered output.
  block(width: 100%, breakable: false, below: 0pt)[
    #heading-block
    #v(spacing-4)
    #box(height: 2em, width: 0pt)[]
  ]
  #v(-2em)
}
