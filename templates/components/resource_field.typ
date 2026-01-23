// Resource Field component template (for localization)
#let resource-field(data) = {
  let key = data.key
  let default_value = data.at("default_value", default: none)
  let locale = data.at("locale", default: none)
  
  // In production, this would lookup the key in a resource bundle
  // For now, display the default value or the key itself
  let display = if default_value != none {
    default_value
  } else {
    "[" + key + "]"
  }
  
  text(size: 10pt, display)
  
  if locale != none {
    h(4pt)
    text(size: 7pt, fill: gray, "(" + locale + ")")
  }
}
