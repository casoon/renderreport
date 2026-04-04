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
