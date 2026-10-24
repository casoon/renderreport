// Minimal Report Template
// Title block on the first page, no header or footer.

#let report(meta, body) = {
  let title = meta.at("title", default: none)
  let subtitle = meta.at("subtitle", default: none)

  set page(header: none, footer: none)

  if title != none {
    align(center)[
      #text(size: font-size-2xl, weight: "bold", fill: color-text)[#title]

      #if subtitle != none {
        v(spacing-2)
        text(size: font-size-lg, fill: color-text-muted)[#subtitle]
      }
    ]
    v(spacing-6)
  }

  body
}
