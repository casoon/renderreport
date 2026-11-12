// Chartlet Component
// SVG chart generated from a chartlet specification (see src/components/chartlet.rs)

#let chartlet(data) = {
  let img-width = if data.width != none { eval(data.width) } else { 100% }

  align(center)[
    #figure(
      image(bytes(data.svg), format: "svg", width: img-width, alt: data.alt),
      caption: if data.caption != none { data.caption } else { none },
    )
  ]
}
