
# 🏗️ Phase 2: Core Routing Engine - Data Structures Study Guide

## 📚 Overview
This phase focuses on understanding how swap routes are represented and executed in the DEX Router. The routing engine is the brain of the protocol - it determines how tokens flow through different DEXs to achieve optimal swap prices.

---

## 1️⃣ Dex Enum - The DEX Universe

### Location
```rust
programs/dex-solana/src/instructions/common_swap.rs
Lines 10-77
```

### What It Is
An enum representing **67 different DEX protocols** that the router can interact with.

### Key DEX Categories

#### 🔄 AMM DEXs (Automated Market Maker)
```rust
SplTokenSwap,          // Basic SPL token swap
StableSwap,            // Stablecoin optimized AMM
RaydiumSwap,           // Raydium v1
RaydiumCpmmSwap,       // Raydium Concentrated Liquidity
```

#### 📊 Concentrated Liquidity (CLMM)
```rust
Whirlpool,             // Orca's CLMM
WhirlpoolV2,           // Orca V2
RaydiumClmmSwap,       // Raydium CLMM
RaydiumClmmSwapV2,     // Raydium CLMM V2
MeteoraDlmm,           // Meteora Dynamic Liquidity
ByrealClmm,            // Byreal CLMM
PancakeSwapV3Swap,     // PancakeSwap V3
```

#### 📖 Order Book DEXs
```rust
OpenBookV2,            // OpenBook V2
Phoenix,               // Phoenix DEX
Manifest,              // Manifest
```

#### 🚀 Specialized Protocols
```rust
PumpfunBuy/Sell,       // Pump.fun meme token platform
BoopfunBuy/Sell,       // Boop.fun
SanctumRouter,         // Liquid staking token router
MeteoraVaultDeposit,   // Meteora vault operations
PerpetualsSwap,        // Perpetuals protocol
```

#### 🎯 Why So Many DEXs?
- **Liquidity fragmentation**: Different DEXs have liquidity for different token pairs
- **Price optimization**: Router can split trades across multiple DEXs for better prices
- **Specialized pools**: Some DEXs are optimized for specific use cases (stablecoins, LSTs, meme tokens)

---

## 2️⃣ HopAccounts - Inter-Hop State Tracking

### Location
```rust
Lines 79-84
```

### Structure
```rust
pub struct HopAccounts {
    pub last_to_account: Pubkey,   // Previous hop's destination account
    pub from_account: Pubkey,       // Current hop's source account
    pub to_account: Pubkey,         // Current hop's destination account
}
```

### Purpose
Tracks token account transitions during **multi-hop swaps**.

### Example Flow
```
Swap: USDC → SOL → BONK (2 hops)

Hop 1 (USDC → SOL):
  last_to_account: 0x000... (no previous hop)
  from_account:    User's USDC account
  to_account:      Intermediate SOL account

Hop 2 (SOL → BONK):
  last_to_account: Intermediate SOL account (from hop 1)
  from_account:    Intermediate SOL account
  to_account:      User's BONK account
```

### Validation (see lines 523-536)
- **First hop**: `from_account` must match user's source token account
- **Last hop**: `to_account` must match user's destination token account
- **Intermediate hops**: `from_account` must equal previous `to_account`

---

## 3️⃣ Route - DEX Path with Weights

### Location
```rust
Lines 86-90
```

### Structure
```rust
pub struct Route {
    pub dexes: Vec<Dex>,      // List of DEXs to use
    pub weights: Vec<u8>,     // Percentage split for each DEX
}
```

### Key Constraints
- `dexes.len()` must equal `weights.len()` (line 459)
- Sum of weights must equal 100 (line 465: `TOTAL_WEIGHT`)
- Weights represent percentage allocation (e.g., 40 = 40%)

### Examples

#### Single DEX Route
```rust
Route {
    dexes: vec![Dex::RaydiumSwap],
    weights: vec![100],  // 100% through Raydium
}
```

#### Split Route (2 DEXs)
```rust
Route {
    dexes: vec![Dex::RaydiumSwap, Dex::Whirlpool],
    weights: vec![60, 40],  // 60% Raydium, 40% Whirlpool
}
```

#### Triple Split Route
```rust
Route {
    dexes: vec![
        Dex::RaydiumSwap,
        Dex::Whirlpool,
        Dex::MeteoraDynamicpool
    ],
    weights: vec![50, 30, 20],  // 50% + 30% + 20% = 100%
}
```

---

## 4️⃣ SwapArgs - The Complete Swap Specification

### Location
```rust
Lines 92-99
```

