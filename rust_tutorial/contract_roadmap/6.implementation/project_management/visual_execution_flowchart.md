# 🎨 DEX Router Execution Flow - Visual Flowchart

## 📐 Complete Execution Flow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                     USER INITIATES SWAP                         │
│                  swap_handler(SwapArgs)                         │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                      VALIDATION PHASE                           │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │ ✓ amounts.len() == routes.len()                          │  │
│  │ ✓ sum(amounts) == amount_in                              │  │
│  │ ✓ expect_amount_out >= min_return                        │  │
│  │ ✓ min_return > 0                                         │  │
│  └───────────────────────────────────────────────────────────┘  │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│              LEVEL 1: ROUTE SPLITTING LOOP                      │
│              for (i, hops) in routes.iter().enumerate()         │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  Route 0: amount_in = amounts[0]                          │ │
│  │  ┌─────────────────────────────────────────────────────┐  │ │
│  │  │  MULTI-HOP LOOP                                      │  │ │
│  │  │  for (hop, route) in hops.iter().enumerate()         │  │ │
│  │  │  ┌───────────────────────────────────────────────┐   │  │ │
│  │  │  │  Hop 0: Route { dexes, weights }              │   │  │ │
│  │  │  │  ┌─────────────────────────────────────────┐  │   │  │ │
│  │  │  │  │  LEVEL 2: DEX WEIGHT SPLIT              │  │   │  │ │
│  │  │  │  │  for (index, dex) in dexes.enumerate()  │  │   │  │ │
│  │  │  │  │  ┌───────────────────────────────────┐  │  │   │  │ │
│  │  │  │  │  │  DEX 0: Calculate fork_amount_in  │  │  │   │  │ │
│  │  │  │  │  │  - If last: use remainder         │  │  │   │  │ │
│  │  │  │  │  │  - Else: amount × weight / 100    │  │  │   │  │ │
│  │  │  │  │  └───────────┬───────────────────────┘  │  │   │  │ │
│  │  │  │  │              ▼                           │  │   │  │ │
│  │  │  │  │  ┌───────────────────────────────────┐  │  │   │  │ │
│  │  │  │  │  │  distribute_swap(dex, ...)        │  │  │   │  │ │
│  │  │  │  │  │  ┌─────────────────────────────┐  │  │  │   │  │ │
│  │  │  │  │  │  │  Match DEX type:            │  │  │  │   │  │ │
│  │  │  │  │  │  │  - RaydiumSwap → raydium::  │  │  │  │   │  │ │
│  │  │  │  │  │  │  - Whirlpool → whirlpool::  │  │  │  │   │  │ │
│  │  │  │  │  │  │  - OpenBookV2 → openbook::  │  │  │  │   │  │ │
│  │  │  │  │  │  │  - ... (67 DEXs total)      │  │  │  │   │  │ │
│  │  │  │  │  │  └─────────────────────────────┘  │  │  │   │  │ │
│  │  │  │  │  │                                   │  │  │   │  │ │
│  │  │  │  │  │  Returns: fork_amount_out        │  │  │   │  │ │
│  │  │  │  │  └───────────┬───────────────────────┘  │  │   │  │ │
│  │  │  │  │              ▼                           │  │   │  │ │
│  │  │  │  │  ┌───────────────────────────────────┐  │  │   │  │ │
│  │  │  │  │  │  emit!(SwapEvent)                 │  │  │   │  │ │
│  │  │  │  │  └───────────────────────────────────┘  │  │   │  │ │
│  │  │  │  │                                          │  │   │  │ │
│  │  │  │  │  Repeat for each DEX in route.dexes    │  │   │  │ │
│  │  │  │  └──────────────────────────────────────────┘  │   │  │ │
│  │  │  │                                                 │   │  │ │
│  │  │  │  Hop Output = sum(all fork_amount_out)         │   │  │ │
│  │  │  └─────────────────────────────────────────────────┘   │  │ │
│  │  │                                                         │  │ │
│  │  │  Repeat for each hop, using previous output as next    │  │ │
│  │  │  hop's input (amount_in = amount_out)                  │  │ │
│  │  └─────────────────────────────────────────────────────────┘  │ │
│  │                                                               │ │
│  │  Validation:                                                 │ │
│  │  - First hop: from_account == user_source_account           │ │
│  │  - Last hop: to_account == user_destination_account         │ │
│  └───────────────────────────────────────────────────────────────┘ │
│                                                                   │
│  Repeat for each route in routes                                 │
└──────────────────────────┬────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────┐
│                    FINAL VALIDATION                             │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Reload destination account                               │  │
│  │  actual_output = after_balance - before_balance           │  │
│  │  require!(actual_output >= min_return)                    │  │
│  └───────────────────────────────────────────────────────────┘  │
└──────────────────────────┬──────────────────────────────────────┘
                           │
                           ▼
                      ┌─────────┐
                      │ SUCCESS │
                      └─────────┘
