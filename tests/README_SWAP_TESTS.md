# Swap Instruction Testing Guide

## Overview

This guide explains how to test the `swap` instruction in the DEX Router Solana program. The test file demonstrates various swap scenarios including single DEX, multi-DEX splits, multi-hop swaps, and parallel routes.

## Test File Structure

The test file (`swap.test.ts`) contains:

1. **Setup & Environment Configuration** - Initializes test mints and token accounts
2. **Basic Swap Tests** - Demonstrates SwapArgs construction patterns
3. **Integration Test Examples** - Shows what real DEX integration requires
4. **Validation Tests** - Verifies SwapArgs constraints
5. **Order ID Tests** - Tests order ID generation

## Prerequisites

### 1. Install Dependencies

```bash
npm install
```

Required packages (should already be in package.json):
- `@coral-xyz/anchor` or `@project-serum/anchor`
- `@solana/web3.js`
- `@solana/spl-token`
- `mocha`
- `chai`
- `ts-mocha`
- `typescript`

### 2. Build the Program

```bash
anchor build
```

This generates the program IDL and types in `target/types/`.

### 3. Install @coral-xyz/anchor

If you don't have it installed:

```bash
npm install --save-dev @coral-xyz/anchor
```

Or if using the older version:

```bash
npm install --save-dev @project-serum/anchor
```

## Running the Tests

### Run All Tests

```bash
anchor test
```

Or using npm:

```bash
npm run test
```

### Run Specific Test File

```bash
npx ts-mocha -p ./tsconfig.json -t 1000000 tests/swap.test.ts
```

### Run with Local Validator

```bash
# Start local validator (in a separate terminal)
solana-test-validator

# Run tests
anchor test --skip-local-validator
```

## Understanding SwapArgs Structure

### Basic Structure

```typescript
{
  amountIn: BN,              // Total input amount
  expectAmountOut: BN,       // Expected output amount
  minReturn: BN,             // Minimum acceptable output (slippage protection)
  amounts: BN[],             // Level 1 split: amount per route
  routes: Route[][]          // Level 2 split: DEXs and weights per hop
}
```

### Example 1: Simple Single-DEX Swap

Swap 100 USDC → SOL via Raydium (100%)

```typescript
const swapArgs = {
  amountIn: new BN(100_000_000),
  expectAmountOut: new BN(50_000_000),
  minReturn: new BN(49_000_000),
  amounts: [new BN(100_000_000)],
  routes: [[
    {
      dexes: [{ raydiumSwap: {} }],
      weights: [100]
    }
  ]]
};
```

### Example 2: Split Across Multiple DEXs

Swap 100 USDC → SOL split: Raydium 50%, Whirlpool 30%, Meteora 20%

```typescript
const swapArgs = {
  amountIn: new BN(100_000_000),
  expectAmountOut: new BN(50_000_000),
  minReturn: new BN(49_000_000),
  amounts: [new BN(100_000_000)],
  routes: [[
    {
      dexes: [
        { raydiumSwap: {} },
        { whirlpool: {} },
        { meteoraDynamicpool: {} }
      ],
      weights: [50, 30, 20]
    }
  ]]
};
```

### Example 3: Multi-Hop Swap

Swap USDC → SOL → BONK (2 hops)

```typescript
const swapArgs = {
  amountIn: new BN(1000_000_000),
  expectAmountOut: new BN(50_000_000_000),
  minReturn: new BN(49_000_000_000),
  amounts: [new BN(1000_000_000)],
  routes: [[
    {
      // Hop 1: USDC → SOL
      dexes: [{ raydiumSwap: {} }],
      weights: [100]
    },
    {
      // Hop 2: SOL → BONK
      dexes: [{ whirlpool: {} }],
      weights: [100]
    }
  ]]
};
```

### Example 4: Parallel Routes

Split 1000 USDC across 2 routes:
- Route 1 (600 USDC): via Raydium
- Route 2 (400 USDC): via Whirlpool

```typescript
const swapArgs = {
  amountIn: new BN(1000_000_000),
  expectAmountOut: new BN(500_000_000),
  minReturn: new BN(490_000_000),
  amounts: [
    new BN(600_000_000),  // Route 1
    new BN(400_000_000)   // Route 2
  ],
  routes: [
    [{ dexes: [{ raydiumSwap: {} }], weights: [100] }],
    [{ dexes: [{ whirlpool: {} }], weights: [100] }]
  ]
};
```

### Example 5: Complex Multi-Hop with Splits

2-hop swap with DEX splits in first hop:
- Hop 1: USDC → SOL (Raydium 50%, Whirlpool 30%, Meteora 20%)
- Hop 2: SOL → BONK (Raydium 100%)

```typescript
const swapArgs = {
  amountIn: new BN(1000_000_000),
  expectAmountOut: new BN(50_000_000_000),
  minReturn: new BN(48_000_000_000),
  amounts: [new BN(1000_000_000)],
  routes: [[
    {
      // Hop 1: Split across 3 DEXs
      dexes: [
        { raydiumSwap: {} },
        { whirlpool: {} },
        { meteoraDynamicpool: {} }
      ],
      weights: [50, 30, 20]
    },
    {
      // Hop 2: Single DEX
      dexes: [{ raydiumSwap: {} }],
      weights: [100]
    }
  ]]
};
```

