// ImpactTriad Component
// Three-column impact card grid (user / risk / conversion)

#let impact-card(card) = {
  let accent = if card.status == "good"    { color-ok }
    else if card.status == "warn"          { color-warn }
    else if card.status == "bad"           { color-bad }
    else if card.status == "neutral"       { color-text-muted }
    else { color-primary }

  block(
    width: 100%,
    height: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
  )[
    // Top accent bar
    #block(
      width: 100%,
      height: 3pt,
      fill: accent,
      radius: (top: 10pt),
    )
    #block(inset: (x: spacing-4, y: spacing-3))[
      // Icon + label row
      #stack(
        dir: ltr,
        spacing: 4pt,
        if card.icon != none [
          #text(size: font-size-base)[#card.icon]
        ],
        label-text(card.label),
      )
      #v(spacing-2)
      #text(size: font-size-base, weight: "bold", fill: color-text)[#card.headline]
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text-muted)[#card.body]
    ]
  ]
}

#let impact-triad(data) = {
  block(width: 100%)[
    #if data.title != none [
      #text(weight: "bold", size: font-size-sm, fill: color-text)[#data.title]
      #v(spacing-3)
    ]
    #grid(
      columns: (1fr, 1fr, 1fr),
      column-gutter: spacing-3,
      impact-card(data.user),
      impact-card(data.risk),
      impact-card(data.conversion),
    )
  ]
}
