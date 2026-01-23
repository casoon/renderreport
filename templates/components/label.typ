// Label component template
#let label(data) = {
  let content = data.text
  let font_size = data.at("font_size", default: none)
  let font_weight = data.at("font_weight", default: none)
  let color = data.at("color", default: none)
  let align_val = data.at("align", default: none)
  
  let size_val = if font_size != none { eval(font_size) } else { 10pt }
  let color_val = if color != none { rgb(color) } else { black }
  let weight_val = if font_weight != none { font_weight } else { "regular" }
  let align_fn = if align_val == "center" { center } 
                 else if align_val == "right" { right }
                 else { left }
  
  align(align_fn, text(
    size: size_val,
    weight: weight_val,
    fill: color_val,
    content
  ))
}
