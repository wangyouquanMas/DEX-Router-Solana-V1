# 🎨 Phase 2 Visual Guide - Routing Engine Data Flow

## 📊 The Two-Level Split Architecture

```
                           SwapArgs
                               │
        ┌──────────────────────┼──────────────────────┐
        │                      │                      │
    amount_in           expect_amount_out        min_return
    (input)              (expected)              (slippage)
        │                                             │
        │                                             │
        └─────────────► amounts ◄─────┬──────────────┘
                           │           │
                    ┌──────┴──────┐    │
                    │             │    │
                [Path 0]      [Path 1] │
                 300 USDC      700 USDC│
                    │             │    │
                    │             │    │
              LEVEL 1 SPLIT       │    │
              (Parallel Paths)    │    │
                    │             │    │
        ┌───────────┴───┐     ┌───┴────┴───┐
        │               │     │            │
    routes[0]      routes[1]  │      routes[n]
        │               │     │            │
        │               │     │            │
   ┌────┴────┐     ┌────┴────┴────┐   │
   │         │     │     │    │    │   │
 Hop 0     Hop 1  Hop 0 Hop 1 Hop 2  ...
   │         │     │     │    │
   │         │     │     │    │
LEVEL 2 SPLIT│     │     │    │
(Multi-Hop) │      │     │    │
   │         │     │     │    │
┌──┴──┐   ┌──┴──┐ │     │    │
│     │   │     │ │     │    │
DEX0 DEX1 DEX0 DEX1     ...  ...
50%  50%  60%  40%

LEVEL 3: DEX SPLITS
(Weight-based routing per hop)
```

## 🔄 Execution Flow: Step-by-Step

### Example: 1000 USDC → BONK (2 paths, multi-hop)

```
┌─────────────────────────────────────────────────────────────┐
│ INPUT: 1000 USDC                                            │
│ SwapArgs {                                                  │
│   amount_in: 1000 USDC                                      │
│   amounts: [300, 700]  ← Level 1 split                      │
│   routes: [                                                 │
│     [Route{...}],      ← Path 0: 1 hop                      │
│     [Route{...}, Route{...}] ← Path 1: 2 hops               │
│   ]                                                         │
│ }                                                           │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┴────────────────┐
              │         Level 1 Split          │
              │    (Parallel Routing Paths)    │
              └───────────────┬────────────────┘
                              │
        ┌─────────────────────┴──────────────────────┐
        │                                            │
        │                                            │
   Path 0 (300 USDC)                          Path 1 (700 USDC)
        │                                            │
        │                                            │
  ┌─────┴─────┐                              ┌──────┴───────┐
  │   Hop 0   │                              │    Hop 0     │
  │ USDC→BONK │                              │   USDC→SOL   │
  │  (Direct) │                              │              │
  └─────┬─────┘                              └──────┬───────┘
        │                                           │
        │ Level 2 Split                             │ Level 2 Split
        │ (DEX weights)                             │ (DEX weights)
        │                                           │
   ┌────┴─────┐                             ┌──────┴──────┬───────┐
   │    60%   │ 40%                          │ 50%   30%   │  20%  │
   │          │                              │             │       │
Raydium   Whirlpool                      Raydium    Whirlpool  Meteora
180 USDC  120 USDC                       350 USDC   210 USDC  140 USDC
   │          │                              │             │       │
   │          │                              │             │       │
   └────┬─────┘                              └──────┬──────┴───────┘
        │                                           │
   ~15k BONK                                    ~0.07 SOL
        │                                           │
        │                                           │
        │                                    ┌──────┴───────┐
        │                                    │    Hop 1     │
        │                                    │   SOL→BONK   │
        │                                    └──────┬───────┘
        │                                           │
        │                                     Level 2 Split
        │                                           │
        │                                    ┌──────┴───────┐
        │                                    │     100%     │
        │                                  Raydium
        │                                  0.07 SOL
        │                                    │
        │                                 ~35k BONK
        │                                    │
        └─────────────────┬──────────────────┘
                          │
                    Final Output
                   ~50,000 BONK
                          │
                 ┌────────┴─────────┐
                 │ Slippage Check:  │
                 │ 50k >= 49k ✅    │
                 │ (min_return)     │
                 └──────────────────┘
```

