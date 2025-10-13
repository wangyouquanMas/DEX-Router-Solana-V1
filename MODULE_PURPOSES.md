# DEX-Router-Solana-V1 Module Architecture Guide

## 📁 Project Structure Overview

This document explains the purpose and responsibility of each module in the DEX Router Solana V1 project.

---

## 🎨 Visual Architecture Diagrams

### 1. High-Level System Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         USER / CLIENT APPLICATION                        │
│                    (Web3 Wallet, dApp, Trading Bot)                     │
└─────────────────────────────────┬───────────────────────────────────────┘
                                  │
                                  │ RPC Call
                                  ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           SOLANA BLOCKCHAIN                              │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │               DEX ROUTER PROGRAM (lib.rs)                         │  │
│  │              Program ID: C5x9...pmR                               │  │
│  │                                                                   │  │
│  │  ┌─────────────────────────────────────────────────────────┐    │  │
│  │  │  INSTRUCTION LAYER (/instructions/)                     │    │  │
│  │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐│    │  │
│  │  │  │  swap    │  │commission│  │  proxy   │  │ swap_v3 ││    │  │
│  │  │  │ handler  │  │  handler │  │ handler  │  │ handler ││    │  │
│  │  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬────┘│    │  │
│  │  │       │             │             │             │      │    │  │
│  │  │       └─────────────┴─────────────┴─────────────┘      │    │  │
│  │  │                          │                             │    │  │
│  │  │                 ┌────────▼────────┐                    │    │  │
│  │  │                 │ common_swap.rs  │                    │    │  │
│  │  │                 │  • SwapArgs     │                    │    │  │
│  │  │                 │  • Route        │                    │    │  │
│  │  │                 │  • Dex enum     │                    │    │  │
│  │  │                 │  • HopAccounts  │                    │    │  │
│  │  │                 └────────┬────────┘                    │    │  │
│  │  └──────────────────────────┼─────────────────────────────┘    │  │
│  │                             │                                  │  │
│  │  ┌──────────────────────────▼──────────────────────────────┐  │  │
│  │  │  ADAPTER LAYER (/adapters/)                             │  │  │
│  │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐│  │  │
│  │  │  │ Raydium  │  │Whirlpool │  │ Meteora  │  │  40+    ││  │  │
│  │  │  │ Adapter  │  │ Adapter  │  │ Adapter  │  │  More   ││  │  │
│  │  │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬────┘│  │  │
│  │  │       │             │             │             │      │  │  │
│  │  │       │   CPI (Cross-Program Invocation)        │      │  │  │
│  │  │       ▼             ▼             ▼             ▼      │  │  │
│  │  └───────┼─────────────┼─────────────┼─────────────┼──────┘  │  │
│  │          │             │             │             │         │  │
│  │  ┌───────▼──────┐ ┌────▼─────┐ ┌────▼─────┐ ┌────▼─────┐   │  │
│  │  │   Raydium    │ │  Orca    │ │ Meteora  │ │ Phoenix  │   │  │
│  │  │   Program    │ │ Program  │ │ Program  │ │ Program  │   │  │
│  │  └──────────────┘ └──────────┘ └──────────┘ └──────────┘   │  │
│  │                                                              │  │
│  │  ┌───────────────────────────────────────────────────────┐  │  │
│  │  │  UTILITY LAYER (/utils/)                              │  │  │
│  │  │  • token.rs (SPL Token operations)                    │  │  │
│  │  │  • fee.rs (Fee calculations)                          │  │  │
│  │  │  • swap.rs (Swap math & validation)                   │  │  │
│  │  │  • logging.rs (Event emission)                        │  │  │
│  │  └───────────────────────────────────────────────────────┘  │  │
│  │                                                              │  │
│  │  ┌───────────────────────────────────────────────────────┐  │  │
│  │  │  STATE LAYER (/state/)                                │  │  │
│  │  │  • config.rs (GlobalConfig account)                   │  │  │
│  │  │  • order.rs (Limit order accounts)                    │  │  │
│  │  │  • event.rs (Event definitions)                       │  │  │
│  │  └───────────────────────────────────────────────────────┘  │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────────┘
```

---

### 2. Swap Execution Flow (Detailed Sequence)

```
USER: "Swap 1000 USDC → SOL using 60% Raydium + 40% Whirlpool"
│
│ 1. Build SwapArgs
│    ├─ amount_in: 1,000,000 (1000 USDC with 6 decimals)
│    ├─ min_return: 970,000 (0.97 SOL minimum)
│    └─ routes: [
│         Route { dexes: [Raydium, Whirlpool], weights: [60, 40] }
│       ]
│
▼
┌───────────────────────────────────────────────────────────────┐
│  STEP 1: lib.rs::swap()                                       │
│  • Receives transaction                                       │
│  • Validates signature & accounts                             │
│  • Routes to instruction handler                              │
└───────────────────────────┬───────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────────┐
│  STEP 2: instructions/swap.rs::swap_handler()                 │
│  • Validates SwapArgs structure                               │
│  • Checks user token balances                                 │
│  • Verifies accounts ownership                                │
│  • Calls common swap logic                                    │
└───────────────────────────┬───────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────────┐
│  STEP 3: instructions/common_swap.rs::distribute_swap()       │
│  • Parses Route: [Raydium: 60%, Whirlpool: 40%]              │
│  • Splits input: 600 USDC + 400 USDC                         │
│  • Initializes HopAccounts tracking                           │
│  • Loops through each DEX in route                            │
└─────────────┬─────────────────────────────┬───────────────────┘
              │                             │
              ▼                             ▼
