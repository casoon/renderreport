#!/usr/bin/env bash
# Builds the Wasm engine for the playground into site/public/playground/ (not committed).
# Needs: rustup target wasm32-unknown-unknown, wasm-bindgen CLI in the version of Cargo.lock;
# wasm-opt (binaryen) is used when present. Run before `pnpm build` in site/.
set -euo pipefail
cd "$(dirname "$0")/../.."

fail() { echo "build-wasm: $1" >&2; exit 1; }

command -v cargo >/dev/null || fail "cargo not found (install Rust: https://rustup.rs)"
command -v wasm-bindgen >/dev/null || fail "wasm-bindgen CLI not found (cargo install wasm-bindgen-cli --version <version in Cargo.lock>)"

want=$(awk '/^name = "wasm-bindgen"$/ {getline; gsub(/[version = "]/, ""); print; exit}' Cargo.lock)
have=$(wasm-bindgen --version | awk '{print $2}')
[ "$want" = "$have" ] || fail "wasm-bindgen $have installed, Cargo.lock needs $want (cargo install wasm-bindgen-cli --version $want --locked)"

if command -v rustup >/dev/null && ! rustup target list --installed | grep -qx wasm32-unknown-unknown; then
  fail "target wasm32-unknown-unknown missing (rustup target add wasm32-unknown-unknown)"
fi

out=site/public/playground
target_dir=${CARGO_TARGET_DIR:-target}

# png: page images for the preview; no barcodes (not needed in the browser).
cargo build --release --locked --target wasm32-unknown-unknown --no-default-features --features wasm,png

rm -rf "$out"
mkdir -p "$out"
wasm-bindgen "$target_dir/wasm32-unknown-unknown/release/renderreport.wasm" \
  --out-dir "$out" --target web --no-typescript

if command -v wasm-opt >/dev/null; then
  wasm-opt -Oz --all-features "$out/renderreport_bg.wasm" -o "$out/renderreport_bg.opt.wasm"
  mv "$out/renderreport_bg.opt.wasm" "$out/renderreport_bg.wasm"
else
  echo "build-wasm: wasm-opt not found, skipping size optimisation" >&2
fi

raw=$(wc -c < "$out/renderreport_bg.wasm")
gz=$(gzip -9c "$out/renderreport_bg.wasm" | wc -c)
echo "build-wasm: $out/renderreport_bg.wasm $((raw / 1024 / 1024)) MiB raw, $((gz / 1024 / 1024)) MiB gzip"
