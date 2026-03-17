// Callout Component
// Highlighted information box

#let callout-style(callout-type) = {
  if callout-type == "warning" {
    (bg: color-warn-soft, border: color-warn, icon: "⚠")
  } else if callout-type == "error" {
    (bg: color-bad-soft, border: color-bad, icon: "✕")
  } else if callout-type == "success" {
    (bg: color-ok-soft, border: color-ok, icon: "✓")
  } else if callout-type == "tip" {
    (bg: color-accent-soft, border: color-primary, icon: "💡")
  } else {
    // info (default)
    (bg: color-info-soft, border: color-info, icon: "ℹ")
  }
}

#let callout(data) = {
  let style = callout-style(data.callout_type)

  block(
    width: 100%,
    inset: spacing-4,
    radius: component-callout-radius,
    fill: style.bg,
    stroke: (left: (paint: style.border, thickness: 3pt)),
  )[
    #set text(fill: color-text)

    #if data.title != none [
      #text(weight: "bold")[#style.icon #data.title]
      #v(spacing-2)
    ]

    #par(justify: true)[#text(size: font-size-base)[#data.content]]
  ]
}
