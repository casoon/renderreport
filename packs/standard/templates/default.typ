// Default Report Template
// Title page, then content pages with a running header and a page-count footer.
//
// `meta` holds `title`, `subtitle` and every `metadata` entry of the request
// (e.g. `author`, `date`).

#let report(meta, body) = {
  let title = meta.at("title", default: none)
  let subtitle = meta.at("subtitle", default: none)
  let author = meta.at("author", default: none)
  let date = meta.at("date", default: none)

  // Title page
  if title != none {
    align(center + horizon)[
      #text(size: font-size-2xl, weight: "bold", fill: color-text)[#title]

      #if subtitle != none {
        v(spacing-3)
        text(size: font-size-lg, fill: color-text-muted)[#subtitle]
      }

      #v(spacing-6)

      #if author != none {
        text(size: font-size-base, fill: color-text-muted)[#author]
        v(spacing-2)
      }

      #if date != none {
        text(size: font-size-sm, fill: color-text-muted)[#date]
      }
    ]
    pagebreak()
  }

  set page(
    header: context {
      if counter(page).get().first() > 1 {
        grid(
          columns: (1fr, auto),
          align(left)[#text(size: font-size-sm, fill: color-text-muted)[#title]],
          align(right)[#text(size: font-size-sm, fill: color-text-muted)[#date]],
        )
        v(spacing-2)
        line(length: 100%, stroke: 0.5pt + color-border)
      }
    },
    footer: context {
      line(length: 100%, stroke: 0.5pt + color-border)
      v(spacing-2)
      grid(
        columns: (1fr, auto),
        align(left)[#text(size: font-size-sm, fill: color-text-muted)[#author]],
        align(right)[#text(size: font-size-sm, fill: color-text-muted)[
          Page #counter(page).display() of #counter(page).final().first()
        ]],
      )
    },
  )

  body
}
