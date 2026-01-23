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
    ()
  }
  
  v(eval(data.spacing_above))
  line(
    length: 100%,
    stroke: (paint: line-color, thickness: eval(data.thickness)) + line-style
  )
  v(eval(data.spacing_below))
}
