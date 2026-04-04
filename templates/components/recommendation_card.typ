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
