// WordSearch Component
// Generates a parameter-driven word search grid with word list and solution highlighting

#let word-search(data) = {
  let cols = data.width
  let rows = data.height
  let cell-sz = 20pt
  let word-cols = if data.at("columns_word_list", default: none) != none { data.columns_word_list } else { 3 }
  let show-sol = data.at("show_solution", default: false)

  block(width: 100%, breakable: false)[
    #if data.title != none [
      #component-title(text(weight: "bold", size: font-size-lg, fill: color-text)[#data.title])
      #v(spacing-2)
    ]

    #if data.description != none [
      #text(size: font-size-sm, fill: color-text-muted)[#data.description]
      #v(spacing-3)
    ]

    #align(center)[
      #block(
        stroke: 1pt + color-border,
        radius: 6pt,
        inset: spacing-2,
        fill: color-surface,
      )[
        #table(
          columns: range(cols).map(_ => cell-sz),
          rows: range(rows).map(_ => cell-sz),
          align: center + horizon,
          stroke: 0.5pt + color-border.lighten(50%),
          fill: (col, row) => {
            let row-cells = data.grid.at(row)
            let cell = row-cells.at(col)
            if show-sol and cell.is_solution {
              color-accent-soft
            } else {
              color-surface
            }
          },
          ..data.grid.flatten().map(cell => [
            #text(
              weight: if show-sol and cell.is_solution { "bold" } else { "medium" },
              fill: if show-sol and cell.is_solution { color-primary } else { color-text },
              size: font-size-base,
            )[#cell.char]
          ])
        )
      ]
    ]

    #v(spacing-4)

    #if data.words.len() > 0 [
      #text(weight: "semibold", size: font-size-sm, fill: color-text-muted)[#data.word_list_label (#data.words.len()):]
      #v(spacing-2)
      #grid(
        columns: range(word-cols).map(_ => 1fr),
        gutter: spacing-2,
        ..data.words.map(w => [
          #block(
            width: 100%,
            fill: color-surface-alt,
            inset: (x: 8pt, y: 5pt),
            radius: 4pt,
            stroke: 0.5pt + color-border,
          )[
            #text(size: font-size-sm, weight: "bold", fill: color-text)[#w.text]
            #if w.at("translation", default: none) != none [
              #v(3pt)
              #if show-sol [
                #text(size: font-size-xs, fill: color-text-muted, style: "italic")[#w.translation]
              ] else [
                #box(width: 100%, height: 18pt, fill: color-surface, stroke: 0.5pt + color-border, radius: 2pt)
              ]
            ]
          ]
        ])
      )
    ]
  ]
}
