// SEO Audit Report Template
// Cover page, table of contents, then content pages with header and footer.
//
// Reads `title`, `url`, `date` and `auditor` from `meta`.

#let report(meta, body) = {
  let title = meta.at("title", default: "SEO Audit Report")
  let url = meta.at("url", default: none)
  let date = meta.at("date", default: none)
  let auditor = meta.at("auditor", default: none)

  // Cover page
  align(center + horizon)[
    #text(size: font-size-2xl, weight: "bold", fill: color-primary)[#title]

    #v(spacing-4)

    #if url != none {
      box(
        inset: spacing-3,
        radius: 4pt,
        fill: color-surface,
        stroke: color-border,
      )[
        #text(size: font-size-lg, fill: color-text)[#url]
      ]
    }

    #v(spacing-6)

    #grid(
      columns: (1fr, 1fr),
      gutter: spacing-4,
      [
        #text(size: font-size-sm, fill: color-text-muted)[Audit Date]
        #v(spacing-1)
        #text(size: font-size-base, weight: "semibold")[#date]
      ],
      [
        #text(size: font-size-sm, fill: color-text-muted)[Auditor]
        #v(spacing-1)
        #text(size: font-size-base, weight: "semibold")[#auditor]
      ],
    )
  ]

  pagebreak()

  // Table of contents
  heading(level: 1)[Table of Contents]
  outline(title: none, indent: auto, depth: 2)

  pagebreak()

  set page(
    header: context {
      if counter(page).get().first() > 2 {
        grid(
          columns: (1fr, auto),
          align(left)[#text(size: font-size-sm, fill: color-text-muted)[SEO Audit: #url]],
          align(right)[#text(size: font-size-sm, fill: color-text-muted)[#date]],
        )
        v(spacing-2)
        line(length: 100%, stroke: 0.5pt + color-border)
      }
    },
    footer: context {
      line(length: 100%, stroke: 0.5pt + color-border)
      v(spacing-2)
      align(center)[
        #text(size: font-size-sm, fill: color-text-muted)[
          Page #counter(page).display() of #counter(page).final().first()
        ]
      ]
    },
  )

  body
}
