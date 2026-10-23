// Divider Component
// Inspired by BIRT Band Elements

#let divider(data) = {
  let line-color = if data.color != none {
    rgb(data.color)
  } else {
    color-border
  }
  
  let line-style = if data.style == "dashed" {
    (dash: "dashed")
  } else if data.style == "dotted" {
    (dash: "dotted")
  } else if data.style == "double" {
    (thickness: 2pt)
  } else {
    (:)
  }
  
  let thickness = eval(data.thickness)
  if thickness == 0pt {
    // An invisible divider is pure spacing. A zero-width line is still
    // content: at the end of a full page it opened a page of its own.
    v(eval(data.spacing_above) + eval(data.spacing_below), weak: true)
  } else {
    v(eval(data.spacing_above))
    line(
      length: 100%,
      stroke: (paint: line-color, thickness: thickness) + line-style
    )
    v(eval(data.spacing_below))
  }
}
