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

  theme-card[
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
}
