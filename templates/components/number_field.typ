// Number Field component template
#let number-field(data) = {
  let value = data.value
  let format = data.at("format", default: none)
  let prefix = data.at("prefix", default: none)
  let suffix = data.at("suffix", default: none)
  let font_size = data.at("font_size", default: none)
  let color = data.at("color", default: none)
  
  let size_val = if font_size != none { eval(font_size) } else { 10pt }
  let color_val = if color != none { rgb(color) } else { black }
  
  // Format number (simplified)
  let formatted = if format != none {
    // Basic number formatting
    if format == "#,##0.00" or format == "#,###.##" {
      str(calc.round(value, digits: 2))
    } else if format == "0.0" {
      str(calc.round(value, digits: 1))
    } else {
      str(value)
    }
  } else {
    str(value)
  }
  
  let display = ""
  if prefix != none { display = prefix + " " }
  display = display + formatted
  if suffix != none { display = display + " " + suffix }
  
  text(size: size_val, fill: color_val, display)
}
