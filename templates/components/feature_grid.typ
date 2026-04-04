// FeatureGrid Component
// Marketing feature/benefit grid

#let feature-grid-item-accent(status) = {
  if status == "positive"  { color-ok }
  else if status == "highlight" { color-primary }
  else { color-text-muted }
}

#let feature-grid(data) = {
  let cols = if data.columns != none { data.columns } else { 2 }

  block(width: 100%)[
    #if data.title != none [
      #text(weight: "semibold", size: font-size-lg)[#data.title]
      #v(spacing-3)
    ]

    #grid(
      columns: range(cols).map(_ => 1fr),
      gutter: spacing-3,

      ..for item in data.items {(
        block(
          width: 100%,
          fill: color-surface,
          stroke: (paint: color-border, thickness: component-card-border-width),
          radius: 8pt,
          inset: spacing-4,
        )[
          #let accent = feature-grid-item-accent(item.status)

          #if item.icon != none [
            #text(size: font-size-xl, fill: accent)[#item.icon]
            #v(spacing-2)
          ]

          #text(weight: "bold", size: font-size-base, fill: accent)[#item.title]

          #if item.description != none [
            #v(spacing-1)
            #text(size: font-size-sm, fill: color-text-muted)[#item.description]
          ]
        ],
      )},
    )
  ]
}
