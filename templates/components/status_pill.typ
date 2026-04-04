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