```

---

## 🔄 Level 1 vs Level 2 Split Comparison

### Level 1: Route-Level Split (Parallel Paths)

```
Total Input: 1000 USDC
┌────────────────────────────────────┐
│      amounts = [600, 400]          │
│      routes.len() = 2              │
└────────────┬───────────────────────┘
             │
    ┌────────┴────────┐
    │                 │
  600 USDC         400 USDC
  Route 0          Route 1
    │                 │
    ▼                 ▼
[Executes          [Executes
independently]     independently]
    │                 │
    ▼                 ▼
 Output A          Output B
    │                 │
    └────────┬────────┘
             │
    Total: A + B
```

**Key Characteristics:**
- Parallel execution paths
- Different hop structures allowed
- Amount split decided upfront
- Each route gets fixed input amount

### Level 2: DEX-Level Split (Weight Distribution)

```
Route Input: 600 USDC
┌────────────────────────────────────┐
│  Route {                           │
│    dexes = [Raydium, Whirlpool,    │
│             Meteora]               │
│    weights = [50, 30, 20]          │
│  }                                 │
└────────────┬───────────────────────┘
             │
    ┌────────┼────────┐
    │        │        │
  300 USDC  180 USDC 120 USDC
  (50%)     (30%)    (20%)
    │        │        │
    ▼        ▼        ▼
 Raydium  Whirlpool Meteora
    │        │        │
    └────────┼────────┘
             │
      Total Output
```

**Key Characteristics:**
- Sequential weight-based split within a hop
- All DEXs execute same token pair
- Weights must sum to 100
- Last DEX uses remainder

---

## 🎯 Example: Complex 3-Route Swap Flow

```
Input: 1000 USDC → BONK
Expected: ~50,000 BONK
Min Return: 49,000 BONK (2% slippage)

┌───────────────────────────────────────────────────────────┐
│              1000 USDC Total Input                        │
└─────────────┬────────────────┬────────────────────────────┘
              │                │                    
        ┌─────┴─────┐    ┌─────┴─────┐       ┌──────────────┐
        │           │    │           │       │              │
     300 USDC    400 USDC 300 USDC  
     (30%)       (40%)    (30%)      
     Route 0     Route 1  Route 2    
        │           │        │       

┌───────┴────────┐  │  ┌────┴──────┐
│ Direct Path    │  │  │Multi-Hop  │
│                │  │  │           │
│ Hop 0:         │  │  │Hop 0:     │
│ USDC → BONK    │  │  │USDC→SOL   │
│ ┌──────────┐   │  │  │┌────────┐ │
│ │ Raydium  │   │  │  ││Whirlpool│
│ │  100%    │   │  │  ││  60%   │ │
│ └──────────┘   │  │  │└────────┘ │
│      │         │  │  │    │      │
│      ▼         │  │  │┌────────┐ │
│ ~14,800 BONK   │  │  ││Meteora │ │
└────────────────┘  │  ││  40%   │ │
                    │  │└────────┘ │
                    │  │    │      │
                    │  │Hop 1:     │
                    │  │SOL → BONK │
                    │  │┌────────┐ │
                    │  ││Raydium │ │
                    │  ││  100%  │ │
                    │  │└────────┘ │
                    │  │    │      │
                    │  │    ▼      │
                    │  │~19,600BONK│
                    │  └───────────┘
                    │
              ┌─────┴──────┐
              │Split Path  │
              │            │
              │Hop 0:      │
              │USDC → BONK │
              │┌─────────┐ │
              ││Raydium  │ │
              ││  70%    │ │
              │└─────────┘ │
              │┌─────────┐ │
              ││Whirlpool│ │
              ││  30%    │ │
              │└─────────┘ │
              │     │      │
              │     ▼      │
              │~14,800BONK │
              └────────────┘

