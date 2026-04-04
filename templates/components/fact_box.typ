// FactBox Component
// Inline info/fact box

#let fact-box-style(variant) = {
  if variant == "tip"     { (accent: color-ok,      bg: color-ok-soft,      icon: "✓") }
  else if variant == "warning" { (accent: color-warn,    bg: color-warn-soft,    icon: "⚠") }
  else if variant == "stat"    { (accent: color-primary, bg: color-accent-soft,  icon: "#") }
  else                         { (accent: color-info,    bg: color-info-soft,    icon: "ℹ") }  // "info" default
}

#let fact-box(data) = {
  let variant = if data.variant != none { data.variant } else { "info" }
  let style   = fact-box-style(variant)

  block(
    width: 100%,
    fill: style.bg,
    stroke: (left: (paint: style.accent, thickness: 3pt)),
    radius: (right: 6pt),
    inset: (x: spacing-3, y: spacing-3),
  )[
    #if data.label != none [
      #text(weight: "bold", size: font-size-xs, fill: style.accent)[#style.icon #upper(data.label)]
      #v(spacing-1)
    ]

    #text(size: font-size-sm, fill: color-text)[#data.body]
  ]
}
