// Image Component
// Image with optional caption

#let report-image(data) = {
  let img-width = if data.width != none { eval(data.width) } else { 100% }
  // PDF/UA-1 requires every image to either carry an alternative description
  // or be explicitly marked as a decorative artifact. Callers that don't
  // supply `alt` almost always mean the image purely decoratively (e.g. a
  // background/illustration), so that is the safe default here — it keeps
  // existing callers compiling instead of forcing a fabricated description.
  let img = if data.alt != none {
    image(data.src, width: img-width, alt: data.alt)
  } else {
    pdf.artifact(image(data.src, width: img-width))
  }

  align(center)[
    #figure(
      img,
      caption: if data.caption != none { data.caption } else { none },
    )
  ]
}