### Structure
```rust
pub struct SwapArgs {
    pub amount_in: u64,                 // Total input amount
    pub expect_amount_out: u64,         // Expected output (for slippage check)
    pub min_return: u64,                // Minimum acceptable output
    pub amounts: Vec<u64>,              // Level 1 split amounts
    pub routes: Vec<Vec<Route>>,        // Level 2 routes (hops × splits)
}
```

### Field Explanations

#### 1. `amount_in` (Total input)
The total amount of source tokens to swap.

#### 2. `expect_amount_out` (Expected output)
The amount you expect to receive based on current prices. Used for:
- Slippage validation
- Front-end price display
- Must be ≥ `min_return` (line 430)

#### 3. `min_return` (Slippage protection)
The minimum amount you'll accept. Transaction reverts if:
```rust
actual_output < min_return  // Line 400-402
```

Example: If you expect 100 USDC but set min_return = 98, you accept up to 2% slippage.

#### 4. `amounts` (Level 1 splits)
How to split the input across different route paths.

**Constraint**: Sum must equal `amount_in` (line 442)

#### 5. `routes` (Level 2 multi-hop routes)
A 2D array representing:
- **Outer Vec**: Different parallel paths (level 1 splits)
- **Inner Vec**: Sequential hops (multi-hop path)
- Each hop is a `Route` (can contain multiple DEXs with weights)

**Constraint**: `amounts.len() == routes.len()` (line 434)

---

## 🎯 Checkpoint Challenge: 2-Hop, 3-DEX Swap

### Scenario
Swap 1000 USDC to BONK via:
- **Hop 1**: USDC → SOL using 3 DEXs (Raydium 50%, Whirlpool 30%, Meteora 20%)
- **Hop 2**: SOL → BONK using 1 DEX (Raydium 100%)

### Solution

```rust
let swap_args = SwapArgs {
    // Total input: 1000 USDC (with 6 decimals)
    amount_in: 1_000_000_000,  // 1000 * 10^6
    
    // Expected to receive ~50,000 BONK (example)
    expect_amount_out: 50_000_000_000,  // 50k * 10^6
    
    // Accept minimum 49,000 BONK (2% slippage)
    min_return: 49_000_000_000,  // 49k * 10^6
    
    // Level 1: Only 1 path, so all amount goes to it
    amounts: vec![1_000_000_000],
    
    // Level 2: 2 hops
    routes: vec![
        vec![
            // Hop 1: USDC → SOL (3 DEXs split)
            Route {
                dexes: vec![
                    Dex::RaydiumSwap,
                    Dex::Whirlpool,
                    Dex::MeteoraDynamicpool,
                ],
                weights: vec![50, 30, 20],  // Must sum to 100
            },
            // Hop 2: SOL → BONK (1 DEX)
            Route {
                dexes: vec![Dex::RaydiumSwap],
                weights: vec![100],
            },
        ],
    ],
};
```

### Execution Flow

```
Input: 1000 USDC
  │
  ├─ Level 1 Split: amounts[0] = 1000 USDC (100%)
  │
  └─ routes[0]: 2 hops
      │
      ├─ Hop 1 (USDC → SOL):
      │   ├─ Raydium:  500 USDC (50%) → ~0.05 SOL
      │   ├─ Whirlpool: 300 USDC (30%) → ~0.03 SOL
      │   └─ Meteora:   200 USDC (20%) → ~0.02 SOL
      │   Total: ~0.1 SOL
      │
      └─ Hop 2 (SOL → BONK):
          └─ Raydium: 0.1 SOL (100%) → ~50,000 BONK
          
Final Output: ~50,000 BONK (checked against min_return)
```

---

## 📊 Advanced Example: Complex Multi-Path Swap

### Scenario
Want to optimize by using **2 parallel paths**:
- Path A: Direct USDC → BONK (if liquidity exists)
- Path B: USDC → SOL → BONK (the 2-hop route)

### Solution

```rust
let advanced_swap = SwapArgs {
    amount_in: 1_000_000_000,       // 1000 USDC
    expect_amount_out: 50_000_000_000,
    min_return: 49_000_000_000,
    
    // Level 1: Split across 2 paths
    amounts: vec![
        300_000_000,  // 300 USDC to Path A (direct)
        700_000_000,  // 700 USDC to Path B (2-hop)
    ],
    
    // Level 2: Each path's hop structure
    routes: vec![
        // Path A: Direct USDC → BONK (1 hop, 2 DEXs)
        vec![
            Route {
                dexes: vec![Dex::RaydiumSwap, Dex::Whirlpool],
                weights: vec![60, 40],
            },
        ],
        
        // Path B: USDC → SOL → BONK (2 hops, 3 DEXs in first hop)
        vec![
            Route {
                dexes: vec![
                    Dex::RaydiumSwap,
                    Dex::Whirlpool,
                    Dex::MeteoraDynamicpool,
                ],
                weights: vec![50, 30, 20],
            },
            Route {
                dexes: vec![Dex::RaydiumSwap],
                weights: vec![100],
            },
        ],
    ],
};
```

