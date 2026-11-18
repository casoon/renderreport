// Soft Page Break Component
// Starts a new page only when less than `room` (fraction of the page height)
// is left. A non-breakable block of that height moves to the next page when
// it does not fit; the negative space after it takes the block back out of
// the flow, so nothing is left behind either way. A `pagebreak` inside
// `context` is ignored by Typst, hence no direct break here.
//
// When the next component follows on the same page, the weak space in front
// is the gap between the two chapters; at the top of a page, and at the end
// of the page it breaks, Typst drops it.

#let soft-page-break(data) = context {
  let need = page.height * data.at("room", default: 0.4)
  v(spacing-5 * 2, weak: true)
  block(breakable: false, width: 0pt, height: need, above: 0pt, below: 0pt)
  v(-need, weak: false)
}