┌──────────────────────────────┐  ┌─────────────────────────────┐
│  STEP 4A: Raydium Path       │  │  STEP 4B: Whirlpool Path    │
│  (60% = 600 USDC)            │  │  (40% = 400 USDC)           │
└──────────────┬───────────────┘  └─────────────┬───────────────┘
               │                                 │
               ▼                                 ▼
┌────────────────────────────────┐  ┌───────────────────────────┐
│  adapters/raydium.rs           │  │  adapters/whirlpool.rs    │
│  ┌──────────────────────────┐  │  │  ┌─────────────────────┐  │
│  │ 1. before_invoke()       │  │  │  │ 1. before_invoke()  │  │
│  │    • Get pre-swap balance│  │  │  │    • Check balance  │  │
│  │                          │  │  │  │                     │  │
│  │ 2. Build swap IX         │  │  │  │ 2. Build swap IX    │  │
│  │    • Prepare accounts    │  │  │  │    • Prepare pools  │  │
│  │    • Set amount: 600 USDC│  │  │  │    • Set amt: 400   │  │
│  │                          │  │  │  │                     │  │
│  │ 3. Execute CPI           │  │  │  │ 3. Execute CPI      │  │
│  │    ┌─────────────────┐   │  │  │  │    ┌──────────────┐ │  │
│  │    │ Raydium Program │   │  │  │  │    │Orca Program  │ │  │
│  │    │ Executes swap   │   │  │  │  │    │Executes swap │ │  │
│  │    │ Returns ~0.588  │   │  │  │  │    │Returns ~0.392│ │  │
│  │    │ SOL             │   │  │  │  │    │SOL           │ │  │
│  │    └─────────────────┘   │  │  │  │    └──────────────┘ │  │
│  │                          │  │  │  │                     │  │
│  │ 4. after_invoke()        │  │  │  │ 4. after_invoke()   │  │
│  │    • Verify output       │  │  │  │    • Verify output  │  │
│  │    • Calculate received  │  │  │  │    • Update balance │  │
│  └──────────────────────────┘  │  │  └─────────────────────┘  │
└────────────┬───────────────────┘  └─────────────┬─────────────┘
             │                                     │
             └──────────────┬──────────────────────┘
                            ▼
┌───────────────────────────────────────────────────────────────┐
│  STEP 5: Aggregate Results                                    │
│  • Raydium output: 0.588 SOL                                  │
│  • Whirlpool output: 0.392 SOL                                │
│  • Total received: 0.98 SOL                                   │
│  • Check: 0.98 >= min_return (0.97) ✓                        │
└───────────────────────────┬───────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────────┐
│  STEP 6: utils/ Operations                                    │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │ token.rs: Transfer 0.98 SOL to user's account          │  │
│  ├─────────────────────────────────────────────────────────┤  │
│  │ fee.rs: Calculate & distribute platform fees           │  │
│  ├─────────────────────────────────────────────────────────┤  │
│  │ logging.rs: Emit SwapEvent                              │  │
│  │   {                                                     │  │
│  │     user: <user_pubkey>,                                │  │
│  │     input_mint: USDC,                                   │  │
│  │     output_mint: SOL,                                   │  │
│  │     amount_in: 1000,                                    │  │
│  │     amount_out: 0.98,                                   │  │
│  │     dexes_used: [Raydium, Whirlpool]                    │  │
│  │   }                                                     │  │
│  └─────────────────────────────────────────────────────────┘  │
└───────────────────────────┬───────────────────────────────────┘
                            │
                            ▼
┌───────────────────────────────────────────────────────────────┐
│  STEP 7: Transaction Complete                                 │
│  ✅ User receives 0.98 SOL                                    │
│  ✅ Event logged on-chain                                     │
│  ✅ UI can display swap details                               │
└───────────────────────────────────────────────────────────────┘
```

---

### 3. Adapter Pattern Implementation

```
┌─────────────────────────────────────────────────────────────────────┐
│  THE ADAPTER PATTERN PROBLEM                                        │
│                                                                     │
│  Challenge: 40+ DEXs, each with different interfaces                │
│  • Raydium: Uses AMM pools, specific account structure              │
│  • Orca: Concentrated liquidity, different math                     │
│  • Phoenix: Orderbook-based, completely different model             │
│  • Meteora: Dynamic bins, unique parameters                         │
│                                                                     │
│  Without adapters: Router needs 40+ different code paths! 😱        │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│  THE SOLUTION: UNIFIED ADAPTER INTERFACE                            │
└─────────────────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────┐
│  adapters/common.rs                                           │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │  pub trait DexProcessor {                               │  │
│  │                                                          │  │
│  │      fn before_invoke(                                  │  │
│  │          &self,                                          │  │
│  │          account_infos: &[AccountInfo]                   │  │
│  │      ) -> Result<u64>;                                   │  │
│  │                                                          │  │
│  │      fn after_invoke(                                   │  │
│  │          &self,                                          │  │
│  │          account_infos: &[AccountInfo],                  │  │
│  │          hop: usize,                                     │  │
│  │          owner_seeds: Option<&[&[&[u8]]]>               │  │
│  │      ) -> Result<u64>;                                   │  │
│  │  }                                                       │  │
│  └─────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────┘
                              │
                              │ ALL adapters implement this trait
                              │
        ┌─────────────────────┴─────────────────────┬──────────────────┐
        │                     │                     │                  │
        ▼                     ▼                     ▼                  ▼
