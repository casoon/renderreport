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
          // `place(top, ..)` over a plain `image(..)`: harmless when the
          // image already fits (the normal case here — desktop screenshots
          // rarely overflow this box) but doesn't reliably fix it when it
          // doesn't — see the mobile column below for why overflow needs to
          // be avoided upstream instead of fixed here.
          place(top, image(data.desktop_src, width: 100%)),
        )
      ],

      // Mobile: phone-style rounded corners, outer box clips to h. A
      // portrait screenshot scaled to this column's width naturally comes
      // out several times taller than `h` — verified empirically (test
      // image with distinct top/mid/bottom color bands) that when this
      // much overflow needs clipping, neither a plain `image` nor
      // `place(top, ..)` nor `image(.., fit: "cover")` reliably keeps the
      // top of the source in frame; the top slice (here: the page header)
      // silently goes missing regardless of `h`. There is no in-template
      // fix for this — the caller MUST pre-crop the source asset to near
      // this box's aspect ratio before handing it to this component (see
      // `register_page_screenshot_assets` in auditmysite), so this box
      // only ever needs to clip a small, harmless margin.
      box(stroke: 0.5pt + rgb("#d1d5db"), radius: 14pt, clip: true, width: 100%, height: h)[
        #place(top, image(data.mobile_src, width: 100%))
      ],
    )
  ])
}
