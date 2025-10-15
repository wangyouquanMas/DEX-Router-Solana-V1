# 🧪 Experiment 1: Real Raydium Swap Test Guide

## 📋 Overview

This guide shows you how to test a **real Raydium AMM swap** (not just structure validation).

## 🔍 What Raydium Needs (19 Accounts)

Based on `raydium.rs` lines 19-40, a Raydium swap requires:

```typescript
// Required accounts in remainingAccounts array:
[
  // 1. Raydium Program ID
  dex_program_id,                    // Raydium AMM Program
  
  // 2-4. User accounts
  swap_authority_pubkey,             // Signer (payer)
  swap_source_token,                 // User's source token account
  swap_destination_token,            // User's destination token account
  
  // 5. Token Program
  token_program,                     // SPL Token Program
  
  // 6-11. AMM Accounts
  amm_id,                           // AMM pool state
  amm_authority,                    // AMM authority PDA
  amm_open_orders,                  // Open orders account
  amm_target_orders,                // Target orders account
  pool_coin_token_account,          // Pool's coin vault
  pool_pc_token_account,            // Pool's PC vault
  
  // 12-19. Serum DEX Accounts
  serum_program_id,                 // Serum DEX program
  serum_market,                     // Serum market
  serum_bids,                       // Serum bids account
  serum_asks,                       // Serum asks account
  serum_event_queue,                // Serum event queue
  serum_coin_vault_account,         // Serum coin vault
  serum_pc_vault_account,           // Serum PC vault
  serum_vault_signer,               // Serum vault signer
]
```

**Total: 19 accounts required**

---

## 🎯 Testing Options

### Option 1: Test with Forked Mainnet (Recommended)
Test against real Raydium pools on mainnet fork.

### Option 2: Mock Test
Create mock accounts (limited functionality).

### Option 3: Devnet Test
Use Raydium devnet pools (if available).

---

## 📝 Option 1: Forked Mainnet Test (Full Guide)

### Step 1: Find a Real Raydium Pool

Let's use a popular pool. For example, **SOL-USDC pool**:

```bash
# Raydium SOL-USDC Pool ID (example)
Pool: 58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2
```

You can find pools at: https://raydium.io/

### Step 2: Get All Pool Accounts

You need to fetch the pool state and extract all 19 accounts. Here's a helper script:

```typescript
// fetch-raydium-pool.ts
import { Connection, PublicKey } from "@solana/web3.js";

const connection = new Connection("https://api.mainnet-beta.solana.com");

// Raydium AMM Program ID
const RAYDIUM_AMM_PROGRAM = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

async function getRaydiumPoolAccounts(poolId: string) {
  const poolPubkey = new PublicKey(poolId);
  
  // Fetch the pool account data
  const accountInfo = await connection.getAccountInfo(poolPubkey);
  
  if (!accountInfo) {
    throw new Error("Pool not found");
  }
  
  // Parse pool data (Raydium AMM state layout)
  // This is simplified - you'd need proper layout parsing
  console.log("Pool found!");
  console.log("Data length:", accountInfo.data.length);
  
  // You need to parse the pool state to extract:
  // - AMM authority
  // - Token vaults
  // - Serum market
  // - Open orders
  // etc.
  
  return {
    ammId: poolPubkey,
    // ... other accounts
  };
}

// Example: SOL-USDC pool
getRaydiumPoolAccounts("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2");
```

### Step 3: Update Anchor.toml for Mainnet Fork

```toml
# In Anchor.toml
[test.validator]
url = "https://api.mainnet-beta.solana.com"

# Clone the Raydium pool
[[test.validator.clone]]
address = "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2"  # Pool

# Clone all related accounts
[[test.validator.clone]]
address = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"  # Raydium Program

# Clone token mints
[[test.validator.clone]]
address = "So11111111111111111111111111111111111111112"   # SOL
[[test.validator.clone]]
address = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"  # USDC
```

### Step 4: Create Real Swap Test