## 🏗️ Data Structure Breakdown

### 1. Dex Enum - The Building Blocks

```
┌─────────────────────────────────────────────────────┐
│                    Dex Enum                         │
│                   (67 variants)                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────┐ │
│  │   AMM DEXs   │  │  CLMM DEXs   │  │OrderBook │ │
│  ├──────────────┤  ├──────────────┤  ├──────────┤ │
│  │ Raydium      │  │ Whirlpool    │  │ OpenBook │ │
│  │ Meteora      │  │ RaydiumCLMM  │  │ Phoenix  │ │
│  │ Lifinity     │  │ MeteoraDLMM  │  │ Manifest │ │
│  │ FluxBeam     │  │ Byreal       │  └──────────┘ │
│  └──────────────┘  └──────────────┘               │
│                                                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────┐ │
│  │ Specialized  │  │ Meme Tokens  │  │   LST    │ │
│  ├──────────────┤  ├──────────────┤  ├──────────┤ │
│  │ StableSwap   │  │ Pumpfun      │  │ Sanctum  │ │
│  │ SanctumRouter│  │ Boopfun      │  │MeteoraLst│ │
│  │ Perpetuals   │  │ Virtuals     │  └──────────┘ │
│  └──────────────┘  └──────────────┘               │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 2. Route Structure - DEX Path + Weights

```
┌────────────────────────────────────────────┐
│              Route                         │
├────────────────────────────────────────────┤
│                                            │
│  dexes: Vec<Dex>                          │
│  ┌──────┬──────┬──────┐                  │
│  │ DEX0 │ DEX1 │ DEX2 │                  │
│  └──┬───┴──┬───┴──┬───┘                  │
│     │      │      │                       │
│  weights: Vec<u8>                         │
│  ┌──┴───┬──┴───┬──┴───┐                  │
│  │  50  │  30  │  20  │                  │
│  └──────┴──────┴──────┘                  │
│                                            │
│  ⚠️  Constraints:                          │
│  • dexes.len() == weights.len()           │
│  • sum(weights) == 100                    │
│                                            │
└────────────────────────────────────────────┘
```

### 3. SwapArgs - The Complete Picture

```
┌────────────────────────────────────────────────────────┐
│                    SwapArgs                            │
├────────────────────────────────────────────────────────┤
│                                                        │
│  amount_in: u64           ┌─────────────────┐         │
│  ┌─────────────────┐      │ Slippage Zone  │         │
│  │  1000 USDC      │      │                 │         │
│  └─────────────────┘      │ expect: 50k ────┼─┐       │
│                           │                 │ │       │
│  expect_amount_out: u64   │ min:    49k ────┼─┤ 2%    │
│  min_return: u64          │                 │ │       │
│                           └─────────────────┘ │       │
│                                          ▲────┘       │
│  amounts: Vec<u64>                       │            │
│  ┌────┬────┬────┐                   Tolerance         │
│  │300 │700 │... │  ← Must sum to amount_in            │
│  └─┬──┴─┬──┴─┬──┘                                     │
│    │    │    │                                        │
│  routes: Vec<Vec<Route>>                              │
│    │    │    │                                        │
│  ┌─┴──┬─┴──┬─┴──┐                                     │
│  │ [] │ [] │ [] │  ← Each is a Vec<Route> (hops)      │
│  └────┴────┴────┘                                     │
│                                                        │
│  ⚠️  Constraints:                                      │
│  • amounts.len() == routes.len()                      │
│  • sum(amounts) == amount_in                          │
│  • expect_amount_out >= min_return                    │
│                                                        │
└────────────────────────────────────────────────────────┘
```

### 4. HopAccounts - Multi-Hop Token Flow

```
Swap Path: USDC → SOL → RAY → BONK
           ├────┤ ├───┤ ├───┤
           Hop 0 Hop 1 Hop 2

