# 🔄 DEX Routing Execution Flow - Deep Dive Guide

## 📋 Mission Objectives

### Primary Goal
Master the **Two-Level Split Mechanism** of the DEX Router and understand the complete routing execution flow from user input to final output.

### This Phase's Role in the Overall Project
- **Foundation** (Phase 2 ✅): Understanding data structures 
- **Execution Flow** (Current Phase) ← **YOU ARE HERE**
- **DEX Adapters** (Phase 4): Understanding DEX-specific interactions
- **Full Implementation** (Phase 5+): Building your own simplified DEX Router

### Why This Phase Matters
Mastering the routing execution flow is the key to understanding the DEX Router's core logic. It connects:
1. Frontend routing parameters (SwapArgs)
2. Actual DEX interactions (distribute_swap)
3. Final token transfers and slippage protection

---

## 🎯 Execution Steps

### Step 1: Understand the Code Entry Point
**Goal**: Locate the swap entry point and understand the call chain

**Actions**:
1. Open file: `programs/dex-solana/src/instructions/swap.rs`
2. Find the `swap_handler()` function
3. Observe how it calls `common_swap()`

**Key Code Location**:
```rust
// swap.rs
pub fn swap_handler(ctx: Context<Swap>, args: SwapArgs) -> Result<()> {
    common_swap(
        &mut ctx.accounts.source_token_account,
        &mut ctx.accounts.destination_token_account,
        ctx.remaining_accounts,
        args,
        // ... other params
    )
}
```

**Success Criteria**: Can answer "What's the first function executed when a user calls the swap instruction?"

---

### Step 2: Analyze Core Function - execute_swap()
**Goal**: Understand the main routing execution logic

**Actions**:
1. Open file: `programs/dex-solana/src/instructions/common_swap.rs`
2. Locate `execute_swap()` function (lines 438-580)
3. Identify three main code blocks:
   - Validation logic (lines 451-476)
   - Level 1 route splitting (lines 481-572)
   - Level 2 DEX weight distribution (lines 507-553)

**Key Code Structure**:
```rust
fn execute_swap(...) -> Result<u64> {
    // 🔍 Block 1: Parameter Validation
    require!(amounts.len() == routes.len());
    require!(total_amounts == real_amount_in);
    
    // 🔍 Block 2: Level 1 - Route Splitting
    for (i, hops) in routes.iter().enumerate() {
        let mut amount_in = amounts[i];  // Get this route's amount
        
        // 🔍 Block 3: Multi-hop Handling
        for (hop, route) in hops.iter().enumerate() {
            // 🔍 Block 4: Level 2 - DEX Weight Distribution
            for (index, dex) in route.dexes.iter().enumerate() {
                let fork_amount_in = calculate_split_amount();
                let fork_amount_out = distribute_swap(dex, ...);
                emit!(SwapEvent { ... });
            }
        }
    }
    
    Ok(amount_out)
}
```

**Success Criteria**:
- Can draw a flowchart of execute_swap
- Can explain the difference between "Level 1" and "Level 2"

---

### Step 3: Deep Dive into Level 1 Split (Route Level)
**Goal**: Understand how input amount is distributed across different routing paths

**Actions**:
1. Find line 481: `for (i, hops) in routes.iter().enumerate()`
2. Understand the meaning of `amounts[i]`
3. Analyze why multiple paths are needed

**Code Breakdown**:
```rust
// Line 481-483
for (i, hops) in routes.iter().enumerate() {
    require!(hops.len() <= MAX_HOPS, ErrorCode::TooManyHops);
    let mut amount_in = amounts[i];  // ← Input amount for this path
```

**Real Example**:
```rust
SwapArgs {
    amount_in: 1000_000_000,  // 1000 USDC total input
    amounts: vec![600_000_000, 400_000_000],  // Level 1 split
    routes: vec![
        vec![...],  // Route 0: 600 USDC
        vec![...],  // Route 1: 400 USDC
    ],
}
```

**Execution Flow**:
```
1000 USDC Total Input
  │
  ├─ Loop iteration i=0: amount_in = 600 USDC → Route 0
  └─ Loop iteration i=1: amount_in = 400 USDC → Route 1
```

