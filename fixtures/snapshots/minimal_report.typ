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
  block(width: 100%, breakable: false, below: 0pt)[
    #content
    #v(spacing)
    #box(height: min-body, width: 0pt)[]
  ]
  v(-min-body)
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
#show heading.where(level: 1): set text(size: font-size-2xl, weight: "bold", fill: color-text)
#show heading.where(level: 2): set text(size: font-size-xl, weight: "bold", fill: color-text)
#show heading.where(level: 3): set text(size: font-size-lg, weight: "bold", fill: color-text)
#show heading.where(level: 4): set text(size: font-size-base, weight: "bold", fill: color-text-muted)


// Component Functions
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


// Finding Component
// Displays an audit finding with severity indicator

#let severity-color(severity) = {
  if severity == "critical" or severity == "high" {
    color-bad
  } else if severity == "medium" {
    color-warn
  } else {
    color-ok
  }
}

#let severity-label(severity) = {
  if severity == "critical" { "CRITICAL" }
  else if severity == "high" { "HIGH" }
  else if severity == "medium" { "MEDIUM" }
  else if severity == "low" { "LOW" }
  else { "INFO" }
}

#let finding(data) = {
  let sev-color = severity-color(data.severity)

  block(
    width: 100%,
    inset: spacing-4,
    radius: component-finding-radius,
    stroke: (left: (paint: sev-color, thickness: 3pt), top: (paint: color-border, thickness: component-card-border-width), right: (paint: color-border, thickness: component-card-border-width), bottom: (paint: color-border, thickness: component-card-border-width)),
    fill: color-surface,
  )[
    #set text(fill: color-text)

    // Header with severity badge
    #grid(
      columns: (auto, 1fr),
      gutter: spacing-3,
      badge-for-severity(data.severity),
      text(weight: "bold", size: font-size-lg)[#data.title]
    )

    #v(spacing-3)

    // Description
    #par(justify: true)[#text(size: font-size-base)[#data.description]]

    // Affected resource
    #if data.affected != none [
      #v(spacing-2)
      #label-text([Betroffen: ])
      #mono-text(data.affected)
    ]

    // Recommendation
    #if data.recommendation != none [
      #v(spacing-3)
      #block(
        width: 100%,
        inset: spacing-3,
        radius: 8pt,
        fill: color-ok-soft,
        stroke: (paint: color-ok, thickness: 0.5pt),
      )[
        #label-text([Recommendation])
        #v(spacing-2)
        #text(size: font-size-base)[#data.recommendation]
      ]
    ]
  ]
}


// Audit Table Component
// Displays tabular data for audit results

#let audit-table(data) = {
  let col-count = data.columns.len()
  
  // Build column widths
  let col-widths = data.columns.map(col => {
    if col.width != none { 
      eval(col.width) 
    } else { 
      1fr 
    }
  })
  
  block(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-lg)[#data.title])
    ]
    
    #set text(size: font-size-sm)
    
    #table(
      columns: col-widths,
      inset: spacing-3,
      stroke: (paint: table-border, thickness: table-border-width),
      fill: (x, y) => {
        if y == 0 { table-header-bg }
        else if data.striped and calc.odd(y) { table-row-alt-bg }
        else { none }
      },
      
      // Header row
      ..data.columns.map(col => {
        text(weight: "semibold", fill: color-text)[#col.header]
      }),
      
      // Data rows
      ..data.rows.flatten()
    )
  ]
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


// Image Component
// Image with optional caption

#let report-image(data) = {
  let img-width = if data.width != none { eval(data.width) } else { 100% }

  align(center)[
    #figure(
      image(data.src, width: img-width, alt: data.alt),
      caption: if data.caption != none { data.caption } else { none },
    )
  ]
}


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
        #text(weight: "bold")[#style.icon #data.title]
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
        #text(weight: "bold")[#style.icon #data.title]
        #v(spacing-2)
      ]

      #par(justify: true)[#text(size: font-size-base)[#data.content]]
    ]
  }
}


// Summary Box Component
// Executive summary widget with key-value pairs

#let status-indicator(status) = {
  if status == none { none }
  else {
    let color = if status == "good" { color-ok }
      else if status == "warning" { color-warn }
      else { color-bad }
    box(
      width: 8pt,
      height: 8pt,
      radius: 50%,
      fill: color,
    )
  }
}

#let summary-box(data) = {
  theme-card[
    #text(weight: "bold", size: font-size-xl, fill: color-text)[#data.title]

    #v(spacing-4)

    #for item in data.items [
      #grid(
        columns: (auto, 1fr, auto),
        gutter: spacing-2,
        align: (left, left, right),

        status-indicator(item.status),
        text(fill: color-text-muted, size: font-size-base)[#item.label],
        text(weight: "bold", size: font-size-base)[#item.value],
      )
      #v(spacing-2)
    ]
  ]
}


// List Component
// Inspired by JasperReports List Component

#let render-list-item(item, level: 0, numbered: false, index: 0) = {
  let indent = spacing-3 * level
  
  [
    #h(indent)
    #if numbered [
      #text(weight: "semibold")[#(index + 1).] #h(spacing-2)
    ] else if item.icon != none [
      #text(fill: color-primary)[#item.icon] #h(spacing-2)
    ] else [
      #text(fill: color-primary)[•] #h(spacing-2)
    ]
    #item.content
    #v(spacing-2)
    
    #if "children" in item and item.children.len() > 0 [
      #for (idx, child) in item.children.enumerate() [
        #render-list-item(child, level: level + 1, numbered: numbered, index: idx)
      ]
    ]
  ]
}

#let list(data) = {
  box(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-base)[#data.title])
    ]
    
    #if data.layout == "grid" [
      #let cols = (1fr,) * data.columns
      #grid(
        columns: cols,
        gutter: spacing-4,
        ..data.items.map(item => [
          #if item.icon != none [
            #text(fill: color-primary)[#item.icon] #h(spacing-2)
          ]
          #item.content
        ])
      )
    ] else [
      #for (idx, item) in data.items.enumerate() [
        #render-list-item(item, numbered: data.numbered, index: idx)
      ]
    ]
  ]
}


// Divider Component
// Inspired by BIRT Band Elements

#let divider(data) = {
  let line-color = if data.color != none {
    rgb(data.color)
  } else {
    color-border
  }
  
  let line-style = if data.style == "dashed" {
    (dash: "dashed")
  } else if data.style == "dotted" {
    (dash: "dotted")
  } else if data.style == "double" {
    (thickness: 2pt)
  } else {
    (:)
  }
  
  v(eval(data.spacing_above))
  line(
    length: 100%,
    stroke: (paint: line-color, thickness: eval(data.thickness)) + line-style
  )
  v(eval(data.spacing_below))
}


// Progress Bar Component
// Inspired by JasperReports Chart Components

#let progress-bar(data) = {
  let bar-color = if data.color != none {
    rgb(data.color)
  } else {
    color-primary
  }
  
  let percentage = data.percentage
  
  box(width: 100%)[
    // Label and percentage
    #grid(
      columns: (1fr, auto),
      text(size: font-size-sm)[#data.label],
      if data.show_percentage [
        #text(size: font-size-sm, weight: "semibold")[#calc.round(percentage, digits: 1)%]
      ]
    )
    
    #v(spacing-2)
    
    // Progress bar
    #box(
      width: 100%,
      height: 16pt,
      radius: 8pt,
      fill: color-surface,
      stroke: (paint: color-border, thickness: 0.5pt),
    )[
      #place(
        left + horizon,
        box(
          width: percentage * 1%,
          height: 100%,
          radius: 8pt,
          fill: bar-color,
        )
      )
      #if percentage >= 20 [
        #place(left + horizon, dx: 8pt,
          text(size: 7pt, weight: "bold", fill: white)[#calc.round(percentage, digits: 0)%]
        )
      ]
    ]
    
    #v(spacing-3)
  ]
}


// Key-Value List Component
// Inspired by BIRT Parameter Elements

#let key-value-list(data) = {
  box(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-base)[#data.title])
    ]
    
    #if data.layout == "horizontal" [
      #for item in data.items [
        #box(
          inset: spacing-2,
          fill: if item.highlight { color-surface } else { none },
        )[
          #text(weight: "semibold", size: font-size-sm)[#item.key:]
          #h(spacing-2)
          #text(size: font-size-sm)[#item.value]
        ]
        #h(spacing-4)
      ]
    ] else [
      #grid(
        columns: (auto, 1fr),
        column-gutter: spacing-3,
        row-gutter: spacing-2,
        align: (left, left),

        ..data.items.map(item => (
          box(
            inset: (x: spacing-2, y: 2pt),
            fill: if item.highlight { color-surface } else { none },
          )[
            #text(weight: "semibold", size: font-size-sm, fill: color-text-muted)[#item.key]
          ],
          box(
            inset: (x: spacing-2, y: 2pt),
            fill: if item.highlight { color-surface } else { none },
          )[
            #text(size: font-size-sm)[#item.value]
          ],
        )).flatten()
      )
    ]
  ]
}


// Watermark Component
// Inspired by Pentaho Watermark Band

#let watermark(data) = {
  place(
    center + horizon,
    rotate(
      data.rotation * 1deg,
      text(
        size: eval(data.size),
        weight: "bold",
        fill: color-text.transparentize(100% - data.opacity * 100%),
      )[#data.text]
    )
  )
}


// Page Break Component
// Inspired by BIRT Page Setup

#let page-break(data) = {
  pagebreak()
}


