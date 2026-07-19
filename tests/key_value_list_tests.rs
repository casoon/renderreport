//! Regression guard for `key-value-list`'s vertical layout: the key column
//! must stay a fixed fraction, never `auto`.
//!
//! `auto` sizes a Typst grid column to its content's natural (unwrapped)
//! width. A short label (the component's intended use) is unaffected, but
//! an unbounded-length key -- a CSS selector, a URL, a filename -- forces
//! the column to claim nearly the full row width, squeezing the value
//! column into a one-word-per-line sliver spanning dozens of rows and
//! overflowing the page boundary (verified visually against a real customer
//! report: `casoon/auditmysite#518`).
//!
//! `render_pdf`/page-count/byte-size do not reliably distinguish the broken
//! and fixed layouts (both compile, both fit the same page count in
//! testing) -- the failure is visual overflow, not a compile error or a
//! page-count blowup, so it can only be caught here by asserting on the
//! template source itself rather than the rendered PDF's byte-level shape.

use std::path::Path;

#[test]
fn vertical_layout_key_column_is_not_auto_sized() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("templates/components/key_value_list.typ");
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));

    assert!(
        !source.contains("columns: (auto, 1fr)"),
        "key-value-list's vertical grid must not use an auto-sized key column \
         (see #518) -- found `columns: (auto, 1fr)` in {}",
        path.display()
    );
}
