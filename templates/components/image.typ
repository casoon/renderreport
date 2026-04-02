// Image Component
// Image with optional caption

#let report-image(data) = {
  let img-width = if data.width != none { eval(data.width) } else { 100% }

  align(center)[
    #figure(
      image(data.src, width: img-width, alt: data.alt),
      caption: if data.caption != none { data.caption } else { none },
    )
  ]
}