// Chart component template
#let chart(data) = {
  let title = data.title
  let chart_type = data.chart_type
  let series = data.series
  let x_label = data.at("x_label", default: none)
  let y_label = data.at("y_label", default: none)
  let show_legend = data.at("show_legend", default: true)
  let width = data.at("width", default: "100%")
  let height = data.at("height", default: "200pt")

  // Color palette for multiple series/data points
  let colors = (
    rgb("#3b82f6"),
    rgb("#22c55e"),
    rgb("#f59e0b"),
    rgb("#ef4444"),
    rgb("#8b5cf6"),
    rgb("#06b6d4"),
    rgb("#ec4899"),
    rgb("#f97316"),
  )

  block(
    width: 100%,
    breakable: false,
    {
      // Title
      if title != "" {
        align(center, text(weight: "bold", size: 12pt, title))
        v(8pt)
      }

      if chart_type == "bar" {
        // Bar chart — render actual data points from first series
        let points = if series.len() > 0 { series.at(0).data } else { () }
        let max_val = if points.len() > 0 {
          calc.max(..points.map(p => p.value), 1)
        } else { 100 }

        rect(
          width: 100%,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 16pt,
          {
            if y_label != none {
              align(left, text(size: 8pt, fill: color-text-muted, y_label))
              v(4pt)
            }

            let chart_height = 140
            grid(
              columns: (1fr,) * points.len(),
              column-gutter: 12pt,
              ..points.enumerate().map(((i, p)) => {
                let bar_height = calc.max(p.value / max_val * chart_height, 4)
                align(center, {
                  text(size: 8pt, weight: "bold", fill: color-text, str(calc.round(p.value)))
                  v(4pt)
                  rect(
                    width: 100%,
                    height: bar_height * 1pt,
                    fill: colors.at(calc.rem(i, colors.len())),
                    radius: (top: 3pt),
                  )
                  v(6pt)
                  text(size: 7pt, fill: color-text-muted, p.label)
                })
              })
            )

            if x_label != none {
              v(8pt)
              align(center, text(size: 8pt, fill: color-text-muted, x_label))
            }
          }
        )
      } else if chart_type == "pie" {
        // Pie chart — show data as colored segments legend (actual circle drawing is limited in Typst)
        let points = if series.len() > 0 { series.at(0).data } else { () }
        let total = if points.len() > 0 {
          points.map(p => p.value).sum()
        } else { 1 }

        rect(
          width: 100%,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 16pt,
          {
            if points.len() > 0 {
              // Stacked horizontal bar as pie chart approximation
              let bar_width = 100
              stack(
                dir: ltr,
                ..points.enumerate().map(((i, p)) => {
                  let segment_width = calc.max(p.value / total * bar_width, 2)
                  rect(
                    width: segment_width * 1%,
                    height: 32pt,
                    fill: colors.at(calc.rem(i, colors.len())),
                  )
                })
              )
              v(12pt)
              // Legend with values
              grid(
                columns: (auto,) * calc.min(points.len(), 3),
                column-gutter: 16pt,
                row-gutter: 6pt,
                ..points.enumerate().map(((i, p)) => {
                  let pct = calc.round(p.value / total * 100)
                  [
                    #box(width: 8pt, height: 8pt, fill: colors.at(calc.rem(i, colors.len())), radius: 1pt)
                    #h(4pt)
                    #text(size: 8pt)[#p.label: #str(calc.round(p.value)) (#str(pct)%)]
                  ]
                })
              )
            }
          }
        )
      } else if chart_type == "line" or chart_type == "area" {
        // Line/Area chart — data-driven with dots and connecting lines
        let all_values = series.map(s => s.data.map(p => p.value)).flatten()
        let max_val = if all_values.len() > 0 { calc.max(..all_values, 1) } else { 100 }
        let min_val = if all_values.len() > 0 { calc.min(..all_values, 0) } else { 0 }
        let val_range = if max_val > min_val { max_val - min_val } else { 1 }
        let labels = if series.len() > 0 { series.at(0).data.map(p => p.label) } else { () }
        let chart_h = 140
        let chart_w_pt = 100 // percentage

        rect(
          width: 100%,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 16pt,
          {
            if y_label != none {
              align(left, text(size: 8pt, fill: color-text-muted, y_label))
              v(4pt)
            }

            // Chart area with placed elements
            let n_points = if labels.len() > 0 { labels.len() } else { 1 }

            for (si, s) in series.enumerate() {
              let color = colors.at(calc.rem(si, colors.len()))
              let points_data = s.data

              if points_data.len() > 1 {
                // Render columns with bars for area, dots for line
                grid(
                  columns: (1fr,) * points_data.len(),
                  column-gutter: 0pt,
                  ..points_data.enumerate().map(((i, p)) => {
                    let bar_height = calc.max((p.value - min_val) / val_range * chart_h, 2)
                    align(center, {
                      // Value label
                      text(size: 7pt, weight: "bold", fill: color-text, str(calc.round(p.value)))
                      v(2pt)
                      // Spacer to push down (inverse height)
                      v((chart_h - bar_height) * 1pt)
                      if chart_type == "area" {
                        // Filled area bar
                        rect(
                          width: 100%,
                          height: bar_height * 1pt,
                          fill: color.lighten(60%),
                          stroke: (top: 2.5pt + color),
                        )
                      } else {
                        // Line chart: thin bar with dot on top
                        rect(
                          width: 100%,
                          height: bar_height * 1pt,
                          fill: none,
                          stroke: none,
                          {
                            place(center + top, circle(radius: 4pt, fill: color, stroke: 1pt + color-background))
                            place(center + top, line(
                              length: 100%,
                              stroke: 2pt + color,
                            ))
                          }
                        )
                      }
                    })
                  })
                )

                // X-axis labels
                grid(
                  columns: (1fr,) * points_data.len(),
                  column-gutter: 0pt,
                  ..points_data.map(p => {
                    align(center, text(size: 7pt, fill: color-text-muted, p.label))
                  })
                )
              }
            }

            if x_label != none {
              v(8pt)
              align(center, text(size: 8pt, fill: color-text-muted, x_label))
            }
          }
        )
      } else if chart_type == "scatter" {
        // Scatter chart — dots positioned by value
        let all_values = series.map(s => s.data.map(p => p.value)).flatten()
        let max_val = if all_values.len() > 0 { calc.max(..all_values, 1) } else { 100 }
        let min_val = if all_values.len() > 0 { calc.min(..all_values, 0) } else { 0 }
        let val_range = if max_val > min_val { max_val - min_val } else { 1 }
        let chart_h = 140

        rect(
          width: 100%,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 16pt,
          {
            if y_label != none {
              align(left, text(size: 8pt, fill: color-text-muted, y_label))
              v(4pt)
            }

            for (si, s) in series.enumerate() {
              let color = colors.at(calc.rem(si, colors.len()))
              let points_data = s.data

              if points_data.len() > 0 {
                grid(
                  columns: (1fr,) * points_data.len(),
                  column-gutter: 4pt,
                  ..points_data.enumerate().map(((i, p)) => {
                    let dot_y = calc.max((p.value - min_val) / val_range * chart_h, 4)
                    align(center, {
                      text(size: 6pt, fill: color-text-muted, str(calc.round(p.value)))
                      v(2pt)
                      v((chart_h - dot_y) * 1pt)
                      circle(radius: 5pt, fill: color, stroke: 1.5pt + color-background)
                      v(dot_y * 1pt - 10pt)
                    })
                  })
                )

                // X-axis labels
                grid(
                  columns: (1fr,) * points_data.len(),
                  column-gutter: 4pt,
                  ..points_data.map(p => {
                    align(center, text(size: 6pt, fill: color-text-muted, p.label))
                  })
                )
              }
            }

            if x_label != none {
              v(8pt)
              align(center, text(size: 8pt, fill: color-text-muted, x_label))
            }
          }
        )
      } else if chart_type == "radar" {
        // Radar chart — horizontal bar comparison per axis
        let labels = if series.len() > 0 { series.at(0).data.map(p => p.label) } else { () }
        let all_values = series.map(s => s.data.map(p => p.value)).flatten()
        let max_val = if all_values.len() > 0 { calc.max(..all_values, 1) } else { 100 }

        rect(
          width: 100%,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          fill: color-surface,
          inset: 16pt,
          {
            for (li, label) in labels.enumerate() {
              // Axis label + value summary on same line
              for (si, s) in series.enumerate() {
                let color = colors.at(calc.rem(si, colors.len()))
                let val = if li < s.data.len() { s.data.at(li).value } else { 0 }
                let pct = calc.min(val / max_val * 100, 100)
                let val-color = if val >= 85 { rgb("#22c55e") } else if val >= 70 { rgb("#3b82f6") } else if val >= 50 { rgb("#f59e0b") } else { rgb("#ef4444") }

                grid(
                  columns: (120pt, 1fr, 36pt),
                  gutter: 8pt,
                  align: (left + horizon, left + horizon, right + horizon),
                  text(size: 9pt, weight: "semibold", fill: color-text, label),
                  box(
                    width: 100%,
                    height: 18pt,
                    radius: 4pt,
                    fill: color-surface-alt,
                    stroke: 0.5pt + color-border,
                  )[
                    #place(left + horizon,
                      box(width: pct * 1%, height: 100%, radius: 4pt, fill: val-color.lighten(20%))
                    )
                    #if pct >= 15 [
                      #place(left + horizon, dx: 6pt,
                        text(size: 7pt, weight: "bold", fill: val-color.darken(30%))[#str(calc.round(val))]
                      )
                    ]
                  ],
                  text(size: 9pt, weight: "bold", fill: val-color)[#str(calc.round(val))],
                )
                v(5pt)
              }
            }
          }
        )
      } else {
        rect(
          width: 100%,
          height: eval(height),
          stroke: 0.5pt + gray,
          radius: 4pt,
          fill: color-surface,
          inset: 12pt,
          align(center + horizon, text(size: 9pt, fill: gray, [Chart: ] + chart_type))
        )
      }

      // Legend for multi-series
      if show_legend and series.len() > 1 {
        v(8pt)
        grid(
          columns: (auto,) * calc.min(series.len(), 4),
          column-gutter: 12pt,
          ..series.enumerate().map(((i, s)) => [
            #box(width: 8pt, height: 8pt, fill: colors.at(calc.rem(i, colors.len())), radius: 1pt)
            #h(4pt)
            #text(size: 9pt, s.name)
          ])
        )
      }
    }
  )
}


// Sparkline component template
#let sparkline(data) = {
  let sparkline_type = data.sparkline_type
  let values = data.data
  let width = data.at("width", default: "80pt")
  let height = data.at("height", default: "20pt")
  let color = data.at("color", default: none)
  let stroke_color = if color != none { rgb(color) } else { rgb("#3b82f6") }
  
  box(
    width: eval(width),
    height: eval(height),
    baseline: 25%,
    {
      if sparkline_type == "bar" {
        // Mini bar chart
        let max_val = calc.max(..values)
        let bar_width = eval(width) / values.len()
        
        grid(
          columns: values.len(),
          column-gutter: 0pt,
          ..values.map(v => {
            let bar_height = if max_val > 0 { (v / max_val) * eval(height) } else { 0pt }
            align(bottom, rect(
              width: bar_width - 1pt,
              height: bar_height,
              fill: stroke_color,
              stroke: none
            ))
          })
        )
      } else {
        // Line sparkline (simplified)
        rect(
          width: 100%,
          height: 100%,
          stroke: 1pt + stroke_color,
          fill: none,
          radius: 2pt
        )
      }
    }
  )
}


// Gauge component template
#let gauge(data) = {
  let label = data.label
  let value = data.value
  let min_val = data.at("min", default: 0.0)
  let max_val = data.at("max", default: 100.0)
  let thresholds = data.at("thresholds", default: ())
  let style = data.at("style", default: "circular")

  // Calculate percentage
  let pct = if max_val > min_val {
    ((value - min_val) / (max_val - min_val)) * 100
  } else {
    0
  }

  // Determine color based on thresholds
  let gauge_color = rgb("#3b82f6")
  for threshold in thresholds {
    if value >= threshold.value {
      gauge_color = rgb(threshold.color)
    }
  }

  block(
    width: 100%,
    breakable: false,
    {
      if style == "circular" {
        // Circular gauge
        align(center, box(width: 84pt, height: 84pt, {
          place(center + horizon, circle(
            radius: 40pt,
            stroke: 3pt + color-border,
            fill: none
          ))
          place(center + horizon, circle(
            radius: 35pt,
            stroke: 5pt + gauge_color,
            fill: none
          ))
          place(center + horizon,
            text(weight: "bold", size: 16pt, fill: color-text, str(calc.round(value, digits: 1)))
          )
        }))
      } else if style == "vertical" {
        // Thermometer style
        align(center, {
          rect(
            width: 30pt,
            height: 120pt,
            stroke: 1pt + color-border,
            fill: color-surface,
            radius: 15pt,
            {
              place(
                bottom,
                rect(
                  width: 100%,
                  height: pct * 1%,
                  fill: gauge_color,
                  stroke: none,
                  radius: (bottom: 15pt)
                )
              )
            }
          )
        })
      } else {
        // Horizontal bar
        rect(
          width: 200pt,
          height: 20pt,
          stroke: 1pt + color-border,
          fill: color-surface,
          radius: 10pt,
          {
            place(
              left,
              rect(
                width: pct * 1%,
                height: 100%,
                fill: gauge_color,
                stroke: none,
                radius: (left: 10pt)
              )
            )
          }
        )
      }

      v(8pt)
      align(center, text(size: 10pt, weight: "medium", fill: color-text, label))
      align(center, text(size: 9pt, fill: color-text-muted, str(value) + " / " + str(max_val)))
    }
  )
}