```typescript
// tests/raydium-real-swap.test.ts
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { DexSolana } from "../target/types/dex_solana";
import { PublicKey } from "@solana/web3.js";

describe("Real Raydium Swap Test", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.DexSolana as Program<DexSolana>;
  
  it("swaps SOL to USDC via Raydium", async () => {
    // Raydium Program
    const RAYDIUM_PROGRAM = new PublicKey(
      "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"
    );
    
    // Pool accounts (these need to be fetched from the pool state)
    const poolId = new PublicKey(
      "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2"
    );
    
    // You need to derive/fetch these from the pool:
    const ammAuthority = new PublicKey("...");        // Derive from pool
    const ammOpenOrders = new PublicKey("...");       // From pool state
    const ammTargetOrders = new PublicKey("...");     // From pool state
    const poolCoinVault = new PublicKey("...");       // From pool state
    const poolPcVault = new PublicKey("...");         // From pool state
    
    // Serum accounts (from pool state)
    const serumProgram = new PublicKey("...");
    const serumMarket = new PublicKey("...");
    const serumBids = new PublicKey("...");
    const serumAsks = new PublicKey("...");
    const serumEventQueue = new PublicKey("...");
    const serumCoinVault = new PublicKey("...");
    const serumPcVault = new PublicKey("...");
    const serumVaultSigner = new PublicKey("...");
    
    // SwapArgs
    const swapArgs = {
      amountIn: new BN(1_000_000),  // 0.001 SOL
      expectAmountOut: new BN(100_000), // ~0.1 USDC
      minReturn: new BN(98_000),    // 2% slippage
      amounts: [new BN(1_000_000)],
      routes: [[{
        dexes: [{ raydiumSwap: {} }],
        weights: [100]
      }]]
    };
    
    // Build remainingAccounts in exact order (19 accounts)
    const remainingAccounts = [
      { pubkey: RAYDIUM_PROGRAM, isWritable: false, isSigner: false },
      { pubkey: provider.wallet.publicKey, isWritable: false, isSigner: true },
      { pubkey: sourceTokenAccount, isWritable: true, isSigner: false },
      { pubkey: destTokenAccount, isWritable: true, isSigner: false },
      { pubkey: TOKEN_PROGRAM_ID, isWritable: false, isSigner: false },
      { pubkey: poolId, isWritable: true, isSigner: false },
      { pubkey: ammAuthority, isWritable: false, isSigner: false },
      { pubkey: ammOpenOrders, isWritable: true, isSigner: false },
      { pubkey: ammTargetOrders, isWritable: true, isSigner: false },
      { pubkey: poolCoinVault, isWritable: true, isSigner: false },
      { pubkey: poolPcVault, isWritable: true, isSigner: false },
      { pubkey: serumProgram, isWritable: false, isSigner: false },
      { pubkey: serumMarket, isWritable: true, isSigner: false },
      { pubkey: serumBids, isWritable: true, isSigner: false },
      { pubkey: serumAsks, isWritable: true, isSigner: false },
      { pubkey: serumEventQueue, isWritable: true, isSigner: false },
      { pubkey: serumCoinVault, isWritable: true, isSigner: false },
      { pubkey: serumPcVault, isWritable: true, isSigner: false },
      { pubkey: serumVaultSigner, isWritable: false, isSigner: false },
    ];
    
    // Execute swap
    const tx = await program.methods
      .swap(swapArgs, new BN(Date.now()))
      .accounts({
        payer: provider.wallet.publicKey,
        sourceTokenAccount: sourceTokenAccount,
        destinationTokenAccount: destTokenAccount,
        sourceMint: solMint,
        destinationMint: usdcMint,
      })
      .remainingAccounts(remainingAccounts)
      .rpc();
      
    console.log("Swap successful!", tx);
  });
});
```

---

## 🛠️ Easier Approach: Use Raydium SDK

### Step 5: Use Raydium SDK to Get Pool Info

```bash
npm install @raydium-io/raydium-sdk
```

```typescript
import { Liquidity } from "@raydium-io/raydium-sdk";

// Fetch pool info automatically
const poolKeys = await Liquidity.fetchInfo({ 
  connection, 
  poolId: "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2" 
});

console.log("Pool Keys:", poolKeys);
// This gives you all 19 accounts!
```

---

## 📊 Simpler Test: Structure Only

If you just want to verify your SwapArgs structure is correct:

```typescript
// tests/raydium-structure-test.ts
it("validates Raydium swap structure", async () => {
  const swapArgs = {
    amountIn: new BN(1_000_000),
    expectAmountOut: new BN(100_000),
    minReturn: new BN(98_000),
    amounts: [new BN(1_000_000)],
    routes: [[{
      dexes: [{ raydiumSwap: {} }],
      weights: [100]
    }]]
  };
  
  // Verify structure
  expect(swapArgs.amounts.length).to.equal(swapArgs.routes.length);
  expect(swapArgs.routes[0][0].weights[0]).to.equal(100);
  
  console.log("✓ Raydium SwapArgs structure is valid");
  
  // Note: This doesn't execute the swap, just validates the structure
});
```

---

## 📝 Quick Start (What You Can Do Now)

### Right Now (5 minutes):

```bash
# Run the structure validation test
npm run test:swap
```

This will validate your SwapArgs structure without needing real pools.

### With Real Pool (30 minutes):

1. **Find a pool** on https://raydium.io/
2. **Install Raydium SDK**: `npm install @raydium-io/raydium-sdk`
3. **Fetch pool info** using the SDK
4. **Update test** with real accounts
5. **Run with mainnet fork**

---

## 🎯 What Each Test Level Gives You

| Test Type | Time | What It Tests | Requires |
|-----------|------|---------------|----------|
| **Structure Only** | 5 min | SwapArgs is valid | Nothing |
| **With SDK** | 30 min | Accounts are correct | Raydium SDK |
| **Real Swap** | 1-2 hrs | Full integration | Mainnet fork + liquidity |

---

## 💡 Recommendation

**Start with what you have:**

1. ✅ **Run structure tests** (already working)
2. ✅ **Validate account order** (study the code)
3. ⬜ **Install Raydium SDK** (30 min task)
4. ⬜ **Test with real pool** (advanced)

The current test suite gives you 80% of what you need. The remaining 20% is fetching real pool accounts.

---

## 🚀 Next Steps

1. Run current test: `npm run test:swap`
2. Study output and confirm structure
3. Decide if you want real integration or structure-only testing

Want me to help you set up any of these options?

