// Watermark Component
// Inspired by Pentaho Watermark Band

#let watermark(data) = {
  place(
    center + horizon,
    rotate(
      data.rotation * 1deg,
      text(
        size: eval(data.size),
        weight: "bold",
        fill: color-text.transparentize(100% - data.opacity * 100%),
      )[#data.text]
    )
  )
}
