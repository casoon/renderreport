// ProcessFlow Component
// Linear process visualization with numbered steps

#let process-flow(data) = {
  let dir = if data.direction != none { data.direction } else { "horizontal" }

  block(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-lg)[#data.title]
      #v(spacing-3)
    ]

    #if dir == "vertical" [
      // Vertical layout: stacked steps with arrow below each
      #for (i, step) in data.steps.enumerate() [
        #block(
          width: 100%,
          fill: color-surface,
          stroke: (paint: color-border, thickness: component-card-border-width),
          radius: 8pt,
          inset: spacing-4,
        )[
          #grid(
            columns: (auto, 1fr),
            column-gutter: spacing-3,
            align: (top, top),

            // Step number circle
            block(
              width: 24pt,
              height: 24pt,
              fill: color-primary,
              radius: 50%,
              inset: 0pt,
            )[
              #align(center + horizon)[
                #text(size: font-size-xs, weight: "bold", fill: white)[#(i + 1)]
              ]
            ],

            stack(
              spacing: 2pt,
              text(weight: "bold", size: font-size-base)[
                #if step.icon != none [#step.icon #h(4pt)]
                #step.label
              ],
              if step.description != none {
                text(size: font-size-sm, fill: color-text-muted)[#step.description]
              },
            ),
          )
        ]
        #if i < data.steps.len() - 1 [
          #align(center)[#text(size: font-size-lg, fill: color-text-muted)[↓]]
        ]
      ]
    ] else [
      // Horizontal layout: row of boxes with → between
      #grid(
        columns: range(data.steps.len() * 2 - 1).map(i => if calc.rem(i, 2) == 0 { 1fr } else { auto }),
        align: center,
        column-gutter: spacing-2,

        ..{
          let cells = ()
          for (i, step) in data.steps.enumerate() {
            cells.push(
              block(
                width: 100%,
                fill: color-surface,
                stroke: (paint: color-border, thickness: component-card-border-width),
                radius: 8pt,
                inset: spacing-3,
              )[
                #align(center)[
                  #block(
                    width: 20pt,
                    height: 20pt,
                    fill: color-primary,
                    radius: 50%,
                  )[
                    #align(center + horizon)[
                      #text(size: font-size-xs, weight: "bold", fill: white)[#(i + 1)]
                    ]
                  ]
                  #v(spacing-1)
                  #if step.icon != none [#text(size: font-size-base)[#step.icon] #linebreak()]
                  #text(weight: "bold", size: font-size-sm)[#step.label]
                  #if step.description != none [
                    #linebreak()
                    #text(size: font-size-xs, fill: color-text-muted)[#step.description]
                  ]
                ]
              ]
            )
            if i < data.steps.len() - 1 {
              cells.push(
                align(center + horizon)[
                  #text(size: font-size-base, fill: color-text-muted)[→]
                ]
              )
            }
          }
          cells
        },
      )
    ]
  ]
}