### Execution Visualization

```
1000 USDC Input
  │
  ├─────────────────────┬─────────────────────┐
  │                     │                     │
  300 USDC (30%)        700 USDC (70%)        
  Path A                Path B                
  │                     │                     
  ├─ Hop 1:             ├─ Hop 1:             
  │  ├─ Raydium 60%     │  ├─ Raydium 50%     
  │  └─ Whirlpool 40%   │  ├─ Whirlpool 30%   
  │  → BONK directly    │  └─ Meteora 20%     
  │                     │  → SOL              
  │                     │                     
  │                     └─ Hop 2:             
  │                        └─ Raydium 100%    
  │                        → BONK             
  │                     │                     
  └─────────────────────┴─────────────────────┘
                        │
                   ~50,000 BONK
```

---

## 🔍 Code Analysis: How Routes Are Executed

### Key Function: `execute_swap` (lines 406-548)

#### Step 1: Validation (lines 420-444)
```rust
// Check amounts match routes
require!(amounts.len() == routes.len());

// Check total amounts equal input
let total_amounts: u64 = amounts.iter().sum();
require!(total_amounts == real_amount_in);
```

#### Step 2: Level 1 Split Loop (line 449)
```rust
for (i, hops) in routes.iter().enumerate() {
    let mut amount_in = amounts[i];  // Get this path's amount
    // ... process hops
}
```

#### Step 3: Multi-Hop Loop (line 455)
```rust
for (hop, route) in hops.iter().enumerate() {
    // Validate weights
    require!(dexes.len() == weights.len());
    require!(weights.sum() == 100);
    // ... process DEX splits
}
```

#### Step 4: Level 2 DEX Split (lines 475-521)
```rust
for (index, dex) in dexes.iter().enumerate() {
    // Calculate split amount based on weight
    let fork_amount_in = if index == dexes.len() - 1 {
        amount_in - acc_fork_in  // Last dex gets remainder
    } else {
        amount_in * weights[index] / 100
    };
    
    // Execute swap on this DEX
    let fork_amount_out = distribute_swap(dex, ...);
    
    // Emit event
    emit!(SwapEvent { dex, amount_in: fork_amount_in, amount_out: fork_amount_out });
}
```

---

## 💡 Key Insights

### 1. Two-Level Split Architecture
- **Level 1**: Parallel paths (different routing strategies)
- **Level 2**: Within each path, multi-hop routing with DEX splits per hop

### 2. Weight-Based Splitting
- Weights are percentages (sum to 100)
- Last DEX in a split gets remainder to avoid rounding errors (line 477)

### 3. Account Validation
- First hop's source must be user's source account
- Last hop's destination must be user's destination account
- Intermediate hops must chain correctly

### 4. Slippage Protection
- `expect_amount_out` must be ≥ `min_return`
- Final output checked against `min_return` (line 400)
- Transaction reverts if slippage exceeded

---

## ✅ Phase 2 Checklist

- [x] **Understand Dex enum**: 67 different DEX protocols
- [x] **Understand Route structure**: dexes + weights (must sum to 100)
- [x] **Understand SwapArgs**: amounts, routes, slippage protection
- [x] **Understand HopAccounts**: tracking inter-hop token accounts
- [x] **Can construct a 2-hop, 3-DEX swap**: See checkpoint solution above

---

## 🚀 Next Steps

### Practice Exercises

1. **Single-Hop, Dual-DEX Swap**
   - Swap 500 USDC → USDT
   - Split 70% Raydium, 30% Whirlpool
   - Write the `SwapArgs`

2. **Triple-Hop Swap**
   - Swap USDC → SOL → RAY → BONK
   - Use different DEXs for each hop
   - Write the `SwapArgs`

3. **Complex Multi-Path**
   - Design a 3-path swap with different hop counts
   - Ensure all constraints are met
   - Calculate total DEX interactions

### Deep Dive Topics

- How does `distribute_swap` match DEX types to swap functions? (lines 561-669)
- How are intermediate token accounts created and managed?
- How does the router handle failed swaps in one split path?
- What are the gas optimization strategies in weight calculations?

---

## 📚 Related Files to Study Next

1. `programs/dex-solana/src/processor/mod.rs` - Swap processors
2. `programs/dex-solana/src/adapters/` - Individual DEX adapters
3. `programs/dex-solana/src/utils/` - Helper functions for account management

---

**🎓 Phase 2 Complete!** You now understand the core data structures that power the DEX Router's multi-hop, multi-DEX swap capabilities.
