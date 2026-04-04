// ImpactGrid Component
// Three-column impact card grid (user / risk / conversion)

#let impact-grid-card(card) = {
  let accent = if card.status == "good"    { color-ok }
    else if card.status == "warn"          { color-warn }
    else if card.status == "bad"           { color-bad }
    else if card.status == "neutral"       { color-text-muted }
    else { color-primary }

  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 8pt,
  )[
    // Top accent bar
    #block(
      width: 100%,
      height: 2.5pt,
      fill: accent,
      radius: (top: 8pt),
    )
    #block(inset: (x: spacing-3, y: spacing-2))[
      // Icon + label row
      #stack(
        dir: ltr,
        spacing: 3pt,
        if card.icon != none [
          #text(size: font-size-sm)[#card.icon]
        ],
        label-text(card.label),
      )
      #v(spacing-1)
      #text(size: font-size-sm, weight: "bold", fill: color-text)[#card.headline]
      #if card.body != none and card.body != "" [
        #v(spacing-1)
        #text(size: font-size-xs, fill: color-text-muted)[#card.body]
      ]
    ]
  ]
}

#let impact-grid(data) = {
  block(width: 100%)[
    #if data.title != none [
      #text(weight: "bold", size: font-size-sm, fill: color-text)[#data.title]
      #v(spacing-3)
    ]
    #stack(
      spacing: spacing-3,
      impact-grid-card(data.user),
      impact-grid-card(data.risk),
      impact-grid-card(data.conversion),
    )
  ]
}
