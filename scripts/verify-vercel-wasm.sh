#!/usr/bin/env bash
set -euo pipefail

pkg_dir="query-engine-wasm/pkg"
wasm_file="$pkg_dir/query_engine_wasm_bg.wasm"
js_file="$pkg_dir/query_engine_wasm.js"
max_bytes=$((5 * 1024 * 1024))

if [[ ! -f "$wasm_file" || ! -f "$js_file" ]]; then
  cat >&2 <<'MESSAGE'
The Vercel build requires the browser WASM package in query-engine-wasm/pkg.

Regenerate it before deploying:
  wasm-pack build query-engine-wasm --target web --out-dir pkg --release
MESSAGE
  exit 1
fi

wasm_bytes=$(wc -c < "$wasm_file")

if (( wasm_bytes > max_bytes )); then
  echo "WASM package is ${wasm_bytes} bytes, above the ${max_bytes} byte Vercel budget." >&2
  exit 1
fi

echo "Vercel WASM package verified: ${wasm_bytes} bytes."
