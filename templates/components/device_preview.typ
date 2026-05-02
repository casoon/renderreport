// Device Preview Component
// Desktop screenshot (3/4) + mobile screenshot (1/4) with Apple-style frames

#let device-preview(data) = {
  let gap = 8pt
  let h = data.height_pt * 1pt
  let dr = data.desktop_ratio

  block(width: 100%, above: 8pt, below: 12pt)[
    #grid(
      columns: (dr * 100% - gap * 0.5, (1.0 - dr) * 100% - gap * 0.5),
      column-gutter: gap,
      align: top,

      // Desktop: browser chrome bar + screenshot
      box(stroke: 0.5pt + rgb("#d1d5db"), radius: 6pt, clip: true, width: 100%)[
        #block(
          fill: rgb("#f3f4f6"),
          width: 100%,
          height: 20pt,
          inset: (left: 8pt, right: 8pt, top: 0pt, bottom: 0pt),
          stroke: (bottom: 0.5pt + rgb("#e5e7eb")),
        )[
          #set align(horizon)
          #stack(dir: ltr, spacing: 5pt,
            circle(radius: 4pt, fill: rgb("#ff5f57")),
            circle(radius: 4pt, fill: rgb("#febc2e")),
            circle(radius: 4pt, fill: rgb("#28c840")),
          )
        ]
        #box(clip: true, width: 100%, height: h - 20pt)[
          #image(data.desktop_src, width: 100%, fit: "cover")
        ]
      ],

      // Mobile: phone-style rounded corners + screenshot
      box(stroke: 0.5pt + rgb("#d1d5db"), radius: 14pt, clip: true, width: 100%)[
        #box(clip: true, width: 100%, height: h)[
          #image(data.mobile_src, width: 100%, fit: "cover")
        ]
      ],
    )
  ]
}
