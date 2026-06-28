// Theme Tokens
#let color-accent-soft = rgb("#EAF2FF")
#let color-background = rgb("#ffffff")
#let color-bad = rgb("#D92D20")
#let color-bad-soft = rgb("#FEE4E2")
#let color-border = rgb("#E4E7EC")
#let color-info = rgb("#1570EF")
#let color-info-soft = rgb("#EFF8FF")
#let color-ok = rgb("#12B76A")
#let color-ok-soft = rgb("#ECFDF3")
#let color-primary = rgb("#155EEF")
#let color-secondary = rgb("#667085")
#let color-surface = rgb("#ffffff")
#let color-surface-alt = rgb("#F2F4F7")
#let color-surface-soft = rgb("#F8FAFC")
#let color-text = rgb("#101828")
#let color-text-muted = rgb("#667085")
#let color-warn = rgb("#F79009")
#let color-warn-soft = rgb("#FEF0C7")
#let component-callout-radius = 10pt
#let component-card-border-width = 0.8pt
#let component-finding-radius = 10pt
#let component-score-card-radius = 10pt
#let font-body = "Helvetica Neue"
#let font-heading = "Helvetica Neue"
#let font-mono = "Menlo"
#let font-size-2xl = 24pt
#let font-size-3xl = 34pt
#let font-size-base = 10.5pt
#let font-size-lg = 13pt
#let font-size-sm = 8.8pt
#let font-size-xl = 18pt
#let font-size-xs = 8.5pt
#let page-footer-height = 1cm
#let page-header-height = 1.5cm
#let page-margin = 18mm
#let page-margin-bottom = 16mm
#let page-margin-top = 18mm
#let spacing-1 = 4pt
#let spacing-2 = 6pt
#let spacing-3 = 10pt
#let spacing-4 = 14pt
#let spacing-5 = 20pt
#let spacing-6 = 28pt
#let spacing-7 = 40pt
#let table-border = rgb("#E4E7EC")
#let table-border-width = 0.5pt
#let table-header-bg = rgb("#F2F4F7")
#let table-row-alt-bg = rgb("#F8FAFC")

// Theme Helpers
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

// ── Orphan-safe component title ─────────────────────────────
// Wraps a title with a breakable:false guard so it is never
// left alone at the bottom of a page.
// spacing: gap between title and component body.
// min-body: minimum body height to reserve after the title.
//   Use 2em for text/list bodies, 4em for table/grid bodies
//   (table rows are ~3-4em tall including padding).
#let component-title(content, spacing: spacing-3, min-body: 4em) = {
  // `sticky: true` keeps the title on the same page as the content that follows
  // it, so a component heading never strands alone at the bottom of a page with
  // its body pushed to the next page. (`min-body` is kept for call-site
  // compatibility but is no longer needed — sticky handles tall bodies too.)
  block(width: 100%, breakable: false, below: spacing, sticky: true)[#content]
}


// Report Metadata
#let report-title = "Snapshot Report"
#let report-date = ""
#let report-author = ""
#let report-footer-link-url = ""
#let report-footer-prefix = ""
#let report-footer-tagline = ""

#set page(
  paper: "a4",
  fill: color-background,
  margin: (
    top: page-margin-top,
    bottom: page-margin-bottom,
    left: page-margin,
    right: page-margin,
  ),
  header: context {
    if counter(page).get().first() > 1 [
      #set text(size: font-size-xs, fill: color-text-muted)
      #grid(
        columns: (1fr, auto),
        gutter: spacing-3,
        [#report-title],
        [#report-date]
      )
      #v(4pt)
      #line(length: 100%, stroke: (paint: color-border, thickness: 0.7pt))
    ]
  },
  footer: context {
    if counter(page).get().first() > 1 [
      #v(1pt)
      #line(length: 100%, stroke: (paint: color-border, thickness: 0.5pt))
      #v(3pt)
      #grid(
        columns: (1fr, auto, 1fr),
        gutter: spacing-3,
        [#text(size: font-size-xs, fill: color-text-muted)[
          #if report-footer-prefix != "" { report-footer-prefix + " " }
          #if report-footer-link-url != "" {
            link(report-footer-link-url)[#text(weight: "semibold", fill: color-text-muted)[#report-author]]
          } else {
            text(weight: "semibold")[#report-author]
          }
        ]],
        [#text(size: font-size-xs, fill: color-text-muted)[#counter(page).display("1 / 1", both: true)]],
        align(right)[#text(size: font-size-xs, fill: color-text-muted)[#report-footer-tagline]]
      )
    ]
  },
)

#set text(
  font: (font-body, "Arial", "Liberation Sans", "Noto Sans"),
  size: font-size-base,
  fill: color-text,
)

