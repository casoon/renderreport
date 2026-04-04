// GlossaryList Component
// Term-definition pairs for glossaries, abbreviations, technical terms

#let glossary-list(data) = {
  block(width: 100%)[
    #if data.title != none [
      #text(size: font-size-lg, weight: "bold", fill: color-text)[#data.title]
      #v(spacing-4)
    ]

    #for (i, item) in data.items.enumerate() [
      #block(width: 100%)[
        #text(size: font-size-base, weight: "bold", fill: color-primary)[#item.term]
        #v(spacing-1)
        #text(size: font-size-sm, fill: color-text)[#item.definition]
      ]
      #if i < data.items.len() - 1 [
        #v(spacing-2)
      ]
    ]
  ]
}
