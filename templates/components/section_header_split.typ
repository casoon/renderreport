// SectionHeaderSplit Component
// Left 1/3: optional eyebrow + heading; Right 2/3: body paragraph

#let section-header-split(data) = {
  block(width: 100%, breakable: false)[
    #grid(
      columns: (1fr, 2fr),
      column-gutter: spacing-5,
      align: (top, top),

      // ── Left: eyebrow + heading ──────────────────────────────────
      pad(top: 1pt)[
        #if data.eyebrow != none [
          #text(
            size: font-size-xs,
            weight: "bold",
            fill: color-primary,
            tracking: 0.08em,
            upper(data.eyebrow),
          )
          #v(spacing-2)
        ]
        #heading(level: data.level, outlined: data.outlined)[#data.title]
      ],

      // ── Right: body paragraph ────────────────────────────────────
      pad(top: 2pt)[
        #par(justify: true)[
          #text(size: font-size-base, fill: color-text)[#data.body]
        ]
      ],
    )

    #if data.divider_below [
      #v(spacing-3)
      #line(length: 100%, stroke: 0.5pt + color-border)
      #v(spacing-2)
    ] else [
      #v(spacing-3)
    ]
  ]
}