┌───────────────┐    ┌───────────────┐    ┌───────────────┐   ┌──────────┐
│ raydium.rs    │    │ whirlpool.rs  │    │ meteora.rs    │   │  40+     │
├───────────────┤    ├───────────────┤    ├───────────────┤   │  more... │
│impl           │    │impl           │    │impl           │   └──────────┘
│DexProcessor { │    │DexProcessor { │    │DexProcessor { │
│               │    │               │    │               │
│ before_invoke │    │ before_invoke │    │ before_invoke │
│  → Get AMM    │    │  → Get CLMM   │    │  → Get DLMM   │
│    pool data  │    │    tick data  │    │    bin data   │
│               │    │               │    │               │
│ after_invoke  │    │ after_invoke  │    │ after_invoke  │
│  → Verify AMM │    │  → Verify     │    │  → Verify     │
│    swap result│    │    position   │    │    bins       │
│}              │    │}              │    │}              │
└───────────────┘    └───────────────┘    └───────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│  ROUTER USAGE (Simplified)                                          │
│  ┌───────────────────────────────────────────────────────────────┐  │
│  │  match dex_type {                                             │  │
│  │      Dex::RaydiumSwap => {                                    │  │
│  │          let adapter = RaydiumAdapter::new();                 │  │
│  │          adapter.before_invoke(accounts)?;  // Unified call   │  │
│  │          // Execute swap...                                   │  │
│  │          adapter.after_invoke(accounts)?;   // Unified call   │  │
│  │      },                                                        │  │
│  │      Dex::Whirlpool => {                                      │  │
│  │          let adapter = WhirlpoolAdapter::new();               │  │
│  │          adapter.before_invoke(accounts)?;  // Same interface │  │
│  │          // Execute swap...                                   │  │
│  │          adapter.after_invoke(accounts)?;   // Same interface │  │
│  │      },                                                        │  │
│  │      // ... 40+ more DEXs, all using the same interface!     │  │
│  │  }                                                             │  │
│  └───────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────────┘

Benefits:
✅ Router code stays clean - doesn't need DEX-specific logic
✅ Easy to add new DEXs - just implement DexProcessor trait
✅ Each adapter encapsulates DEX complexity
✅ Testable in isolation
```

---

### 4. Data Structure Hierarchy

```
┌────────────────────────────────────────────────────────────────────┐
│  SwapArgs (The Complete Swap Specification)                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │  pub struct SwapArgs {                                       │  │
│  │      pub amount_in: u64,        // Total input: 1000 USDC   │  │
│  │      pub expect_amount_out: u64, // Expected: ~0.98 SOL     │  │
│  │      pub min_return: u64,       // Minimum: 0.97 SOL        │  │
│  │      pub amounts: Vec<u64>,     // Level 1 split ────────┐  │  │
│  │      pub routes: Vec<Vec<Route>>, // Level 2 split ─────┐│  │  │
│  │  }                                                        ││  │  │
│  └───────────────────────────────────────────────────────────┼┼──┘  │
└──────────────────────────────────────────────────────────────┼┼─────┘
                                                               ││
┌──────────────────────────────────────────────────────────────┘│
│  Level 1 Split: Route-Level Distribution                      │
│  amounts: [600, 400]  ← Split input into parallel routes     │
│                                                               │
│  Route 1: 600 USDC ─────┐       Route 2: 400 USDC ─────┐    │
│                         │                               │    │
│                         ▼                               ▼    │
│                  ┌─────────────┐               ┌─────────────┐│
│                  │  Vec<Route> │               │  Vec<Route> ││
│                  │  (Multi-hop)│               │  (Multi-hop)││
│                  └──────┬──────┘               └──────┬──────┘│
│                         │                             │       │
│                         ▼                             ▼       │
│                   [ Route 1 ]                   [ Route 1,    │
│                                                   Route 2 ]   │
│                                                  (2-hop path) │
└───────────────────────────┬──────────────────────────┬────────┘
                            │                          │
                            ▼                          │
┌────────────────────────────────────────────────────┐ │
│  Level 2 Split: DEX-Weight Distribution            │ │
│  ┌──────────────────────────────────────────────┐  │ │
│  │  pub struct Route {                          │  │ │
│  │      pub dexes: Vec<Dex>,   // DEX list ─────┼──┼─┤
│  │      pub weights: Vec<u8>,  // Weight % ─────┼──┼─┤
│  │  }                                           │  │ │
│  └──────────────────────────────────────────────┘  │ │
└─────────────────────────────┬──────────────────────┘ │
                              │                        │
                              ▼                        ▼
