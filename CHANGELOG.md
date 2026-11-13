# Changelog

All notable changes to this project are documented in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Releases before 0.3.0 are listed on the
[GitHub releases page](https://github.com/casoon/renderreport/releases).

## [Unreleased]

### Changed

- **Breaking, default texts:** the built-in texts of `cover-page`, `severity-overview`, `benchmark-table` and `dominant-issue-spotlight` come from a language table and follow `metadata.lang` (`de` or English, English when `lang` is unset or unknown). They were fixed German before. A label in the request still wins, so requests that set their labels or `lang: "de"` render as before. `BenchmarkTable::new` no longer sets the title "Website-Ranking" (use `with_title`). See the components page.

## [0.7.4] - 2026-11-12

### Added

- **CLI:** `renderreport validate` checks a request without rendering and reports each problem with its field path; `renderreport schema [--component <id>]` prints the JSON Schema of a request or of one component. Library: `validate::validate_request`, the Wasm function `validate` and the `schema` module (feature `schema`, part of `cli`).
- **Templates:** a request can be bound to data at render time: `{{ path }}` placeholders (filters `upper`, `lower`, `round`), `{"for": "x in list", ...}` and `{"if": "path", ...}`. Available as `render::bind`, as `--data <file.json|file.csv>` on `render` and `validate`, and as the Wasm function `render_with_data`. See the guide "Templates and data".
- **CLI:** `renderreport from <format> <file>` builds a report from `markdown`, `csv`, `lighthouse`, `sarif` or `junit` without a hand-written request (`--emit-request` prints the request, `--lang en|de`). No new dependencies; the adapters sit behind the `cli` feature. See the guide "Reports from files".
- **Output:** PNG and SVG per page: `Engine::render_pages` with `PageFormat`, `render --format pdf|png|svg --pages --ppi`, and the Wasm function `render_page_png`. New features `png` and `svg` (part of `cli`, not of `default`).
- **Components:** the optional feature `chartlet` adds the component `chartlet`, a [chartlet](https://github.com/casoon/chartlet) specification embedded as an SVG image with PDF/UA alternative text. Not part of `default` or the Wasm build.
- **Components:** component aliases (`aliases` in `component_catalog!`); first alias `table` for `audit-table` (Rust: `Table`).
- **CoverPage:** a cover without score block (`CoverPage::plain(title)`, or `cover-page` data without `score`). Existing covers render as before.
- **Docs:** a playground at `/playground/` runs the engine in the browser as Wasm (about 11 MiB gzip, loaded on the first Render), with example requests and component samples. Three showcase reports that are not audits (coding-agent usage, Rust workspace analysis, monthly traffic report). See the guide "Working with agents" and the agent skill `skills/renderreport/SKILL.md`.

### Changed

- **Docs:** renderreport is described as an engine for multi-page reports built from components; the Cargo and npm descriptions and keywords no longer mention audits.
- **CoverPage:** the field `score` is an `Option<u32>` (`CoverPage::new` still takes a plain score).
- **Components:** the description of `audit-table` is general; `DominantIssueSpotlight` is deprecated in favour of `SpotlightCard`.
- **Library:** `RenderOutput::Html`, an unused stub, is removed.

### Fixed

- **Components:** `chart.height` was ignored; a `height` other than the default `200pt` now sets the plot area of vertical bar charts. The default still renders as before.
- **Components:** `section.subtitle` was documented but never rendered; it is now an optional muted line under the heading.
- **Components:** `section.content` was accepted but never rendered; `validate` now reports a non-empty `content` and says to put the components after the section.
- **CLI:** `validate` rejected `{"type": "page-break", "data": {}}` and a missing `data` although `render` accepts both; components without fields now accept `null`, `{}` and no `data`.
- **Components:** `align` on a column of `table` / `audit-table` was ignored; the template now applies it to header and cells. Columns without `align` stay left-aligned.
- **Docs:** the project table of the agentmeter showcase report no longer splits over a page break.

## [0.7.3] - 2026-11-12

### Added

- **BenchmarkTable:** the headers of the fixed rank, domain, score, accessibility and
  critical-issues columns can be set (`with_labels`); without them the German defaults stay.
- **Releases:** the crate is published by the release workflow through crates.io Trusted
  Publishing, after the GitHub release and an approval in the `crates-io` environment; no
  registry token is stored.

### Changed

- **Docs:** the project site uses the current shared theme (0.12.12) and ends with a link to the
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
