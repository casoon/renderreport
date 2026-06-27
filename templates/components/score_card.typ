// Score Card Component
// Displays a metric with score visualization

#let score-card-standard(data, status-color) = {
  let body = [
    #set text(fill: color-text)

    #label-text(data.title)

    #v(spacing-2)

    #text(size: font-size-2xl, weight: "bold", fill: status-color)[
      #data.score
    ]

    #v(spacing-3)
    #theme-progress-bar(data.score, max: data.max_score, bar-color: status-color)

    #if data.description != none [
      #v(spacing-3)
      #small-text(data.description)
    ]
  ]
  theme-card[#body]
}

#let score-card-tall(data, status-color) = {
  let h = if data.height != none { eval(data.height) } else { 100pt }
  let body = [
    #set text(fill: color-text)
    #label-text(data.title)
    #v(spacing-2)
    #text(size: font-size-3xl, weight: "bold", fill: status-color)[
      #data.score
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
      height: h,
      fill: status-color,
      radius: (left: 10pt),
    ),
    block(
      width: 100%,
      height: h,
      fill: color-surface,
      stroke: (top: component-card-border-width + color-border,
               right: component-card-border-width + color-border,
               bottom: component-card-border-width + color-border),
      radius: (right: 10pt),
      inset: (x: spacing-4, y: spacing-4),
    )[ #body ]
  )
}

#let score-card-inverted(data, status-color) = {
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
      #data.score
    ]
    #if data.description != none [
      #v(spacing-3)
      #text(size: font-size-sm, fill: white.transparentize(20%))[#data.description]
    ]
  ]
}

#let score-card-compact(data, status-color) = {
  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: component-score-card-radius,
    inset: (x: spacing-3, y: spacing-2),
  )[
    #set text(fill: color-text)
    #label-text(data.title)
    #v(spacing-1)
    #text(size: font-size-xl, weight: "bold", fill: status-color)[
      #data.score
    ]
    #if data.description != none [
      #v(spacing-1)
      #small-text(data.description)
    ]
  ]
}

#let score-card(data) = {
  let status-color = if data.computed_status == "good" {
    color-ok
  } else if data.computed_status == "warning" {
    color-warn
  } else {
    color-bad
  }

  let variant = data.at("variant", default: "standard")

  if variant == "inverted" {
    score-card-inverted(data, status-color)
  } else if variant == "tall" {
    score-card-tall(data, status-color)
  } else if variant == "compact" {
    score-card-compact(data, status-color)
  } else {
    score-card-standard(data, status-color)
  }
}
