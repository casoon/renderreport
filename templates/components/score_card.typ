// Score Card Component
// Displays a metric with score visualization

#let score-card(data) = {
  let status-color = if data.computed_status == "good" {
    color-ok
  } else if data.computed_status == "warning" {
    color-warn
  } else {
    color-bad
  }

  let body = [
    #set text(fill: color-text)

    #label-text(data.title)

    #v(spacing-2)

    #text(size: font-size-2xl, weight: "bold", fill: status-color)[
      #data.score#text(size: font-size-base, weight: "regular", fill: color-text-muted)[\/#{data.max_score}]
    ]

    #v(spacing-3)
    #theme-progress-bar(data.score, max: data.max_score, bar-color: status-color)

    #if data.description != none [
      #v(spacing-3)
      #small-text(data.description)
    ]
  ]

  if data.height != none {
    block(
      width: 100%,
      height: eval(data.height),
      fill: color-surface,
      stroke: (paint: color-border, thickness: component-card-border-width),
      radius: 10pt,
      inset: (x: spacing-4, y: spacing-4),
    )[ #body ]
  } else {
    theme-card[#body]
  }
}
