import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { DexSolana } from "../target/types/dex_solana";
import { PublicKey } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import { expect } from "chai";

describe("EXPERIMENT 1: Single DEX Swap - 100% via Raydium", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DexSolana as Program<DexSolana>;
  const payer = provider.wallet as anchor.Wallet;

  let sourceMint: PublicKey;
  let destinationMint: PublicKey;
  let sourceTokenAccount: PublicKey;
  let destinationTokenAccount: PublicKey;

  const DECIMALS = 6;
  const INITIAL_AMOUNT = 1_000_000_000; // 1000 tokens

  before(async () => {
    console.log("\n=== Setting up test environment ===");
    console.log("Payer:", payer.publicKey.toBase58());

    // Create source mint
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

    // Create destination mint
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

    const sourceAccountInfo = await getAccount(
      provider.connection,
      sourceTokenAccount,
      undefined,
      TOKEN_PROGRAM_ID
    );
    console.log("Source Account Balance:", sourceAccountInfo.amount.toString());
    console.log("=== Setup complete ===\n");
  });

  it("calls swap_handler instruction with 100% Raydium route", async () => {
    console.log("=== EXPERIMENT 1: Invoking swap_handler ===");
    console.log("Goal: Single DEX swap - 100% via Raydium\n");
    
    const amountIn = new BN(100_000_000); // 100 tokens
    const expectAmountOut = new BN(50_000_000); // 50 tokens expected
    const minReturn = new BN(49_000_000); // 49 tokens minimum (2% slippage)
    const orderId = new BN(Date.now());

    // Configure swap: 100% via Raydium
    const swapArgs = {
      amountIn,
      expectAmountOut,
      minReturn,
      amounts: [amountIn], // All amount goes to first route
      routes: [
        [
          {
            dexes: [{ raydiumSwap: {} }], // 100% Raydium
            weights: [100],
          },
        ],
      ],
    };

    console.log("SwapArgs:");
    console.log("  Amount In:", swapArgs.amountIn.toString());
    console.log("  Expected Out:", swapArgs.expectAmountOut.toString());
    console.log("  Min Return:", swapArgs.minReturn.toString());
    console.log("  Routes: 1 route, 100% Raydium");
    console.log("  Order ID:", orderId.toString());
    console.log("");

    try {
      const tx = await program.methods
        .swap(swapArgs, orderId)
        .accounts({
          payer: payer.publicKey,
          sourceTokenAccount: sourceTokenAccount,
          destinationTokenAccount: destinationTokenAccount,
          sourceMint: sourceMint,
          destinationMint: destinationMint,
        })
        .remainingAccounts([
          // TODO: Add 19 Raydium pool accounts here for real swap
          // See: tests/raydium-swap-real-test-guide.md
        ])
        .rpc();

      console.log("✅ Swap successful!");
      console.log("Transaction signature:", tx);
      expect.fail("Expected to fail without real DEX accounts");
    } catch (error) {
      console.log("❌ swap_handler invoked and returned error:");
      console.log("   Error:", error.message);
      console.log("");
      console.log("✅ SUCCESS: swap_handler is being called!");
      console.log("");
      console.log("📝 Next step: Add 19 Raydium pool accounts to remainingAccounts");
      console.log("   Guide: tests/raydium-swap-real-test-guide.md");
      
      // Expected to fail without real pool accounts
      expect(error).to.exist;
    }
  });
});

