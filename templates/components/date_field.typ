// Date Field component template
#let date-field(data) = {
  let date = data.date
  let format = data.at("format", default: "YYYY-MM-DD")
  let font_size = data.at("font_size", default: none)
  
  let size_val = if font_size != none { eval(font_size) } else { 10pt }
  
  // Note: In production, this would parse ISO date and format accordingly
  // For now, display the date string as-is
  text(size: size_val, date)
}
