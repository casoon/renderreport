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