#set text(hyphenate: true)
#set par(justify: false, leading: 0.75em)

#set heading(numbering: none)
#show heading: set par(justify: false)
#show heading: set block(sticky: true)
#show heading.where(level: 1): set text(size: font-size-2xl, weight: "bold", fill: color-text)
#show heading.where(level: 2): set text(size: font-size-xl, weight: "bold", fill: color-text)
#show heading.where(level: 3): set text(size: font-size-lg, weight: "bold", fill: color-text)
#show heading.where(level: 4): set text(size: font-size-base, weight: "bold", fill: color-text-muted)


// Component Functions
// Callout Component
// Highlighted information box (supports neutral type and inverted mode)

#let callout-style(callout-type) = {
  if callout-type == "warning" {
    (bg: color-warn-soft, border: color-warn, icon: "⚠")
  } else if callout-type == "error" {
    (bg: color-bad-soft, border: color-bad, icon: "✕")
  } else if callout-type == "success" {
    (bg: color-ok-soft, border: color-ok, icon: "✓")
  } else if callout-type == "tip" {
    (bg: color-accent-soft, border: color-primary, icon: "💡")
  } else if callout-type == "neutral" {
    (bg: color-surface-alt, border: color-text-muted, icon: "–")
  } else {
    // info (default)
    (bg: color-info-soft, border: color-info, icon: "ℹ")
  }
}

#let callout(data) = {
  let style = callout-style(data.callout_type)
  let is-inv = data.at("inverted", default: false)

  if is-inv {
    block(
      width: 100%,
      inset: spacing-4,
      radius: component-callout-radius,
      fill: style.border,
      breakable: false,
    )[
      #set text(fill: white)

      #if data.title != none [
        #text(weight: "bold")[#data.title]
        #v(spacing-2)
      ]

      #par(justify: true)[#text(size: font-size-base)[#data.content]]
    ]
  } else {
    block(
      width: 100%,
      inset: spacing-4,
      radius: component-callout-radius,
      fill: style.bg,
      stroke: (left: (paint: style.border, thickness: 3pt)),
      breakable: false,
    )[
      #set text(fill: color-text)

      #if data.title != none [
        #text(weight: "bold")[#data.title]
        #v(spacing-2)
      ]

      #par(justify: true)[#text(size: font-size-base)[#data.content]]
    ]
  }
}


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


// Section Component
// Per-level visual accent + orphan protection via phantom height.
//
// Level 2 — major section: primary left bar + soft background
// Level 3 — sub-section: thin primary left bar, no background
// Level 4+ — plain bold, muted color

#let section(data) = {
  let lv = data.level

  let heading-block = if lv == 2 {
    block(
      width: 100%,
      fill: color-surface-soft,
      stroke: (left: 3pt + color-primary),
      inset: (left: 10pt, right: 8pt, y: 6pt),
      radius: (right: 4pt),
    )[#heading(level: lv)[#data.title]]
  } else if lv == 3 {
    block(
      width: 100%,
      stroke: (left: 2pt + color-primary),
      inset: (left: 8pt, y: 3pt),
    )[#heading(level: lv)[#data.title]]
  } else {
    block(width: 100%)[#heading(level: lv)[#data.title]]
  }

  // Orphan protection: ghost height forces Typst to reserve ~2 lines of space
  // after the heading on the same page. Cancelled by v(-2em) below so the gap
  // is not visible in the rendered output.
  block(width: 100%, breakable: false, below: 0pt)[
    #heading-block
    #v(spacing-4)
    #box(height: 2em, width: 0pt)[]
  ]
  v(-2em)
}


// Report Content
#block(width: 100%, height: 100%, breakable: false)[
  #v(1fr)
  #block(width: 60pt, height: 4pt, fill: color-primary, radius: 2pt)
  #v(spacing-5)
  #block(width: 100%)[#set par(leading: 0.4em); #text(size: 36pt, weight: "bold", fill: color-text, tracking: -0.02em)[Snapshot Report]]
  #v(spacing-3)
  #text(size: 18pt, fill: color-text-muted)[]
  #v(1fr)
  #line(length: 100%, stroke: 0.5pt + color-border)
  #v(spacing-3)
  #text(size: font-size-xs, fill: color-text-muted)[#h(1fr)]
]
#pagebreak()

#block(breakable: false)[
  #section((content: (), level: 2, title: "Heading"))
  #v(spacing-3)
  #score-card((computed_status: "warning", description: none, good_threshold: 90, height: none, inverted: false, max_score: 100, score: 85, status: none, title: "Score", variant: "standard", warn_threshold: 50))
]

#v(spacing-4)

#callout((callout_type: "info", content: "Stable canonical content.", inverted: false, title: none, variant: "info"))

#v(spacing-4)