┌─────────────────────────────────────────────────────┐
│ Hop 0: USDC → SOL                                  │
│ ┌─────────────────────────────────────────────┐   │
│ │ HopAccounts {                                │   │
│ │   last_to_account: 0x000... (none)          │   │
│ │   from_account:    User's USDC account ─────┼──┐│
│ │   to_account:      Intermediate SOL acct ───┼─┐││
│ │ }                                            │ │││
│ └─────────────────────────────────────────────┘ │││
└─────────────────────────────────────────────────┼┼┘
                                                  ││
┌─────────────────────────────────────────────────┼┼┐
│ Hop 1: SOL → RAY                                │││
│ ┌─────────────────────────────────────────────┐ │││
│ │ HopAccounts {                                │ │││
│ │   last_to_account: Intermediate SOL acct ◄──┼─┘││
│ │   from_account:    Intermediate SOL acct ◄──┼──┘│
│ │   to_account:      Intermediate RAY acct ───┼─┐ │
│ │ }                                            │ │ │
│ └─────────────────────────────────────────────┘ │ │
└─────────────────────────────────────────────────┼─┘
                                                  │
┌─────────────────────────────────────────────────┼─┐
│ Hop 2: RAY → BONK                               │ │
│ ┌─────────────────────────────────────────────┐ │ │
│ │ HopAccounts {                                │ │ │
│ │   last_to_account: Intermediate RAY acct ◄──┼─┘ │
│ │   from_account:    Intermediate RAY acct ◄──┼───┘
│ │   to_account:      User's BONK account ─────┼──┐
│ │ }                                            │  │
│ └─────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────┼─┘
                                                   │
                                    ✅ Validated: │
                                    Must match user's
                                    destination account
```

## 🔢 Weight Calculation Algorithm

### How 2nd Level Splits Are Calculated

```rust
// Given:
amount_in = 1000 USDC
weights = [50, 30, 20]
dexes = [Raydium, Whirlpool, Meteora]

// Calculation:
for each DEX (except last):
    fork_amount = (amount_in * weight) / 100
    
// Last DEX gets remainder (prevents rounding errors):
last_amount = amount_in - sum(previous_forks)

// Result:
┌─────────────────────────────────────────┐
│ DEX 0 (Raydium):                        │
│   1000 * 50 / 100 = 500 USDC           │
│   acc_fork_in = 500                     │
│                                         │
│ DEX 1 (Whirlpool):                      │
│   1000 * 30 / 100 = 300 USDC           │
│   acc_fork_in = 800                     │
│                                         │
│ DEX 2 (Meteora) - LAST:                 │
│   1000 - 800 = 200 USDC ✅             │
│   (not 1000 * 20 / 100 to avoid error) │
└─────────────────────────────────────────┘
```

## 🎯 Checkpoint Solution Visualized

### Scenario: 1000 USDC → BONK (2-hop, 3-DEX)

```
┌──────────────────────────────────────────────────────────┐
│ SwapArgs Construction                                    │
├──────────────────────────────────────────────────────────┤
│                                                          │
│ amount_in: 1,000,000,000  (1000 USDC)                   │
│                                                          │
│ amounts: [1,000,000,000]                                │
│           └─ Only 1 path, all input goes here           │
│                                                          │
│ routes: [                                                │
│   [                    ← Path 0 (hops array)            │
│     Route {            ← Hop 0: USDC → SOL              │
│       dexes: [                                           │
│         Dex::RaydiumSwap,        ← 500 USDC (50%)       │
│         Dex::Whirlpool,          ← 300 USDC (30%)       │
│         Dex::MeteoraDynamicpool  ← 200 USDC (20%)       │
│       ],                                                 │
│       weights: [50, 30, 20]  ← Must sum to 100          │
│     },                                                   │
│                                                          │
│     Route {            ← Hop 1: SOL → BONK              │
│       dexes: [                                           │
│         Dex::RaydiumSwap  ← All SOL from hop 0          │
│       ],                                                 │
│       weights: [100]       ← 100% through Raydium       │
│     }                                                    │
│   ]                                                      │
│ ]                                                        │
│                                                          │
└──────────────────────────────────────────────────────────┘

