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

  let is-inverted = data.at("inverted", default: false)

  if is-inverted {
    // Inverted variant: colored background, white text
    let h = if data.height != none { eval(data.height) } else { auto }
    block(
      width: 100%,
      height: h,
      fill: status-color,
      radius: 10pt,
      inset: (x: spacing-4, y: spacing-4),
    )[
      #text(size: font-size-xs, weight: "bold", fill: white.transparentize(25%))[#data.title]
      #v(spacing-2)
      #text(size: font-size-3xl, weight: "bold", fill: white)[
        #data.score#text(size: font-size-base, weight: "regular", fill: white.transparentize(35%))[\/#{data.max_score}]
      ]
      #if data.description != none [
        #v(spacing-3)
        #text(size: font-size-sm, fill: white.transparentize(20%))[#data.description]
      ]
    ]
  } else {
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
      // Full-width variant: left accent strip + larger score number
      let tall-body = [
        #set text(fill: color-text)
        #label-text(data.title)
        #v(spacing-2)
        #text(size: font-size-3xl, weight: "bold", fill: status-color)[
          #data.score#text(size: font-size-base, weight: "regular", fill: color-text-muted)[\/#{data.max_score}]
        ]
        #v(spacing-3)
        #theme-progress-bar(data.score, max: data.max_score, bar-color: status-color)
        #if data.description != none [
          #v(spacing-3)
          #small-text(data.description)
        ]
      ]
      grid(
        columns: (5pt, 1fr),
        gutter: 0pt,
        block(
          width: 5pt,
          height: eval(data.height),
          fill: status-color,
          radius: (left: 10pt),
        ),
        block(
          width: 100%,
          height: eval(data.height),
          fill: color-surface,
          stroke: (top: component-card-border-width + color-border,
                   right: component-card-border-width + color-border,
                   bottom: component-card-border-width + color-border),
          radius: (right: 10pt),
          inset: (x: spacing-4, y: spacing-4),
        )[ #tall-body ]
      )
    } else {
      theme-card[#body]
    }
  }
}
