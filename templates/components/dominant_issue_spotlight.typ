// DominantIssueSpotlight Component
// Compact full-width spotlight with left severity stripe

#let dominant-issue-spotlight(data) = {
  let sev-color = if data.severity == "critical" or data.severity == "high" { color-bad }
    else if data.severity == "medium" { color-warn }
    else if data.severity == "low"    { color-info }
    else { color-text-muted }

  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
  )[
    #grid(
      columns: (4pt, 1fr),
      gutter: 0pt,

      // Left severity stripe
      block(
        width: 4pt,
        fill: sev-color,
        radius: (left: 10pt),
      ),

      // Main content area — compact padding
      block(inset: (x: spacing-3, y: spacing-3))[
        // Eyebrow + badge inline
        #if data.at("eyebrow", default: none) != none [
          #grid(
            columns: (auto, 1fr),
            gutter: spacing-2,
            text(
              size: font-size-xs,
              weight: "bold",
              fill: color-primary,
              tracking: 0.08em,
              upper(data.eyebrow),
            ),
            if data.at("affected_count", default: none) != none {
              text(size: font-size-xs, fill: color-text-muted)[#str(data.affected_count)% Anteil]
            },
          )
          #v(spacing-1)
        ]

        // Title
        #text(size: font-size-base, weight: "bold", fill: color-text)[#data.title]
        #v(spacing-2)

        // Body — single line, compact
        #text(size: font-size-sm, fill: color-text)[#data.body]
        #v(spacing-2)

        // Impact + Recommendation inline as compact grid
        #grid(
          columns: (1fr, 1fr),
          column-gutter: spacing-3,

          stack(
            spacing: spacing-1,
            label-text("Nutzer-Wirkung"),
            text(size: font-size-xs, fill: color-text)[#data.user_impact],
          ),
          stack(
            spacing: spacing-1,
            label-text("Empfehlung"),
            text(size: font-size-xs, fill: color-text)[#data.recommendation],
          ),
        )
      ],
    )
  ]
}