┌──────────────────────────────────────┐  ┌───────────────────────┐
│  Route Example 1 (Simple)            │  │  Route Example 2      │
│  ┌────────────────────────────────┐  │  │  (Split)              │
│  │  dexes: [Raydium]              │  │  │  ┌─────────────────┐  │
│  │  weights: [100]                │  │  │  │ dexes:          │  │
│  │                                │  │  │  │  [Whirlpool,    │  │
│  │  Meaning:                      │  │  │  │   Meteora]      │  │
│  │  100% via Raydium              │  │  │  │                 │  │
│  └────────────────────────────────┘  │  │  │ weights: [60,40]│  │
└──────────────────────────────────────┘  │  │                 │  │
                                          │  │ Meaning:        │  │
                                          │  │ 60% Whirlpool   │  │
                                          │  │ 40% Meteora     │  │
                                          │  └─────────────────┘  │
                                          └───────────────────────┘
                              │
                              ▼
┌────────────────────────────────────────────────────────────────┐
│  Dex Enum (77 Supported DEXs)                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  #[derive(AnchorSerialize, AnchorDeserialize, Clone)]   │  │
│  │  pub enum Dex {                                          │  │
│  │      SplTokenSwap,          // #0                        │  │
│  │      Whirlpool,             // #1                        │  │
│  │      RaydiumSwap,           // #2                        │  │
│  │      Meteora,               // #3                        │  │
│  │      MeteoraDlmm,           // #4                        │  │
│  │      OpenBookV2,            // #5                        │  │
│  │      Phoenix,               // #6                        │  │
│  │      Pumpfun,               // #7                        │  │
│  │      // ... 69 more DEXs                                 │  │
│  │  }                                                        │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────┐
│  HopAccounts (Multi-Hop State Tracking)                        │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  pub struct HopAccounts {                                │  │
│  │      pub last_to_account: Pubkey,   // Previous hop out │  │
│  │      pub from_account: Pubkey,      // Current hop in   │  │
│  │      pub to_account: Pubkey,        // Current hop out  │  │
│  │  }                                                       │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                │
│  Example: USDC → USDT → SOL (2-hop)                           │
│                                                                │
│  Hop 1 State:                                                  │
│    last_to_account: ZERO (first hop)                          │
│    from_account: user_usdc_account                            │
│    to_account: temp_usdt_account                              │
│                                                                │
│  Hop 2 State:                                                  │
│    last_to_account: temp_usdt_account  ← Must match Hop 1 out│
│    from_account: temp_usdt_account                            │
│    to_account: user_sol_account                               │
└────────────────────────────────────────────────────────────────┘
```

---

### 5. Module Dependency Graph (Detailed)

```
                    ┌─────────────────────────────┐
                    │        lib.rs               │
                    │   (Program Entry Point)     │
                    │                             │
                    │  • Declares program ID      │
                    │  • Exposes instructions     │
                    │  • Links all modules        │
                    └──────────┬──────────────────┘
                               │
                ┌──────────────┼──────────────┐
                │              │              │
                ▼              ▼              ▼
    ┌─────────────────┐  ┌─────────────┐  ┌──────────────┐
    │  instructions/  │  │   state/    │  │global_config/│
    │                 │  │             │  │              │
    │  Entry points   │  │ Data schemas│  │Admin funcs   │
    └────────┬────────┘  └─────────────┘  └──────────────┘
             │
             ├─────────┬──────────┬──────────┬──────────┬──────────┐
             ▼         ▼          ▼          ▼          ▼          ▼
    ┌─────────────┐ ┌────────┐ ┌──────────┐ ┌───────┐ ┌────────┐ ┌─────┐
    │common_swap  │ │ swap   │ │commission│ │ proxy │ │swap_v3 │ │ ... │
    │    .rs      │ │  .rs   │ │  _swap   │ │ _swap │ │  .rs   │ │     │
    │             │ │        │ │   .rs    │ │  .rs  │ │        │ │     │
    │  Core Data  │ │ Basic  │ │ With fee │ │Delegat│ │ Latest │ │     │
    │ Structures: │ │ logic  │ │  logic   │ │  ed   │ │version │ │     │
    │             │ │        │ │          │ │       │ │        │ │     │
    │• SwapArgs   │ │        │ │          │ │       │ │        │ │     │
    │• Route      │ │        │ │          │ │       │ │        │ │     │
    │• Dex enum   │ │        │ │          │ │       │ │        │ │     │
    │• HopAccounts│ │        │ │          │ │       │ │        │ │     │
    └──────┬──────┘ └───┬────┘ └────┬─────┘ └───┬───┘ └───┬────┘ └─────┘
           │            │           │            │         │
           │            └───────────┴────────────┴─────────┘
           │                        │
           │                        │ Uses SwapArgs & Route
           │                        │
           ▼                        ▼
    ┌──────────────────────────────────────────────────────────┐
    │              adapters/ (40+ DEX Integrations)            │
    ├──────────────────────────────────────────────────────────┤
    │  ┌────────────┐                                          │
    │  │ common.rs  │  ← Defines DexProcessor trait            │
    │  └─────┬──────┘                                          │
    │        │                                                 │
    │        │ Implemented by:                                 │
    │        │                                                 │
    │  ┌─────┴──────────────────────────────────────────┐     │
    │  │                                                 │     │
    │  ▼                    ▼                    ▼       ▼     │
    │ ┌──────────┐  ┌────────────┐  ┌─────────┐ ┌─────────┐  │
    │ │ raydium  │  │ whirlpool  │  │ meteora │ │  40+    │  │
    │ │  .rs     │  │    .rs     │  │  .rs    │ │ more... │  │
    │ └──────────┘  └────────────┘  └─────────┘ └─────────┘  │
    │      │              │               │           │       │
    │      └──────────────┴───────────────┴───────────┘       │
    │                     │                                   │
    │                     │ Uses utils for:                   │
    │                     │                                   │
    └─────────────────────┼───────────────────────────────────┘
                          │
                          ▼
    ┌──────────────────────────────────────────────────────────┐
    │              utils/ (Shared Utilities)                   │
    ├──────────────────────────────────────────────────────────┤
    │  ┌────────────────────────────────────────────────────┐  │
    │  │  token.rs                                          │  │
    │  │  • transfer_from()                                 │  │
    │  │  • transfer_to()                                   │  │
    │  │  • create_token_account()                          │  │
    │  │  • get_balance()                                   │  │
    │  │  • wrap_sol() / unwrap_sol()                       │  │
    │  └────────────────────────────────────────────────────┘  │
    │  ┌────────────────────────────────────────────────────┐  │
    │  │  fee.rs                                            │  │
    │  │  • calculate_commission()                          │  │
    │  │  • calculate_platform_fee()                        │  │
    │  │  • distribute_fees()                               │  │
    │  └────────────────────────────────────────────────────┘  │
    │  ┌────────────────────────────────────────────────────┐  │
    │  │  swap.rs                                           │  │
    │  │  • validate_slippage()                             │  │
    │  │  • calculate_min_out()                             │  │
    │  │  • verify_swap_result()                            │  │
    │  └────────────────────────────────────────────────────┘  │
    │  ┌────────────────────────────────────────────────────┐  │
    │  │  logging.rs                                        │  │
    │  │  • emit_swap_event()                               │  │
    │  │  • log_error()                                     │  │
    │  │  • track_metrics()                                 │  │
    │  └────────────────────────────────────────────────────┘  │
    └──────────────────────────────────────────────────────────┘
                          │
                          │ Accesses
                          ▼
    ┌──────────────────────────────────────────────────────────┐
    │              state/ (On-Chain Accounts)                  │
    ├──────────────────────────────────────────────────────────┤
    │  ┌────────────────────────────────────────────────────┐  │
    │  │  config.rs → GlobalConfig account                  │  │
    │  │  • admin: Pubkey                                   │  │
    │  │  • resolvers: [Pubkey; 5]                          │  │
    │  │  • trade_fee: u64                                  │  │
    │  │  • paused: bool                                    │  │
    │  └────────────────────────────────────────────────────┘  │
    │  ┌────────────────────────────────────────────────────┐  │
    │  │  order.rs → Order account (Limit orders)           │  │
    │  │  • user: Pubkey                                    │  │
    │  │  • making_amount: u64                              │  │
    │  │  • expect_taking_amount: u64                       │  │
    │  │  • status: OrderStatus                             │  │
    │  └────────────────────────────────────────────────────┘  │
    │  ┌────────────────────────────────────────────────────┐  │
    │  │  event.rs → Event definitions                      │  │
    │  │  • SwapEvent                                       │  │
    │  │  • OrderFilledEvent                                │  │
    │  │  • FeeCollectedEvent                               │  │
    │  └────────────────────────────────────────────────────┘  │
    └──────────────────────────────────────────────────────────┘

