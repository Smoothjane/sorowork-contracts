#!/bin/bash
set -euo pipefail

echo "=== Seeding Testnet with Sample Community Work ==="

KEY="${STELLAR_KEY:-mykey}"
NETWORK="testnet"
ESCROW_ID="${ESCROW_CONTRACT_ID:?Set ESCROW_CONTRACT_ID}"

echo "Creating sample bounties..."

# Park cleanup task
stellar contract invoke \
  --id "$ESCROW_ID" \
  --source "$KEY" \
  --network "$NETWORK" \
  -- create_bounty \
  --creator "$(stellar keys address $KEY)" \
  --token "${USDC_TOKEN:-CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC}" \
  --amount 15000000000

echo "Task 1: Park cleanup — 150 USDC"

# Pothole repair task
stellar contract invoke \
  --id "$ESCROW_ID" \
  --source "$KEY" \
  --network "$NETWORK" \
  -- create_bounty \
  --creator "$(stellar keys address $KEY)" \
  --token "${USDC_TOKEN:-CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC}" \
  --amount 30000000000

echo "Task 2: Pothole repair — 300 USDC"

# Fence painting task
stellar contract invoke \
  --id "$ESCROW_ID" \
  --source "$KEY" \
  --network "$NETWORK" \
  -- create_bounty \
  --creator "$(stellar keys address $KEY)" \
  --token "${USDC_TOKEN:-CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC}" \
  --amount 20000000000

echo "Task 3: Fence painting — 200 USDC"

echo ""
echo "=== Seeding Complete ==="
echo "Created 3 sample community work bounties"
