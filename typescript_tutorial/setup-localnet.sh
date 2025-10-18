#!/bin/bash

# Setup environment variables for local Solana validator
export ANCHOR_PROVIDER_URL=http://127.0.0.1:8899
export ANCHOR_WALLET=~/.config/solana/id.json

echo "Environment variables set:"
echo "ANCHOR_PROVIDER_URL=$ANCHOR_PROVIDER_URL"
echo "ANCHOR_WALLET=$ANCHOR_WALLET"

echo ""
echo "Now you can run:"
echo "1. solana-test-validator (in another terminal)"
echo "2. npm test tests/simple_swap_vitest.test.ts"