// Crosstab/Pivot Table component template
#let crosstab(data) = {
  let title = data.at("title", default: none)
  let row_dim = data.row_dimension
  let col_dim = data.column_dimension
  let measure = data.measure
  let aggregation = data.at("aggregation", default: "sum")
  let show_row_totals = data.at("show_row_totals", default: true)
  let show_col_totals = data.at("show_column_totals", default: true)
  let show_grand = data.at("show_grand_total", default: true)
  
  block(
    width: 100%,
    breakable: false,
    {
      if title != none {
        text(weight: "bold", size: 11pt, title)
        v(6pt)
      }
      
      // Simplified crosstab visualization
      table(
        columns: 4,
        stroke: 0.5pt + gray,
        fill: (x, y) => {
          if y == 0 or x == 0 { gray.lighten(70%) }
          else { white }
        },
        
        // Header row
        table.header(
          [],
          [#text(weight: "bold", size: 9pt, "Col 1")],
          [#text(weight: "bold", size: 9pt, "Col 2")],
          if show_row_totals [#text(weight: "bold", size: 9pt, "Total")] else []
        ),
        
        // Data rows
        [#text(weight: "bold", size: 9pt, "Row 1")], [100], [150], if show_row_totals [250] else [],
        [#text(weight: "bold", size: 9pt, "Row 2")], [200], [180], if show_row_totals [380] else [],
        
        // Totals row
        ..if show_col_totals {
          ([#text(weight: "bold", size: 9pt, "Total")], [300], [330],
           if show_grand [#text(weight: "bold", "630")] else [])
        } else { () }
      )
      
      v(6pt)
      text(size: 8pt, fill: gray, [Aggregation: #aggregation of #measure by #row_dim × #col_dim])
    }
  )
}


// Pivot Table component template
#let pivot-table(data) = {
  let title = data.at("title", default: none)
  let row_headers = data.row_headers
  let col_headers = data.column_headers
  let values = data.values
  let show_borders = data.at("show_borders", default: true)
  
  let stroke_val = if show_borders { 0.5pt + gray } else { none }
  let num_cols = col_headers.len() + 1
  
  block(
    width: 100%,
    breakable: true,
    {
      if title != none {
        text(weight: "bold", size: 11pt, title)
        v(6pt)
      }
      
      table(
        columns: num_cols,
        stroke: stroke_val,
        fill: (x, y) => {
          if y == 0 or x == 0 { gray.lighten(70%) }
          else { white }
        },
        
        // Build header cells
        text(weight: "bold", size: 9pt, ""),
        ..col_headers.map(h => text(weight: "bold", size: 9pt, h)),
        
        // Build data rows - must be flat array
        ..{
          let cells = ()
          for i in range(row_headers.len()) {
            // Add row header
            cells.push(text(weight: "bold", size: 9pt, row_headers.at(i)))
            // Add row values
            for v in values.at(i) {
              cells.push(text(size: 9pt, v))
            }
          }
          cells
        }
      )
    }
  )
}


// Barcode component template – renders real encoded patterns
#let barcode(data) = {
  let barcode_data = data.data
  let format = data.format
  let width = data.at("width", default: "150pt")
  let height = data.at("height", default: "50pt")
  let show_text = data.at("show_text", default: true)

  align(center, block(
    width: 100%,
    breakable: false,
    {
      if data.at("encoding_2d", default: none) != none {
        // 2D barcode (QR Code / Data Matrix) – draw module matrix
        let matrix = data.encoding_2d
        let cols = data.at("qr_width", default: 21)
        let rows = data.at("qr_height", default: cols)
        let target_w = eval(width)
        // Quiet zone: 1 module on each side
        let quiet = 2
        let module_size = calc.min(target_w / (cols + quiet), target_w / (rows + quiet))
        let padding = module_size
        let render_w = cols * module_size + 2 * padding
        let render_h = rows * module_size + 2 * padding

        box(
          width: render_w,
          height: render_h,
          {
            for (row_idx, row) in matrix.enumerate() {
              for (col_idx, cell) in row.enumerate() {
                if cell == 1 {
                  place(
                    dx: padding + col_idx * module_size,
                    dy: padding + row_idx * module_size,
                    rect(
                      width: module_size,
                      height: module_size,
                      fill: black,
                      stroke: none,
                    )
                  )
                }
              }
            }
          }
        )
      } else if data.at("encoding", default: none) != none {
        // 1D barcode – draw actual bar pattern
        let bars = data.encoding
        let bar_count = bars.len()
        let target_width = eval(width)
        let target_height = eval(height)
        let bar_width = target_width / bar_count

        box(
          width: target_width,
          height: target_height,
          clip: true,
          {
            for (i, bar) in bars.enumerate() {
              if bar == 1 {
                place(
                  dx: i * bar_width,
                  rect(
                    width: bar_width + 0.3pt,
                    height: target_height,
                    fill: black,
                    stroke: none,
                  )
                )
              }
            }
          }
        )
      } else if data.at("unsupported", default: false) {
        // Unsupported 2D format – placeholder with notice
        rect(
          width: eval(width),
          height: eval(width),
          fill: luma(240),
          stroke: 1pt + gray,
          align(center + horizon, text(size: 8pt, fill: gray, [#format encoding\ not yet supported]))
        )
      } else {
        // Fallback placeholder
        rect(
          width: eval(width),
          height: eval(height),
          fill: luma(240),
          stroke: 1pt + gray,
          align(center + horizon, text(size: 8pt, fill: gray, [Encoding failed]))
        )
      }

      // Show text below barcode
      if show_text {
        v(4pt)
        text(size: 8pt, font: "Courier New", barcode_data)
      }

      v(4pt)
      text(size: 7pt, fill: gray, format)
    }
  ))
}


// Label component template
#let label(data) = {
  let content = data.text
  let font_size = data.at("font_size", default: none)
  let font_weight = data.at("font_weight", default: none)
  let color = data.at("color", default: none)
  let align_val = data.at("align", default: none)
  
  let size_val = if font_size != none { eval(font_size) } else { 10pt }
  let color_val = if color != none { rgb(color) } else { black }
  let weight_val = if font_weight != none { font_weight } else { "regular" }
  let align_fn = if align_val == "center" { center } 
                 else if align_val == "right" { right }
                 else { left }
  
  align(align_fn, text(
    size: size_val,
    weight: weight_val,
    fill: color_val,
    content
  ))
}


// Text component template (renamed to textblock to avoid conflict with built-in text())
#let textblock(data) = {
  let content = data.content
  let font_size = data.at("font_size", default: none)
  let line_height = data.at("line_height", default: none)
  let align_val = data.at("align", default: "left")
  let max_width = data.at("max_width", default: none)
  
  let size_val = if font_size != none { eval(font_size) } else { 10pt }
  let leading_val = if line_height != none {
    let v = eval(line_height)
    if type(v) == length { v } else { v * 1em }
  } else { 0.65em }
  let align_fn = if align_val == "center" { center }
                 else if align_val == "right" { right }
                 else { left }
  let width_val = if max_width != none { eval(max_width) } else { 100% }
  
  block(
    width: width_val,
    {
      set par(leading: leading_val, justify: true)
      align(align_fn, text(size: size_val, content))
    }
  )
}


// Number Field component template
#let number-field(data) = {
  let value = data.value
  let format = data.at("format", default: none)
  let prefix = data.at("prefix", default: none)
  let suffix = data.at("suffix", default: none)
  let font_size = data.at("font_size", default: none)
  let color = data.at("color", default: none)
  
  let size_val = if font_size != none { eval(font_size) } else { 10pt }
  let color_val = if color != none { rgb(color) } else { black }
  
  // Format number (simplified)
  let formatted = if format != none {
    // Basic number formatting
    if format == "#,##0.00" or format == "#,###.##" {
      str(calc.round(value, digits: 2))
    } else if format == "0.0" {
      str(calc.round(value, digits: 1))
    } else {
      str(value)
    }
  } else {
    str(value)
  }
  
  let display = ""
  if prefix != none { display = prefix + " " }
  display = display + formatted
  if suffix != none { display = display + " " + suffix }
  
  text(size: size_val, fill: color_val, display)
}


// Date Field component template
#let date-field(data) = {
  let date = data.date
  let format = data.at("format", default: "YYYY-MM-DD")
  let font_size = data.at("font_size", default: none)
  
  let size_val = if font_size != none { eval(font_size) } else { 10pt }
  
  // Note: In production, this would parse ISO date and format accordingly
  // For now, display the date string as-is
  text(size: size_val, date)
}


// Resource Field component template (for localization)
#let resource-field(data) = {
  let key = data.key
  let default_value = data.at("default_value", default: none)
  let locale = data.at("locale", default: none)
  
  // In production, this would lookup the key in a resource bundle
  // For now, display the default value or the key itself
  let display = if default_value != none {
    default_value
  } else {
    "[" + key + "]"
  }
  
  text(size: 10pt, display)
  
  if locale != none {
    h(4pt)
    text(size: 7pt, fill: gray, "(" + locale + ")")
  }
}


// SideLabel Component
// Left 1/3: section heading, Right 2/3: content (bullets or text)

#let side-label(data) = {
  let subheading = data.at("subheading", default: none)
  let text-val = data.at("text", default: none)
  block(
    width: 100%,
    breakable: false,
    {
      grid(
        columns: (1fr, 2fr),
        column-gutter: spacing-5,
        align: (top, top),

        // ── Left: heading ──────────────────────────────────────────
        pad(top: 1pt)[
          #text(weight: "bold", size: font-size-sm, fill: color-text)[#data.heading]
          #if subheading != none [
            #v(2pt)
            #text(size: font-size-xs, fill: color-text-muted)[#subheading]
          ]
        ],

        // ── Right: items or text ───────────────────────────────────
        box(width: 100%)[
          #if data.items.len() > 0 [
            #for (i, item) in data.items.enumerate() [
              #grid(
                columns: (10pt, 1fr),
                gutter: 4pt,
                align: (top, top),
                text(size: font-size-sm, fill: color-primary)[•],
                text(size: font-size-sm, fill: color-text)[#item],
              )
              #if i < data.items.len() - 1 [ #v(3pt) ]
            ]
          ] else if text-val != none [
            #text(size: font-size-sm, fill: color-text)[#text-val]
          ]
        ]
      )

      // Optional bottom divider
      if data.divider [
        #v(spacing-2)
        #line(length: 100%, stroke: 0.5pt + color-border)
        #v(spacing-1)
      ] else [
        #v(spacing-3)
      ]
    }
  )
}


#let table-of-contents(data) = {
  heading(level: 1)[#data.title]
  v(spacing-3)
  {
    let font-size = if "font_size" in data and data.font_size != none { eval(data.font_size) } else { font-size-sm }
    set text(size: font-size)
    set par(leading: 1.35em)
    show outline.entry: set text(size: font-size)
    outline(
      title: none,
      indent: 1.2em,
      depth: data.depth,
    )
  }
  pagebreak()
}


// Metric Card Component
// Compact KPI display with accent color

#let metric-card(data) = {
  let accent = if data.accent_color != none { rgb(data.accent_color) } else { color-primary }
  let body = [
    #label-text(data.title)
    #v(spacing-2)
    #text(size: font-size-2xl, weight: "bold", fill: accent)[#data.value]
    #if data.subtitle != none [
      #v(spacing-2)
      #small-text(data.subtitle)
    ]
  ]

  if data.height != none {
    block(
      width: 100%,
      height: eval(data.height),
      fill: color-surface,
      stroke: (paint: color-border, thickness: component-card-border-width),
      radius: 10pt,
      inset: (x: spacing-4, y: spacing-4),
    )[ #body ]
  } else {
    theme-card[#body]
  }
}


// Hero Summary Component

#let hero-summary(data) = {
  let status-color = if data.computed_status == "good" {
    color-ok
  } else if data.computed_status == "warning" {
    color-warn
  } else {
    color-bad
  }

  // ── Score + Domain header ──────────────────────────────────
  grid(
    columns: (1.1fr, 1.4fr),
    column-gutter: spacing-5,

    // Left: Score card
    theme-card[
      #align(center)[
        #small-text(data.domain)
        #v(spacing-3)
        #text(size: 52pt, weight: "bold", fill: status-color)[#data.score]
        #v(spacing-2)
        #text(size: font-size-lg, weight: "bold", fill: status-color)[Grade #data.grade]
        #v(spacing-3)
        #theme-progress-bar(data.score, bar-color: status-color)
      ]
    ],

    // Right: Verdict
    theme-card[
      #text(size: font-size-2xl, weight: "bold")[Kurzfazit]
      #v(spacing-3)
      #text(size: font-size-base)[#data.verdict]
    ],
  )

  v(spacing-5)

  // ── KPI Metric cards ───────────────────────────────────────
  if data.metrics.len() > 0 {
    let boxes = data.metrics.map(metric => {
      let accent = if metric.accent_color != none { rgb(metric.accent_color) } else { color-primary }
      theme-card[
        #label-text(metric.title)
        #v(spacing-2)
        #text(size: font-size-2xl, weight: "bold", fill: accent)[#metric.value]
      ]
    })

    grid(
      columns: (1fr,) * data.metrics.len(),
      column-gutter: spacing-3,
      ..boxes,
    )

    v(spacing-5)
  }

  // ── Top 3 Actions ──────────────────────────────────────────
  if data.top_actions.len() > 0 {
    text(size: font-size-xl, weight: "bold")[Top Actions]
    v(spacing-3)

    for (i, action) in data.top_actions.enumerate() {
      theme-card(fill: color-surface-soft)[
        #grid(
          columns: (auto, 1fr),
          gutter: spacing-3,
          text(size: font-size-lg, weight: "bold", fill: color-primary)[#{i + 1}],
          text(size: font-size-base)[#action],
        )
      ]
      v(spacing-2)
    }

    v(spacing-3)
  }

  // ── Positive aspects ───────────────────────────────────────
  if data.positive_aspects.len() > 0 {
    text(size: font-size-xl, weight: "bold", fill: color-ok)[Strengths]
    v(spacing-3)
    for aspect in data.positive_aspects {
      text(size: font-size-base)[#text(fill: color-ok)[✓ ]#aspect]
      v(spacing-2)
    }
  }
}


// ProductHero Component
// Hero page for product/project introduction — occupies full page

#let product-hero(data) = {
  // Full-page hero with vertical centering
  block(width: 100%, height: 100%, breakable: false)[
    #v(1fr)

    // Decorative accent line
    #block(width: 60pt, height: 4pt, fill: color-primary, radius: 2pt)

    #v(spacing-5)

    // Title — large, bold
    #text(size: 52pt, weight: "bold", fill: color-text, tracking: -0.02em)[#data.title]

    #v(spacing-3)

    // Subtitle — lighter, generous size
    #text(size: 20pt, fill: color-text-muted, weight: "regular")[#data.subtitle]

    // Tagline — small, muted, italic
    #if data.tagline != none [
      #v(spacing-4)
      #block(
        width: 75%,
      )[
        #text(size: font-size-base, fill: color-text-muted, style: "italic")[#data.tagline]
      ]
    ]

    #v(spacing-7)

    // Highlights — 2-column grid with left accent bar
    #if data.highlights.len() > 0 [
      #{
        let items = data.highlights.map(highlight => {
          block(
            width: 100%,
            fill: color-accent-soft,
            stroke: (left: (paint: color-primary, thickness: 3pt)),
            radius: (right: 5pt),
            inset: (x: spacing-4, y: spacing-3),
          )[
            #text(size: font-size-sm, fill: color-text)[#highlight]
          ]
        })
        grid(
          columns: (1fr, 1fr),
          row-gutter: spacing-3,
          column-gutter: spacing-4,
          ..items,
        )
      }
    ]

    #v(1fr)

    // Footer line with optional CTA
    #line(length: 100%, stroke: 0.5pt + color-border)
    #v(spacing-3)
    #{
      if data.cta_url != none and data.cta_label != none {
        text(size: font-size-xs, fill: color-text-muted)[
          #link(data.cta_url)[#text(fill: color-primary, weight: "semibold")[#data.cta_label]]
        ]
      }
    }
  ]
}


// CardDashboard Component

#let card-dashboard(data) = {
  if data.title != none {
    component-title(text(size: font-size-xl, weight: "bold")[#data.title])
  }

  let boxes = data.modules.map(module => {
    let status-color = if module.computed_status == "good" {
      color-ok
    } else if module.computed_status == "warning" {
      color-warn
    } else {
      color-bad
    }

    theme-card[
      #text(size: font-size-lg, weight: "bold")[#module.name]
      #v(spacing-2)
      #text(size: 22pt, weight: "bold", fill: status-color)[#module.score#text(size: font-size-sm, weight: "regular", fill: color-text-muted)[\/100]]
      #v(spacing-3)
      #theme-progress-bar(module.score, bar-color: status-color)
      #v(spacing-3)
      #small-text(module.interpretation)
    ]
  })

  grid(
    columns: (1fr,) * data.modules.len(),
    column-gutter: spacing-3,
    ..boxes,
  )
}


// RoadmapBlock Component

#let roadmap-block(data) = {
  let col-boxes = data.columns.map(column => {
    let accent = if column.accent_color != none { rgb(column.accent_color) } else { color-primary }

    box(width: 100%)[
      // Column header
      #box(
        width: 100%,
        inset: (x: spacing-4, y: spacing-3),
        radius: (top-left: 10pt, top-right: 10pt),
        fill: accent.lighten(90%),
        stroke: (paint: accent, thickness: 0.8pt),
      )[
        #text(size: font-size-lg, weight: "bold", fill: accent)[#column.title]
        #h(spacing-2)
        #small-text([#column.items.len() actions])
      ]

      #v(spacing-2)

      // Items
      #for item in column.items {
        theme-card(fill: color-surface-soft)[
          #text(size: font-size-base, weight: "bold")[#item.action]
          #v(spacing-2)
          #grid(
            columns: (auto, 1fr),
            row-gutter: spacing-1,
            column-gutter: spacing-3,
            label-text([Impact]), text(size: font-size-sm)[#item.priority],
            label-text([Effort]), text(size: font-size-sm)[#{if item.effort != none { item.effort } else { "—" }}],
            label-text([Role]), text(size: font-size-sm)[#item.role],
          )
          #if item.benefit != "" [
            #v(spacing-2)
            #small-text(item.benefit)
          ]
        ]
        v(spacing-2)
      }
    ]
  })

  grid(
    columns: (1fr,) * data.columns.len(),
    column-gutter: spacing-4,
    ..col-boxes,
  )
}