┌──────────────┬──────────────┬──────────────┐
│              │              │              │
│ ~14,800 BONK │ ~19,600 BONK │ ~14,800 BONK │
│              │              │              │
└──────────────┴──────────────┴──────────────┘
                      │
                      ▼
              ┌───────────────┐
              │ ~49,200 BONK  │
              │ >= 49,000 ✅  │
              └───────────────┘
```

---

## 🔍 Weight Calculation Deep Dive

### Standard Weight Calculation (Non-Last DEX)

```
Given:
  amount_in = 600 USDC
  weights = [50, 30, 20]
  TOTAL_WEIGHT = 100

For DEX 0 (index = 0, not last):
┌─────────────────────────────────────┐
│ fork_amount_in = amount_in          │
│                  × weights[0]       │
│                  ÷ TOTAL_WEIGHT     │
│                                     │
│                = 600 × 50 ÷ 100     │
│                = 300                │
│                                     │
│ acc_fork_in += fork_amount_in       │
│              = 0 + 300 = 300        │
└─────────────────────────────────────┘

For DEX 1 (index = 1, not last):
┌─────────────────────────────────────┐
│ fork_amount_in = 600 × 30 ÷ 100     │
│                = 180                │
│                                     │
│ acc_fork_in += 180                  │
│              = 300 + 180 = 480      │
└─────────────────────────────────────┘

For DEX 2 (index = 2, LAST):
┌─────────────────────────────────────┐
│ fork_amount_in = amount_in          │
│                  - acc_fork_in      │
│                                     │
│                = 600 - 480          │
│                = 120                │
│                                     │
│ (No update to acc_fork_in)          │
└─────────────────────────────────────┘

Verification:
  300 + 180 + 120 = 600 ✅
```

### Why Remainder for Last DEX?

```
❌ Wrong Approach (Calculate All):
┌─────────────────────────────────────┐
│ DEX 0: 600 × 50 ÷ 100 = 300.000     │
│ DEX 1: 600 × 30 ÷ 100 = 180.000     │
│ DEX 2: 600 × 20 ÷ 100 = 120.000     │
│                                     │
│ Total: 300 + 180 + 120 = 600        │
│ Looks fine, but...                  │
└─────────────────────────────────────┘

Problem with integer division:
┌─────────────────────────────────────┐
│ Example: 1000 ÷ 3                   │
│ weights = [33, 33, 34]              │
│                                     │
│ DEX 0: 1000 × 33 ÷ 100 = 330        │
│ DEX 1: 1000 × 33 ÷ 100 = 330        │
│ DEX 2: 1000 × 34 ÷ 100 = 340        │
│                                     │
│ Sum: 330 + 330 + 340 = 1000         │
│ BUT if all calculated same way:     │
│ Rounding could cause 999 or 1001!   │
└─────────────────────────────────────┘

✅ Correct Approach (Remainder):
┌─────────────────────────────────────┐
│ DEX 0: 1000 × 33 ÷ 100 = 330        │
│   acc = 330                         │
│                                     │
│ DEX 1: 1000 × 33 ÷ 100 = 330        │
│   acc = 330 + 330 = 660             │
│                                     │
│ DEX 2: 1000 - 660 = 340 (exact!)    │
│                                     │
│ Guaranteed: 330 + 330 + 340 = 1000  │
└─────────────────────────────────────┘
```

---

## 🔗 Multi-Hop Account Chain

### 3-Hop Example: USDC → SOL → RAY → BONK

```
┌────────────────────────────────────────────────────────────┐
│                      HOP 0: USDC → SOL                     │
├────────────────────────────────────────────────────────────┤
│  HopAccounts {                                             │
│    last_to_account: 0x000...000 (none)                     │
│    from_account:    user_usdc_account ──────┐             │
│    to_account:      intermediate_sol_account │             │
│  }                                           │             │
│                                              │             │
│  ✅ Validation: from_account == user_source  │             │
└──────────────────────────────────────────────┼─────────────┘
                                               │
                                               │ Output: 0.1 SOL
                                               │