Execution Flow:
═══════════════

1000 USDC (input)
    │
    ├─ amounts[0] = 1000 USDC (100% to Path 0)
    │
    └─ Path 0:
        │
        ├─ Hop 0 (USDC → SOL):
        │   │
        │   ├─ Raydium:  500 USDC (50%) → 0.05 SOL
        │   ├─ Whirlpool: 300 USDC (30%) → 0.03 SOL
        │   └─ Meteora:   200 USDC (20%) → 0.02 SOL
        │   │
        │   └─ Combined: 0.1 SOL
        │
        └─ Hop 1 (SOL → BONK):
            │
            └─ Raydium: 0.1 SOL (100%) → 50,000 BONK
            
Final Output: 50,000 BONK ✅ (>= min_return: 49,000)
```

## 📐 Common Patterns

### Pattern 1: Simple Direct Swap
```
Input → [1 DEX] → Output
```

### Pattern 2: Split Liquidity (Same Hop)
```
Input → [DEX1 (60%)] ┐
     → [DEX2 (40%)] ┴→ Output
```

### Pattern 3: Multi-Hop (Sequential)
```
Input → [DEX1] → Intermediate1 → [DEX2] → Output
```

### Pattern 4: Multi-Hop with Splits
```
Input → [DEX1 (50%)] ┐
     → [DEX2 (50%)] ┴→ Int → [DEX3] → Output
```

### Pattern 5: Multi-Path (Parallel Routes)
```
Input → Path A (30%): [DEX1] ────────────┐
     → Path B (70%): [DEX2] → [DEX3] → Output
```

### Pattern 6: Complex (Everything Combined)
```
Input → Path A (40%): [DEX1 (60%)] ┐     ┐
                   → [DEX2 (40%)] ┴→ Out ┤
     → Path B (60%): [DEX3] → [DEX4 (50%)] ┐  ┤→ Final
                            → [DEX5 (50%)] ┴→ Out ┘
```

## 🧪 Validation Checklist

When constructing `SwapArgs`, verify:

```
✅ amounts.len() == routes.len()
✅ sum(amounts) == amount_in
✅ expect_amount_out >= min_return

For each Route:
  ✅ dexes.len() == weights.len()
  ✅ sum(weights) == 100
  ✅ Each weight > 0

For HopAccounts (runtime):
  ✅ First hop: from_account == user_source_account
  ✅ Last hop:  to_account == user_destination_account
  ✅ Between hops: current.from == previous.to
```

## 🚀 Performance Considerations

### Trade-offs in Route Design

```
┌─────────────────┬──────────┬────────────┬──────────┐
│ Route Type      │ Gas Cost │ Complexity │ Price    │
├─────────────────┼──────────┼────────────┼──────────┤
│ Single DEX      │   LOW    │    LOW     │   OK     │
│ 2-DEX Split     │   MED    │    MED     │  BETTER  │
│ 3-DEX Split     │   HIGH   │    HIGH    │   BEST   │
│ Multi-Hop       │  +HIGH   │   +HIGH    │  VARIED  │
│ Multi-Path      │ ++HIGH   │  ++HIGH    │ OPTIMAL  │
└─────────────────┴──────────┴────────────┴──────────┘

Rule of Thumb:
  More splits  = Better price, Higher gas
  More hops    = Access to more pairs
  More paths   = Maximum optimization, Maximum cost
```

## 🎓 Key Takeaways

1. **Two-Level Architecture**: Level 1 (parallel paths) + Level 2 (multi-hop with DEX splits)

2. **Weights are Percentages**: Always sum to 100, last DEX gets remainder

3. **HopAccounts Chains**: Each hop's `to_account` becomes next hop's `from_account`

4. **Slippage Protection**: `min_return` is your safety net

5. **Flexibility**: Can represent simple 1-DEX swaps to complex 4-path, 3-hop, multi-DEX routes

---

**🏆 You're now ready to design and understand complex swap routes!**

