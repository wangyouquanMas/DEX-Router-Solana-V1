#!/bin/bash

echo "🚀 Starting Solana Swap Instruction Test"
echo ""

# Set environment variables
export ANCHOR_PROVIDER_URL=http://127.0.0.1:8899
export ANCHOR_WALLET=/root/.config/solana/id.json

echo "Environment variables:"
echo "  ANCHOR_PROVIDER_URL: $ANCHOR_PROVIDER_URL"
echo "  ANCHOR_WALLET: $ANCHOR_WALLET"
echo ""

echo "⚠️  Make sure you have a Solana validator running:"
echo "   solana-test-validator --reset"
echo ""

echo "🧪 Running the test..."
npm test tests/simple_swap_vitest.test.ts
