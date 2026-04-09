// Theme Helper Functions
// Utility functions for modern B2B report design

// ── Typography helpers ──────────────────────────────────────
#let display-text(body) = text(size: font-size-3xl, weight: "bold", fill: color-text, body)
#let label-text(body) = text(size: font-size-xs, weight: "bold", fill: color-text-muted, body)
#let small-text(body) = text(size: font-size-sm, fill: color-text-muted, body)
#let mono-text(body) = text(font: font-mono, size: font-size-sm, fill: color-text, body)

// ── Card system ─────────────────────────────────────────────
#let theme-card(body, fill: color-surface) = block(
  width: 100%,
  fill: fill,
  stroke: (paint: color-border, thickness: component-card-border-width),
  radius: 10pt,
  inset: (x: spacing-4, y: spacing-4),
  breakable: false,
  body,
)

#let soft-card(body) = theme-card(fill: color-surface-soft, body)

// ── Badge system ────────────────────────────────────────────
#let theme-badge(body, fg, bg) = box(
  fill: bg,
  radius: 999pt,
  inset: (x: 8pt, y: 4pt),
  text(size: font-size-xs, weight: "bold", fill: fg, body),
)

#let badge-high() = theme-badge([HIGH], color-bad, color-bad-soft)
#let badge-medium() = theme-badge([MEDIUM], color-warn, color-warn-soft)
#let badge-low() = theme-badge([LOW], color-info, color-info-soft)
#let badge-good() = theme-badge([GOOD], color-ok, color-ok-soft)

#let badge-for-severity(severity) = {
  if severity == "critical" or severity == "high" { badge-high() }
  else if severity == "medium" { badge-medium() }
  else if severity == "low" { badge-low() }
  else { badge-good() }
}

// ── Chart helpers ───────────────────────────────────────────
#let theme-progress-bar(value, max: 100, bar-color: color-primary, bar-label: none) = {
  let pct = calc.min(1.0, value / max)
  stack(
    spacing: spacing-2,
    if bar-label != none [#small-text(bar-label)],
    box(
      fill: color-surface-alt,
      radius: 999pt,
      width: 100%,
      height: 8pt,
      place(left)[
        #box(
          fill: bar-color,
          radius: 999pt,
          width: 100% * pct,
          height: 8pt,
        )
      ]
    )
  )
}

#let theme-severity-strip(high, medium, low) = {
  let total = calc.max(1, high + medium + low)
  let h-pct = high / total * 100%
  let m-pct = medium / total * 100%
  let l-pct = low / total * 100%
  grid(
    columns: (h-pct, m-pct, l-pct),
    gutter: 0pt,
    box(fill: color-bad, height: 8pt, radius: (left: 999pt)),
    box(fill: color-warn, height: 8pt),
    box(fill: color-ok, height: 8pt, radius: (right: 999pt)),
  )
}

// ── Code display helpers ────────────────────────────────────
#let code-panel(title, body, fill: color-surface-soft) = block(
  width: 100%,
  fill: fill,
  radius: 10pt,
  inset: spacing-4,
)[
  #label-text(title)
  #v(spacing-2)
  #mono-text(body)
]

#let before-after(bad, good) = grid(
  columns: (1fr, 1fr),
  gutter: spacing-3,
  code-panel("✕ Wrong", bad, fill: color-bad-soft),
  code-panel("✓ Right", good, fill: color-ok-soft),
)

// ── Status color helper ─────────────────────────────────────
#let status-color-for(status) = {
  if status == "good" { color-ok }
  else if status == "warning" { color-warn }
  else { color-bad }
}

#let score-color-for(score) = {
  if score >= 85 { color-ok }
  else if score >= 70 { color-info }
  else if score >= 50 { color-warn }
  else { color-bad }
}