External Dependencies (via CPI):
┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────┐
│   Raydium    │  │     Orca     │  │   Meteora    │  │   40+    │
│   Program    │  │   Program    │  │   Program    │  │   DEX    │
│              │  │              │  │              │  │ Programs │
└──────────────┘  └──────────────┘  └──────────────┘  └──────────┘
```

---

### 6. Two-Level Splitting Mechanism (Visual)

```
┌────────────────────────────────────────────────────────────────────┐
│  INPUT: 1000 USDC to swap to SOL                                   │
└────────────────────────────────────────────────────────────────────┘
                              │
                              │ SwapArgs.amounts = [600, 400]
                              │
            ┌─────────────────┴─────────────────┐
            │                                   │
            ▼                                   ▼
┌─────────────────────────┐        ┌─────────────────────────┐
│   LEVEL 1: ROUTE SPLIT  │        │   LEVEL 1: ROUTE SPLIT  │
│                         │        │                         │
│   Route 1: 600 USDC     │        │   Route 2: 400 USDC     │
│   (60% of total)        │        │   (40% of total)        │
└───────────┬─────────────┘        └───────────┬─────────────┘
            │                                  │
            │ Single hop                       │ Two hops
            │                                  │
            ▼                                  │
┌─────────────────────────┐                   │
│  LEVEL 2: DEX SPLIT     │                   │
│  (Hop 1 of Route 1)     │                   │
│                         │                   │
│  Route {                │                   │
│    dexes: [Raydium],    │                   │
│    weights: [100]       │                   │
│  }                      │                   │
│                         │                   │
│  100% → Raydium         │                   │
│  600 USDC → Raydium     │                   │
│         ↓               │                   │
│  Output: ~0.588 SOL     │                   │
└─────────────────────────┘                   │
                                              │
                                              ▼
                              ┌───────────────────────────────┐
                              │  LEVEL 2: DEX SPLIT          │
                              │  (Hop 1 of Route 2)          │
                              │                              │
                              │  Route {                     │
                              │    dexes: [Whirlpool,        │
                              │            Meteora],         │
                              │    weights: [60, 40]         │
                              │  }                           │
                              │                              │
                              │  Split 400 USDC:             │
                              │  ├─ 60% → Whirlpool (240)    │
                              │  └─ 40% → Meteora (160)      │
                              │                              │
                              │  Execute both swaps:         │
                              │  USDC → USDT                 │
                              │  ├─ Whirlpool: 240 → ~237.6  │
                              │  └─ Meteora: 160 → ~158.4    │
                              │  Total: ~396 USDT            │
                              └──────────┬────────────────────┘
                                         │
                                         │ Hop 2
                                         ▼
                              ┌───────────────────────────────┐
                              │  LEVEL 2: DEX SPLIT          │
                              │  (Hop 2 of Route 2)          │
                              │                              │
                              │  Route {                     │
                              │    dexes: [Raydium],         │
                              │    weights: [100]            │
                              │  }                           │
                              │                              │
                              │  100% → Raydium              │
                              │  396 USDT → Raydium          │
                              │         ↓                    │
                              │  Output: ~0.392 SOL          │
                              └───────────────────────────────┘