**Success Criteria**:
- Can explain why `amounts.len()` must equal `routes.len()`
- Can calculate the input amount for each route in a given SwapArgs

---

### Step 4: Deep Dive into Level 2 Split (DEX Weight Distribution)
**Goal**: Understand how a single hop distributes amounts across multiple DEXs by weight

**Actions**:
1. Find lines 507-524: DEX weight calculation logic
2. Understand why the last DEX uses remainder
3. Analyze why weights must sum to 100

**Code Breakdown**:
```rust
// Lines 507-524: Core weight distribution logic
for (index, dex) in dexes.iter().enumerate() {
    let fork_amount_in = if index == dexes.len() - 1 {
        // Last DEX: use remaining amount (avoid rounding errors)
        amount_in.checked_sub(acc_fork_in)
            .ok_or(ErrorCode::CalculationError)?
    } else {
        // Non-last DEX: calculate by weight
        let temp_amount = amount_in
            .checked_mul(weights[index] as u64)
            .ok_or(ErrorCode::CalculationError)?
            .checked_div(TOTAL_WEIGHT as u64)  // TOTAL_WEIGHT = 100
            .ok_or(ErrorCode::CalculationError)?;
        acc_fork_in = acc_fork_in
            .checked_add(temp_amount)
            .ok_or(ErrorCode::CalculationError)?;
        temp_amount
    };
```

**Weight Calculation Formula**:
```
fork_amount_in = amount_in × (weight / 100)

Example:
amount_in = 600 USDC
weights = [50, 30, 20]

DEX 0: 600 × 50/100 = 300 USDC
DEX 1: 600 × 30/100 = 180 USDC
DEX 2: 600 - (300 + 180) = 120 USDC  ← Uses remainder
```

**Why Last DEX Uses Remainder?**
Avoids floating-point rounding errors that could lead to fund loss:
```
Wrong approach:
600 × 20/100 = 120.00...001 (precision loss in rounding)

Correct approach:
600 - 300 - 180 = 120 (exact)
```

**Success Criteria**:
- Manually calculate amounts for a 3-DEX split
- Explain the purpose of the acc_fork_in variable

---

### Step 5: Understanding distribute_swap()
**Goal**: Understand how DEX enum maps to concrete swap implementations

**Actions**:
1. Find `distribute_swap()` function (lines 582-702)
2. Observe the match statement structure
3. Identify 3 different types of DEX implementations

**Code Structure**:
```rust
// Lines 582-702
fn distribute_swap<'a>(
    dex: &Dex,
    remaining_accounts: &'a [AccountInfo<'a>],
    amount_in: u64,
    offset: &mut usize,
    hop_accounts: &mut HopAccounts,
    // ...
) -> Result<u64> {
    let swap_function = match dex {
        // AMM types
        Dex::RaydiumSwap => raydium::swap,
        Dex::Whirlpool => whirlpool::swap,
        
        // CLMM types
        Dex::RaydiumClmmSwap => raydium::swap_clmm,
        Dex::MeteoraDlmm => meteora::dlmm_swap,
        
        // Order book types
        Dex::OpenBookV2 => openbookv2::place_take_order,
        Dex::Phoenix => phoenix::swap,
        
        // Special handling (direct return)
        Dex::SanctumRouter => {
            return sanctum_router::sanctum_router_handler(...);
        }
        
        // ... other 67 DEXs
    };
    
    // Call the matched swap function
    swap_function(remaining_accounts, amount_in, offset, hop_accounts, ...)
}
```

