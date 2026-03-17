// Barcode component template – renders real encoded patterns
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
      if data.at("encoding_2d", default: none) != none {
        // 2D barcode (QR Code / Data Matrix) – draw module matrix
        let matrix = data.encoding_2d
        let cols = data.at("qr_width", default: 21)
        let rows = data.at("qr_height", default: cols)
        let target_w = eval(width)
        // Compute module size from the wider dimension, keep aspect ratio
        let module_size = calc.min(target_w / cols, target_w / rows)
        let render_w = cols * module_size
        let render_h = rows * module_size

        box(
          width: render_w,
          height: render_h,
          clip: true,
          {
            for (row_idx, row) in matrix.enumerate() {
              for (col_idx, cell) in row.enumerate() {
                if cell == 1 {
                  place(
                    dx: col_idx * module_size,
                    dy: row_idx * module_size,
                    rect(
                      width: module_size + 0.5pt,
                      height: module_size + 0.5pt,
                      fill: black,
                      stroke: none,
                    )
                  )
                }
              }
            }
          }
        )
      } else if data.at("encoding", default: none) != none {
        // 1D barcode – draw actual bar pattern
        let bars = data.encoding
        let bar_count = bars.len()
        let target_width = eval(width)
        let target_height = eval(height)
        let bar_width = target_width / bar_count

        box(
          width: target_width,
          height: target_height,
          clip: true,
          {
            for (i, bar) in bars.enumerate() {
              if bar == 1 {
                place(
                  dx: i * bar_width,
                  rect(
                    width: bar_width + 0.3pt,
                    height: target_height,
                    fill: black,
                    stroke: none,
                  )
                )
              }
            }
          }
        )
      } else if data.at("unsupported", default: false) {
        // Unsupported 2D format – placeholder with notice
        rect(
          width: eval(width),
          height: eval(width),
          fill: luma(240),
          stroke: 1pt + gray,
          align(center + horizon, text(size: 8pt, fill: gray, [#format encoding\ not yet supported]))
        )
      } else {
        // Fallback placeholder
        rect(
          width: eval(width),
          height: eval(height),
          fill: luma(240),
          stroke: 1pt + gray,
          align(center + horizon, text(size: 8pt, fill: gray, [Encoding failed]))
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
