// Barcode component template
#let barcode(data) = {
  let barcode_data = data.data
  let format = data.format
  let width = data.at("width", default: "150pt")
  let height = data.at("height", default: "50pt")
  let show_text = data.at("show_text", default: true)
  
  align(center, block(
    width: 100%,
    breakable: false,
    {
      // Barcode visualization placeholder
      // In production, this would integrate with a barcode generation library
      
      if format == "QR_CODE" or format == "DATA_MATRIX" or format == "PDF417" {
        // 2D barcode (square)
        rect(
          width: eval(width),
          height: eval(width),
          fill: pattern(size: (4pt, 4pt))[
            #square(size: 2pt, fill: black)
          ],
          stroke: 1pt + gray
        )
      } else {
        // 1D barcode (rectangular with vertical bars)
        rect(
          width: eval(width),
          height: eval(height),
          stroke: 1pt + gray,
          {
            // Simulate barcode bars
            grid(
              columns: 20,
              column-gutter: 0pt,
              ..range(20).map(i => {
                let bar_height = if calc.rem(i, 3) == 0 { 100% } else { 90% }
                align(bottom, rect(
                  width: 100%,
                  height: bar_height,
                  fill: if calc.rem(i, 2) == 0 { black } else { white },
                  stroke: none
                ))
              })
            )
          }
        )
      }
      
      // Show text below barcode
      if show_text {
        v(4pt)
        text(size: 8pt, font: "Courier New", barcode_data)
      }
      
      v(4pt)
      text(size: 7pt, fill: gray, format)
    }
  ))
}
