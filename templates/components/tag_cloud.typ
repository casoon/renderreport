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
