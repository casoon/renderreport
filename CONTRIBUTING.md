# Contributing to RenderReport

Thank you for your interest in contributing to RenderReport!

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git

### Getting Started

```bash
# Clone the repository
git clone https://github.com/casoon/renderreport
cd renderreport

# Build the project
cargo build

# Run tests
cargo test

# Run examples
cargo run --example seo_audit
```

## Project Structure

- `src/` - Core library code
  - `engine/` - Rendering engine
  - `components/` - Standard components
  - `theme/` - Theme and token system
  - `pack/` - Template pack system
  - `render/` - Typst compilation
  - `vfs/` - Virtual filesystem
- `templates/` - Built-in Typst templates
- `packs/` - Bundled template packs
- `examples/` - Usage examples
- `tests/` - Integration tests
- `docs/` - Additional documentation

## Coding Guidelines

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add documentation for public APIs
- Write tests for new functionality

### Commit Messages

- Use conventional commits format
- Examples:
  - `feat: add new component type`
  - `fix: correct token rendering`
  - `docs: update API documentation`
  - `test: add tests for theme system`

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Make your changes
4. Add tests
5. Ensure all tests pass (`cargo test`)
6. Commit your changes
7. Push to your fork
8. Create a Pull Request

### Testing

- Write unit tests for new functionality
- Add integration tests for user-facing features
- Ensure all tests pass before submitting a PR
- Aim for >80% code coverage

The test suite includes a **Typst compile smoke test** (`tests/compile_smoke.rs`) that
renders a minimal PDF through the full Typst engine. It catches syntax errors in `.typ`
component templates (e.g. invalid `#` usage in code-mode blocks) that the Rust compiler
cannot see. Run it with:

```bash
cargo test --test compile_smoke
```

This test must pass before any renderreport release.

## Adding a New Component

A component consists of three parts:

1. **Rust struct** in `src/components/standard.rs` or `advanced.rs` — derive `Serialize`,
   implement `Component` (`component_id()` + `to_data()`).
2. **Typst template** in `templates/components/<name>.typ` — a `#let` function named with
   hyphens matching the component ID.
3. **Registry entry** in `src/components/registry.rs` — `self.register(ComponentId::new("name"), include_str!(...))`.

### Optional title — use `component-title()`

If your component has an optional title field, render it with the `component-title()`
helper defined in `templates/theme_helpers.typ`:

```typst
if data.title != none {
  component-title(text(size: font-size-xl, weight: "bold")[#data.title])
}
```

**Do not** inline the orphan-protection block manually. `component-title()` keeps the
title attached to the component body across page breaks, so it is never left alone at the
bottom of a page.

The `spacing` parameter defaults to `spacing-3`; pass a different value when the
component body needs more breathing room:

```typst
component-title(text(...)[#data.title], spacing: spacing-4)
```

### Catalog entry

Every registered component must appear in `examples/component_catalog.rs` with a
`// @id: <component-id>` comment. The `catalog_completeness` test enforces this:

```bash
cargo test --test catalog_completeness
```

## Areas for Contribution

### High Priority

- Additional standard components
- More template packs (web-quality, security-audit)
- Performance improvements
- Documentation improvements

### Medium Priority

- HTML output support (experimental)
- Remote pack loading
- Pack validation tools
- Visual regression testing

### Low Priority

- WASM support
- Additional themes
- Preview server for pack development

## Questions?

Open an issue or discussion on GitHub.

## Code of Conduct

Be respectful, professional, and welcoming to all contributors.
