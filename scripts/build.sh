#!/usr/bin/env bash
# Build (and optimize) contract WASM.
# Usage: ./scripts/build.sh [package|all]
set -euo pipefail

PKG="${1:-all}"
cd "$(dirname "$0")/.."

if [ "$PKG" = "all" ]; then
  stellar contract build
else
  stellar contract build --package "$PKG"
fi

for wasm in target/wasm32v1-none/release/*.wasm; do
  case "$wasm" in *.optimized.wasm) continue ;; esac
  stellar contract optimize --wasm "$wasm"
  echo "built: $wasm"
done
