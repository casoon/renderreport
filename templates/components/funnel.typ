// Funnel Component
// Conversion funnel with decreasing-width bars

#let funnel-default-colors = (color-primary, color-info, color-ok, color-warn, color-bad)

#let funnel(data) = {
  let n = data.steps.len()

  block(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-lg)[#data.title]
      #v(spacing-3)
    ]

    #for (i, step) in data.steps.enumerate() [
      #{
        // Width shrinks from 100% to ~40% across steps
        let pct = if n <= 1 { 100 } else {
          int(100 - (i * 60 / (n - 1)))
        }

        let bar-color = if step.at("color", default: none) != none {
          rgb(step.color)
        } else {
          funnel-default-colors.at(calc.rem(i, funnel-default-colors.len()))
        }

        // Center the bar
        align(center)[
          #block(
            width: 1% * pct,
            fill: bar-color,
            radius: 4pt,
            inset: (x: spacing-3, y: spacing-2),
          )[
            #set text(fill: white)
            #grid(
              columns: (1fr, auto),
              align: (left, right),
              text(weight: "semibold", size: font-size-sm)[#step.label],
              text(weight: "bold", size: font-size-sm)[
                #step.value
                #if step.at("unit", default: none) != none [#text(weight: "regular")[#step.unit]]
              ],
            )
          ]
        ]
        v(spacing-1)
      }
    ]
  ]
}