┌──────────────────────────────────────────────┼─────────────┐
│                      HOP 1: SOL → RAY        │             │
├──────────────────────────────────────────────┼─────────────┤
│  amount_in = 0.1 SOL (from hop 0 output)     │             │
│                                              │             │
│  HopAccounts {                               │             │
│    last_to_account: intermediate_sol_account ◄─────────────┘
│    from_account:    intermediate_sol_account ──┐
│    to_account:      intermediate_ray_account   │
│  }                                             │
│                                                │
│  ✅ Implicit validation: from == last_to       │
└────────────────────────────────────────────────┼───────────┘
                                                 │
                                                 │ Output: 5 RAY
                                                 │
┌────────────────────────────────────────────────┼───────────┐
│                      HOP 2: RAY → BONK         │           │
├────────────────────────────────────────────────┼───────────┤
│  amount_in = 5 RAY (from hop 1 output)         │           │
│                                                │           │
│  HopAccounts {                                 │           │
│    last_to_account: intermediate_ray_account ◄─┘
│    from_account:    intermediate_ray_account ──┐
│    to_account:      user_bonk_account          │
│  }                                             │
│                                                │
│  ✅ Validation: to_account == user_destination │
└────────────────────────────────────────────────┴───────────┘
```

**Key Validation Points:**
- ✅ Hop 0 (first): `from_account` validated against user's source
- ⚙️ Hop 1 (middle): Implicit validation through `last_to_account` tracking
- ✅ Hop 2 (last): `to_account` validated against user's destination

---

## 📊 Event Emission Flow

```
┌─────────────────────────────────────────────────────────────┐
│                   Swap Execution Timeline                   │
└─────────────────────────────────────────────────────────────┘

Time ─────────────────────────────────────────────────────────►

Route 0, Hop 0, DEX 0 (Raydium):
  │
  ├─► Execute: raydium::swap(300 USDC)
  │             └─► Returns: 0.03 SOL
  │
  └─► Emit: SwapEvent {
        dex: Dex::RaydiumSwap,
        amount_in: 300_000_000,
        amount_out: 30_000_000,
      }

Route 0, Hop 0, DEX 1 (Whirlpool):
  │
  ├─► Execute: whirlpool::swap(300 USDC)
  │             └─► Returns: 0.031 SOL
  │
  └─► Emit: SwapEvent {
        dex: Dex::Whirlpool,
        amount_in: 300_000_000,
        amount_out: 31_000_000,
      }

Route 0, Hop 1, DEX 0 (Meteora):
  │
  ├─► Execute: meteora::swap(0.061 SOL)
  │             └─► Returns: 30,000 BONK
  │
  └─► Emit: SwapEvent {
        dex: Dex::MeteoraDynamicpool,
        amount_in: 61_000_000,
        amount_out: 30_000_000_000,
      }

... (Continue for all routes, hops, and DEXs)

Final Validation:
  │
  ├─► Reload destination account
  │
  ├─► Calculate: total_out = 50_000 BONK
  │
  └─► require!(50_000 >= min_return(49,000)) ✅
```

---

## 🎓 Quick Reference: Code Line Numbers

| Component | File | Lines | Purpose |
|-----------|------|-------|---------|
| Entry Point | `swap.rs` | ~20-40 | User-facing swap instruction |
| Validation | `common_swap.rs` | 451-476 | SwapArgs validation |
| Level 1 Loop | `common_swap.rs` | 481-572 | Route iteration |
| Level 2 Loop | `common_swap.rs` | 507-553 | DEX weight split |
| Weight Calc | `common_swap.rs` | 509-524 | Fork amount calculation |
| DEX Dispatch | `common_swap.rs` | 582-702 | Match DEX to adapter |
| Hop Validation | `common_swap.rs` | 555-568 | Account continuity |
| Final Check | `common_swap.rs` | 430-435 | Min return validation |

---

**💡 Pro Tip**: Print this flowchart and keep it next to you while reading the code!

