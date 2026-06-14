#!/usr/bin/env sh
set -eu

WASM_PACK="${WASM_PACK:-wasm-pack}"
PACKAGE_DIR="query-engine-wasm/pkg"
MAX_SIZE_KIB="${MAX_SIZE_KIB:-5120}"

"$WASM_PACK" build query-engine-wasm --target web --out-dir pkg --release

package_size_kib="$(du -sk "$PACKAGE_DIR" | awk '{print $1}')"

printf 'WASM package size: %s KiB (limit: %s KiB)\n' "$package_size_kib" "$MAX_SIZE_KIB"

if [ "$package_size_kib" -gt "$MAX_SIZE_KIB" ]; then
  printf 'WASM package exceeds the size budget.\n' >&2
  exit 1
fi
