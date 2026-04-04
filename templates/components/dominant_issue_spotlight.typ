// DominantIssueSpotlight Component
// Full-width spotlight with left severity stripe, title/body on left, KV on right

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
      columns: (5pt, 1fr),
      gutter: 0pt,

      // Left severity stripe
      block(
        width: 5pt,
        height: 100%,
        fill: sev-color,
        radius: (left: 10pt),
      ),

      // Main content area
      block(inset: (x: spacing-4, y: spacing-4))[
        #grid(
          columns: (1fr, 1fr),
          column-gutter: spacing-5,
          align: (top, top),

          // ── Left: eyebrow, title, badge, body ─────────────────
          stack(
            spacing: spacing-2,
            if data.eyebrow != none {
              text(
                size: font-size-xs,
                weight: "bold",
                fill: color-primary,
                tracking: 0.08em,
                upper(data.eyebrow),
              )
            },
            text(size: font-size-lg, weight: "bold", fill: color-text)[#data.title],
            badge-for-severity(data.severity),
            if data.affected_count != none {
              small-text(str(data.affected_count) + " betroffene Elemente")
            },
            par(justify: true)[#text(size: font-size-sm, fill: color-text)[#data.body]],
          ),

          // ── Right: user impact + recommendation ───────────────
          stack(
            spacing: spacing-3,
            stack(
              spacing: spacing-1,
              label-text("Nutzerauswirkung"),
              text(size: font-size-sm, fill: color-text)[#data.user_impact],
            ),
            stack(
              spacing: spacing-1,
              label-text("Empfehlung"),
              text(size: font-size-sm, fill: color-text)[#data.recommendation],
            ),
          ),
        )
      ],
    )
  ]
}
