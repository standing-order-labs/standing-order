#!/usr/bin/env bash
# Deploy a contract. Usage: ./scripts/deploy.sh <package> <network> <source-account>
# Example: ./scripts/deploy.sh subscription-manager testnet deployer
# Prerequisites: stellar-cli installed, network configured, key funded
#   stellar keys generate deployer --network testnet --fund
set -euo pipefail

PKG="${1:?package name required}"
NETWORK="${2:?network required (testnet|mainnet)}"
SOURCE="${3:?source account alias required}"

cd "$(dirname "$0")/.."
WASM_NAME="${PKG//-/_}"
WASM="target/wasm32v1-none/release/${WASM_NAME}.optimized.wasm"

if [ ! -f "$WASM" ]; then
  echo "Missing $WASM - run ./scripts/build.sh $PKG first" >&2
  exit 1
fi

ID=$(stellar contract deploy --wasm "$WASM" --source-account "$SOURCE" --network "$NETWORK")
echo "Deployed $PKG to $NETWORK: $ID"
echo "Record it in docs/deployments/${NETWORK}.md"
