// BeforeAfter Component
// Before/after comparison block

#let before-after(data) = {
  let lbl-before = if data.label_before != none { data.label_before } else { "Before" }
  let lbl-after  = if data.label_after  != none { data.label_after  } else { "After"  }

  block(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-lg)[#data.title]
      #v(spacing-3)
    ]

    #grid(
      columns: (1fr, 1fr),
      gutter: spacing-3,

      // Before — muted/grey
      block(
        width: 100%,
        fill: color-surface-alt,
        stroke: (paint: color-border, thickness: component-card-border-width),
        radius: 8pt,
        inset: spacing-4,
      )[
        #label-text(lbl-before)
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#data.before]
      ],

      // After — accent color
      block(
        width: 100%,
        fill: color-accent-soft,
        stroke: (left: (paint: color-primary, thickness: 3pt)),
        radius: (right: 8pt),
        inset: spacing-4,
      )[
        #label-text(lbl-after)
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#data.after]
      ],
    )

    #if data.note != none [
      #v(spacing-2)
      #small-text(data.note)
    ]
  ]
}
