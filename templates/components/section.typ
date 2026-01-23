// Section Component
// Document section with heading

#let section(data) = {
  let heading-size = if data.level == 1 { font-size-2xl }
    else if data.level == 2 { font-size-xl }
    else if data.level == 3 { font-size-lg }
    else { font-size-base }
  
  [
    #heading(level: data.level)[
      #text(size: heading-size, weight: "bold", fill: color-text)[#data.title]
    ]
    
    #v(spacing-4)
  ]
}
