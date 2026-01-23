// Default Report Template
// Standard layout with header, footer, and content area

// Page setup is handled by the engine based on theme tokens
// This template defines the document structure

#let report(
  title: none,
  subtitle: none,
  author: none,
  date: none,
  logo: none,
  body
) = {
  // Title page
  if title != none {
    align(center + horizon)[
      #if logo != none {
        image(logo, width: 40%)
        v(spacing-6)
      }
      
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
  
  // Set page header/footer for content pages
  set page(
    header: context {
      if counter(page).get().first() > 1 {
        grid(
          columns: (1fr, auto),
          align(left)[#text(size: font-size-sm, fill: color-text-muted)[#title]],
          align(right)[#text(size: font-size-sm, fill: color-text-muted)[#date]]
        )
        v(spacing-2)
        line(length: 100%, stroke: 0.5pt + color-border)
      }
    },
    footer: context {
      line(length: 100%, stroke: 0.5pt + color-border)
      v(spacing-2)
      grid(
        columns: (1fr, auto, 1fr),
        align(left)[#text(size: font-size-sm, fill: color-text-muted)[#author]],
        align(center)[#text(size: font-size-sm, fill: color-text-muted)[
          Page #counter(page).display() of #locate(loc => counter(page).final(loc).first())
        ]],
        align(right)[]
      )
    }
  )
  
  // Content
  body
}

// Table of contents helper
#let toc() = {
  heading(level: 1)[Table of Contents]
  outline(
    title: none,
    indent: true,
    depth: 3
  )
  pagebreak()
}