┌────────────────────────────────────────────────────────────────────┐
│  FINAL AGGREGATION                                                  │
│                                                                     │
│  Route 1 Output:  0.588 SOL                                        │
│  Route 2 Output:  0.392 SOL                                        │
│  ─────────────────────────────                                     │
│  Total Output:    0.980 SOL ✅                                     │
│                                                                     │
│  Min acceptable:  0.970 SOL                                        │
│  Status: PASS (within slippage tolerance)                          │
└────────────────────────────────────────────────────────────────────┘

KEY INSIGHTS:
┌────────────────────────────────────────────────────────────────────┐
│  Level 1 (Route Split): WHY?                                       │
│  • Risk diversification: Don't put all eggs in one basket          │
│  • Liquidity aggregation: Access multiple liquidity sources        │
│  • Path optimization: Some paths better for different amounts      │
│  • Example: Direct path + indirect path for better price           │
└────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────┐
│  Level 2 (DEX Weight Split): WHY?                                  │
│  • Price optimization: Get best price across multiple DEXs         │
│  • Liquidity depth: Large swaps need multiple pools                │
│  • Slippage reduction: Split across pools = less price impact      │
│  • Example: 60% best DEX + 40% second best = better than 100% one  │
└────────────────────────────────────────────────────────────────────┘
```

---

## 🗂️ Root Level Configuration Files

### `Anchor.toml`
**Purpose:** Anchor framework configuration file
- Defines program deployment settings
- Specifies cluster endpoints (localnet, devnet, mainnet-beta)
- Configures test script paths
- Sets wallet paths for deployment

### `Cargo.toml` (Workspace Level)
**Purpose:** Rust workspace configuration
- Defines workspace members (links to `programs/dex-solana`)
- Manages shared dependencies across the workspace
- Sets up common build configurations

---

## 📂 `/programs/dex-solana/` - Main Program Directory

This directory contains the entire Solana program (smart contract) logic.

### `Cargo.toml` (Program Level)
**Purpose:** Program-specific Rust dependencies
- Defines dependencies: Anchor framework, Solana SDK, SPL Token, etc.
- Specifies program features and compilation flags
- Sets the crate type to `cdylib` for Solana deployment

### `lib.rs` - Program Entry Point
**Purpose:** Main program module that exposes all public instructions
- **Declares program ID:** `C5x9ZuZETH3RA8QEU83xhFjCkGjPWVVzrWmkV4kS7pmR`
- **Defines public instruction handlers:** All functions users can call (swap, commission_swap, proxy_swap, etc.)
- **Organizes modules:** Links together adapters, instructions, state, utils, etc.
- **Exports key functionality:** Makes modules accessible to external callers

**Key Instructions Exposed:**
- `swap()` - Basic token swap
- `commission_swap()` - Swap with commission fee
- `proxy_swap()` - Swap on behalf of another user
- `swap_v3()` - Latest version with platform fees
- Limit order functions: `place_order()`, `fill_order()`, `cancel_order()`
- Global config management: `init_global_config()`, `set_admin()`, `pause()`, etc.

---

## 📂 `/programs/dex-solana/src/adapters/` - DEX Integrations

### **Purpose:**
The adapters directory contains **integration code for 40+ different DEX protocols**. Each adapter acts as a **translator** between the Router's unified interface and each DEX's specific implementation.

### **Why Do We Need Adapters?**
Different DEXs on Solana have different:
- Account structures
- Instruction formats
- Calculation methods
- Token handling approaches

Adapters **abstract away these differences** so the router can treat all DEXs uniformly.

### **How Adapters Work:**
Each adapter implements the `DexProcessor` trait (defined in `common.rs`):
```rust
pub trait DexProcessor {
    fn before_invoke(&self, account_infos: &[AccountInfo]) -> Result<u64>;
    fn after_invoke(&self, account_infos: &[AccountInfo], hop: usize, owner_seeds: Option<&[&[&[u8]]]>) -> Result<u64>;
}
```

### **Key Adapter Files:**

#### `common.rs`
- Defines the `DexProcessor` trait interface
- Provides shared validation logic (`before_check`, `after_check`)
- Implements common account verification
- Manages token account state tracking across swaps

#### Individual DEX Adapters:
- **`raydium.rs`** - Raydium AMM integration (largest Solana DEX)
- **`whirlpool.rs`** - Orca Whirlpool concentrated liquidity pools
- **`meteora.rs`** - Meteora DLMM (Dynamic Liquidity Market Maker)
- **`openbookv2.rs`** - OpenBook orderbook DEX
- **`phoenix.rs`** - Phoenix orderbook DEX
- **`pumpfun.rs`** - Pump.fun meme token launchpad
- **`sanctum.rs`** - Sanctum liquid staking swaps
- **`lifinity.rs`** - Lifinity proactive market maker
- **`pancake_swap_v3.rs`** - PancakeSwap V3 integration
- ... and 30+ more DEX protocols

Each adapter file typically contains:
1. Account structure definitions specific to that DEX
2. Swap instruction building logic
3. Token amount calculation methods
4. CPI (Cross-Program Invocation) call handling

### **Example: How an Adapter Works**
When a user wants to swap USDC → SOL using Raydium:
1. Router receives the swap request
2. Identifies `Dex::RaydiumSwap` from the route
3. Calls `raydium.rs` adapter
4. Adapter prepares Raydium-specific accounts and instructions
5. Executes CPI call to Raydium program
6. Returns swap results back to router

---

## 📂 `/programs/dex-solana/src/instructions/` - Instruction Handlers

### **Purpose:**
Contains the **business logic** for all program instructions. These are the functions that get called when users interact with the program.

### **Key Instruction Files:**

#### `common_swap.rs` ⭐ **MOST IMPORTANT**
**Core data structures and swap logic:**
- Defines `Dex` enum (77 DEX types)
- Defines `Route` struct (DEX list + weights)
- Defines `SwapArgs` struct (complete swap parameters with 2-level splitting)
- Defines `HopAccounts` (tracks multi-hop account state)
- Contains the main swap distribution logic

#### `swap.rs`
**Basic swap instruction:**
- Implements `swap_handler()` - the primary swap entry point
- Validates user accounts and inputs
- Calls routing execution logic
- Emits swap events

#### `commission_swap.rs`
**Commission-based swaps:**
- Handles swaps where a referrer earns commission
- Calculates and distributes commission fees
- Separate handlers for SPL tokens vs SOL

#### `proxy_swap.rs`
**Delegated swaps:**
- Allows third-party to execute swaps on user's behalf
- Used by dApps/wallets to swap for users
- Maintains proper authority checks

#### `swap_v3.rs`
**Latest swap version:**
- Combines commission + platform fees
- More efficient fee handling
- Supports "Take on Commission" (TOC) and "Take on Buy" (TOB) models

#### `commission_v3.rs`
**V3 commission logic:**
- Advanced fee calculation
- Multi-tier fee structures
- Optimized gas costs

#### `from_swap.rs`
**Bridge integration:**
- Handles swaps after cross-chain bridges
- Integrates with Wormhole/other bridges
- Processes logged swap events from bridges

#### `platform_fee_*.rs` files
**Platform fee management:**
- Calculate platform fees (protocol revenue)
- Distribute fees to treasury
- Support different fee models

#### `commission_wrap_unwrap.rs`
**SOL wrapping/unwrapping:**
- Converts SOL ↔ wSOL (wrapped SOL)
- Handles commission on wrap/unwrap operations
- Required since SPL Token standard needs wrapped SOL

---

## 📂 `/programs/dex-solana/src/state/` - State Definitions

### **Purpose:**
Defines **on-chain account structures** that persist data on the Solana blockchain.

### **Key State Files:**

#### `config.rs`
**Global program configuration:**
```rust
pub struct GlobalConfig {
    pub bump: u8,                    // PDA bump seed
    pub admin: Pubkey,               // Program administrator
    pub resolvers: [Pubkey; 5],      // Authorized order resolvers
    pub trade_fee: u64,              // Base trading fee
    pub paused: bool,                // Emergency pause switch
    pub fee_multiplier: u8,          // Fee scaling factor
    pub padding: [u8; 127],          // Reserved for upgrades
}
```
**Purpose:**
- Stores global program settings
- Controls who can manage the program
- Manages fee parameters
- Supports emergency pause functionality

#### `order.rs`
**Limit order state:**
```rust
pub struct Order {
    pub user: Pubkey,                   // Order owner
    pub making_amount: u64,             // Amount user is selling
    pub expect_taking_amount: u64,      // Expected receive amount
    pub min_return_amount: u64,         // Minimum acceptable amount
    pub deadline: i64,                  // Order expiration
    // ... more fields
}
```
**Purpose:**
- Stores limit order details
- Tracks order status (open, filled, cancelled)
- Manages order escrow accounts
- Enables off-chain orderbook matching

#### `event.rs`
**Event definitions:**
- Defines event structures emitted by the program
- Used for indexing and tracking swap history
- Includes SwapEvent, OrderEvent, FeeEvent, etc.
- Enables building transaction history dashboards

---

## 📂 `/programs/dex-solana/src/utils/` - Utility Functions

### **Purpose:**
Provides **reusable helper functions** used across the program. These are common operations that don't fit into a specific instruction or adapter.

### **Key Utility Files:**

#### `swap.rs`
**Swap calculation utilities:**
- Token amount calculations
- Slippage validation
- Route optimization helpers
- Multi-hop swap logic

#### `token.rs`
**Token operation helpers:**
- SPL Token transfers
- Account creation
- Balance checking
- Token account validation
- Wrapping/unwrapping SOL

#### `fee.rs`
**Fee calculation utilities:**
- Commission calculations
- Platform fee math
- Fee distribution logic
- Rounding and precision handling

#### `logging.rs`
**Logging and events:**
- Event emission helpers
- Debug logging macros
- Transaction logging
- Error reporting utilities

---

## 📂 `/tests/` - Test Suite

### **Purpose:**
Contains **integration tests** and **unit tests** for the program.

**Typical test files:**
- `swap.test.ts` - Tests basic swap functionality
- `commission.test.ts` - Tests commission features
- `limit_order.test.ts` - Tests orderbook functionality
- `multi_hop.test.ts` - Tests complex routing scenarios

**What tests do:**
- Deploy program to local validator
- Create mock token accounts
- Execute swap transactions
- Verify expected outcomes
- Test edge cases and error conditions

---

## 🔄 How All Modules Work Together

### **Example: USDC → SOL Swap Flow**

1. **User calls `swap()` in `lib.rs`**
   - Passes `SwapArgs` with routing information
   - Specifies amount, slippage, route

2. **`lib.rs` delegates to `swap.rs` instruction handler**
   - `swap_handler()` validates inputs
   - Checks user accounts and balances

3. **`swap.rs` uses `common_swap.rs` logic**
   - Parses route from `SwapArgs`
   - Identifies `Dex::RaydiumSwap` (60%) + `Dex::Whirlpool` (40%)
   - Splits input amount: 600 USDC + 400 USDC

4. **For each DEX, calls corresponding adapter:**
   - **Raydium adapter (`raydium.rs`):**
     - Prepares Raydium-specific accounts
     - Builds swap instruction
     - Executes CPI to Raydium program
     - Returns ~0.588 SOL
   
   - **Whirlpool adapter (`whirlpool.rs`):**
     - Prepares Whirlpool-specific accounts
     - Builds swap instruction
     - Executes CPI to Whirlpool program
     - Returns ~0.392 SOL

5. **Utils helpers assist throughout:**
   - `token.rs` - Transfers tokens
   - `fee.rs` - Calculates fees
   - `swap.rs` - Validates slippage
   - `logging.rs` - Emits swap event

6. **State updated:**
   - Swap event logged in `event.rs`
   - User receives ~0.98 SOL (minus fees)
   - Transaction completes successfully

---

## 📊 Module Dependency Graph

```
lib.rs (Entry Point)
    │
    ├─── instructions/ ────┐
    │         │             │
    │         └─── common_swap.rs (Core Data Structures)
    │                        │
    ├─── adapters/ ──────────┤
    │    (40+ DEX integrations)
    │                        │
    ├─── state/ ────────────┤
    │    (On-chain accounts) │
    │                        │
    └─── utils/ ────────────┘
         (Helper functions)
