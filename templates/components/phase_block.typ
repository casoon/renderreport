// PhaseBlock Component
// Colored phase header, bullet list, optional "+ N weitere" footer

#let phase-block(data) = {
  // Default colors per phase number (1=bad, 2=warn, 3=info, 4+ = primary)
  let default-color = if data.phase_number == 1 { color-bad }
    else if data.phase_number == 2 { color-warn }
    else if data.phase_number == 3 { color-info }
    else { color-primary }

  let accent = if data.accent_color != none { rgb(data.accent_color) } else { default-color }

  stack(
    spacing: spacing-2,

    // Header block
    block(
      width: 100%,
      fill: accent,
      radius: 8pt,
      inset: (x: spacing-4, y: spacing-3),
    )[
      #set text(fill: white)
      #grid(
        columns: (auto, 1fr),
        column-gutter: spacing-3,
        align: horizon,

        text(size: font-size-3xl, weight: "bold")[#data.phase_number],

        stack(
          spacing: 3pt,
          text(size: font-size-lg, weight: "bold")[#data.phase_label],
          text(size: font-size-xs, fill: white.transparentize(20%))[#data.description],
        ),
      )
    ],

    // Items list
    block(
      width: 100%,
      fill: color-surface,
      stroke: (paint: color-border, thickness: component-card-border-width),
      radius: 8pt,
      inset: (x: spacing-4, y: spacing-3),
    )[
      #for (i, item) in data.items.enumerate() [
        #grid(
          columns: (10pt, 1fr),
          gutter: 4pt,
          align: (top, top),
          text(size: font-size-sm, fill: accent)[•],
          text(size: font-size-sm, fill: color-text)[#item],
        )
        #if i < data.items.len() - 1 [ #v(3pt) ]
      ]
    ],
  )
}
