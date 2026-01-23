// List Component
// Inspired by JasperReports List Component

#let render-list-item(item, level: 0, numbered: false, index: 0) = {
  let indent = spacing-3 * level
  
  [
    #h(indent)
    #if numbered [
      #text(weight: "semibold")[#(index + 1).] #h(spacing-2)
    ] else if item.icon != none [
      #text(fill: color-primary)[#item.icon] #h(spacing-2)
    ] else [
      #text(fill: color-primary)[•] #h(spacing-2)
    ]
    #item.content
    #v(spacing-2)
    
    #if "children" in item and item.children.len() > 0 [
      #for (idx, child) in item.children.enumerate() [
        #render-list-item(child, level: level + 1, numbered: numbered, index: idx)
      ]
    ]
  ]
}

#let list(data) = {
  box(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-base)[#data.title]
      #v(spacing-3)
    ]
    
    #if data.layout == "grid" [
      #let cols = (1fr,) * data.columns
      #grid(
        columns: cols,
        gutter: spacing-4,
        ..data.items.map(item => [
          #if item.icon != none [
            #text(fill: color-primary)[#item.icon] #h(spacing-2)
          ]
          #item.content
        ])
      )
    ] else [
      #for (idx, item) in data.items.enumerate() [
        #render-list-item(item, numbered: data.numbered, index: idx)
      ]
    ]
  ]
}
