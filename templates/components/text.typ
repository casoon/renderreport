// Text component template (renamed to textblock to avoid conflict with built-in text())
#let textblock(data) = {
  let content = data.content
  let font_size = data.at("font_size", default: none)
  let line_height = data.at("line_height", default: none)
  let align_val = data.at("align", default: "left")
  let max_width = data.at("max_width", default: none)
  
  let size_val = if font_size != none { eval(font_size) } else { 10pt }
  let leading_val = if line_height != none {
    let v = eval(line_height)
    if type(v) == length { v } else { v * 1em }
  } else { 0.65em }
  let align_fn = if align_val == "center" { center }
                 else if align_val == "right" { right }
                 else { left }
  let width_val = if max_width != none { eval(max_width) } else { 100% }
  
  block(
    width: width_val,
    {
      set par(leading: leading_val, justify: true)
      align(align_fn, text(size: size_val, content))
    }
  )
}
