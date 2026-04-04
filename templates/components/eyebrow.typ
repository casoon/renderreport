// Eyebrow Component
// Small uppercase category label with letter-spacing

#let eyebrow(data) = {
  let fill-color = if data.color != none { rgb(data.color) } else { color-primary }
  let body-text = if data.uppercase { upper(data.text) } else { data.text }
  text(
    size: font-size-xs,
    weight: "bold",
    fill: fill-color,
    tracking: 0.08em,
    body-text,
  )
}
