// WrongRightBlock Component
// Before/after comparison using before-after() helper from theme_helpers.typ

#let wrong-right-block(data) = {
  let wrong-label = if data.at("wrong_label", default: none) != none { data.wrong_label } else { "✗ Wrong" }
  let right-label = if data.at("right_label", default: none) != none { data.right_label } else { "✓ Right" }

  // Short examples stay together: split across a page, the label sat alone
  // on one page and the code on the next. Long examples may still break.
  let length = str(data.wrong).len() + str(data.right).len()
  block(width: 100%, breakable: length > 900)[
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

    #if data.at("note", default: none) != none [
      #v(spacing-2)
      #small-text(data.note)
    ]
  ]
}
