#!/bin/bash
set -euo pipefail

echo "=== SoroWork Contracts — Testnet Deployment ==="

KEY="${STELLAR_KEY:-mykey}"
NETWORK="testnet"

echo "Building contracts..."
cargo build --target wasm32-unknown-unknown --release

echo "Deploying bounty-escrow..."
ESCROW_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/bounty_escrow.wasm \
  --source "$KEY" \
  --network "$NETWORK")
echo "Bounty Escrow: $ESCROW_ID"

echo "Deploying community-treasury..."
TREASURY_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/community_treasury.wasm \
  --source "$KEY" \
  --network "$NETWORK")
echo "Community Treasury: $TREASURY_ID"

echo "Deploying contribution-registry..."
REGISTRY_ID=$(stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/contribution_registry.wasm \
  --source "$KEY" \
  --network "$NETWORK")
echo "Contribution Registry: $REGISTRY_ID"

ADMIN_ADDR=$(stellar keys address "$KEY")

echo "Initializing treasury..."
stellar contract invoke \
  --id "$TREASURY_ID" \
  --source "$KEY" \
  --network "$NETWORK" \
  -- initialize \
  --admin "$ADMIN_ADDR" \
  --token "${USDC_TOKEN:-CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC}"

echo "Initializing registry with escrow as authorized caller..."
stellar contract invoke \
  --id "$REGISTRY_ID" \
  --source "$KEY" \
  --network "$NETWORK" \
  -- initialize \
  --authorized_caller "$ESCROW_ID"

echo ""
echo "=== Deployment Complete ==="
echo "ESCROW_CONTRACT_ID=$ESCROW_ID"
echo "TREASURY_CONTRACT_ID=$TREASURY_ID"
echo "REGISTRY_CONTRACT_ID=$REGISTRY_ID"
