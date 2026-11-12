# Changelog

All notable changes to this project are documented in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Releases before 0.3.0 are listed on the
[GitHub releases page](https://github.com/casoon/renderreport/releases).

## [Unreleased]

### Added

- **Templates:** a request can be a template bound to data at render time: `{{ path.to.value }}` placeholders in strings (a string that is only a placeholder keeps the JSON type; the filters `upper`, `lower`, `round` are available), `{"for": "x in list", "component": {…}}` and `{"if": "path", "component": {…}}` anywhere in an array, also nested in component data. It is a plain JSON-to-JSON step with its own small syntax (no new dependency), available as `render::bind`, as `--data <file.json|file.csv>` on `renderreport render` and `validate` (`validate` checks the resolved request) and as the Wasm function `render_with_data`; the request schema allows the `for`/`if` entries. Errors name the template location and the missing data path. See the new guide "Templates and data" and `examples/requests/monthly-report/`.
- **Output:** PNG and SVG per page: `Engine::render_pages(&request, PageFormat::Png { ppi } | PageFormat::Svg)` returns one byte sequence per page, behind the new optional features `png` and `svg` (both part of `cli`, neither of `default`). `renderreport render` takes `--format pdf|png|svg` (also inferred from the `-o` extension), `--pages 1-3,5` and `--ppi` (default 144); several pages need `{n}` in the output path, e.g. `-o page-{n}.png`. With the `wasm` and `png` features the Wasm function `render_page_png(request_json, page_index, ppi)` is available. HTML output is not planned; see Changed.
- **CLI:** `renderreport validate` checks a request without rendering it and reports each problem with its path, e.g. `components[0].data: score-card: score: invalid type`; unknown component types are reported too.
- **CLI:** `renderreport schema [--component <id>]` prints the JSON Schema of a request or of one component's data.
- **Library:** `validate::validate_request`, the Wasm function `validate` and, with the new `schema` feature (part of `cli`), the `schema` module.
- **Components:** aliases: a component can be registered with further names (`aliases` in `component_catalog!` and `ComponentDescriptor`). `ComponentCatalog::get`, `validate`, the schema and `list-components` know them; a request's `type` is rewritten to the component id before rendering, so templates stay unchanged, and a name defined by the request's pack wins over an alias. First alias: `table` for `audit-table`, in Rust `Table` for `AuditTable`.
- **CoverPage:** a cover without score block: `CoverPage::plain(title)`, or in a request `cover-page` data without `score`; `brand`, `domain`, `subtitle`, `date` and `grade` are optional there. Existing covers render as before.

### Changed

- **CoverPage:** the field `score` is an `Option<u32>` (`CoverPage::new` still takes a plain score).
- **Components:** the description of `audit-table` is general (it is a table, not only for audits); the id stays. The docs list the audit-specific components as a group, and `DominantIssueSpotlight` is deprecated in favour of `SpotlightCard`. `DiagnosisPanel` and `WrongRightBlock` stay without deprecation for now.
- **Library:** `RenderOutput::Html`, an unused stub, is removed; `RenderOutput` keeps its single `Pdf` variant.
- **Docs:** the project site uses the current shared theme (0.12.3) and ends with a link to the
  quickstart.

## [0.7.2] - 2026-10-26

### Added

- **Releases:** the binaries carry a signed build provenance attestation (verify with `gh attestation verify`) and the release lists a combined `SHA256SUMS`.

### Changed

- **npm:** `@casoon/renderreport` is published from the release workflow via Trusted Publishing (with provenance), after approval in the `npm` environment.
- **Docs:** the start page of the [project site](https://casoon.github.io/renderreport/) shows
  the rendered health-check report next to the install commands instead of the JSON request
  above it.

## [0.7.1] - 2026-10-24

### Added

- **CoverPage:** optional indicator strip (`with_indicator_gauges`) below the module gauges for values that are shown but not graded, in smaller neutral rings.

### Changed

- **CoverPage:** up to six module gauges share one row (more still wrap in rows of four).

### Fixed

- **Table of contents:** the contents page no longer lists its own heading or the report title; both stay PDF bookmarks.
- **AuditTable:** a row moves to the next page as a whole instead of splitting a cell across the page break.

## [0.7.0] - 2026-10-24

### Breaking

- **Packs:** a request's `pack_id` must name a loaded pack, and an unknown `template_id` with a
  pack is an error (`default` keeps the built-in layout); before, both were ignored.
- **Packs:** the loader refuses packs whose manifest lists missing files or whose components and
  templates do not define their Typst function. Templates are keyed by their manifest name
  (`seo-report`), not their file name.
- **Components:** `ComponentDescriptor` has the new fields `fn_name` and `normalize`; without
  the `barcodes` feature, `barcode` is no longer registered.

### Added

- **Packs:** pack components (also inside grids and flow groups, and as overrides of built-in
  components), the template selected by `template_id`, its default theme and pack assets are
  rendered. Details: [PACKS_README](https://github.com/casoon/renderreport/blob/main/docs/PACKS_README.md).
- **Packs:** the bundled `standard` and `seo-audit` packs load and render.
- **CLI:** `render --pack-path <DIR>`; a request with `pack_id` loads that pack.

### Changed

- **Components:** a component is registered by one `component_catalog!` call next to its
  struct; the grid and flow-group dispatchers are generated from the catalog.
- **Crate:** no stray PDFs and rendered example outputs in the published package.
- **Docs:** all documentation lives on the [project site](https://casoon.github.io/renderreport/);
  new pages for [template packs](https://casoon.github.io/renderreport/docs/guides/packs/) and
  [report patterns](https://casoon.github.io/renderreport/docs/guides/patterns/). The README is a
  short entry point.
- **Contributors:** CI runs on pull requests and `main` only; beta tests and coverage run on
  demand.
- **Contributors:** component structs live in one module per catalog category; public paths
  (`components::*`, `components::advanced::*`) are unchanged.

### Fixed

- **Packs:** the default search path `~/.renderreport/packs` resolves the home directory; before,
  the `~` was taken literally.

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
