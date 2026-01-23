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