```

---

## 🎯 Key Takeaways

1. **`adapters/`** = "How to talk to each DEX"
   - Each file handles one DEX protocol
   - Translates router commands to DEX-specific calls

2. **`instructions/`** = "What users can do"
   - Each file implements one type of operation
   - Contains business logic and validation

3. **`state/`** = "What data we store on-chain"
   - Persistent account structures
   - Configuration and order data

4. **`utils/`** = "Shared helper tools"
   - Reusable functions across the program
   - Token operations, math, logging

5. **`lib.rs`** = "Main entrance"
   - Exposes all public functions
   - Routes calls to appropriate handlers

---

## 🚀 Understanding the Architecture Benefits

### **Why This Structure?**

1. **Modularity:** Each adapter is independent - can add new DEXs without touching existing code
2. **Maintainability:** Clear separation of concerns - swap logic ≠ DEX integration ≠ state management
3. **Testability:** Can test each module in isolation
4. **Scalability:** Easy to add new features (new instructions) or DEXs (new adapters)
5. **Code Reuse:** Utils are shared across all instructions and adapters

### **Design Pattern: Adapter Pattern**
The adapter directory implements the classic **Adapter Design Pattern**:
- **Problem:** 40+ DEXs with different interfaces
- **Solution:** Create adapter layer that converts router's unified interface to each DEX's specific interface
- **Benefit:** Router code doesn't need to know about DEX-specific details

---

*Generated: 2025-10-13*  
*For: DEX-Router-Solana-V1 Understanding*