// Severity Overview Component
// Visual severity breakdown with cards and strip bar

#let severity-overview(data) = {
  if data.title != none {
    component-title(text(size: font-size-xl, weight: "bold")[#data.title])
  }

  // Severity cards row
  let total = data.critical + data.serious + data.moderate + data.minor

  grid(
    columns: (1fr, 1fr, 1fr, 1fr),
    column-gutter: spacing-3,

    // Critical
    block(
      width: 100%,
      fill: color-surface,
      stroke: (top: (paint: color-bad, thickness: 3pt), left: (paint: color-border, thickness: component-card-border-width), right: (paint: color-border, thickness: component-card-border-width), bottom: (paint: color-border, thickness: component-card-border-width)),
      radius: 10pt,
      inset: (x: spacing-3, y: spacing-3),
    )[
      #label-text([Kritisch])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-bad)[#data.critical]
    ],

    // Serious
    block(
      width: 100%,
      fill: color-surface,
      stroke: (top: (paint: color-bad, thickness: 3pt), left: (paint: color-border, thickness: component-card-border-width), right: (paint: color-border, thickness: component-card-border-width), bottom: (paint: color-border, thickness: component-card-border-width)),
      radius: 10pt,
      inset: (x: spacing-3, y: spacing-3),
    )[
      #label-text([Schwerwiegend])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-bad)[#data.serious]
    ],

    // Moderate
    block(
      width: 100%,
      fill: color-surface,
      stroke: (top: (paint: color-warn, thickness: 3pt), left: (paint: color-border, thickness: component-card-border-width), right: (paint: color-border, thickness: component-card-border-width), bottom: (paint: color-border, thickness: component-card-border-width)),
      radius: 10pt,
      inset: (x: spacing-3, y: spacing-3),
    )[
      #label-text([Moderat])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-warn)[#data.moderate]
    ],

    // Minor
    block(
      width: 100%,
      fill: color-surface,
      stroke: (top: (paint: color-ok, thickness: 3pt), left: (paint: color-border, thickness: component-card-border-width), right: (paint: color-border, thickness: component-card-border-width), bottom: (paint: color-border, thickness: component-card-border-width)),
      radius: 10pt,
      inset: (x: spacing-3, y: spacing-3),
    )[
      #label-text([Gering])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-ok)[#data.minor]
    ],
  )

  v(spacing-3)

  // Severity strip bar
  if total > 0 {
    let high-count = data.critical + data.serious
    theme-severity-strip(high-count, data.moderate, data.minor)
    v(spacing-2)
    small-text([#total Verstöße insgesamt])
  }
}


// Cover Page Component
// Professional report cover with score preview

#let cover-page(data) = {
  let status-color = if data.computed_status == "good" {
    color-ok
  } else if data.computed_status == "warning" {
    color-warn
  } else {
    color-bad
  }

  // ── Accent bar at top ─────────────────────────────────────────
  place(top + left, dx: -page-margin, dy: -page-margin-top,
    rect(width: 100% + 2 * page-margin, height: 6pt, fill: color-primary)
  )

  v(12mm)

  // ── Brand + Date header ───────────────────────────────────────
  grid(
    columns: (1fr, auto),
    gutter: spacing-3,
    text(size: font-size-base, weight: "bold", fill: color-primary)[#data.brand],
    text(size: font-size-sm, fill: color-text-muted)[#data.date],
  )

  v(8mm)
  line(length: 100%, stroke: 0.5pt + color-border)
  v(16mm)

  // ── Title block ───────────────────────────────────────────────
  text(size: 32pt, weight: "bold", fill: color-text, tracking: -0.5pt)[#data.title]
  v(spacing-3)
  text(size: 18pt, weight: "semibold", fill: color-primary)[#data.domain]
  v(spacing-4)
  text(size: font-size-base, fill: color-text-muted)[#data.subtitle]

  v(16mm)

  // ── Score card ────────────────────────────────────────────────
  block(
    width: 100%,
    fill: color-surface-soft,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 8pt,
    inset: spacing-5,
  )[
    #grid(
      columns: (1fr, 1fr, 1fr),
      gutter: spacing-5,
      // Score column
      [
        #text(size: font-size-xs, weight: "bold", fill: color-text-muted, tracking: 1pt)[SCORE]
        #v(spacing-2)
        #text(size: 48pt, weight: "bold", fill: status-color)[#data.score]
        #v(spacing-2)
        #theme-progress-bar(data.score, bar-color: status-color)
      ],
      // Grade column
      [
        #text(size: font-size-xs, weight: "bold", fill: color-text-muted, tracking: 1pt)[GRADE]
        #v(spacing-2)
        #text(size: 28pt, weight: "bold", fill: status-color)[#data.grade]
        #v(spacing-3)
        #text(size: font-size-sm, fill: color-text-muted)[von 100 Punkten]
      ],
      // Issues column
      [
        #text(size: font-size-xs, weight: "bold", fill: color-text-muted, tracking: 1pt)[ISSUES]
        #v(spacing-2)
        #text(size: 28pt, weight: "bold", fill: color-text)[#data.total_issues]
        #v(spacing-3)
        #if data.critical_issues > 0 [
          #box(
            fill: color-bad-soft,
            radius: 4pt,
            inset: (x: 6pt, y: 3pt),
            text(size: font-size-xs, weight: "bold", fill: color-bad)[#data.critical_issues critical]
          )
        ] else [
          #box(
            fill: color-ok-soft,
            radius: 4pt,
            inset: (x: 6pt, y: 3pt),
            text(size: font-size-xs, weight: "bold", fill: color-ok)[none critical]
          )
        ]
      ],
    )
  ]

  v(spacing-6)

  // ── Module tags ───────────────────────────────────────────────
  if data.modules.len() > 0 {
    stack(dir: ltr, spacing: 6pt,
      ..data.modules.map(m =>
        box(
          fill: color-surface,
          stroke: 0.5pt + color-border,
          radius: 4pt,
          inset: (x: 10pt, y: 5pt),
          text(size: font-size-xs, weight: "medium", fill: color-text-muted)[#m]
        )
      )
    )
  }

  // ── Bottom accent ─────────────────────────────────────────────
  place(bottom + left, dx: -page-margin, dy: page-margin-bottom,
    rect(width: 100% + 2 * page-margin, height: 3pt, fill: color-primary.lighten(60%))
  )

  pagebreak()
}


// Module Comparison Component
// Horizontal score comparison rows

#let module-comparison(data) = {
  if data.title != none {
    component-title(text(size: font-size-xl, weight: "bold")[#data.title], spacing: spacing-4)
  }

  theme-card[
    #for (i, module) in data.modules.enumerate() {
      let bar-color = if module.accent_color != none { rgb(module.accent_color) }
        else if module.computed_status == "good" { color-ok }
        else if module.computed_status == "warning" { color-warn }
        else { color-bad }

      grid(
        columns: (28mm, 1fr, 14mm),
        gutter: spacing-3,
        align: (left, left, right),
        text(size: font-size-base)[#module.name],
        theme-progress-bar(module.score, bar-color: bar-color),
        text(size: font-size-base, weight: "bold", fill: bar-color)[#module.score],
      )
      if i < data.modules.len() - 1 {
        v(spacing-3)
      }
    }
  ]
}


// PortfolioSummary Component
// Portfolio-level KPI cards for batch reports

#let portfolio-summary(data) = {
  let avg-color = if data.average_score >= 75 { color-ok }
    else if data.average_score >= 50 { color-warn }
    else { color-bad }

  grid(
    columns: (1fr, 1fr, 1fr, 1fr),
    column-gutter: spacing-3,

    // Total sites
    theme-card[
      #label-text([Sites Audited])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold")[#data.total_sites]
    ],

    // Average score
    theme-card[
      #label-text([Average])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: avg-color)[#data.average_score]
      #v(spacing-2)
      #theme-progress-bar(data.average_score, bar-color: avg-color)
    ],

    // Best
    theme-card[
      #label-text([Best Site])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-ok)[#data.best_score]
      #v(spacing-2)
      #small-text(data.best_domain)
    ],

    // Worst
    theme-card[
      #label-text([Weakest Site])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-bad)[#data.worst_score]
      #v(spacing-2)
      #small-text(data.worst_domain)
    ],
  )

  v(spacing-4)

  // Issue summary row
  grid(
    columns: (1fr, 1fr),
    column-gutter: spacing-3,

    theme-card[
      #label-text([Total Issues])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold")[#data.total_issues]
    ],

    theme-card[
      #label-text([Critical Issues])
      #v(spacing-2)
      #text(size: font-size-2xl, weight: "bold", fill: color-bad)[#data.critical_issues]
    ],
  )
}


// Benchmark Table Component
// Ranking table for website comparison

#let benchmark-table(data) = {
  if data.title != none {
    component-title(text(size: font-size-xl, weight: "bold")[#data.title], spacing: spacing-4)
  }

  // Header
  block(
    width: 100%,
    fill: color-surface-alt,
    radius: (top-left: 10pt, top-right: 10pt),
    inset: (x: spacing-4, y: spacing-3),
  )[
    #set text(size: font-size-xs, weight: "bold", fill: color-text-muted)
    #grid(
      columns: (8mm, 1fr, 16mm, 16mm, 16mm, 16mm, 16mm, 18mm),
      gutter: spacing-2,
      [Nr], [Domain], [Score], [A11y], [SEO], [Perf], [Sec], [Krit.],
    )
  ]

  // Rows
  for row in data.rows {
    let row-color = if row.computed_status == "good" { color-ok }
      else if row.computed_status == "warning" { color-warn }
      else { color-bad }

    block(
      width: 100%,
      inset: (x: spacing-4, y: spacing-3),
      stroke: (bottom: (paint: color-border, thickness: 0.5pt)),
    )[
      #grid(
        columns: (8mm, 1fr, 16mm, 16mm, 16mm, 16mm, 16mm, 18mm),
        gutter: spacing-2,
        text(size: font-size-sm, fill: color-text-muted)[#row.rank],
        text(size: font-size-sm, weight: "bold")[#row.domain],
        text(size: font-size-sm, weight: "bold", fill: row-color)[#row.score],
        text(size: font-size-sm)[#row.accessibility],
        {
          if row.seo != none { text(size: font-size-sm)[#row.seo] }
          else { text(size: font-size-sm, fill: color-text-muted)[—] }
        },
        {
          if row.performance != none { text(size: font-size-sm)[#row.performance] }
          else { text(size: font-size-sm, fill: color-text-muted)[—] }
        },
        {
          if row.security != none { text(size: font-size-sm)[#row.security] }
          else { text(size: font-size-sm, fill: color-text-muted)[—] }
        },
        {
          if row.critical_issues > 0 {
            text(size: font-size-sm, weight: "bold", fill: color-bad)[#row.critical_issues]
          } else {
            text(size: font-size-sm, fill: color-text-muted)[0]
          }
        },
      )
    ]
  }
}


// Eyebrow Component
// Small uppercase category label with letter-spacing

#let eyebrow(data) = {
  let fill-color = if data.color != none { rgb(data.color) } else { color-primary }
  let body-text = if data.uppercase { upper(data.text) } else { data.text }
  text(
    size: font-size-xs,
    weight: "bold",
    fill: fill-color,
    tracking: 0.08em,
    body-text,
  )
}


// Status Pill Component
// Colored badge pill using theme-badge()

#let status-pill(data) = {
  let body-text = if data.uppercase { upper(data.label) } else { data.label }
  if data.status == "good" {
    theme-badge(body-text, color-ok, color-ok-soft)
  } else if data.status == "warn" {
    theme-badge(body-text, color-warn, color-warn-soft)
  } else if data.status == "bad" {
    theme-badge(body-text, color-bad, color-bad-soft)
  } else if data.status == "info" {
    theme-badge(body-text, color-info, color-info-soft)
  } else {
    // neutral
    theme-badge(body-text, color-text-muted, color-surface-alt)
  }
}


// Tag Cloud Component
// Inline-wrapping pill badges — multiple status-pill boxes in one block.

#let tag-cloud(data) = {
  let gap = if "gap" in data and data.gap != none { eval(data.gap) } else { 4pt }

  block(width: 100%, inset: (y: 2pt))[
    #if "title" in data and data.title != none {
      text(size: font-size-sm, weight: "semibold", fill: color-text-muted, data.title)
      linebreak()
      v(4pt)
    }
    #for item in data.items {
      let lbl = item.label
      let st  = item.status
      if st == "good" {
        theme-badge(lbl, color-ok, color-ok-soft)
      } else if st == "warn" {
        theme-badge(lbl, color-warn, color-warn-soft)
      } else if st == "bad" {
        theme-badge(lbl, color-bad, color-bad-soft)
      } else if st == "info" {
        theme-badge(lbl, color-info, color-info-soft)
      } else {
        theme-badge(lbl, color-text-muted, color-surface-alt)
      }
      h(gap)
    }
  ]
}


// Stat Component
// Large value KPI with label on top, optional trend badge below

#let stat(data) = {
  let accent = if data.accent_color != none { rgb(data.accent_color) } else { color-primary }

  theme-card[
    #label-text(data.label)
    #v(spacing-2)
    #stack(
      dir: ltr,
      spacing: 4pt,
      text(size: font-size-2xl, weight: "bold", fill: accent)[#data.value],
      if data.unit != none [
        #pad(top: 4pt)[#small-text(data.unit)]
      ],
    )
    #if data.trend != none [
      #v(spacing-2)
      #{
        let is-pos = data.trend_positive
        let trend-color = if is-pos == true { color-ok } else if is-pos == false { color-bad } else { color-text-muted }
        let trend-bg   = if is-pos == true { color-ok-soft } else if is-pos == false { color-bad-soft } else { color-surface-alt }
        theme-badge(data.trend, trend-color, trend-bg)
      }
    ]
  ]
}


// StatPair Component
// Two stats side-by-side with optional vertical divider

#let stat-entry(entry) = {
  let accent = if entry.accent_color != none { rgb(entry.accent_color) } else { color-primary }
  stack(
    spacing: spacing-2,
    label-text(entry.label),
    stack(
      dir: ltr,
      spacing: 4pt,
      text(size: font-size-2xl, weight: "bold", fill: accent)[#entry.value],
      if entry.unit != none [
        #pad(top: 4pt)[#small-text(entry.unit)]
      ],
    ),
  )
}

#let stat-pair(data) = {
  theme-card[
    #grid(
      columns: if data.divider { (1fr, 0pt, 1fr) } else { (1fr, 1fr) },
      column-gutter: spacing-4,
      align: (top, center, top),

      stat-entry(data.left),

      if data.divider [
        #line(length: 100%, angle: 90deg, stroke: 0.5pt + color-border)
      ],

      stat-entry(data.right),
    )
  ]
}


// ScoreBand Component
// Three color-coded segments (bad/warn/good) with a triangle marker at value position

#let score-band(data) = {
  let val       = calc.min(100, calc.max(0, data.value))
  let good-t    = data.good_threshold
  let warn-t    = data.warn_threshold

  // Segment widths as percentages of total 100
  let bad-pct   = warn-t * 1%
  let warn-pct  = (good-t - warn-t) * 1%
  let good-pct  = (100 - good-t) * 1%

  // Marker position as fraction of total width
  let marker-pos = val * 1%

  block(width: 100%)[
    #if data.label != none [
      #label-text(data.label)
      #v(spacing-2)
    ]

    // Color bar
    #box(width: 100%, height: 28pt)[
      #grid(
        columns: (bad-pct, warn-pct, good-pct),
        gutter: 0pt,
        box(fill: color-bad,  height: 28pt, width: 100%, radius: (left: 4pt)),
        box(fill: color-warn, height: 28pt, width: 100%),
        box(fill: color-ok,   height: 28pt, width: 100%, radius: (right: 4pt)),
      )
      // Triangle marker
      #place(
        left + horizon,
        dx: marker-pos - 5pt,
        polygon(
          fill: color-text,
          (0pt, 0pt),
          (10pt, 0pt),
          (5pt, 10pt),
        )
      )
    ]

    #if data.show_value [
      #v(spacing-1)
      #align(center)[#text(size: font-size-sm, weight: "bold", fill: color-text)[#val]]
    ]
  ]
}


