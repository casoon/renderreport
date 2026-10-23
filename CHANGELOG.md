# Changelog

All notable changes to this project are documented in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Releases before 0.3.0 are listed on the
[GitHub releases page](https://github.com/casoon/renderreport/releases).

## [0.6.2] - 2026-10-23

### Fixed

- Wrong/right block: short examples (up to 900 characters) stay on one page; split across a page
  break, the label sat alone on one page and the code on the next.
- Divider with `thickness: "0pt"` renders as weak spacing instead of a zero-width line. At the
  end of a full page the line was still content and opened an otherwise blank page.

## [0.6.1] - 2026-10-13

### Added

- Component data from JSON needs only the fields its Rust builder requires: the engine fills in
  defaults and derived fields (a score card's `computed_status`, a callout's type, a word search's
  grid) before rendering. Unknown fields are kept; data that does not fit is passed on unchanged.
- CLI: `renderreport render --typ` writes the generated Typst source instead of the PDF.

### Fixed

- Typst errors read as message, failing component and hints instead of the diagnostic's debug
  form with internal span ids.
- Theme tokens `font.body`, `font.heading` and `font.mono` accept a plain font name from JSON or
  TOML; before, it was written to Typst unquoted and the render failed. Rust themes no longer lose
  their fonts in a JSON round trip.
- README: Rust 1.92, version 0.6 and 80 components instead of outdated values.
- Bar charts: vertical bars stand on a shared baseline instead of hanging from the top; every
  series is drawn (grouped bars in legend colours), not only the first; values keep one decimal.
- CLI: errors show their cause down to the Typst diagnostic instead of only
  `failed to render PDF`.
- npm README: describes the CLI the package installs instead of a Node API it does not ship.

## [0.6.0] - 2026-10-12

### Breaking

- `engine::world::FontCache` no longer exposes its `book` and `fonts` fields.
- `rust-version` raised to 1.92.

### Added

- Word search: allowed reading directions shown as arrows on the puzzle page.
- Word search: with translations, the word list becomes a vocabulary table; grid and table share
  one page where possible.
- Reports without a title or cover page take the first section's title as the PDF document title.
- docs.rs documents the `cli` and `barcodes` features.

### Changed

- The Typst world is built on embedded-typst: font files from disk are read lazily, in-memory
  `.typ` files can be imported, and `datetime.today(offset: …)` honours the offset.
- Word search: words on the same line keep at least one letter between them; the word list keeps
  spaces as entered (`GUINEA PIG`); larger grid letters.
- Wasm `render_wordsearch`: no running header, smaller top margin, level-1 heading.

### Removed

- Prebuilt CLI binary for macOS Intel (`x86_64-apple-darwin`).

## [0.5.2] - 2026-09-17

### Added

- `Engine::with_config_and_fonts` takes font bytes in addition to the configured fonts.
- `install.sh` installs the CLI from the latest release and verifies its checksum.

### Fixed

- Font collections are no longer re-read and re-parsed for every face.

## [0.5.1] - 2026-09-15

### Fixed

- Adjacent page breaks no longer leave a blank page.

## [0.5.0] - 2026-08-30

### Breaking

- `benchmark-table`: the extra ranking columns are defined by the caller.

## [0.4.1] - 2026-08-30

### Added

- `benchmark-table`: optional HTML-conformance column.

## [0.4.0] - 2026-08-29

### Changed

- Typst 0.15; every PDF is written as tagged PDF with PDF/UA-1 enforced.

## [0.3.2] - 2026-08-17

### Fixed

- `checklist-panel`: the card body can break across pages instead of overflowing.

## [0.3.1] - 2026-07-30

### Changed

- Word search: better random placement and a word list heading for translation mode.
- Licence is MIT only.

## [0.3.0] - 2026-07-30

### Added

- `word-search` component.
- Wasm bindings (`render`, `render_wordsearch`) with an embedded font fallback.

### Fixed

- Panics and swallowed errors found in a code review; MSRV declared.
