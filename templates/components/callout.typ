// Callout Component
// Highlighted information box

#let callout-style(callout-type) = {
  if callout-type == "warning" {
    (bg: rgb("#fffbeb"), border: color-warn, icon: "⚠")
  } else if callout-type == "error" {
    (bg: rgb("#fef2f2"), border: color-bad, icon: "✕")
  } else if callout-type == "success" {
    (bg: rgb("#f0fdf4"), border: color-ok, icon: "✓")
  } else if callout-type == "tip" {
    (bg: rgb("#f0f9ff"), border: color-primary, icon: "💡")
  } else {
    // info (default)
    (bg: rgb("#eff6ff"), border: color-primary, icon: "ℹ")
  }
}

#let callout(data) = {
  let style = callout-style(data.callout_type)
  
  box(
    width: 100%,
    inset: spacing-4,
    radius: component-callout-radius,
    fill: style.bg,
    stroke: (left: (paint: style.border, thickness: 3pt)),
  )[
    #set text(fill: color-text)
    
    #if data.title != none [
      #text(weight: "semibold")[#style.icon #data.title]
      #v(spacing-2)
    ]
    
    #text(size: font-size-sm)[#data.content]
  ]
}
