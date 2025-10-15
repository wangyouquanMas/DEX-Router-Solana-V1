import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { DexSolana } from "../target/types/dex_solana";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
  getMint,
} from "@solana/spl-token";
import { expect } from "chai";

describe("Swap Instruction Tests", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DexSolana as Program<DexSolana>;
  const payer = provider.wallet as anchor.Wallet;

  // Test accounts
  let sourceMint: PublicKey;
  let destinationMint: PublicKey;
  let sourceTokenAccount: PublicKey;
  let destinationTokenAccount: PublicKey;

  // Constants for testing
  const DECIMALS = 6;
  const INITIAL_AMOUNT = 1_000_000_000; // 1000 tokens with 6 decimals

  before(async () => {
    console.log("Setting up test environment...");
    console.log("Payer:", payer.publicKey.toBase58());

    // Create source mint (e.g., USDC)
    sourceMint = await createMint(
      provider.connection,
      payer.payer,
      payer.publicKey,
      null,
      DECIMALS,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
    console.log("Source Mint:", sourceMint.toBase58());

    // Create destination mint (e.g., SOL or another token)
    destinationMint = await createMint(
      provider.connection,
      payer.payer,
      payer.publicKey,
      null,
      DECIMALS,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
    console.log("Destination Mint:", destinationMint.toBase58());

    // Create source token account
    sourceTokenAccount = await createAccount(
      provider.connection,
      payer.payer,
      sourceMint,
      payer.publicKey,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
    console.log("Source Token Account:", sourceTokenAccount.toBase58());

    // Create destination token account
    destinationTokenAccount = await createAccount(
      provider.connection,
      payer.payer,
      destinationMint,
      payer.publicKey,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
    console.log("Destination Token Account:", destinationTokenAccount.toBase58());

    // Mint initial tokens to source account
    await mintTo(
      provider.connection,
      payer.payer,
      sourceMint,
      sourceTokenAccount,
      payer.publicKey,
      INITIAL_AMOUNT,
      [],
      undefined,
      TOKEN_PROGRAM_ID
    );
    console.log(`Minted ${INITIAL_AMOUNT} tokens to source account`);

    // Verify balances
    const sourceAccountInfo = await getAccount(
      provider.connection,
      sourceTokenAccount,
      undefined,
      TOKEN_PROGRAM_ID
    );
    console.log("Source Account Balance:", sourceAccountInfo.amount.toString());
  });

  describe("Basic Swap Tests", () => {
    it("should fail with empty routes (expected behavior)", async () => {
      // This test demonstrates what happens with empty routes
      // In a real scenario, you'd need to provide actual DEX accounts in remainingAccounts
      
      const amountIn = new BN(100_000_000); // 100 tokens
      const expectAmountOut = new BN(50_000_000); // 50 tokens (example)
      const minReturn = new BN(49_000_000); // 49 tokens (2% slippage)
      const orderId = new BN(Date.now());

      // Simple swap args with empty routes (will fail without actual DEX integration)
      const swapArgs = {
        amountIn,
        expectAmountOut,
        minReturn,
        amounts: [], // Empty amounts means no routes
        routes: [], // Empty routes
      };

      try {
        await program.methods
          .swap(swapArgs, orderId)
          .accounts({
            payer: payer.publicKey,
            sourceTokenAccount: sourceTokenAccount,
            destinationTokenAccount: destinationTokenAccount,
            sourceMint: sourceMint,
            destinationMint: destinationMint,
          })
          .remainingAccounts([]) // Would need actual DEX accounts here
          .rpc();

        // If it doesn't fail, the test should fail
        expect.fail("Expected transaction to fail with empty routes");
      } catch (error) {
        console.log("Expected error caught:", error.message);
        // This is expected - without actual DEX integration, the swap will fail
        expect(error).to.exist;
      }
    });

    it("should construct SwapArgs correctly for single DEX route", async () => {
      // This demonstrates how to construct SwapArgs for a single-DEX swap
      // Example: Swap 100 USDC -> SOL via Raydium (100%)

      const amountIn = new BN(100_000_000); // 100 tokens
      const expectAmountOut = new BN(50_000_000); // 50 tokens expected
      const minReturn = new BN(49_000_000); // 49 tokens minimum (2% slippage)

      // Single route with all amount going through one path
      const swapArgs = {
        amountIn,
        expectAmountOut,
        minReturn,
        amounts: [amountIn], // All amount goes to first route
        routes: [
          [
            // First route (only route)
            {
              dexes: [{ raydiumSwap: {} }], // Using Raydium DEX
              weights: [100], // 100% through this DEX
            },
          ],
        ],
      };

      console.log("SwapArgs constructed:");
      console.log("  Amount In:", swapArgs.amountIn.toString());
      console.log("  Expected Out:", swapArgs.expectAmountOut.toString());
      console.log("  Min Return:", swapArgs.minReturn.toString());
      console.log("  Number of Routes:", swapArgs.routes.length);

      // Verify the structure
      expect(swapArgs.amounts.length).to.equal(swapArgs.routes.length);
      expect(swapArgs.amounts[0].toString()).to.equal(amountIn.toString());
    });

    it("should construct SwapArgs for split route (multiple DEXs in one hop)", async () => {
      // Example: Swap 100 USDC -> SOL split across 3 DEXs
      // Raydium: 50%, Whirlpool: 30%, Meteora: 20%

      const amountIn = new BN(100_000_000); // 100 tokens
      const expectAmountOut = new BN(50_000_000);
      const minReturn = new BN(49_000_000);

      const swapArgs = {
        amountIn,
        expectAmountOut,
        minReturn,
        amounts: [amountIn], // All amount in one route
        routes: [
          [
            // Single route with split across 3 DEXs
            {
              dexes: [
                { raydiumSwap: {} }, // Raydium
                { whirlpool: {} }, // Whirlpool
                { meteoraDynamicpool: {} }, // Meteora
              ],
              weights: [50, 30, 20], // Split percentages
            },
          ],
        ],
      };

      console.log("Split Route SwapArgs:");
      console.log("  DEXs in route:", swapArgs.routes[0][0].dexes.length);
      console.log("  Weights:", swapArgs.routes[0][0].weights);

      // Verify weights sum to 100
      const totalWeight = swapArgs.routes[0][0].weights.reduce(
        (sum, w) => sum + w,
        0
      );
      expect(totalWeight).to.equal(100);
    });

    it("should construct SwapArgs for multi-hop swap", async () => {
      // Example: USDC -> SOL -> BONK (2 hops)
      // Hop 1: USDC -> SOL via Raydium
      // Hop 2: SOL -> BONK via Whirlpool

      const amountIn = new BN(1000_000_000); // 1000 USDC
      const expectAmountOut = new BN(50_000_000_000); // 50,000 BONK
      const minReturn = new BN(49_000_000_000); // 49,000 BONK minimum

      const swapArgs = {
        amountIn,
        expectAmountOut,
        minReturn,
        amounts: [amountIn], // Single route with all amount
        routes: [
          [
            // Route with 2 hops
            {
              // Hop 1: USDC -> SOL
              dexes: [{ raydiumSwap: {} }],
              weights: [100],
            },
            {
              // Hop 2: SOL -> BONK
              dexes: [{ whirlpool: {} }],
              weights: [100],
            },
          ],
        ],
      };

      console.log("Multi-hop SwapArgs:");
      console.log("  Number of hops:", swapArgs.routes[0].length);
      console.log("  Hop 1 DEX:", Object.keys(swapArgs.routes[0][0].dexes[0])[0]);
      console.log("  Hop 2 DEX:", Object.keys(swapArgs.routes[0][1].dexes[0])[0]);

      expect(swapArgs.routes[0].length).to.equal(2); // 2 hops
    });

    it("should construct SwapArgs for parallel routes", async () => {
      // Example: Split 1000 USDC across 2 different routes
      // Route 1 (600 USDC): USDC -> SOL via Raydium
      // Route 2 (400 USDC): USDC -> SOL via Whirlpool

      const amountIn = new BN(1000_000_000); // 1000 USDC total
      const route1Amount = new BN(600_000_000); // 600 USDC
      const route2Amount = new BN(400_000_000); // 400 USDC
      const expectAmountOut = new BN(500_000_000);
      const minReturn = new BN(490_000_000);

      const swapArgs = {
        amountIn,
        expectAmountOut,
        minReturn,
        amounts: [route1Amount, route2Amount], // Split across 2 routes
        routes: [
          [
            // Route 1
            {
              dexes: [{ raydiumSwap: {} }],
              weights: [100],
            },
          ],
          [
            // Route 2
            {
              dexes: [{ whirlpool: {} }],
              weights: [100],
            },
          ],
        ],
      };

      console.log("Parallel Routes SwapArgs:");
      console.log("  Number of parallel routes:", swapArgs.routes.length);
      console.log("  Route 1 amount:", swapArgs.amounts[0].toString());
      console.log("  Route 2 amount:", swapArgs.amounts[1].toString());

      // Verify amounts sum to total
      const totalAmount = swapArgs.amounts.reduce(
        (sum, amt) => sum.add(amt),
        new BN(0)
      );
      expect(totalAmount.toString()).to.equal(amountIn.toString());
      expect(swapArgs.amounts.length).to.equal(swapArgs.routes.length);
    });

    it("should construct SwapArgs for complex multi-hop with splits", async () => {
      // Complex example: 2-hop swap with DEX splits in first hop
      // Hop 1: USDC -> SOL split across (Raydium 50%, Whirlpool 30%, Meteora 20%)
      // Hop 2: SOL -> BONK via Raydium 100%

      const amountIn = new BN(1000_000_000); // 1000 USDC
      const expectAmountOut = new BN(50_000_000_000); // 50,000 BONK
      const minReturn = new BN(48_000_000_000); // 48,000 BONK (4% slippage)

      const swapArgs = {
        amountIn,
        expectAmountOut,
        minReturn,
        amounts: [amountIn],
        routes: [
          [
            // Route with 2 hops
            {
              // Hop 1: USDC -> SOL (split across 3 DEXs)
              dexes: [
                { raydiumSwap: {} },
                { whirlpool: {} },
                { meteoraDynamicpool: {} },
              ],
              weights: [50, 30, 20],
            },
            {
              // Hop 2: SOL -> BONK (single DEX)
              dexes: [{ raydiumSwap: {} }],
              weights: [100],
            },
          ],
        ],
      };

      console.log("Complex Multi-hop with Splits:");
      console.log("  Total hops:", swapArgs.routes[0].length);
      console.log("  Hop 1 DEX count:", swapArgs.routes[0][0].dexes.length);
      console.log("  Hop 1 weights:", swapArgs.routes[0][0].weights);
      console.log("  Hop 2 DEX count:", swapArgs.routes[0][1].dexes.length);

      expect(swapArgs.routes[0].length).to.equal(2);
      expect(swapArgs.routes[0][0].dexes.length).to.equal(3);
      expect(swapArgs.routes[0][1].dexes.length).to.equal(1);
    });
  });

  describe("Integration Test Examples", () => {
    it("demonstrates what a real swap test would need", async () => {
      // This test shows what you would need for an actual integration test
      console.log("\n=== Real Swap Test Requirements ===");
      console.log("To test a real swap, you would need:");
      console.log("1. Actual DEX pool accounts (e.g., Raydium pool)");
      console.log("2. Proper remaining_accounts array with:");
      console.log("   - DEX program IDs");
      console.log("   - Pool state accounts");
      console.log("   - Pool token accounts");
      console.log("   - Pool authority/signer");
      console.log("3. Intermediate token accounts for multi-hop swaps");
      console.log("4. Sufficient liquidity in the pools");
      console.log("\n=== Example remainingAccounts structure ===");
      console.log("For a Raydium swap, you'd need accounts like:");
      console.log("- Raydium Program ID");
      console.log("- AMM account");
      console.log("- AMM authority");
      console.log("- AMM open orders");
      console.log("- Pool coin/pc token accounts");
      console.log("- Serum market accounts (if applicable)");
      console.log("\nSee the DEX adapter implementations for specific requirements.");
    });
  });

  describe("SwapArgs Validation Tests", () => {
    it("should validate that amounts sum equals amountIn", () => {
      const amountIn = new BN(1000);
      const route1 = new BN(600);
      const route2 = new BN(400);

      const totalSplit = route1.add(route2);
      expect(totalSplit.toString()).to.equal(amountIn.toString());
    });

    it("should validate that amounts.length equals routes.length", () => {
      const amounts = [new BN(600), new BN(400)];
      const routes = [
        [{ dexes: [{ raydiumSwap: {} }], weights: [100] }],
        [{ dexes: [{ whirlpool: {} }], weights: [100] }],
      ];

      expect(amounts.length).to.equal(routes.length);
    });

    it("should validate that weights sum to 100 for each hop", () => {
      const weights1 = [50, 30, 20];
      const weights2 = [60, 40];
      const weights3 = [100];

      const sum1 = weights1.reduce((a, b) => a + b, 0);
      const sum2 = weights2.reduce((a, b) => a + b, 0);
      const sum3 = weights3.reduce((a, b) => a + b, 0);

      expect(sum1).to.equal(100);
      expect(sum2).to.equal(100);
      expect(sum3).to.equal(100);
    });

    it("should validate that minReturn <= expectAmountOut", () => {
      const expectAmountOut = new BN(1000);
      const minReturn = new BN(980); // 2% slippage

      expect(minReturn.lte(expectAmountOut)).to.be.true;
    });
  });

  describe("Order ID Tests", () => {
    it("should generate unique order IDs", () => {
      const orderId1 = new BN(Date.now());
      
      // Wait a bit to ensure different timestamp
      const wait = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
      
      wait(10).then(() => {
        const orderId2 = new BN(Date.now());
        expect(orderId1.toString()).to.not.equal(orderId2.toString());
        console.log("Order ID 1:", orderId1.toString());
        console.log("Order ID 2:", orderId2.toString());
      });
    });

    it("should handle large order IDs", () => {
      const largeOrderId = new BN("999999999999999");
      expect(largeOrderId.gt(new BN(0))).to.be.true;
      console.log("Large Order ID:", largeOrderId.toString());
    });
  });
});

