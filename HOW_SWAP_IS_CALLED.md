# 🔄 How Swap Function is Called - Complete Flow

## 📊 Visual Flow Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│  1. USER RUNS COMMAND                                           │
│  $ npm run test:swap                                            │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│  2. NPM EXECUTES (package.json line 8)                          │
│  ts-mocha -p ./tsconfig.json -t 1000000 tests/swap.test.ts      │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│  3. MOCHA LOADS TEST FILE (swap.test.ts)                        │
│                                                                 │
│  ┌────────────────────────────────────────────────────────┐    │
│  │ describe("Swap Instruction Tests", () => {             │    │
│  │   before(async () => {                                 │    │
│  │     // Setup: Create test accounts                     │    │
│  │   });                                                  │    │
│  │                                                        │    │
│  │   it("test case 1", async () => {                     │    │
│  │     // Run test                                        │    │
│  │   });                                                  │    │
│  │ });                                                    │    │
│  └────────────────────────────────────────────────────────┘    │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│  4. MOCHA RUNS "before()" HOOK (lines 39-115)                   │
│                                                                 │
│  - Sets up Anchor provider                                     │
│  - Gets program instance                                       │
│  - Creates test token mints                                    │
│  - Creates token accounts                                      │
│  - Mints initial tokens                                        │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│  5. MOCHA RUNS EACH "it()" TEST                                 │
│                                                                 │
│  Test 1: Empty routes (lines 118-156)                          │
│  Test 2: Single DEX route (lines 158-192) ← EXPERIMENT 1       │
│  Test 3: Split route (lines 194-232)                           │
│  ... and so on                                                 │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼ (Focus on Test 2 - Experiment 1)
┌─────────────────────────────────────────────────────────────────┐
│  6. INSIDE TEST 2: "Single DEX route" (lines 158-192)          │
│                                                                 │
│  Line 162: const amountIn = new BN(100_000_000);               │
│  Line 163: const expectAmountOut = new BN(50_000_000);         │
│  Line 164: const minReturn = new BN(49_000_000);               │
│                                                                 │
│  Lines 167-181: Build swapArgs object                          │
│  const swapArgs = {                                            │
│    amountIn,                                                   │
│    expectAmountOut,                                            │
│    minReturn,                                                  │
│    amounts: [amountIn],                                        │
│    routes: [[                                                  │
│      {                                                         │
│        dexes: [{ raydiumSwap: {} }],                           │
│        weights: [100],                                         │
│      }                                                         │
│    ]]                                                          │
│  };                                                            │
│                                                                 │
│  Lines 183-187: Log the values                                 │
│  console.log("SwapArgs constructed:");                         │
│  console.log("  Amount In:", swapArgs.amountIn.toString());    │
│                                                                 │
│  Lines 190-191: Validate structure                             │
│  expect(swapArgs.amounts.length).to.equal(...);                │
│                                                                 │
│  ⚠️ NOTE: This test does NOT call the swap function!           │
│            It only validates the structure.                    │
└─────────────────────────────────────────────────────────────────┘


NOW LET'S SEE WHERE SWAP IS ACTUALLY CALLED:

┌─────────────────────────────────────────────────────────────────┐
│  7. SWAP FUNCTION IS CALLED IN TEST 1 (lines 137-147)          │
│     "should fail with empty routes"                            │
│                                                                 │
│  Line 137-147:                                                 │
│  await program.methods                                         │
│    .swap(swapArgs, orderId)  ← THIS CALLS THE FUNCTION!       │
│    .accounts({                                                 │
│      payer: payer.publicKey,                                   │
│      sourceTokenAccount: sourceTokenAccount,                   │
│      destinationTokenAccount: destinationTokenAccount,         │
│      sourceMint: sourceMint,                                   │
│      destinationMint: destinationMint,                         │
│    })                                                          │
│    .remainingAccounts([])                                      │
│    .rpc();  ← Sends transaction to Solana                      │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│  8. ANCHOR FRAMEWORK PROCESSES THE CALL                         │
│                                                                 │
│  program.methods.swap(...) does these steps:                   │
│                                                                 │
│  a) Serializes swapArgs to binary format                       │
│  b) Creates instruction data                                   │
│  c) Adds accounts array                                        │
│  d) Builds Solana transaction                                  │
│  e) Signs transaction with payer                               │
│  f) Sends to Solana RPC                                        │
└──────────────────────┬──────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────────┐
│  9. SOLANA RUNTIME EXECUTES                                     │
│                                                                 │
│  Transaction reaches your program on Solana                    │
│                                                                 │
│  Program: C5x9ZuZETH3RA8QEU83xhFjCkGjPWVVzrWmkV4kS7pmR         │
│                                                                 │
│  Calls: pub fn swap() in lib.rs (line 26)                      │
│                      ↓                                          │
│         instructions::swap_handler() in swap.rs (line 29)      │
│                      ↓                                          │
│         common_swap() in common_swap.rs (line 141)             │
│                      ↓                                          │
│         Executes the swap logic                                │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🔍 Detailed Breakdown: How `program.methods.swap()` Works

### The Call (Line 137 in swap.test.ts)

```typescript
await program.methods
  .swap(swapArgs, orderId)
  .accounts({ ... })
  .remainingAccounts([])
  .rpc();
```

### Breaking It Down Step-by-Step:

#### 1. `program.methods`
```typescript
const program = anchor.workspace.DexSolana as Program<DexSolana>;
//                                                    ↑
//                            This is your Rust program
```
- `program` is a JavaScript object that represents your Rust program
- Created by Anchor from the IDL (Interface Definition Language)
- IDL is in `target/idl/dex_solana.json` (generated when you run `anchor build`)

