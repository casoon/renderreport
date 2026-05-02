// Device Preview Component
// Desktop screenshot (70%) + mobile screenshot (30%) with Apple-style frames
// Centered at 88% width for breathing room on both sides

#let device-preview(data) = {
  let gap = 8pt
  let h = data.height_pt * 1pt
  let dr = data.desktop_ratio

  align(center, block(width: 88%, above: 2pt, below: 6pt, breakable: false)[
    #set align(left)
    #grid(
      columns: (dr * 100% - gap * 0.5, (1.0 - dr) * 100% - gap * 0.5),
      column-gutter: gap,
      align: top,

      // Desktop: chrome bar stacked directly above screenshot, zero gap
      box(stroke: 0.5pt + rgb("#d1d5db"), radius: 6pt, clip: true, width: 100%, height: h)[
        #stack(dir: ttb, spacing: 0pt,
          block(
            fill: rgb("#f3f4f6"),
            width: 100%,
            height: 20pt,
            above: 0pt,
            below: 0pt,
            stroke: (bottom: 0.5pt + rgb("#e5e7eb")),
            inset: (left: 8pt, right: 8pt, top: 0pt, bottom: 0pt),
          )[
            #set align(left + horizon)
            #stack(dir: ltr, spacing: 5pt,
              circle(radius: 4pt, fill: rgb("#ff5f57")),
              circle(radius: 4pt, fill: rgb("#febc2e")),
              circle(radius: 4pt, fill: rgb("#28c840")),
            )
          ],
          // Image at full width, outer box clips to h - 20pt — shows top of page
          image(data.desktop_src, width: 100%),
        )
      ],

      // Mobile: phone-style rounded corners, outer box clips to h — shows top
      box(stroke: 0.5pt + rgb("#d1d5db"), radius: 14pt, clip: true, width: 100%, height: h)[
        #image(data.mobile_src, width: 100%)
      ],
    )
  ])
}
