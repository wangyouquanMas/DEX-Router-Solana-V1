# Quick Start Guide: Testing the Swap Instruction

This guide will help you quickly set up and run tests for the `swap` instruction.

## 🚀 Quick Setup (5 minutes)

### Step 1: Install Dependencies

```bash
npm install
```

This installs:
- `@coral-xyz/anchor` - Anchor framework
- `@solana/spl-token` - SPL Token library
- `@solana/web3.js` - Solana web3 library
- Testing tools (mocha, chai, ts-mocha)

### Step 2: Build the Program

```bash
anchor build
```

This generates the program binary and TypeScript types.

### Step 3: Run the Tests

```bash
# Run all tests
anchor test

# Or run only the swap tests
npm run test:swap
```

## 📝 What the Tests Cover

The `tests/swap.test.ts` file includes:

### ✅ **Test 1: SwapArgs Construction**
Shows how to construct swap arguments for different scenarios:
- Single DEX swap
- Multi-DEX split
- Multi-hop swap
- Parallel routes
- Complex combinations

### ✅ **Test 2: Validation Tests**
Verifies SwapArgs constraints:
- Amounts sum equals amountIn
- Arrays have matching lengths
- Weights sum to 100
- Slippage validation

### ✅ **Test 3: Order ID Tests**
Tests order ID generation and uniqueness

### ⚠️ **Note on Integration Tests**
The tests demonstrate SwapArgs construction but don't execute actual swaps because that requires:
- Real DEX pool accounts
- Proper remaining_accounts
- Actual liquidity

## 🎯 Simple Test Example

Here's the minimal test structure:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { DexSolana } from "../target/types/dex_solana";

describe("Swap Test", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.DexSolana as Program<DexSolana>;

  it("constructs a simple swap", async () => {
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
    
    console.log("SwapArgs created successfully!");
  });
});
```

## 🔍 Understanding SwapArgs

### Basic Fields

```typescript
{
  amountIn: BN,           // Total tokens to swap
  expectAmountOut: BN,    // Expected output
  minReturn: BN,          // Minimum acceptable (slippage protection)
  amounts: BN[],          // Amount per route
  routes: Route[][]       // DEX routing structure
}
```

### Example: Swap 100 USDC → SOL via Raydium

```typescript
{
  amountIn: new BN(100_000_000),      // 100 USDC (6 decimals)
  expectAmountOut: new BN(50_000),    // ~0.05 SOL (9 decimals)
  minReturn: new BN(49_000),          // 2% slippage tolerance
  amounts: [new BN(100_000_000)],     // All goes to route 1
  routes: [[                          // Route 1
    {
      dexes: [{ raydiumSwap: {} }],   // Use Raydium
      weights: [100]                  // 100% through Raydium
    }
  ]]
}
```

## 📊 Test Output

When you run the tests, you'll see:

```
Swap Instruction Tests
  Setting up test environment...
  Payer: <PUBLIC_KEY>
  Source Mint: <MINT_ADDRESS>
  Destination Mint: <MINT_ADDRESS>
  ...
  
  Basic Swap Tests
    ✓ should construct SwapArgs correctly for single DEX route
    ✓ should construct SwapArgs for split route
    ✓ should construct SwapArgs for multi-hop swap
    ✓ should construct SwapArgs for parallel routes
    ✓ should construct SwapArgs for complex multi-hop with splits
    
  SwapArgs Validation Tests
    ✓ should validate that amounts sum equals amountIn
    ✓ should validate that amounts.length equals routes.length
    ✓ should validate that weights sum to 100
    ✓ should validate that minReturn <= expectAmountOut
```

## 🛠️ Troubleshooting

### Error: "Cannot find module '@coral-xyz/anchor'"

```bash
npm install --save-dev @coral-xyz/anchor
```

### Error: "Program not built"

```bash
anchor build
```

### Error: "Validator failed to start"

Make sure you have Solana CLI installed:

```bash
solana --version
# Should show version 1.18+ or 2.x
```

### Error: "Transaction simulation failed"

This is expected! The tests show structure but don't execute real swaps without DEX integration.

## 📚 Next Steps

1. **Study the Test File**: Open `tests/swap.test.ts` to see all examples
2. **Read the README**: Check `tests/README_SWAP_TESTS.md` for detailed documentation
3. **Explore SwapArgs Patterns**: Try modifying the test cases
4. **Study DEX Adapters**: Look at `programs/dex-solana/src/adapters/` for DEX integration
5. **Build Real Tests**: Fork mainnet and test with actual pools

## 📖 Additional Resources

- **Swap Test File**: `tests/swap.test.ts`
- **Detailed Guide**: `tests/README_SWAP_TESTS.md`
- **SwapArgs Definition**: `programs/dex-solana/src/instructions/common_swap.rs` (lines 125-131)
- **Phase 2 Roadmap**: `phase2_roadmap.md`

## 💡 Pro Tips

### 1. Enable Verbose Logging

Add this to see more details:

```typescript
console.log("SwapArgs:", JSON.stringify(swapArgs, null, 2));
```

### 2. Test with Different Scenarios

Try creating your own SwapArgs:

```typescript
it("my custom swap test", async () => {
  const swapArgs = {
    amountIn: new BN(YOUR_AMOUNT),
    // ... customize your test
  };
});
```

### 3. Validate Your Routes

Before calling the instruction, verify:

```typescript
// Check amounts sum
const total = swapArgs.amounts.reduce((sum, amt) => sum.add(amt), new BN(0));
expect(total.toString()).to.equal(swapArgs.amountIn.toString());

// Check array lengths match
expect(swapArgs.amounts.length).to.equal(swapArgs.routes.length);
```

## 🎓 Learning Path

1. ✅ **Run the tests** - Get familiar with the structure
2. ✅ **Read the examples** - Understand SwapArgs patterns
3. ⬜ **Modify test cases** - Try different configurations
4. ⬜ **Study adapters** - Learn how DEXs are integrated
5. ⬜ **Build integration tests** - Test with real pools

## 📞 Support

If you encounter issues:

1. Check the test output for error messages
2. Review `tests/README_SWAP_TESTS.md` for detailed docs
3. Examine the `common_swap.rs` implementation
4. Look at working examples in `phase2_examples.rs`

---

**Happy Testing! 🚀**

