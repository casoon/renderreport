// WordSearch Component
// Generates a parameter-driven word search grid with word list and solution highlighting

#let word-search(data) = context {
  let cols = data.width
  let rows = data.height
  let word-cols = if data.at("columns_word_list", default: none) != none { data.columns_word_list } else { 3 }
  let show-sol = data.at("show_solution", default: false)
  let letter-size = font-size-base + 1.5pt // grid letters, 12pt with the default theme
  let has-translations = data.at("has_translations", default: false)
  // Vocabulary table: two word|line groups side by side, three from 27 words
  // on (at most 13 rows each at the 39-word limit).
  let n-groups = if data.words.len() > 26 { 3 } else { 2 }
  let n-rows = calc.ceil(data.words.len() / n-groups)
  let list-label = text(weight: "semibold", size: font-size-sm, fill: color-text-muted)[#data.word_list_label (#data.words.len()):]
  let body-w = page.width - 2 * page-margin
  let body-h = page.height - page-margin-top - page-margin-bottom
  // Word column of the vocabulary table: as wide as the longest word (the
  // same in every group, at most half a group), so the rest of each row is
  // left for the translation.
  let group-w = (body-w - (n-groups - 1) * spacing-4) / n-groups
  let word-col = calc.min(
    group-w / 2,
    calc.max(0pt, ..data.words.map(w => measure(text(size: font-size-sm, weight: "bold")[#w.text]).width)) + 16pt,
  )

  let grid-part(cell-sz) = block(width: 100%, breakable: false)[
    #if data.title != none [
      #component-title(text(weight: "bold", size: font-size-lg, fill: color-text)[#data.title])
      #v(spacing-2)
    ]

    // Description and direction hint share one line to save height. Arrows
    // are "→" rotated by each direction's angle — the embedded font has no
    // diagonal arrow glyphs.
    #let show-directions = not show-sol and data.at("direction_angles", default: none) != none
    #if data.description != none or show-directions [
      #if data.description != none [#text(size: font-size-sm, fill: color-text-muted)[#data.description]]
      // Label and arrows never split across lines.
      #if show-directions [#box[
        #text(size: font-size-sm, fill: color-text-muted)[#data.directions_label]
        #h(4pt)
        #text(size: font-size-base, weight: "bold", fill: color-text)[#data.direction_angles.map(a => box(rotate(a * 1deg)[→])).join(h(6pt))]
      ]]
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
              size: letter-size,
            )[#cell.char]
          ])
        )
      ]
    ]

    #if data.words.len() > 0 and not has-translations [
      #v(spacing-4)
      #list-label
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
          ]
        ])
      )
    ]
  ]

  // With translations the word list becomes a vocabulary table: framed
  // groups side by side (word | line to fill in), filled top to bottom. The
  // table stays in one piece.
  let vocab-table(entries, row-h) = block(
    width: 100%,
    stroke: 0.75pt + color-border,
    radius: 4pt,
    clip: true,
    table(
      columns: (word-col, 1fr),
      inset: (x: 8pt, y: 5pt),
      align: left + bottom,
      stroke: (x, y) => (left: if x == 1 { 0.5pt + color-border }),
      ..range(n-rows).map(i => {
        let w = entries.at(i, default: none)
        let translation = if w != none { w.at("translation", default: none) }
        // Rows are at least row-h tall; the first one by the top inset more,
        // so it gets the same writing height above its line as the rows below
        // (line to line). A word wrapping in its column grows its row instead
        // of running into the next one.
        let min-h = (if i == 0 { row-h + 5pt } else { row-h }) - 10pt
        (
          if w != none { text(size: font-size-sm, weight: "bold", fill: color-text)[#w.text] } else { [] },
          block(width: 100%, height: min-h, align(left + bottom, if translation == none { [] } else if show-sol {
            text(size: font-size-sm, fill: color-text-muted, style: "italic", hyphenate: false)[#translation]
          } else {
            line(length: 100%, stroke: 0.5pt + color-text-muted)
          })),
        )
      }).flatten(),
    ),
  )

  let vocab-part(row-h) = {
    v(spacing-4, weak: true)
    block(width: 100%, breakable: false)[
      #list-label
      #v(spacing-2)
      #grid(
        columns: range(n-groups).map(_ => 1fr),
        column-gutter: spacing-4,
        ..range(n-groups).map(g => {
          let n = data.words.len()
          vocab-table(data.words.slice(calc.min(g * n-rows, n), calc.min((g + 1) * n-rows, n)), row-h)
        }),
      )
    ]
  }

  // Sizing. Letters always keep letter-size; only the cell padding shrinks.
  // Cells narrow from 20pt so the grid fits the page width, and — with a
  // vocabulary table — down to 1.2 × letter-size so grid and table share the
  // page (the component starts on a fresh page). Table rows always keep 22pt
  // (≈ 8 mm) of writing height; if the table doesn't fit even with the
  // smallest cells, the grid keeps its size and the table moves to the next
  // page.
  let frame = 2 * spacing-2 + 2pt // grid block inset + stroke, both sides
  let cell-sz = calc.min(20pt, (body-w - frame) / cols)
  let row-h = 22pt
  if has-translations {
    // Cells are fixed-size tracks, so the total height is linear in the
    // cell size — one measurement yields the constant part.
    let total = measure(block(width: body-w, grid-part(cell-sz) + vocab-part(row-h))).height
    let avail = body-h - (total - rows * cell-sz) - 2pt
    let fit-cell = avail / rows
    if fit-cell >= letter-size * 1.2 {
      cell-sz = calc.min(cell-sz, fit-cell)
    }
  }

  grid-part(cell-sz)
  if has-translations {
    vocab-part(row-h)
  }
}