// TrendTile Component
// Compact card: label, large delta with direction arrow, optional reference

#let trend-tile(data) = {
  let dir = data.direction
  let pos-up = data.positive_is_up

  // Determine arrow and color based on direction and positive_is_up semantics
  let (arrow, tile-color) = if dir == "up" {
    let c = if pos-up { color-ok } else { color-bad }
    ("↑", c)
  } else if dir == "down" {
    let c = if pos-up { color-bad } else { color-ok }
    ("↓", c)
  } else {
    ("→", color-text-muted)
  }

  theme-card[
    #label-text(data.label)
    #v(spacing-2)
    #text(size: font-size-2xl, weight: "bold", fill: tile-color)[#arrow #data.delta]
    #if data.reference != none [
      #v(spacing-1)
      #small-text(data.reference)
    ]
  ]
}


// SectionHeaderSplit Component
// Full-width: optional eyebrow + heading on top, body paragraph below

#let section-header-split(data) = {
  block(width: 100%, breakable: false)[
    #if data.eyebrow != none [
      #text(
        size: font-size-xs,
        weight: "bold",
        fill: color-primary,
        tracking: 0.08em,
        upper(data.eyebrow),
      )
      #v(spacing-2)
    ]

    #heading(level: data.level, outlined: data.outlined)[#data.title]

    #v(spacing-2)

    #par(justify: true)[
      #text(size: font-size-base, fill: color-text)[#data.body]
    ]

    #if data.divider_below [
      #v(spacing-3)
      #line(length: 100%, stroke: 0.5pt + rgb("#cbd5e1"))
      #v(spacing-2)
    ] else [
      #v(spacing-3)
    ]
  ]
}


// PhaseBlock Component
// Colored phase header, bullet list, optional "+ N weitere" footer

#let phase-block(data) = {
  // Default colors per phase number (1=bad, 2=warn, 3=info, 4+ = primary)
  let default-color = if data.phase_number == 1 { color-bad }
    else if data.phase_number == 2 { color-warn }
    else if data.phase_number == 3 { color-info }
    else { color-primary }

  let accent = if data.accent_color != none { rgb(data.accent_color) } else { default-color }

  // Header stays with at least the start of the items list.
  // Note: inside content blocks (`[...]`) every function call must be `#`-prefixed,
  // otherwise it is rendered as literal text. See issue auditmysite#239.
  block(width: 100%, breakable: false, below: 0pt)[
    #block(
      width: 100%,
      fill: accent,
      radius: 8pt,
      inset: (x: spacing-4, y: spacing-3),
    )[
      #set text(fill: white)
      #grid(
        columns: (auto, 1fr),
        column-gutter: spacing-3,
        align: horizon,

        text(size: font-size-3xl, weight: "bold")[#data.phase_number],

        stack(
          spacing: spacing-2,
          text(size: font-size-lg, weight: "bold")[#data.phase_label],
          text(size: font-size-xs, fill: white.transparentize(20%))[#data.description],
        ),
      )
    ]
    #v(spacing-2)
    #box(height: 3em, width: 0pt)[]
  ]
  v(spacing-2 - 3em)

  // Items list (breakable — can span pages for long lists)
  block(
      width: 100%,
      fill: color-surface,
      stroke: (paint: color-border, thickness: component-card-border-width),
      radius: 8pt,
      inset: (x: spacing-4, y: spacing-3),
    )[
      #for (i, item) in data.items.enumerate() [
        #grid(
          columns: (10pt, 1fr),
          gutter: 4pt,
          align: (top, top),
          text(size: font-size-sm, fill: accent)[•],
          text(size: font-size-sm, fill: color-text)[#item],
        )
        #if i < data.items.len() - 1 [ #v(3pt) ]
      ]
    ]
}


// ChecklistPanel Component
// Card with label–diagnosis rows and optional status indicators

#let checklist-panel(data) = {
  theme-card[
    #if data.title != none [
      #component-title(text(weight: "bold", size: font-size-sm, fill: color-text)[#data.title])
    ]

    #for (i, row) in data.rows.enumerate() [
      #grid(
        columns: (1fr, 2fr),
        column-gutter: spacing-4,
        align: (top, top),

        // Label + optional status dot
        stack(
          dir: ltr,
          spacing: 4pt,
          if row.status != none {
            let dot-color = if row.status == "good" { color-ok }
              else if row.status == "warn" { color-warn }
              else if row.status == "bad"  { color-bad }
              else { color-text-muted }
            box(
              width: 7pt,
              height: 7pt,
              radius: 999pt,
              fill: dot-color,
            )
          },
          text(size: font-size-sm, weight: "bold", fill: color-text)[#row.label],
        ),

        text(size: font-size-sm, fill: color-text)[#row.diagnosis],
      )

      #if i < data.rows.len() - 1 [
        #v(spacing-2)
        #line(length: 100%, stroke: 0.5pt + color-border)
        #v(spacing-2)
      ]
    ]
  ]
}


// MetricStrip Component
// Horizontal row of equal KPI cells with right-border dividers

#let metric-strip(data) = {
  let n = data.items.len()
  let cols = (1fr,) * n
  let padding = if data.compact { spacing-3 } else { spacing-4 }

  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
  )[
    #grid(
      columns: cols,
      gutter: 0pt,
      ..data.items.enumerate().map(((i, item)) => {
        let accent = if item.accent_color != none { rgb(item.accent_color) }
          else if item.status == "good"    { color-ok }
          else if item.status == "warn"    { color-warn }
          else if item.status == "bad"     { color-bad }
          else { color-primary }

        block(
          width: 100%,
          inset: (x: padding, y: padding),
          stroke: if i < n - 1 { (right: 0.5pt + color-border) } else { none },
        )[
          #label-text(item.label)
          #v(spacing-2)
          #stack(
            dir: ltr,
            spacing: 3pt,
            text(size: if data.compact { font-size-sm } else { font-size-xl }, weight: "bold", fill: accent)[#item.value],
            if item.unit != none [
              #pad(top: 3pt)[#small-text(item.unit)]
            ],
          )
        ]
      })
    )
  ]
}


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
      #component-title(text(weight: "bold", size: font-size-sm, fill: color-text)[#data.title])
    ]
    #stack(
      spacing: spacing-3,
      impact-grid-card(data.user),
      impact-grid-card(data.risk),
      impact-grid-card(data.conversion),
    )
  ]
}


// SpotlightCard Component
// General-purpose spotlight card with left severity stripe

#let spotlight-card-color(variant) = {
  if variant == "critical"         { color-bad }
  else if variant == "info"        { color-info }
  else if variant == "feature"     { color-ok }
  else if variant == "opportunity" { color-warn }
  else { color-primary }
}

#let spotlight-card(data) = {
  let variant      = if data.variant != none { data.variant } else { "info" }
  let stripe-color = spotlight-card-color(variant)

  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
    clip: true,
  )[
    #block(
      width: 100%,
      inset: (left: 4pt),
      fill: stripe-color,
      radius: (left: 10pt),
    )[
      #block(
        width: 100%,
        fill: color-surface,
        inset: (x: spacing-4, y: spacing-4),
      )[
        #grid(
          columns: if data.metric != none { (1fr, auto) } else { (1fr,) },
          column-gutter: spacing-4,
          align: (top, right),

          // Left: eyebrow, title, body, detail, action
          stack(
            spacing: spacing-2,

            if data.eyebrow != none {
              text(
                size: font-size-xs,
                weight: "bold",
                fill: stripe-color,
                tracking: 0.08em,
                upper(data.eyebrow),
              )
            },

            text(size: font-size-lg, weight: "bold", fill: color-text)[#data.title],

            par(justify: true)[#text(size: font-size-sm, fill: color-text)[#data.body]],

            if data.detail != none {
              text(size: font-size-xs, fill: color-text-muted)[#data.detail]
            },

            if data.action != none {
              text(weight: "semibold", size: font-size-sm, fill: stripe-color)[#data.action →]
            },
          ),

          // Right: big metric (only if present)
          ..if data.metric != none {(
            align(right + top)[
              #text(size: font-size-3xl, weight: "bold", fill: stripe-color)[#data.metric]
            ],
          )},
        )
      ]
    ]
  ]
}


// ComparisonBlock Component
// Before/after comparison using before-after() helper from theme_helpers.typ

#let comparison-block(data) = {
  let wrong-label = if data.wrong_label != none { data.wrong_label } else { "✕ " + data.label_left }
  let right-label = if data.right_label != none { data.right_label } else { "✓ " + data.label_right }

  block(width: 100%)[
    #grid(
      columns: (1fr, 1fr),
      gutter: spacing-3,

      // Wrong side
      block(
        width: 100%,
        fill: color-bad-soft,
        radius: 10pt,
        inset: spacing-4,
      )[
        #label-text(wrong-label)
        #v(spacing-2)
        #if data.is_code [
          #mono-text(data.wrong)
        ] else [
          #text(size: font-size-sm, fill: color-text)[#data.wrong]
        ]
      ],

      // Right side
      block(
        width: 100%,
        fill: color-ok-soft,
        radius: 10pt,
        inset: spacing-4,
      )[
        #label-text(right-label)
        #v(spacing-2)
        #if data.is_code [
          #mono-text(data.right)
        ] else [
          #text(size: font-size-sm, fill: color-text)[#data.right]
        ]
      ],
    )

    #if data.note != none [
      #v(spacing-2)
      #small-text(data.note)
    ]
  ]
}


// ComparisonCluster Component
// Grid of comparable items with value, label, optional sub-text and status dot

#let comparison-cluster(data) = {
  let n-cols = data.columns
  let cols = (1fr,) * n-cols

  block(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "bold", size: font-size-sm, fill: color-text)[#data.title])
    ]

    #grid(
      columns: cols,
      column-gutter: spacing-3,
      row-gutter: spacing-3,
      ..data.items.map(item => {
        let dot-color = if item.status == "good"    { color-ok }
          else if item.status == "warn"             { color-warn }
          else if item.status == "bad"              { color-bad }
          else { none }

        theme-card[
          #if dot-color != none [
            #stack(
              dir: ltr,
              spacing: 4pt,
              box(width: 8pt, height: 8pt, radius: 999pt, fill: dot-color),
              label-text(item.label),
            )
          ] else [
            #label-text(item.label)
          ]
          #v(spacing-2)
          #text(size: font-size-xl, weight: "bold", fill: color-text)[#item.value]
          #if item.sub != none [
            #v(spacing-1)
            #small-text(item.sub)
          ]
        ]
      })
    )
  ]
}


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
      #component-title(text(weight: "semibold", size: font-size-lg)[#data.title])
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


// CTABox Component
// Call to action block

#let cta-box-bg(tone) = {
  if tone == "urgent"  { color-bad }
  else if tone == "neutral" { color-surface-alt }
  else { color-primary }  // "primary" is default
}

#let cta-box(data) = {
  let tone = if data.tone != none { data.tone } else { "primary" }
  let bg   = cta-box-bg(tone)
  let is-dark = tone != "neutral"

  block(
    width: 100%,
    fill: bg,
    radius: 10pt,
    inset: spacing-5,
  )[
    #set text(fill: if is-dark { white } else { color-text })

    #text(weight: "bold", size: font-size-xl)[#data.headline]

    #if data.body != none [
      #v(spacing-2)
      #text(size: font-size-sm)[#data.body]
    ]

    #if data.action_label != none [
      #v(spacing-4)
      #{
        if data.action_url != none {
          link(data.action_url)[
            #text(weight: "bold", size: font-size-base)[#data.action_label →]
          ]
        } else {
          text(weight: "bold", size: font-size-base)[#data.action_label →]
        }
      }
    ]
  ]
}


