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
      #text(weight: "semibold", size: font-size-lg)[#data.title]
      #v(spacing-3)
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
