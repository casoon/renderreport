// ComparisonBlock Component
// Before/after comparison using before-after() helper from theme_helpers.typ

#let comparison-block(data) = {
  let wrong-label = if data.wrong_label != none { data.wrong_label } else { "✕ " + data.label_left }
  let right-label = if data.right_label != none { data.right_label } else { "✓ " + data.label_right }

  block(width: 100%)[
    #grid(
      columns: (1fr, 1fr),
      gutter: spacing-3,

      // Wrong side
      block(
        width: 100%,
        fill: color-bad-soft,
        radius: 10pt,
        inset: spacing-4,
      )[
        #label-text(wrong-label)
        #v(spacing-2)
        #if data.is_code [
          #mono-text(data.wrong)
        ] else [
          #text(size: font-size-sm, fill: color-text)[#data.wrong]
        ]
      ],

      // Right side
      block(
        width: 100%,
        fill: color-ok-soft,
        radius: 10pt,
        inset: spacing-4,
      )[
        #label-text(right-label)
        #v(spacing-2)
        #if data.is_code [
          #mono-text(data.right)
        ] else [
          #text(size: font-size-sm, fill: color-text)[#data.right]
        ]
      ],
    )

    #if data.note != none [
      #v(spacing-2)
      #small-text(data.note)
    ]
  ]
}