// Testimonial Component
// Customer quote / statement

#let testimonial(data) = {
  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
    inset: spacing-5,
  )[
    // Decorative quote mark
    #text(size: 4em, fill: color-primary.transparentize(60%))["]

    #v(-spacing-4)

    // Quote text
    #par(justify: true)[
      #text(style: "italic", size: font-size-base, fill: color-text)[#data.quote]
    ]

    #v(spacing-3)

    // Attribution
    #{
      let parts = ("— " + data.author,)
      let role-company = if data.role != none and data.company != none {
        data.role + ", " + data.company
      } else if data.role != none {
        data.role
      } else if data.company != none {
        data.company
      } else {
        none
      }

      stack(
        spacing: 2pt,
        text(weight: "semibold", size: font-size-sm, fill: color-text)[— #data.author],
        if role-company != none {
          text(size: font-size-xs, fill: color-text-muted)[#role-company]
        },
      )
    }
  ]
}


// ProcessFlow Component
// Linear process visualization with numbered steps

#let process-flow(data) = {
  let dir = if data.direction != none { data.direction } else { "horizontal" }

  block(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-lg)[#data.title])
    ]

    #if dir == "vertical" [
      // Vertical layout: stacked steps with arrow below each
      #for (i, step) in data.steps.enumerate() [
        #block(
          width: 100%,
          fill: color-surface,
          stroke: (paint: color-border, thickness: component-card-border-width),
          radius: 8pt,
          inset: spacing-4,
        )[
          #grid(
            columns: (auto, 1fr),
            column-gutter: spacing-3,
            align: (top, top),

            // Step number circle
            block(
              width: 24pt,
              height: 24pt,
              fill: color-primary,
              radius: 50%,
              inset: 0pt,
            )[
              #align(center + horizon)[
                #text(size: font-size-xs, weight: "bold", fill: white)[#(i + 1)]
              ]
            ],

            stack(
              spacing: 2pt,
              text(weight: "bold", size: font-size-base)[
                #if step.icon != none [#step.icon #h(4pt)]
                #step.label
              ],
              if step.description != none {
                text(size: font-size-sm, fill: color-text-muted)[#step.description]
              },
            ),
          )
        ]
        #if i < data.steps.len() - 1 [
          #align(center)[#text(size: font-size-lg, fill: color-text-muted)[↓]]
        ]
      ]
    ] else [
      // Horizontal layout: row of boxes with → between
      #grid(
        columns: range(data.steps.len() * 2 - 1).map(i => if calc.rem(i, 2) == 0 { 1fr } else { auto }),
        align: center,
        column-gutter: spacing-2,

        ..{
          let cells = ()
          for (i, step) in data.steps.enumerate() {
            cells.push(
              block(
                width: 100%,
                fill: color-surface,
                stroke: (paint: color-border, thickness: component-card-border-width),
                radius: 8pt,
                inset: spacing-3,
              )[
                #align(center)[
                  #block(
                    width: 20pt,
                    height: 20pt,
                    fill: color-primary,
                    radius: 50%,
                  )[
                    #align(center + horizon)[
                      #text(size: font-size-xs, weight: "bold", fill: white)[#(i + 1)]
                    ]
                  ]
                  #v(spacing-1)
                  #if step.icon != none [#text(size: font-size-base)[#step.icon] #linebreak()]
                  #text(weight: "bold", size: font-size-sm)[#step.label]
                  #if step.description != none [
                    #linebreak()
                    #text(size: font-size-xs, fill: color-text-muted)[#step.description]
                  ]
                ]
              ]
            )
            if i < data.steps.len() - 1 {
              cells.push(
                align(center + horizon)[
                  #text(size: font-size-base, fill: color-text-muted)[→]
                ]
              )
            }
          }
          cells
        },
      )
    ]
  ]
}


// Timeline Component
// Project phases / milestone planning

#let timeline-dot-color(status) = {
  if status == "done"    { color-ok }
  else if status == "active"  { color-primary }
  else { color-text-muted }
}

#let timeline(data) = {
  block(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-lg)[#data.title])
    ]

    #for (i, item) in data.items.enumerate() [
      #{
        let dot-color = timeline-dot-color(item.at("status", default: none))

        grid(
          columns: (8pt, spacing-4, 1fr),
          gutter: 0pt,
          align: (center, left, left),

          // Dot column
          stack(
            spacing: 0pt,
            // Connecting line above (not for first item)
            if i > 0 {
              block(width: 1pt, height: spacing-2, fill: color-border)
            },
            // The dot
            block(
              width: 8pt,
              height: 8pt,
              fill: dot-color,
              radius: 50%,
            ),
            // Connecting line below (not for last item)
            if i < data.items.len() - 1 {
              block(width: 1pt, height: spacing-2, fill: color-border)
            },
          ),

          // Spacer
          none,

          // Content
          block(
            inset: (bottom: spacing-4),
          )[
            #text(size: font-size-xs, weight: "semibold", fill: dot-color)[#item.date]
            #v(2pt)
            #text(weight: "bold", size: font-size-base)[#item.title]
            #if item.at("description", default: none) != none [
              #v(2pt)
              #text(size: font-size-sm, fill: color-text-muted)[#item.description]
            ]
          ],
        )
      }
    ]
  ]
}


// Funnel Component
// Conversion funnel with decreasing-width bars

#let funnel-default-colors = (color-primary, color-info, color-ok, color-warn, color-bad)

#let funnel(data) = {
  let n = data.steps.len()

  block(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-lg)[#data.title])
    ]

    #for (i, step) in data.steps.enumerate() [
      #{
        // Width shrinks from 100% to ~40% across steps
        let pct = if n <= 1 { 100 } else {
          int(100 - (i * 60 / (n - 1)))
        }

        let bar-color = if step.at("color", default: none) != none {
          rgb(step.color)
        } else {
          funnel-default-colors.at(calc.rem(i, funnel-default-colors.len()))
        }

        // Center the bar
        align(center)[
          #block(
            width: 1% * pct,
            fill: bar-color,
            radius: 4pt,
            inset: (x: spacing-3, y: spacing-2),
          )[
            #set text(fill: white)
            #grid(
              columns: (1fr, auto),
              align: (left, right),
              text(weight: "semibold", size: font-size-sm)[#step.label],
              text(weight: "bold", size: font-size-sm)[
                #step.value
                #if step.at("unit", default: none) != none [#text(weight: "regular")[#step.unit]]
              ],
            )
          ]
        ]
        v(spacing-1)
      }
    ]
  ]
}


// ProblemSolution Component
// Two-part problem/solution block

#let problem-solution(data) = {
  let prob-label = if data.problem_label != none { data.problem_label } else { "Problem" }
  let sol-label  = if data.solution_label != none { data.solution_label } else { "Solution" }

  block(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-lg)[#data.title])
    ]

    #grid(
      columns: (1fr, 1fr),
      gutter: spacing-3,

      // Problem (left — red accent)
      block(
        width: 100%,
        fill: color-bad-soft,
        stroke: (left: (paint: color-bad, thickness: 3pt)),
        radius: (right: 8pt),
        inset: spacing-4,
      )[
        #label-text(prob-label)
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#data.problem]
      ],

      // Solution (right — green accent)
      block(
        width: 100%,
        fill: color-ok-soft,
        stroke: (left: (paint: color-ok, thickness: 3pt)),
        radius: (right: 8pt),
        inset: spacing-4,
      )[
        #label-text(sol-label)
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#data.solution]
      ],
    )

    #if data.impact != none or data.effort != none [
      #v(spacing-2)
      #grid(
        columns: if data.impact != none and data.effort != none { (1fr, 1fr) } else { (1fr,) },
        gutter: spacing-3,

        ..if data.impact != none {(
          block(
            fill: color-surface,
            stroke: (paint: color-border, thickness: component-card-border-width),
            radius: 6pt,
            inset: spacing-3,
          )[
            #label-text("Impact")
            #v(spacing-1)
            #text(size: font-size-sm)[#data.impact]
          ],
        )},

        ..if data.effort != none {(
          block(
            fill: color-surface,
            stroke: (paint: color-border, thickness: component-card-border-width),
            radius: 6pt,
            inset: spacing-3,
          )[
            #label-text("Effort")
            #v(spacing-1)
            #text(size: font-size-sm)[#data.effort]
          ],
        )},
      )
    ]
  ]
}


// BeforeAfter Component
// Before/after comparison block

#let before-after(data) = {
  let lbl-before = if data.label_before != none { data.label_before } else { "Before" }
  let lbl-after  = if data.label_after  != none { data.label_after  } else { "After"  }

  block(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-lg)[#data.title])
    ]

    #grid(
      columns: (1fr, 1fr),
      gutter: spacing-3,

      // Before — muted/grey
      block(
        width: 100%,
        fill: color-surface-alt,
        stroke: (paint: color-border, thickness: component-card-border-width),
        radius: 8pt,
        inset: spacing-4,
      )[
        #label-text(lbl-before)
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#data.before]
      ],

      // After — accent color
      block(
        width: 100%,
        fill: color-accent-soft,
        stroke: (left: (paint: color-primary, thickness: 3pt)),
        radius: (right: 8pt),
        inset: spacing-4,
      )[
        #label-text(lbl-after)
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#data.after]
      ],
    )

    #if data.note != none [
      #v(spacing-2)
      #small-text(data.note)
    ]
  ]
}


// WhyItMatters Component
// Context / relevance block for storytelling

#let why-it-matters(data) = {
  let heading = if data.title != none { data.title } else { "Why it matters" }

  block(
    width: 100%,
    fill: color-surface,
    stroke: (
      left: (paint: color-warn, thickness: 4pt),
      top: (paint: color-border, thickness: component-card-border-width),
      right: (paint: color-border, thickness: component-card-border-width),
      bottom: (paint: color-border, thickness: component-card-border-width),
    ),
    radius: (right: 8pt),
    inset: spacing-4,
  )[
    #text(weight: "bold", size: font-size-base, fill: color-warn)[#heading]
    #v(spacing-2)
    #par(justify: true)[#text(size: font-size-base, fill: color-text)[#data.body]]

    #if data.evidence != none [
      #v(spacing-3)
      #text(style: "italic", size: font-size-sm, fill: color-text-muted)[#data.evidence]
    ]

    #if data.urgency != none [
      #v(spacing-3)
      #block(
        fill: color-warn-soft,
        radius: 4pt,
        inset: (x: spacing-2, y: 3pt),
      )[
        #text(weight: "semibold", size: font-size-xs, fill: color-warn)[#data.urgency]
      ]
    ]
  ]
}


// FactBox Component
// Inline info/fact box

#let fact-box-style(variant) = {
  if variant == "tip"     { (accent: color-ok,      bg: color-ok-soft,      icon: "✓") }
  else if variant == "warning" { (accent: color-warn,    bg: color-warn-soft,    icon: "⚠") }
  else if variant == "stat"    { (accent: color-primary, bg: color-accent-soft,  icon: "#") }
  else                         { (accent: color-info,    bg: color-info-soft,    icon: "ℹ") }  // "info" default
}

#let fact-box(data) = {
  let variant = if data.variant != none { data.variant } else { "info" }
  let style   = fact-box-style(variant)

  block(
    width: 100%,
    fill: style.bg,
    stroke: (left: (paint: style.accent, thickness: 3pt)),
    radius: (right: 6pt),
    inset: (x: spacing-3, y: spacing-3),
  )[
    #if data.label != none [
      #text(weight: "bold", size: font-size-xs, fill: style.accent)[#style.icon #upper(data.label)]
      #v(spacing-1)
    ]

    #text(size: font-size-sm, fill: color-text)[#data.body]
  ]
}


// QuoteBlock Component
// Quote / core statement / pull quote

#let quote-block(data) = {
  let is-emphasis = data.at("emphasis", default: false) == true
  let quote-size  = if is-emphasis { font-size-xl } else { font-size-base }

  block(
    width: 100%,
    fill: color-surface,
    stroke: (left: (paint: color-primary, thickness: 4pt)),
    radius: (right: 8pt),
    inset: spacing-4,
  )[
    // Decorative opening quote glyph
    #text(size: 3em, fill: color-primary.transparentize(55%), weight: "bold")["]

    #v(-spacing-3)

    // Quote text
    #par(justify: true)[
      #text(style: "italic", size: quote-size, fill: color-text)[#data.quote]
    ]

    #if data.author != none [
      #v(spacing-3)
      #text(weight: "semibold", size: font-size-sm, fill: color-text-muted)[— #data.author]
    ]
  ]
}


// BenefitStrip Component
// Horizontal benefit strip (3-5 points) for marketing, features, value props

#let benefit-strip(data) = {
  let items = range(data.titles.len()).map(i => {
    block(width: 100%)[
      #if i < data.icons.len() and data.icons.at(i) != "" [
        #text(size: 32pt)[#data.icons.at(i)]
        #v(spacing-2)
      ]
      #text(size: font-size-lg, weight: "bold", fill: color-text)[#data.titles.at(i)]
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text-muted)[#data.descriptions.at(i)]
    ]
  })

  block(width: 100%)[
    #grid(
      columns: (1fr,) * data.columns,
      column-gutter: spacing-4,
      ..items,
    )
  ]
}


// PricingCard Component
// Single pricing/plan card with features and CTA

