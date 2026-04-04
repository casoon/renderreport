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
