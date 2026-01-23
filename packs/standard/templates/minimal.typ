// Minimal Report Template
// Simple layout without header/footer, suitable for single-page reports

#let report(
  title: none,
  subtitle: none,
  body
) = {
  // Simple title block
  if title != none {
    align(center)[
      #text(size: font-size-2xl, weight: "bold", fill: color-text)[#title]
      
      #if subtitle != none {
        v(spacing-2)
        text(size: font-size-lg, fill: color-text-muted)[#subtitle]
      }
    ]
    v(spacing-6)
  }
  
  // Content
  body
}
