// SideLabel Component
// Left 1/3: section heading, Right 2/3: content (bullets or text)

#let side-label(data) = {
  let subheading = data.at("subheading", default: none)
  let text-val = data.at("text", default: none)
  block(
    width: 100%,
    breakable: false,
    {
      grid(
        columns: (1fr, 2fr),
        column-gutter: spacing-5,
        align: (top, top),

        // ── Left: heading ──────────────────────────────────────────
        pad(top: 1pt)[
          #text(weight: "bold", size: font-size-sm, fill: color-text)[#data.heading]
          #if subheading != none [
            #v(2pt)
            #text(size: font-size-xs, fill: color-text-muted)[#subheading]
          ]
        ],

        // ── Right: items or text ───────────────────────────────────
        box(width: 100%)[
          #if data.items.len() > 0 [
            #for (i, item) in data.items.enumerate() [
              #grid(
                columns: (10pt, 1fr),
                gutter: 4pt,
                align: (top, top),
                text(size: font-size-sm, fill: color-primary)[•],
                text(size: font-size-sm, fill: color-text)[#item],
              )
              #if i < data.items.len() - 1 [ #v(3pt) ]
            ]
          ] else if text-val != none [
            #text(size: font-size-sm, fill: color-text)[#text-val]
          ]
        ]
      )

      // Optional bottom divider
      if data.divider [
        #v(spacing-2)
        #line(length: 100%, stroke: 0.5pt + color-border)
        #v(spacing-1)
      ] else [
        #v(spacing-3)
      ]
    }
  )
}