#### 2. `.swap(swapArgs, orderId)`
```typescript
.swap(swapArgs, orderId)
//    ↑         ↑
//    arg1      arg2
```
- Calls the `swap` function defined in your Rust program (`lib.rs` line 26)
- `swapArgs`: The route/amount data (SwapArgs struct)
- `orderId`: A unique ID for this swap (u64)

**Maps to this Rust function:**
```rust
// lib.rs line 26
pub fn swap<'a>(
    ctx: Context<'_, '_, 'a, 'a, SwapAccounts<'a>>,
    data: SwapArgs,    // ← swapArgs goes here
    order_id: u64,     // ← orderId goes here
) -> Result<()>
```

#### 3. `.accounts({ ... })`
```typescript
.accounts({
  payer: payer.publicKey,
  sourceTokenAccount: sourceTokenAccount,
  destinationTokenAccount: destinationTokenAccount,
  sourceMint: sourceMint,
  destinationMint: destinationMint,
})
```
- Specifies the Solana accounts needed by the instruction
- **Maps to the `SwapAccounts` struct** in `swap.rs` lines 7-27

**Maps to this Rust struct:**
```rust
// swap.rs lines 7-27
#[derive(Accounts)]
pub struct SwapAccounts<'info> {
    pub payer: Signer<'info>,
    pub source_token_account: InterfaceAccount<'info, TokenAccount>,
    pub destination_token_account: InterfaceAccount<'info, TokenAccount>,
    pub source_mint: InterfaceAccount<'info, Mint>,
    pub destination_mint: InterfaceAccount<'info, Mint>,
}
```

#### 4. `.remainingAccounts([])`
```typescript
.remainingAccounts([])
```
- Additional accounts not in the main `SwapAccounts` struct
- For Raydium, this should have **19 accounts** (pool, AMM, Serum, etc.)
- Currently empty `[]` so swap will fail

#### 5. `.rpc()`
```typescript
.rpc()
```
- **Actually sends the transaction** to Solana blockchain
- Returns a transaction signature (string)
- This is where the actual execution happens!

**Alternative methods:**
- `.rpc()` - Send and confirm
- `.simulate()` - Simulate without sending
- `.transaction()` - Build transaction without sending

---

## 📝 Simple Analogy

Think of it like making a phone call:

```
program.methods              → Pick up the phone (get your program)
  .swap(args, id)           → Dial the number (which function to call)
  .accounts({...})          → Identify yourself (who are you, what accounts)
  .remainingAccounts([])    → Additional context (extra info needed)
  .rpc()                    → Press "Call" button (actually make the call)
```

---

## 🎯 Current Test Behavior

### Test 1 (lines 118-156): "Empty routes"
```typescript
await program.methods
  .swap(swapArgs, orderId)
  .accounts({...})
  .remainingAccounts([])  // Empty!
  .rpc();  // ← Tries to execute, but FAILS (expected)
```
**Result**: ❌ Fails because no DEX accounts provided

### Test 2 (lines 158-192): "Single DEX route" ← EXPERIMENT 1
```typescript
const swapArgs = { ... };  // Build args
console.log(...);          // Log values
expect(...);               // Validate structure
// ⚠️ Does NOT call .rpc()! Just validates the structure
```
**Result**: ✅ Passes - only validates structure, doesn't execute

---

## 🔄 Complete Call Chain

```
TypeScript Test File
    ↓
program.methods.swap()
    ↓ (Anchor serializes)
Solana Transaction
    ↓ (RPC sends)
Solana Runtime
    ↓ (Invokes program)
Your Program: dex_solana
    ↓
lib.rs: pub fn swap()
    ↓
instructions/swap.rs: swap_handler()
    ↓
instructions/common_swap.rs: common_swap()
    ↓
adapters/raydium.rs: swap() (if using Raydium)
    ↓
Raydium Program on Solana
    ↓
Token Transfer Happens
```

---

## 💡 Key Takeaways

1. **`program.methods.swap()`** = JavaScript function that calls your Rust `swap` function
2. **`.accounts()`** = Maps to your `SwapAccounts` struct in Rust
3. **`.rpc()`** = Actually executes (sends transaction to blockchain)
4. **Experiment 1 test** = Only validates structure, doesn't call `.rpc()`
5. **To really test Raydium** = Need to add 19 accounts to `.remainingAccounts([])`

---

## 🚀 To Actually Execute Experiment 1

You would need:

```typescript
it("REAL Raydium swap", async () => {
  const swapArgs = {
    amountIn: new BN(100_000_000),
    expectAmountOut: new BN(50_000_000),
    minReturn: new BN(49_000_000),
    amounts: [new BN(100_000_000)],
    routes: [[{
      dexes: [{ raydiumSwap: {} }],
      weights: [100]
    }]]
  };

  // Add all 19 Raydium accounts
  const remainingAccounts = [
    // ... 19 accounts here
  ];

  await program.methods
    .swap(swapArgs, new BN(Date.now()))
    .accounts({
      payer: payer.publicKey,
      sourceTokenAccount: sourceTokenAccount,
      destinationTokenAccount: destinationTokenAccount,
      sourceMint: sourceMint,
      destinationMint: destinationMint,
    })
    .remainingAccounts(remainingAccounts)  // ← Need real accounts!
    .rpc();  // ← This would execute the swap!
});
```

---

## ❓ Questions?

Want me to:
1. Show you how to run the current test?
2. Explain any specific part in more detail?
3. Help you add the 19 Raydium accounts for a real test?

