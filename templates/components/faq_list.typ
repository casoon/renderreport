// FaqList Component
// Question-answer pairs for FAQs and knowledge bases

#let faq-list(data) = {
  block(width: 100%)[
    #if data.title != none [
      #text(size: font-size-lg, weight: "bold", fill: color-text)[#data.title]
      #v(spacing-4)
    ]

    #for (i, item) in data.items.enumerate() [
      #block(width: 100%)[
        #text(size: font-size-base, weight: "bold", fill: color-text)[#item.question]
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#item.answer]
      ]
      #if i < data.items.len() - 1 [
        #v(spacing-3)
      ]
    ]
  ]
}