#let pricing-card(data) = {
  let border-width = if data.highlighted { 2pt } else { component-card-border-width }
  let border-color = if data.highlighted { color-primary } else { color-border }
  let bg-color = if data.highlighted { color-accent-soft } else { color-surface }

  block(
    width: 100%,
    fill: bg-color,
    stroke: (paint: border-color, thickness: border-width),
    radius: 10pt,
    inset: spacing-4,
  )[
    #if data.highlighted [
      #text(size: font-size-xs, weight: "bold", fill: color-primary, tracking: 0.08em)[RECOMMENDED]
      #v(spacing-2)
    ]

    #text(size: font-size-lg, weight: "bold", fill: color-text)[#data.name]
    #v(spacing-3)

    #text(size: font-size-3xl, weight: "bold", fill: color-primary)[#data.price]
    #if data.billing_period != none [
      #text(size: font-size-sm, fill: color-text-muted)[#data.billing_period]
    ]

    #if data.description != none [
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text-muted)[#data.description]
    ]

    #v(spacing-3)

    #if data.features.len() > 0 [
      #{
        for feature in data.features [
          #grid(
            columns: (10pt, 1fr),
            gutter: spacing-2,
            text(size: font-size-sm, fill: color-ok)[✓],
            text(size: font-size-sm, fill: color-text)[#feature],
          )
          #v(spacing-1)
        ]
      }
      #v(spacing-2)
    ]

    #if data.cta_label != none [
      #text(size: font-size-sm, weight: "semibold", fill: color-primary)[#data.cta_label →]
    ]
  ]
}


// RecommendationCard Component
// Lightweight recommendation: title + description + impact/effort/priority badges

#let recommendation-card-priority-color(priority) = {
  if priority == "critical" { color-bad }
  else if priority == "high" { color-warn }
  else if priority == "medium" { color-info }
  else { color-primary }
}

#let recommendation-card-effort-color(effort) = {
  if effort == "high" { color-bad }
  else if effort == "medium" { color-warn }
  else { color-ok }
}

#let recommendation-card(data) = {
  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 8pt,
    inset: spacing-3,
  )[
    #text(size: font-size-base, weight: "bold", fill: color-text)[#data.title]
    #v(spacing-2)
    #text(size: font-size-sm, fill: color-text)[#data.description]

    #if data.impact != none or data.effort != none or data.priority != none [
      #v(spacing-2)
      #let badges = (
        if data.impact != none {(
          block(
            fill: color-info.lighten(80%),
            stroke: (paint: color-info, thickness: 0.5pt),
            radius: 4pt,
            inset: (x: spacing-2, y: 2pt),
          )[
            #text(size: font-size-xs, weight: "semibold", fill: color-info)[Impact: #data.impact]
          ],
        )} else {()},
        if data.effort != none {(
          block(
            fill: recommendation-card-effort-color(data.effort).lighten(80%),
            stroke: (paint: recommendation-card-effort-color(data.effort), thickness: 0.5pt),
            radius: 4pt,
            inset: (x: spacing-2, y: 2pt),
          )[
            #text(size: font-size-xs, weight: "semibold", fill: recommendation-card-effort-color(data.effort))[Effort: #data.effort]
          ],
        )} else {()},
        if data.priority != none {(
          block(
            fill: recommendation-card-priority-color(data.priority).lighten(80%),
            stroke: (paint: recommendation-card-priority-color(data.priority), thickness: 0.5pt),
            radius: 4pt,
            inset: (x: spacing-2, y: 2pt),
          )[
            #text(size: font-size-xs, weight: "semibold", fill: recommendation-card-priority-color(data.priority))[Priority: #data.priority]
          ],
        )} else {()},
      ).flatten()
      #grid(
        columns: (auto,) * badges.len(),
        column-gutter: spacing-2,
        ..badges,
      )
    ]
  ]
}


// StepCardRow Component
// Numbered steps displayed horizontally (2-4 steps)

#let step-card-row(data) = {
  let items = range(data.titles.len()).map(i => {
    block(width: 100%)[
      #block(
        width: 40pt,
        height: 40pt,
        fill: color-primary,
        stroke: (paint: color-primary, thickness: 2pt),
        radius: 50%,
        align(center + horizon)[
          #text(size: font-size-lg, weight: "bold", fill: white)[#{i + 1}]
        ],
      )
      #v(spacing-3)
      #text(size: font-size-base, weight: "bold", fill: color-text)[#data.titles.at(i)]
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text-muted)[#data.descriptions.at(i)]
    ]
  })

  block(width: 100%)[
    #grid(
      columns: (1fr,) * data.columns,
      column-gutter: spacing-4,
      ..items,
    )
  ]
}


// Columns Layout Component
// Asymmetric two-column layout with flexible width ratio

#let columns(data) = {
  let gap = if data.gap != none { data.gap } else { spacing-4 }
  let right_width = 1.0 - data.left_width

  block(width: 100%)[
    #grid(
      columns: (data.left_width * 100% - gap * 0.5, right_width * 100% - gap * 0.5),
      column-gutter: gap,

      block(width: 100%)[#data.left],
      block(width: 100%)[#data.right],
    )
  ]
}


// Device Preview Component
// Desktop screenshot (70%) + mobile screenshot (30%) with Apple-style frames
// Centered at 88% width for breathing room on both sides

#let device-preview(data) = {
  let gap = 8pt
  let h = data.height_pt * 1pt
  let dr = data.desktop_ratio

  align(center, block(width: 88%, above: 2pt, below: 6pt, breakable: false)[
    #set align(left)
    #grid(
      columns: (dr * 100% - gap * 0.5, (1.0 - dr) * 100% - gap * 0.5),
      column-gutter: gap,
      align: top,

      // Desktop: chrome bar stacked directly above screenshot, zero gap
      box(stroke: 0.5pt + rgb("#d1d5db"), radius: 6pt, clip: true, width: 100%, height: h)[
        #stack(dir: ttb, spacing: 0pt,
          block(
            fill: rgb("#f3f4f6"),
            width: 100%,
            height: 20pt,
            above: 0pt,
            below: 0pt,
            stroke: (bottom: 0.5pt + rgb("#e5e7eb")),
            inset: (left: 8pt, right: 8pt, top: 0pt, bottom: 0pt),
          )[
            #set align(left + horizon)
            #stack(dir: ltr, spacing: 5pt,
              circle(radius: 4pt, fill: rgb("#ff5f57")),
              circle(radius: 4pt, fill: rgb("#febc2e")),
              circle(radius: 4pt, fill: rgb("#28c840")),
            )
          ],
          // Image at full width, outer box clips to h - 20pt — shows top of page
          image(data.desktop_src, width: 100%),
        )
      ],

      // Mobile: phone-style rounded corners, outer box clips to h — shows top
      box(stroke: 0.5pt + rgb("#d1d5db"), radius: 14pt, clip: true, width: 100%, height: h)[
        #image(data.mobile_src, width: 100%)
      ],
    )
  ])
}


// FaqList Component
// Question-answer pairs for FAQs and knowledge bases

#let faq-list(data) = {
  block(width: 100%)[
    #if data.title != none [
      #component-title(text(size: font-size-lg, weight: "bold", fill: color-text)[#data.title], spacing: spacing-4)
    ]

    #for (i, item) in data.items.enumerate() [
      #block(width: 100%)[
        #text(size: font-size-base, weight: "bold", fill: color-text)[#item.question]
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#item.answer]
      ]
      #if i < data.items.len() - 1 [
        #v(spacing-3)
      ]
    ]
  ]
}


// UseCaseCard Component
// Single use case: context + problem + solution + optional outcome

#let use-case-card(data) = {
  block(
    width: 100%,
    fill: color-surface,
    stroke: (paint: color-border, thickness: component-card-border-width),
    radius: 10pt,
    inset: spacing-4,
  )[
    #text(size: font-size-lg, weight: "bold", fill: color-text)[#data.title]
    #v(spacing-2)
    #text(size: font-size-xs, fill: color-primary, weight: "semibold", tracking: 0.06em)[#upper(data.context)]
    #v(spacing-3)

    // Problem section
    #block(width: 100%, fill: color-bad-soft, radius: 6pt, inset: spacing-3)[
      #label-text("Problem")
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text)[#data.problem]
    ]
    #v(spacing-3)

    // Solution section
    #block(width: 100%, fill: color-ok-soft, radius: 6pt, inset: spacing-3)[
      #label-text("Solution")
      #v(spacing-2)
      #text(size: font-size-sm, fill: color-text)[#data.solution]
    ]

    #if data.outcome != none [
      #v(spacing-3)
      #block(width: 100%, fill: color-info.lighten(85%), radius: 6pt, inset: spacing-3)[
        #label-text("Outcome")
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text)[#data.outcome]
      ]
    ]
  ]
}


// LogoStrip Component
// Display logos/names of customers, partners, certifications

#let logo-strip(data) = {
  block(width: 100%)[
    #if data.title != none [
      #component-title(text(size: font-size-sm, weight: "bold", fill: color-text-muted, tracking: 0.06em)[#upper(data.title)])
    ]

    #grid(
      columns: (1fr,) * data.columns,
      column-gutter: spacing-3,
      row-gutter: spacing-3,
      ..data.labels.map(label => {
        block(
          width: 100%,
          fill: color-surface,
          stroke: (paint: color-border, thickness: 0.5pt),
          radius: 6pt,
          inset: spacing-3,
        )[
          #align(center + horizon)[
            #text(size: font-size-sm, weight: "semibold", fill: color-text)[#label]
          ]
        ]
      }),
    )
  ]
}


// PullQuote Component
// Large, visually prominent full-width quote (centered)

#let pull-quote(data) = {
  block(width: 100%, fill: color-accent-soft, radius: 8pt, inset: spacing-5)[
    #align(center)[
      #text(size: 28pt, weight: "bold", fill: color-primary)["]
      #v(spacing-3)
      #text(size: 20pt, style: "italic", fill: color-text)[#data.quote]
      #v(spacing-4)
      #if data.attribution != none [
        #text(size: font-size-sm, fill: color-text-muted)[— #data.attribution]
      ]
    ]
  ]
}


// BigNumber Component
// Large metric display for marketing/impact stats

#let big-number(data) = {
  block(width: 100%, fill: color-surface, stroke: (paint: color-primary, thickness: 2pt), radius: 10pt, inset: spacing-4)[
    #align(center)[
      #text(size: 48pt, weight: "bold", fill: color-primary)[#data.value]
      #v(spacing-2)
      #text(size: font-size-lg, weight: "bold", fill: color-text)[#data.label]
      #if data.context != none [
        #v(spacing-2)
        #text(size: font-size-sm, fill: color-text-muted)[#data.context]
      ]
    ]
  ]
}


// GlossaryList Component
// Term-definition pairs for glossaries, abbreviations, technical terms

#let glossary-list(data) = {
  block(width: 100%)[
    #if data.title != none [
      #component-title(text(size: font-size-lg, weight: "bold", fill: color-text)[#data.title], spacing: spacing-4)
    ]

    #for (i, item) in data.items.enumerate() [
      #block(width: 100%)[
        #text(size: font-size-base, weight: "bold", fill: color-primary)[#item.term]
        #v(spacing-1)
        #text(size: font-size-sm, fill: color-text)[#item.definition]
      ]
      #if i < data.items.len() - 1 [
        #v(spacing-2)
      ]
    ]
  ]
}


// DiagnosisPanel Component
// Card with label–diagnosis rows and optional status indicators

#let diagnosis-panel(data) = {
  theme-card[
    #if data.at("title", default: none) != none [
      #text(weight: "bold", size: font-size-sm, fill: color-text)[#data.title]
      #v(spacing-3)
    ]

    #for (i, row) in data.rows.enumerate() [
      #grid(
        columns: (1fr, 2fr),
        column-gutter: spacing-4,
        align: (top, top),

        // Label + optional status dot
        stack(
          dir: ltr,
          spacing: 4pt,
          if row.at("status", default: none) != none {
            let dot-color = if row.status == "good" { color-ok }
              else if row.status == "warn" { color-warn }
              else if row.status == "bad"  { color-bad }
              else { color-text-muted }
            box(
              width: 7pt,
              height: 7pt,
              radius: 999pt,
              fill: dot-color,
            )
          },
          text(size: font-size-sm, weight: "bold", fill: color-text)[#row.label],
        ),

        text(size: font-size-sm, fill: color-text)[#row.diagnosis],
      )

      #if i < data.rows.len() - 1 [
        #v(spacing-2)
        #line(length: 100%, stroke: 0.5pt + color-border)
        #v(spacing-2)
      ]
    ]
  ]
}


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


// WrongRightBlock Component
// Before/after comparison using before-after() helper from theme_helpers.typ

#let wrong-right-block(data) = {
  let wrong-label = if data.at("wrong_label", default: none) != none { data.wrong_label } else { "✕ Wrong" }
  let right-label = if data.at("right_label", default: none) != none { data.right_label } else { "✓ Right" }

  block(width: 100%)[
    #grid(
      columns: (1fr, 1fr),
      gutter: spacing-3,

      // Wrong side
      block(
        width: 100%,
        fill: color-bad-soft,
        radius: 10pt,
        inset: spacing-4,
      )[
        #label-text(wrong-label)
        #v(spacing-2)
        #if data.is_code [
          #mono-text(data.wrong)
        ] else [
          #text(size: font-size-sm, fill: color-text)[#data.wrong]
        ]
      ],

      // Right side
      block(
        width: 100%,
        fill: color-ok-soft,
        radius: 10pt,
        inset: spacing-4,
      )[
        #label-text(right-label)
        #v(spacing-2)
        #if data.is_code [
          #mono-text(data.right)
        ] else [
          #text(size: font-size-sm, fill: color-text)[#data.right]
        ]
      ],
    )

    #if data.at("note", default: none) != none [
      #v(spacing-2)
      #small-text(data.note)
    ]
  ]
}


// Grid Component
// Inspired by Pentaho Row/Block Layout