**Three Handling Approaches**:
1. **Standard Mapping**: Most DEXs → `dex => module::swap`
2. **Special Handling**: Some DEXs need custom logic (direct return)
3. **Parameterized Functions**: Different versions of same DEX (e.g., Raydium's swap vs swap_stable)

**Success Criteria**:
- Can find at least 5 different DEX mappings
- Understand why SanctumRouter needs special handling

---

### Step 6: Analyze HopAccounts Validation Logic
**Goal**: Understand account continuity validation in multi-hop swaps

**Actions**:
1. Find lines 555-568: hop validation logic
2. Understand why first and last hops need special validation
3. Track how HopAccounts is passed between hops

**Code Breakdown**:
```rust
// Lines 555-568: Hop boundary validation
if hop == 0 {
    // First hop: from_account must be user's source account
    require!(
        source_account.key() == hop_accounts.from_account,
        ErrorCode::InvalidSourceTokenAccount
    );
}
if hop == hops.len() - 1 {
    // Last hop: to_account must be user's destination account
    require!(
        destination_account.key() == hop_accounts.to_account,
        ErrorCode::InvalidDestinationTokenAccount
    );
}

// Line 569-570: Intermediate hop validation
amount_in = amount_out;  // Next hop's input = current hop's output
last_to_account = hop_accounts.to_account;  // Record current hop's to account
```

**Multi-hop Continuity Example**:
```
User swap: USDC → SOL → BONK

Hop 0 (USDC → SOL):
  ✅ hop_accounts.from_account == user_usdc_account (validated)
  hop_accounts.to_account = intermediate_sol_account
  last_to_account = intermediate_sol_account

Hop 1 (SOL → BONK):
  hop_accounts.from_account must == last_to_account (implicit validation)
  ✅ hop_accounts.to_account == user_bonk_account (validated)
```

**Success Criteria**:
- Draw an account flow diagram for a 3-hop swap
- Explain why intermediate hops don't need explicit validation

---

### Step 7: Complete Practical Exercise
**Goal**: Solidify understanding through concrete examples

**Exercise**:
Trace a swap with the following parameters:
- 1000 USDC input
- 2 routes (60/40 split)
- Route 1: Raydium (100%)
- Route 2: Whirlpool (70%) + Meteora (30%)

**Requirements**:
1. Calculate input amount for each route
2. Calculate input amount for each DEX in Route 2
3. Draw the complete execution flowchart
4. Write the corresponding SwapArgs structure

**Instructions**:
1. Try to calculate yourself first
2. Check the detailed solution below
3. Verify calculations with code

---

## 📊 Practical Exercise - Detailed Solution

### Problem Review
```
Input: 1000 USDC
Level 1 split: 60% Route 1, 40% Route 2
Route 1: Raydium (100%)
Route 2: Whirlpool (70%) + Meteora (30%)
```

### Solution Part 1: Calculate Level 1 Split Amounts

```rust
// Level 1 split
amount_in = 1000 USDC = 1_000_000_000 (assuming 6 decimals)

Route 0 amount = 1_000_000_000 × 60% = 600_000_000
Route 1 amount = 1_000_000_000 × 40% = 400_000_000

Verification: 600_000_000 + 400_000_000 = 1_000_000_000 ✅
```

### Solution Part 2: Calculate Level 2 DEX Split Amounts

**Route 0 (only 1 DEX)**:
```rust
Raydium: 600_000_000 × 100% = 600_000_000
```

**Route 1 (2 DEXs)**:
```rust
amount_in = 400_000_000

// DEX 0 (Whirlpool) - not last, calculate by weight
fork_amount_in[0] = 400_000_000 × 70 / 100 = 280_000_000
acc_fork_in = 280_000_000

// DEX 1 (Meteora) - last, use remainder
fork_amount_in[1] = 400_000_000 - 280_000_000 = 120_000_000

Verification: 280_000_000 + 120_000_000 = 400_000_000 ✅
```

### Solution Part 3: Complete SwapArgs Construction

```rust
let swap_args = SwapArgs {
    // Total input amount
    amount_in: 1_000_000_000,
    
    // Expected output (assuming current price can get ~950 USDT)
    expect_amount_out: 950_000_000,
    
    // Minimum acceptable output (2% slippage protection)
    min_return: 931_000_000,  // 950 × 98%
    
    // Level 1: Amount allocation for 2 routes
    amounts: vec![
        600_000_000,  // Route 0: 60%
        400_000_000,  // Route 1: 40%
    ],
    
    // Level 2: Hop and DEX configuration for each route
    routes: vec![
        // Route 0: Single DEX direct swap
        vec![
            Route {
                dexes: vec![Dex::RaydiumSwap],
                weights: vec![100],
            },
        ],
        
        // Route 1: Dual DEX split
        vec![
            Route {
                dexes: vec![Dex::Whirlpool, Dex::MeteoraDynamicpool],
                weights: vec![70, 30],
            },
        ],
    ],
};
```

### Solution Part 4: Execution Flow Visualization

```
┌─────────────────────────────────────────────────────────┐
│                  1000 USDC Input                        │
└─────────────────────┬───────────────────────────────────┘
                      │
        ┌─────────────┴─────────────┐
        │                           │
    600 USDC (60%)              400 USDC (40%)
    Route 0                      Route 1
        │                           │
        │                     ┌─────┴─────┐
        │                     │           │
        │                 280 USDC    120 USDC
        │                 (70%)       (30%)
        │                     │           │
        ▼                     ▼           ▼
   ┌─────────┐          ┌──────────┐ ┌─────────┐
   │ Raydium │          │Whirlpool │ │ Meteora │
   │  Swap   │          │   Swap   │ │  Swap   │
   └────┬────┘          └────┬─────┘ └────┬────┘
        │                    │            │
        │  ~590 USDT         │ ~275 USDT  │ ~118 USDT
        │                    │            │
        └────────────────────┴────────────┘
                      │
                 ~983 USDT
            (hypothetical total output)
                      │
                      ▼
            ┌──────────────────┐
            │  Slippage Check   │
            │ 983 >= min_return │
            │ 983 >= 931 ✅     │
            └──────────────────┘
```

### Solution Part 5: Code Execution Trace

```rust
// Layer 1 Loop - Level 1 Split
// Iteration 0: i=0, hops=routes[0]
{
    amount_in = amounts[0] = 600_000_000
    
    // Layer 2 Loop - Multi-hop (only 1 hop)
    // hop=0, route=routes[0][0]
    {
        dexes = [Dex::RaydiumSwap]
        weights = [100]
        
        // Layer 3 Loop - DEX weight split
        // index=0, dex=Dex::RaydiumSwap
        {
            // Last DEX, use remainder
            fork_amount_in = 600_000_000 - 0 = 600_000_000
            
            fork_amount_out = distribute_swap(
                &Dex::RaydiumSwap,
                ...,
                600_000_000
            ) 
            // → calls raydium::swap
            // → returns ~590_000_000
            
            emit!(SwapEvent {
                dex: Dex::RaydiumSwap,
                amount_in: 600_000_000,
                amount_out: 590_000_000,
            });
        }
    }
}

// Iteration 1: i=1, hops=routes[1]
{
    amount_in = amounts[1] = 400_000_000
    
    // hop=0, route=routes[1][0]
    {
        dexes = [Dex::Whirlpool, Dex::MeteoraDynamicpool]
        weights = [70, 30]
        
        // index=0, dex=Dex::Whirlpool
        {
            fork_amount_in = 400_000_000 × 70 / 100 = 280_000_000
            acc_fork_in = 280_000_000
            
            fork_amount_out = distribute_swap(
                &Dex::Whirlpool,
                ...,
                280_000_000
            )
            // → calls whirlpool::swap
            // → returns ~275_000_000
            
            emit!(SwapEvent {
                dex: Dex::Whirlpool,
                amount_in: 280_000_000,
                amount_out: 275_000_000,
            });
        }
        
        // index=1, dex=Dex::MeteoraDynamicpool
        {
            // Last DEX, use remainder
            fork_amount_in = 400_000_000 - 280_000_000 = 120_000_000
            
            fork_amount_out = distribute_swap(
                &Dex::MeteoraDynamicpool,
                ...,
                120_000_000
            )
            // → calls meteora::swap
            // → returns ~118_000_000
            
            emit!(SwapEvent {
                dex: Dex::MeteoraDynamicpool,
                amount_in: 120_000_000,
                amount_out: 118_000_000,
            });
        }
    }
}

// Final Validation
total_output = 590_000_000 + 275_000_000 + 118_000_000 = 983_000_000
require!(983_000_000 >= min_return(931_000_000)) ✅
```

---

## ✅ Completion Standards

### Knowledge Mastery Checklist
After completing this phase, you should be able to:

1. **Code Navigation** ✅
   - [ ] Quickly locate swap entry point (swap_handler)
   - [ ] Locate core execution function (execute_swap)
   - [ ] Find DEX adapter dispatch logic (distribute_swap)

2. **Flow Understanding** ✅
   - [ ] Draw complete swap execution flowchart
   - [ ] Explain difference and purpose of Level 1 vs Level 2
   - [ ] Trace a swap from input to output

3. **Calculation Skills** ✅
   - [ ] Manually calculate Level 1 split amounts
   - [ ] Manually calculate Level 2 DEX weight distribution
   - [ ] Explain why last DEX uses remainder

4. **Code Reading** ✅
   - [ ] Understand execute_swap's three-layer loop structure
   - [ ] Explain HopAccounts validation logic
   - [ ] Understand distribute_swap's match statement

5. **Practical Skills** ✅
   - [ ] Independently complete exercise calculations
   - [ ] Construct a valid SwapArgs
   - [ ] Identify errors in SwapArgs configuration

### Verification Tests

#### Test 1: Quick Q&A
1. Q: What is Level 1 split?
   - A: Distributing total input amount across different routing paths (parallel paths)

2. Q: What is Level 2 split?
   - A: Within a single hop, distributing amount across multiple DEXs by weight

3. Q: Why does the last DEX use remainder instead of calculating weight?
   - A: To avoid floating-point rounding errors and ensure all funds are used

4. Q: What's the purpose of HopAccounts?
   - A: Track token account continuity in multi-hop swaps

#### Test 2: Error Identification
Find errors in the following SwapArgs:

```rust
SwapArgs {
    amount_in: 1000_000_000,
    min_return: 950_000_000,
    expect_amount_out: 900_000_000,  // ❌ Error!
    amounts: vec![600_000_000, 500_000_000],  // ❌ Error!
    routes: vec![
        vec![
            Route {
                dexes: vec![Dex::RaydiumSwap, Dex::Whirlpool],
                weights: vec![60, 50],  // ❌ Error!
            },
        ],
    ],
}
```

**Answers**:
1. `expect_amount_out < min_return` (should be >= min_return)
2. `sum(amounts) = 1100 != amount_in` (should equal 1000)
3. `sum(weights) = 110 != 100` (should equal 100)

#### Test 3: Construct Complex SwapArgs
Construct a 3-route SwapArgs with multi-hop:
- Route 1: 30%, direct USDC→USDT (Raydium 100%)
- Route 2: 40%, USDC→SOL→USDT (2 hops, hop 1 uses Whirlpool, hop 2 uses Meteora)
- Route 3: 30%, USDC→USDT (Raydium 60% + Whirlpool 40%)

**Reference Answer**: See code block below

```rust
SwapArgs {
    amount_in: 1_000_000_000,
    expect_amount_out: 990_000_000,
    min_return: 970_000_000,
    
    amounts: vec![
        300_000_000,  // 30% → Route 1
        400_000_000,  // 40% → Route 2
        300_000_000,  // 30% → Route 3
    ],
    
    routes: vec![
        // Route 1: single hop, single DEX
        vec![
            Route {
                dexes: vec![Dex::RaydiumSwap],
                weights: vec![100],
            },
        ],
        
        // Route 2: 2-hop
        vec![
            // Hop 1: USDC → SOL
            Route {
                dexes: vec![Dex::Whirlpool],
                weights: vec![100],
            },
            // Hop 2: SOL → USDT
            Route {
                dexes: vec![Dex::MeteoraDynamicpool],
                weights: vec![100],
            },
        ],
        
        // Route 3: single hop, dual DEX
        vec![
            Route {
                dexes: vec![Dex::RaydiumSwap, Dex::Whirlpool],
                weights: vec![60, 40],
            },
        ],
    ],
}
```

---

## 🔬 Advanced Practice Problems

### Exercise 1: Three-DEX Split Precision
Input: 500 USDC
Weights: [33, 33, 34] (Raydium, Whirlpool, Meteora)

Calculate exact input amount for each DEX (consider integer division)

**Hint**:
```rust
DEX 0: 500 × 33 / 100 = 165
DEX 1: 500 × 33 / 100 = 165
DEX 2: 500 - 165 - 165 = 170  // remainder handling
```

### Exercise 2: Multi-hop Account Chain
Draw account flow for the following swap:
- USDC → SOL → RAY → BONK (3 hops)
- User accounts: user_usdc, user_bonk
- Intermediate accounts: intermediate_sol, intermediate_ray

**Answer Format**:
```
Hop 0: user_usdc → intermediate_sol
Hop 1: intermediate_sol → intermediate_ray
Hop 2: intermediate_ray → user_bonk
```

### Exercise 3: Slippage Calculation
Given different slippage tolerances, calculate min_return:
- expect_amount_out = 1000 USDC
- 1% slippage: min_return = ?
- 2% slippage: min_return = ?
- 5% slippage: min_return = ?

**Answer**:
```rust
1% slippage: 1000 × (1 - 0.01) = 990
2% slippage: 1000 × (1 - 0.02) = 980
5% slippage: 1000 × (1 - 0.05) = 950
```

---

## 📚 Related Learning Path

### Completed
- ✅ Phase 2: Data Structures (Dex, Route, SwapArgs, HopAccounts)
- ✅ Current: Routing Execution Flow (execute_swap, distribute_swap)

### Next Steps
1. **DEX Adapters** (Phase 4)
   - `programs/dex-solana/src/adapters/raydium.rs`
   - `programs/dex-solana/src/adapters/whirlpool.rs`
   - Understand how to interact with specific DEXs

2. **Account Management** (Phase 5)
   - `programs/dex-solana/src/utils/account.rs`
   - Understand intermediate account creation and management

3. **Commission System** (Phase 6)
   - `programs/dex-solana/src/instructions/common_commission.rs`
   - Understand how fees are extracted from swaps

---

## 💡 Key Takeaways

### 1. Two-Level Split Architecture
```
Level 1 (Route Level)
  └─> Distribute total input to different parallel paths
      └─> Each path can have different hop structures

Level 2 (DEX Level)
  └─> Within each hop, distribute by weight to multiple DEXs
      └─> Weights must sum to 100
```

### 2. Three-Layer Loop Execution
```rust
for each route in routes {              // Level 1
    for each hop in route.hops {        // Multi-hop
        for each dex in hop.dexes {     // Level 2
            execute_swap_on_dex();
        }
    }
}
```

### 3. Critical Validation Points
- ✅ `amounts.len() == routes.len()`
- ✅ `sum(amounts) == amount_in`
- ✅ `dexes.len() == weights.len()`
- ✅ `sum(weights) == 100`
- ✅ `expect_amount_out >= min_return`
- ✅ First hop: `from_account == user_source`
- ✅ Last hop: `to_account == user_destination`

### 4. Remainder Handling Strategy
Last DEX uses remainder instead of calculating weight to ensure:
- No leftover funds (due to rounding)
- All input is utilized
- Mathematical precision

---

## 🎓 Learning Outcomes Assessment

After completing this phase, you should be able to:

| Capability | Description | Self-Check |
|------------|-------------|------------|
| Code Navigation | Quickly locate all key swap-related functions | [ ] |
| Flow Understanding | Draw complete swap execution flowchart | [ ] |
| Parameter Construction | Construct a valid SwapArgs | [ ] |
| Amount Calculation | Manually calculate all split amounts | [ ] |
| Error Identification | Identify configuration errors in SwapArgs | [ ] |
| Code Tracing | Trace code execution line by line | [ ] |
| Concept Explanation | Explain two-level split mechanism to others | [ ] |

---

**🎉 Upon completing this guide, you've mastered the core execution logic of the DEX Router!**

Next Step: Deep dive into specific DEX adapter implementations (Phase 4)

