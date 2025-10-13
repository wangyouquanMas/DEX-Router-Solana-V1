# 🗺️ DEX Router Learning Roadmap

A comprehensive guide to mastering the multi-DEX aggregation system in this Solana DEX router.

---

## 📊 Overview

This roadmap will take you from beginner to expert in understanding how this router aggregates 77+ DEXs to provide optimal swap execution on Solana.

**Estimated Time:** 4-6 weeks (depending on prior knowledge)  
**Prerequisites:** Basic Rust, TypeScript, and blockchain concepts

---

## 🎯 Phase 1: Foundation (Week 1)

### 1.1 Prerequisites Knowledge
**Goal:** Build the foundational knowledge needed

- [ ] **Solana Basics**
  - Account model (Programs, PDAs, Token Accounts)
  - Transaction structure
  - Cross-Program Invocation (CPI)
  - [Resource: Solana Cookbook](https://solanacookbook.com/)

- [ ] **Anchor Framework**
  - Program structure
  - Account constraints
  - Context and remaining_accounts
  - [Resource: Anchor Book](https://book.anchor-lang.com/)

- [ ] **Token Programs**
  - SPL Token vs Token-2022
  - Token accounts and ATAs
  - Transfer authorities

**📝 Checkpoint:** Can you explain what a PDA is and why we need it?

### 1.2 Project Architecture
**Files to Study:**

```bash
# Start here
├── README.md                          # Overall architecture
├── Anchor.toml                        # Build configuration
├── programs/dex-solana/src/lib.rs     # Entry points (15 min)
└── programs/dex-solana/src/constants.rs  # Key constants (10 min)
```

**Key Questions:**
- What are the main instruction types?
- What's the difference between swap, proxy_swap, and commission_swap?
- What are the MAX_HOPS and TOTAL_WEIGHT constants?

**🎓 Exercise:** Draw a diagram of the program structure showing all entry points

---

## 🏗️ Phase 2: Core Routing Engine (Week 2)

### 2.1 Data Structures
**Goal:** Understand how routes are represented

**Study Files:**
```rust
programs/dex-solana/src/instructions/common_swap.rs
  Lines 10-96:  Dex enum (77 DEXs)
  Lines 86-96:  Route structure
  Lines 92-96:  SwapArgs structure
```

**Key Concepts:**
- [ ] `Dex` enum - 77 different DEX types
- [ ] `Route` structure - dexes + weights
- [ ] `SwapArgs` - amounts, routes, slippage
- [ ] `HopAccounts` - tracking inter-hop state

**📝 Checkpoint:** Can you construct a SwapArgs for a 2-hop, 3-DEX swap?

### 2.2 Routing Execution Flow
**Goal:** Understand the two-level split mechanism

**Study Path:**
```
1. programs/dex-solana/src/instructions/swap.rs
   └─> swap_handler() - Entry point

2. programs/dex-solana/src/instructions/common_swap.rs
   └─> common_swap() - Core logic
   └─> execute_swap() - Lines 406-548
       ├─> Level 1: Route splitting (lines 448-449)
       ├─> Level 2: DEX weight distribution (lines 467-492)
       └─> distribute_swap() - Lines 550-669
```

**Deep Dive - execute_swap() function:**
```rust
// Lines 446-521: The heart of routing
for (i, hops) in routes.iter().enumerate() {
    // Level 1: Split by route
    let mut amount_in = amounts[i];
    
    for (hop, route) in hops.iter().enumerate() {
        // Level 2: Split by DEX weight
        for (index, dex) in route.dexes.iter().enumerate() {
            // Calculate split amount
            // Execute swap via adapter
            // Emit event
        }
    }
}
```

**🎓 Exercise:** 
Trace a swap with:
- 1000 USDC input
- 2 routes (60/40 split)
- Route 1: Raydium (100%)
- Route 2: Whirlpool (70%) + Meteora (30%)

Calculate exact amounts for each DEX.

---

## 🔌 Phase 3: DEX Adapters (Week 3)

### 3.1 Adapter Pattern
**Goal:** Understand how different DEXs are integrated

**Study Files:**
```bash
programs/dex-solana/src/adapters/
├── mod.rs              # All adapter exports
├── common.rs           # Shared adapter logic
├── raydium.rs          # Example: Complex adapter
└── whirlpool.rs        # Example: CLMM adapter
```

**Key Concepts:**
- [ ] Account parsing with offset tracking
- [ ] Instruction building for each DEX
- [ ] CPI invocation patterns
- [ ] Before/after validation

### 3.2 Raydium Adapter Deep Dive
**Study: programs/dex-solana/src/adapters/raydium.rs**

**Pattern to Learn:**
```rust
// 1. Parse accounts from remaining_accounts
let swap_accounts = RaydiumSwapAccounts::parse_accounts(remaining_accounts, *offset)?;

// 2. Validate
before_check(swap_authority, source_token, dest_token, hop_accounts, ...)?;

// 3. Build instruction
let instruction = Instruction { program_id, accounts, data };

// 4. Execute via CPI
invoke_process(...)?;

// 5. Update offset
*offset += ACCOUNTS_LEN;
```

**Account Complexity:**
- RaydiumSwap: 19 accounts
- RaydiumCLMM: 14 accounts
- RaydiumCPMM: 14 accounts

**🎓 Exercise:** 
1. Add logging to track offset changes
2. Map out account flow for a 3-DEX swap
3. Calculate total accounts needed for complex route

### 3.3 Build Your Own Adapter
**Challenge:** Study 3 different adapter types:

1. **AMM Style** (raydium.rs)
   - Pool-based swaps
   - Serum integration

2. **CLMM Style** (whirlpool.rs)
   - Concentrated liquidity
   - Tick arrays

3. **Order Book** (phoenix.rs, manifest.rs)
   - Limit order books
   - Market orders

**📝 Checkpoint:** Can you explain the differences in account structures?

---

## 🎨 Phase 4: Advanced Features (Week 4)

### 4.1 Multi-Hop Swaps
**Goal:** Understand intermediate account management

**Study Files:**
```rust
programs/dex-solana/src/adapters/common.rs
  Lines 25-76:  before_check() - Hop validation
  Lines 177-194: post_swap_check() - State updates
```

**HopAccounts Tracking:**
```rust
pub struct HopAccounts {
    pub last_to_account: Pubkey,   // Output of previous hop
    pub from_account: Pubkey,       // Current source
    pub to_account: Pubkey,         // Current destination
}
```

**Validation Rules:**
- Hop N output = Hop N+1 input
- Authority validation per hop
- PDA authority for intermediate hops

**🎓 Exercise:** Trace a 3-hop swap (A→B→C→D) and track HopAccounts changes

### 4.2 Commission System
**Study Files:**
```rust
programs/dex-solana/src/instructions/
├── commission_swap.rs
└── commission_proxy_swap.rs
```

**Commission Encoding:**
```typescript
// 32-bit packed format
const commissionInfo = 
  (direction ? 1 << 31 : 0) |  // Bit 31: direction
  (rate & 0x3FFFFFFF);          // Bits 0-30: rate
```

**Types:**
- Commission from input (before swap)
- Commission from output (after swap)
- SPL token commission
- SOL commission

**📝 Checkpoint:** Calculate commission for 1000 USDC at 1% rate, both directions

### 4.3 Platform Fees (V2 & V3)
**Study Files:**
```rust
programs/dex-solana/src/lib.rs
  Lines 147-181: platform_fee_*_v2
  Lines 192-224: swap_v3, swap_tob_v3
```

**Fee Hierarchy:**
```
User Input
  ↓
[Commission] ← Referrer/Partner fee
  ↓
[Platform Fee] ← Protocol fee
  ↓
[Trim (optional)] ← MEV protection
  ↓
DEX Swap
```

**🎓 Exercise:** Calculate final swap amount with:
- Input: 1000 USDC
- Commission: 50 bps (0.5%)
- Platform fee: 30 bps (0.3%)
- Trim: 10 bps (0.1%)

### 4.4 Proxy Swaps
**Goal:** Understand delegated trading

**Study Files:**
```rust
programs/dex-solana/src/instructions/proxy_swap.rs
programs/dex-solana/src/processor/proxy_swap_processor.rs
```

**Authority Flow:**
- User delegates to PDA
- PDA executes swaps
- Prevents front-running
- Enables gasless transactions

---

## 🧪 Phase 5: Testing & Integration (Week 5)

### 5.1 Test Suite Analysis
**Study Files:**
```bash
tests/
├── clone.test.js
├── swap.test.ts
└── integration/
```

**Learn:**
- [ ] How to set up test environment
- [ ] Account mocking strategies
- [ ] Fork testing patterns
- [ ] Integration test structure

### 5.2 Build Test Cases
**Exercises:**

**Test 1: Basic Swap**
```typescript
// Single route, single DEX
const route = {
  amounts: [new BN(1_000_000)],
  routes: [[{
    dexes: [Dex.RaydiumSwap],
    weights: [100]
  }]]
};
```

**Test 2: Split Route**
```typescript
// Two routes, different DEXs
const route = {
  amounts: [new BN(600_000), new BN(400_000)],
  routes: [
    [{ dexes: [Dex.RaydiumSwap], weights: [100] }],
    [{ dexes: [Dex.Whirlpool], weights: [100] }]
  ]
};
```

**Test 3: Complex Multi-hop**
```typescript
// Two routes, one with multi-hop
const route = {
  amounts: [new BN(500_000), new BN(500_000)],
  routes: [
    [{ dexes: [Dex.RaydiumSwap], weights: [100] }],
    [
      { dexes: [Dex.Whirlpool], weights: [100] },  // Hop 1
      { dexes: [Dex.Meteora], weights: [100] }     // Hop 2
    ]
  ]
};
```

**🎓 Exercise:** Write tests for edge cases:
- Maximum hops (3)
- Maximum weight splits
- Slippage failures
- Invalid account ordering

### 5.3 Client SDK Integration
**Build:**
1. Quote aggregator (off-chain)
2. Route optimizer
3. Account builder
4. Transaction sender

**📝 Checkpoint:** Can you build a working swap from scratch?

---

## 🚀 Phase 6: Optimization & Extension (Week 6)

### 6.1 Performance Analysis
**Study:**

**Gas Optimization:**
- Account packing strategies
- Compute unit usage
- Instruction batching

**Files to Analyze:**
```rust
programs/dex-solana/src/constants.rs
  DEFAULT_COMPUTE_UNIT_LIMIT: 200_000
```

### 6.2 Add New DEX Adapter
**Challenge:** Implement a new DEX integration

**Steps:**
1. Study the DEX's program interface
2. Define account structure
3. Implement parse_accounts()
4. Build swap instruction
5. Add to Dex enum
6. Add to distribute_swap() match
7. Write tests

**Template:**
```rust
// programs/dex-solana/src/adapters/my_dex.rs

pub struct MyDexAccounts<'info> {
    pub dex_program_id: &'info AccountInfo<'info>,
    pub swap_authority: &'info AccountInfo<'info>,
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub dest_token: InterfaceAccount<'info, TokenAccount>,
    // ... DEX-specific accounts
}

pub fn swap<'a>(
    remaining_accounts: &'a [AccountInfo<'a>],
    amount_in: u64,
    offset: &mut usize,
    hop_accounts: &mut HopAccounts,
    hop: usize,
    proxy_swap: bool,
    owner_seeds: Option<&[&[&[u8]]]>,
) -> Result<u64> {
    // Implementation
}
```

### 6.3 Advanced Routing Strategies
**Off-chain Optimization:**

**Study Topics:**
- [ ] Graph-based pathfinding
- [ ] Price impact curves
- [ ] Liquidity depth analysis
- [ ] MEV protection strategies

**Algorithm Design:**
```
1. Query all DEX pools
2. Build price graph
3. Find optimal splits
4. Account for gas costs
5. Minimize slippage
6. Generate SwapArgs
```

---

## 📚 Reference Materials

### Code Map
```
Key Files Priority Order:

1. ★★★ programs/dex-solana/src/instructions/common_swap.rs
   - Core routing logic
   - Lines 406-548: execute_swap()
   - Lines 550-669: distribute_swap()

2. ★★★ programs/dex-solana/src/adapters/common.rs
   - Shared adapter patterns
   - before_check(), invoke_process()

3. ★★★ programs/dex-solana/src/adapters/raydium.rs
   - Reference adapter implementation
   - All account patterns

4. ★★☆ programs/dex-solana/src/lib.rs
   - Program entry points
   - Instruction variants

5. ★★☆ programs/dex-solana/src/processor/
   - Processor implementations
   - Fee calculations

6. ★☆☆ programs/dex-solana/src/limitorder/
   - Limit order system
   - Advanced feature
```

### Critical Concepts Matrix

| Concept | Difficulty | Importance | Files |
|---------|-----------|------------|-------|
| Two-level routing | ★★★ | Critical | common_swap.rs:446-521 |
| Offset tracking | ★★☆ | Critical | All adapters |
| Hop validation | ★★☆ | Critical | common.rs:25-76 |
| CPI patterns | ★★☆ | Critical | common.rs:78-155 |
| Commission encoding | ★☆☆ | Important | commission_swap.rs |
| PDA authority | ★★☆ | Important | constants.rs:77-81 |

### Debugging Checklist

**Common Issues:**
- [ ] Offset miscalculation
- [ ] Account ordering mismatch
- [ ] Authority validation failure
- [ ] Hop account inconsistency
- [ ] Weight sum != 100
- [ ] Slippage tolerance exceeded

**Debugging Tools:**
```bash
# View logs
anchor test --skip-local-validator

# Program logs
solana logs --url <cluster>

# Account inspection
solana account <address> --url <cluster>
```

---

## 🎯 Final Project: Build a Complete Swap Interface

### Requirements:
1. **Quote Engine**
   - Query 5+ DEXs
   - Calculate optimal route
   - Generate SwapArgs

2. **Transaction Builder**
   - Assemble all accounts
   - Handle remaining_accounts ordering
   - Add priority fees

3. **UI Integration**
   - Input token amount
   - Display route visualization
   - Show fee breakdown
   - Execute swap

4. **Advanced Features**
   - Multi-hop routing
   - Commission integration
   - Slippage configuration
   - Transaction history

### Success Criteria:
- [ ] Can execute basic swap
- [ ] Can execute split route
- [ ] Can execute multi-hop
- [ ] Handles all edge cases
- [ ] Optimized gas usage
- [ ] Clean error handling

---

## 📈 Progress Tracker

**Week 1 - Foundation**
- [ ] Completed Solana basics
- [ ] Understood Anchor framework
- [ ] Mapped project structure
- [ ] Drew architecture diagram

**Week 2 - Core Routing**
- [ ] Understood data structures
- [ ] Traced execute_swap logic
- [ ] Analyzed routing algorithm
- [ ] Completed split calculation exercise

**Week 3 - Adapters**
- [ ] Studied adapter pattern
- [ ] Deep-dived Raydium adapter
- [ ] Compared 3 adapter types
- [ ] Mapped account flows

**Week 4 - Advanced Features**
- [ ] Understood multi-hop
- [ ] Implemented commission
- [ ] Analyzed platform fees
- [ ] Studied proxy swaps

**Week 5 - Testing**
- [ ] Analyzed test suite
- [ ] Wrote basic tests
- [ ] Built integration tests
- [ ] Created client SDK

**Week 6 - Mastery**
- [ ] Optimized performance
- [ ] Added new DEX
- [ ] Built complete swap UI
- [ ] Deployed to devnet

---

## 🤝 Community Resources

- **Discord:** [OKX DEX API Community](https://discord.gg/okxdexapi)
- **GitHub Issues:** Report bugs and ask questions
- **Documentation:** Read the latest updates

---

## 📝 Notes Section

Use this space for your own notes as you progress:

### Week 1 Notes:
```
[Your notes here]
```

### Week 2 Notes:
```
[Your notes here]
```

### Questions to Ask:
```
1. 
2. 
3. 
```

---

**Good luck on your journey to mastering DEX aggregation! 🚀**