// Component dispatch for nested rendering
#let _grid-dispatch(c) = {
  if type(c) == dictionary and "type" in c and "data" in c {
    let comp-type = c.at("type")
    let comp-data = c.at("data")
    // Dispatch to known component functions
    if comp-type == "score-card" { score-card(comp-data) }
    else if comp-type == "gauge" { gauge(comp-data) }
    else if comp-type == "chart" { chart(comp-data) }
    else if comp-type == "sparkline" { sparkline(comp-data) }
    else if comp-type == "finding" { finding(comp-data) }
    else if comp-type == "callout" { callout(comp-data) }
    else if comp-type == "progress-bar" { progress-bar(comp-data) }
    else if comp-type == "summary-box" { summary-box(comp-data) }
    else if comp-type == "key-value-list" { key-value-list(comp-data) }
    else if comp-type == "metric-card" { metric-card(comp-data) }
    else if comp-type == "label" { label(comp-data) }
    else if comp-type == "textblock" { textblock(comp-data) }
    else if comp-type == "number-field" { number-field(comp-data) }
    else if comp-type == "date-field" { date-field(comp-data) }
    else if comp-type == "resource-field" { resource-field(comp-data) }
    else if comp-type == "barcode" { barcode(comp-data) }
    else if comp-type == "image" { report-image(comp-data) }
    else if comp-type == "section" { section(comp-data) }
    else if comp-type == "audit-table" { audit-table(comp-data) }
    else if comp-type == "severity-overview" { severity-overview(comp-data) }
    else if comp-type == "crosstab" { crosstab(comp-data) }
    else if comp-type == "pivot-table" { pivot-table(comp-data) }
    else if comp-type == "list" { list(comp-data) }
    else if comp-type == "divider" { divider(comp-data) }
    else if comp-type == "watermark" { watermark(comp-data) }
    else if comp-type == "page-break" { page-break(comp-data) }
    else if comp-type == "tag-cloud" { tag-cloud(comp-data) }
    else if comp-type == "side-label" { side-label(comp-data) }
    else if comp-type == "table-of-contents" { table-of-contents(comp-data) }
    else if comp-type == "eyebrow" { eyebrow(comp-data) }
    else if comp-type == "status-pill" { status-pill(comp-data) }
    else if comp-type == "stat" { stat(comp-data) }
    else if comp-type == "stat-pair" { stat-pair(comp-data) }
    else if comp-type == "score-band" { score-band(comp-data) }
    else if comp-type == "trend-tile" { trend-tile(comp-data) }
    else if comp-type == "hero-summary" { hero-summary(comp-data) }
    else if comp-type == "product-hero" { product-hero(comp-data) }
    else if comp-type == "card-dashboard" { card-dashboard(comp-data) }
    else if comp-type == "roadmap-block" { roadmap-block(comp-data) }
    else if comp-type == "cover-page" { cover-page(comp-data) }
    else if comp-type == "module-comparison" { module-comparison(comp-data) }
    else if comp-type == "portfolio-summary" { portfolio-summary(comp-data) }
    else if comp-type == "benchmark-table" { benchmark-table(comp-data) }
    else if comp-type == "checklist-panel" { checklist-panel(comp-data) }
    else if comp-type == "metric-strip" { metric-strip(comp-data) }
    else if comp-type == "impact-grid" { impact-grid(comp-data) }
    else if comp-type == "spotlight-card" { spotlight-card(comp-data) }
    else if comp-type == "phase-block" { phase-block(comp-data) }
    else if comp-type == "section-header-split" { section-header-split(comp-data) }
    else if comp-type == "comparison-block" { comparison-block(comp-data) }
    else if comp-type == "comparison-cluster" { comparison-cluster(comp-data) }
    else if comp-type == "feature-grid" { feature-grid(comp-data) }
    else if comp-type == "cta-box" { cta-box(comp-data) }
    else if comp-type == "testimonial" { testimonial(comp-data) }
    else if comp-type == "process-flow" { process-flow(comp-data) }
    else if comp-type == "timeline" { timeline(comp-data) }
    else if comp-type == "funnel" { funnel(comp-data) }
    else if comp-type == "problem-solution" { problem-solution(comp-data) }
    else if comp-type == "before-after" { before-after(comp-data) }
    else if comp-type == "why-it-matters" { why-it-matters(comp-data) }
    else if comp-type == "fact-box" { fact-box(comp-data) }
    else if comp-type == "quote-block" { quote-block(comp-data) }
    else if comp-type == "benefit-strip" { benefit-strip(comp-data) }
    else if comp-type == "pricing-card" { pricing-card(comp-data) }
    else if comp-type == "recommendation-card" { recommendation-card(comp-data) }
    else if comp-type == "step-card-row" { step-card-row(comp-data) }
    else if comp-type == "columns" { columns(comp-data) }
    else if comp-type == "device-preview" { device-preview(comp-data) }
    else if comp-type == "faq-list" { faq-list(comp-data) }
    else if comp-type == "use-case-card" { use-case-card(comp-data) }
    else if comp-type == "logo-strip" { logo-strip(comp-data) }
    else if comp-type == "pull-quote" { pull-quote(comp-data) }
    else if comp-type == "big-number" { big-number(comp-data) }
    else if comp-type == "glossary-list" { glossary-list(comp-data) }
    else if comp-type == "diagnosis-panel" { diagnosis-panel(comp-data) }
    else if comp-type == "dominant-issue-spotlight" { dominant-issue-spotlight(comp-data) }
    else if comp-type == "wrong-right-block" { wrong-right-block(comp-data) }
    else if comp-type == "grid-component" { grid-component(comp-data) }
    else if comp-type == "flow-group" { flow-group(comp-data) }
    else {
      // Fallback for unknown types
      text(size: 9pt, fill: gray, "[" + comp-type + "]")
    }
  } else if type(c) == dictionary {
    // Raw data without type wrapper — generic display
    let inner = c
    if inner.at("title", default: none) != none {
      text(weight: "bold", size: 10pt, inner.title)
      if inner.at("score", default: none) != none {
        v(4pt)
        text(size: 20pt, weight: "bold", str(inner.score))
        if inner.at("max_score", default: none) != none {
          text(size: 10pt, fill: gray, " / " + str(inner.max_score))
        }
      }
      if inner.at("description", default: none) != none {
        v(4pt)
        text(size: 9pt, fill: gray, inner.description)
      }
    } else {
      for (key, val) in inner {
        if type(val) == str or type(val) == int or type(val) == float {
          text(size: 9pt)[#text(weight: "bold")[#key:] #str(val)]
          linebreak()
        }
      }
    }
  } else if type(c) == str {
    text(size: 10pt, c)
  } else {
    [#c]
  }
}

#let grid-component(data) = {
  box(width: 100%)[
    #if data.title != none [
      #component-title(text(weight: "semibold", size: font-size-lg)[#data.title], spacing: spacing-4)
    ]

    #let col-widths = (1fr,) * data.columns
    #let item-min-height = if data.item_min_height != none { eval(data.item_min_height) } else { none }

    #grid(
      columns: col-widths,
      column-gutter: eval(data.column_gap),
      row-gutter: eval(data.row_gap),

      ..data.items.map(item => {
        let c = item.content
        let item-body = if type(c) == dictionary and "type" in c and "data" in c {
          [#_grid-dispatch(c)]
        } else {
          box(
            width: 100%,
            inset: spacing-3,
            fill: color-surface,
            radius: 4pt,
            stroke: (paint: color-border, thickness: 0.5pt),
          )[#_grid-dispatch(c)]
        }

        if item-min-height != none {
          box(width: 100%, height: item-min-height)[#item-body]
        } else {
          box(width: 100%)[#item-body]
        }
      })
    )
  ]
}


// Flow Group Component
// Soft keep-together wrapper for headings + first follow-up blocks.

#let _flow-dispatch(c) = {
  if type(c) == dictionary and "type" in c and "data" in c {
    let comp-type = c.at("type")
    let comp-data = c.at("data")
    if comp-type == "score-card" { score-card(comp-data) }
    else if comp-type == "gauge" { gauge(comp-data) }
    else if comp-type == "chart" { chart(comp-data) }
    else if comp-type == "sparkline" { sparkline(comp-data) }
    else if comp-type == "finding" { finding(comp-data) }
    else if comp-type == "callout" { callout(comp-data) }
    else if comp-type == "progress-bar" { progress-bar(comp-data) }
    else if comp-type == "summary-box" { summary-box(comp-data) }
    else if comp-type == "key-value-list" { key-value-list(comp-data) }
    else if comp-type == "metric-card" { metric-card(comp-data) }
    else if comp-type == "label" { label(comp-data) }
    else if comp-type == "textblock" { textblock(comp-data) }
    else if comp-type == "number-field" { number-field(comp-data) }
    else if comp-type == "date-field" { date-field(comp-data) }
    else if comp-type == "resource-field" { resource-field(comp-data) }
    else if comp-type == "barcode" { barcode(comp-data) }
    else if comp-type == "image" { report-image(comp-data) }
    else if comp-type == "section" { section(comp-data) }
    else if comp-type == "audit-table" { audit-table(comp-data) }
    else if comp-type == "severity-overview" { severity-overview(comp-data) }
    else if comp-type == "crosstab" { crosstab(comp-data) }
    else if comp-type == "pivot-table" { pivot-table(comp-data) }
    else if comp-type == "list" { list(comp-data) }
    else if comp-type == "divider" { divider(comp-data) }
    else if comp-type == "watermark" { watermark(comp-data) }
    else if comp-type == "page-break" { page-break(comp-data) }
    else if comp-type == "tag-cloud" { tag-cloud(comp-data) }
    else if comp-type == "side-label" { side-label(comp-data) }
    else if comp-type == "table-of-contents" { table-of-contents(comp-data) }
    else if comp-type == "eyebrow" { eyebrow(comp-data) }
    else if comp-type == "status-pill" { status-pill(comp-data) }
    else if comp-type == "stat" { stat(comp-data) }
    else if comp-type == "stat-pair" { stat-pair(comp-data) }
    else if comp-type == "score-band" { score-band(comp-data) }
    else if comp-type == "trend-tile" { trend-tile(comp-data) }
    else if comp-type == "hero-summary" { hero-summary(comp-data) }
    else if comp-type == "product-hero" { product-hero(comp-data) }
    else if comp-type == "card-dashboard" { card-dashboard(comp-data) }
    else if comp-type == "roadmap-block" { roadmap-block(comp-data) }
    else if comp-type == "cover-page" { cover-page(comp-data) }
    else if comp-type == "module-comparison" { module-comparison(comp-data) }
    else if comp-type == "portfolio-summary" { portfolio-summary(comp-data) }
    else if comp-type == "benchmark-table" { benchmark-table(comp-data) }
    else if comp-type == "checklist-panel" { checklist-panel(comp-data) }
    else if comp-type == "metric-strip" { metric-strip(comp-data) }
    else if comp-type == "impact-grid" { impact-grid(comp-data) }
    else if comp-type == "spotlight-card" { spotlight-card(comp-data) }
    else if comp-type == "phase-block" { phase-block(comp-data) }
    else if comp-type == "section-header-split" { section-header-split(comp-data) }
    else if comp-type == "comparison-block" { comparison-block(comp-data) }
    else if comp-type == "comparison-cluster" { comparison-cluster(comp-data) }
    else if comp-type == "feature-grid" { feature-grid(comp-data) }
    else if comp-type == "cta-box" { cta-box(comp-data) }
    else if comp-type == "testimonial" { testimonial(comp-data) }
    else if comp-type == "process-flow" { process-flow(comp-data) }
    else if comp-type == "timeline" { timeline(comp-data) }
    else if comp-type == "funnel" { funnel(comp-data) }
    else if comp-type == "problem-solution" { problem-solution(comp-data) }
    else if comp-type == "before-after" { before-after(comp-data) }
    else if comp-type == "why-it-matters" { why-it-matters(comp-data) }
    else if comp-type == "fact-box" { fact-box(comp-data) }
    else if comp-type == "quote-block" { quote-block(comp-data) }
    else if comp-type == "benefit-strip" { benefit-strip(comp-data) }
    else if comp-type == "pricing-card" { pricing-card(comp-data) }
    else if comp-type == "recommendation-card" { recommendation-card(comp-data) }
    else if comp-type == "step-card-row" { step-card-row(comp-data) }
    else if comp-type == "columns" { columns(comp-data) }
    else if comp-type == "device-preview" { device-preview(comp-data) }
    else if comp-type == "faq-list" { faq-list(comp-data) }
    else if comp-type == "use-case-card" { use-case-card(comp-data) }
    else if comp-type == "logo-strip" { logo-strip(comp-data) }
    else if comp-type == "pull-quote" { pull-quote(comp-data) }
    else if comp-type == "big-number" { big-number(comp-data) }
    else if comp-type == "glossary-list" { glossary-list(comp-data) }
    else if comp-type == "diagnosis-panel" { diagnosis-panel(comp-data) }
    else if comp-type == "dominant-issue-spotlight" { dominant-issue-spotlight(comp-data) }
    else if comp-type == "wrong-right-block" { wrong-right-block(comp-data) }
    else if comp-type == "grid-component" { grid-component(comp-data) }
    else if comp-type == "flow-group" { flow-group(comp-data) }
    else {
      text(size: 9pt, fill: gray, "[" + comp-type + "]")
    }
  } else if type(c) == str {
    text(size: 10pt, c)
  } else {
    [#c]
  }
}

#let flow-group(data) = context {
  let gap = if data.spacing != none { eval(data.spacing) } else { spacing-4 }
  let content = stack(
    spacing: gap,
    ..data.items.map(item => [#_flow-dispatch(item)]),
  )

  if data.keep_together_if_under != none {
    let threshold = eval(data.keep_together_if_under)
    let measured = measure(content)
    if measured.height <= threshold {
      block(width: 100%, breakable: false)[#content]
    } else {
      content
    }
  } else {
    content
  }
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

#section(json.decode("{\"content\":[],\"level\":2,\"title\":\"Heading\"}"))

#score-card(json.decode("{\"computed_status\":\"warning\",\"description\":null,\"good_threshold\":90,\"height\":null,\"inverted\":false,\"max_score\":100,\"score\":85,\"status\":null,\"title\":\"Score\",\"warn_threshold\":50}"))

#v(spacing-4)

#callout(json.decode("{\"callout_type\":\"info\",\"content\":\"Stable canonical content.\",\"inverted\":false,\"title\":null}"))

#v(spacing-4)

