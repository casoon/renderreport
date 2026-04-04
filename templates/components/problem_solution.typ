// ProblemSolution Component
// Two-part problem/solution block

#let problem-solution(data) = {
  let prob-label = if data.problem_label != none { data.problem_label } else { "Problem" }
  let sol-label  = if data.solution_label != none { data.solution_label } else { "Solution" }

  block(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-lg)[#data.title]
      #v(spacing-3)
    ]

    #grid(
      columns: (1fr, 1fr),
      gutter: spacing-3,

      // Problem (left — red accent)
      block(
        width: 100%,
        fill: color-bad-soft,
        stroke: (left: (paint: color-bad, thickness: 3pt)),
        radius: (right: 8pt),
        inset: spacing-4,
      )[
        #label-text(prob-label)
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#data.problem]
      ],

      // Solution (right — green accent)
      block(
        width: 100%,
        fill: color-ok-soft,
        stroke: (left: (paint: color-ok, thickness: 3pt)),
        radius: (right: 8pt),
        inset: spacing-4,
      )[
        #label-text(sol-label)
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#data.solution]
      ],
    )

    #if data.impact != none or data.effort != none [
      #v(spacing-2)
      #grid(
        columns: if data.impact != none and data.effort != none { (1fr, 1fr) } else { (1fr,) },
        gutter: spacing-3,

        ..if data.impact != none {(
          block(
            fill: color-surface,
            stroke: (paint: color-border, thickness: component-card-border-width),
            radius: 6pt,
            inset: spacing-3,
          )[
            #label-text("Impact")
            #v(spacing-1)
            #text(size: font-size-sm)[#data.impact]
          ],
        )},

        ..if data.effort != none {(
          block(
            fill: color-surface,
            stroke: (paint: color-border, thickness: component-card-border-width),
            radius: 6pt,
            inset: spacing-3,
          )[
            #label-text("Effort")
            #v(spacing-1)
            #text(size: font-size-sm)[#data.effort]
          ],
        )},
      )
    ]
  ]
}
