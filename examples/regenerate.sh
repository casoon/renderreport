#!/usr/bin/env bash
# Rebuilds the PDFs the project site shows (site/src/showcase.ts) from this
# checkout: the Rust examples write to examples/output/, the CLI renders the
# JSON requests in examples/requests/ next to their sources. Run after
# changing an example, a component or a template, then commit the result.
set -euo pipefail
cd "$(dirname "$0")/.."

for example in simple_report custom_theme wordsearch_demo component_catalog; do
  cargo run --release --quiet --example "$example"
done

cargo build --release --quiet --features cli --bin renderreport
for request in examples/requests/*.json; do
  target/release/renderreport render "$request" -o "${request%.json}.pdf"
done

# The hero scene of the site replays the CLI output above. After the byte count
# or the component count changes, update examples/terminal/render.terminal.yaml
# and rebuild its SVG:
#   cd examples/terminal && npx -y -p @casoon/castwright@0.4.0 -p shiki \
#     castwright build render.terminal.yaml --format svg -o .