## Available DEX Enums

The test uses enum variants for different DEXs. Here are some common ones:

- `raydiumSwap` - Raydium AMM
- `whirlpool` - Orca Whirlpool
- `whirlpoolV2` - Orca Whirlpool V2
- `meteoraDynamicpool` - Meteora Dynamic Pool
- `meteoraDlmm` - Meteora DLMM
- `raydiumClmmSwap` - Raydium CLMM
- `raydiumClmmSwapV2` - Raydium CLMM V2
- `openBookV2` - OpenBook V2
- `phoenix` - Phoenix DEX
- `pumpfunBuy` / `pumpfunSell` - Pump.fun

See `programs/dex-solana/src/instructions/common_swap.rs` for the complete Dex enum.

## SwapArgs Validation Rules

The tests verify these important constraints:

1. **Amount Sum**: `amounts.sum() == amountIn`
   ```typescript
   amounts.reduce((sum, amt) => sum.add(amt), new BN(0)) === amountIn
   ```

2. **Array Length**: `amounts.length == routes.length`
   ```typescript
   amounts.length === routes.length
   ```

3. **Weight Sum**: Each hop's weights must sum to 100
   ```typescript
   route.weights.reduce((a, b) => a + b, 0) === 100
   ```

4. **Slippage**: `minReturn <= expectAmountOut`
   ```typescript
   minReturn.lte(expectAmountOut)
   ```

## Real Integration Testing

The current tests demonstrate SwapArgs construction but don't execute actual swaps. For real integration testing, you need:

### 1. DEX Pool Accounts

For each DEX, you need specific accounts. Example for Raydium:
- Raydium Program ID
- AMM account
- AMM authority
- AMM open orders
- Pool coin token account
- Pool PC token account
- Serum market accounts (if applicable)

### 2. Remaining Accounts Array

```typescript
.remainingAccounts([
  { pubkey: raydiumProgramId, isWritable: false, isSigner: false },
  { pubkey: ammId, isWritable: true, isSigner: false },
  { pubkey: ammAuthority, isWritable: false, isSigner: false },
  // ... more accounts
])
```

### 3. Intermediate Token Accounts

For multi-hop swaps, create intermediate token accounts:

```typescript
// For USDC → SOL → BONK
const intermediateSolAccount = await createAccount(
  connection,
  payer,
  wrappedSolMint,
  payer.publicKey
);
```

### 4. Test with Forked Mainnet

Use Anchor's test validator with cloned mainnet accounts:

```toml
# In Anchor.toml
[[test.validator.clone]]
address = "RAYDIUM_POOL_ADDRESS"

[[test.validator.clone]]
address = "WHIRLPOOL_ADDRESS"
```

## Debugging Tips

### 1. Enable Logging

```typescript
console.log("SwapArgs:", JSON.stringify(swapArgs, null, 2));
```

### 2. Check Account Balances

```typescript
const account = await getAccount(connection, tokenAccount);
console.log("Balance:", account.amount.toString());
```

### 3. Verify Transaction Logs

```typescript
try {
  const txSignature = await program.methods.swap(...).rpc();
  const tx = await connection.getTransaction(txSignature);
  console.log("Logs:", tx?.meta?.logMessages);
} catch (error) {
  console.error("Error:", error);
}
```

### 4. Test Validator Logs

Watch the test validator output for program logs:

```bash
solana logs | grep "Program C5x9ZuZETH3RA8QEU83xhFjCkGjPWVVzrWmkV4kS7pmR"
```

## Common Issues & Solutions

### Issue: "Cannot find module '@coral-xyz/anchor'"

**Solution:**
```bash
npm install --save-dev @coral-xyz/anchor
```

### Issue: "Error: failed to send transaction: Transaction simulation failed"

**Solution:** This is expected without real DEX integration. The tests show structure, not actual execution.

### Issue: "Type mismatch on SwapArgs"

**Solution:** Ensure you're using proper BN types and enum variants match the IDL.

### Issue: "Account not found"

**Solution:** Make sure you've built the program and the validator has the account:
```bash
anchor build
anchor test
```

## Next Steps

1. **Study DEX Adapters**: Look at `programs/dex-solana/src/adapters/` to understand how each DEX is integrated.

2. **Test with Real Pools**: Fork mainnet and test against actual pool accounts.

3. **Build Client SDK**: Use these patterns to build a client-side routing library.

4. **Optimize Routes**: Implement routing algorithms that generate optimal SwapArgs.

## Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Web3.js](https://solana-labs.github.io/solana-web3.js/)
- [SPL Token Program](https://spl.solana.com/token)
- [Project Phase 2 Roadmap](../phase2_roadmap.md)

## Support

For issues or questions:
1. Check existing tests for patterns
2. Review the `common_swap.rs` implementation
3. Examine DEX adapter implementations
4. Read the routing execution flow guide in `rust_tutorial/`

