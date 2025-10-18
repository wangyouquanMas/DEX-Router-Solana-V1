import { describe, it, expect, beforeAll } from 'vitest';
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { DexSolana } from "../target/types/dex_solana";
import { PublicKey, Connection, Keypair } from "@solana/web3.js";
import * as fs from 'fs';
import * as path from 'path';
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";

describe("DEX Router Swap Instruction Test", () => {
  let program: Program<DexSolana>;
  let connection: Connection;
  let payer: Keypair;
  
  // Token mints and accounts
  let sourceMint: PublicKey;
  let destinationMint: PublicKey;
  let sourceTokenAccount: PublicKey;
  let destinationTokenAccount: PublicKey;
  
  // Swap parameters
  let swapArgs: any;
  let orderId: BN;

  beforeAll(async () => {
    console.log("🚀 Setting up test environment...");
    
    // Step 1: Environment Setup
    // Load program and setup connection
    program = anchor.workspace.DexSolana;
    connection = new Connection("http://127.0.0.1:8899", "confirmed");
    payer = anchor.web3.Keypair.generate();

    console.log("✓ Program loaded:", program.programId.toString());
    console.log("✓ Connection established to localnet");
    console.log("✓ Payer keypair generated:", payer.publicKey.toString());

    // Fund the payer account with SOL for transaction fees
    try {
      const signature = await connection.requestAirdrop(
        payer.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      await connection.confirmTransaction(signature);
      console.log("✓ Payer funded with 2 SOL");
    } catch (error) {
      console.log("⚠️  Airdrop failed, using existing wallet balance:", error.message);
    }

    // Verify the program is deployed
    try {
      const programInfo = await connection.getAccountInfo(program.programId);
      if (programInfo) {
        console.log("✓ Program is deployed and accessible");
      } else {
        throw new Error("Program not found");
      }
    } catch (error) {
      console.log("❌ Program verification failed:", error.message);
      throw error;
    }

    console.log("✅ Environment setup complete!");
    
    // Step 2: Create Test Tokens
    console.log("🪙 Creating test tokens...");
    
    // Create source and destination token mints
    sourceMint = await createMint(connection, payer, payer.publicKey, null, 6);
    destinationMint = await createMint(connection, payer, payer.publicKey, null, 6);
    
    console.log("✓ Source mint created:", sourceMint.toString());
    console.log("✓ Destination mint created:", destinationMint.toString());
    
    // Create token accounts
    sourceTokenAccount = await createAccount(connection, payer, sourceMint, payer.publicKey);
    destinationTokenAccount = await createAccount(connection, payer, destinationMint, payer.publicKey);
    
    console.log("✓ Source token account created:", sourceTokenAccount.toString());
    console.log("✓ Destination token account created:", destinationTokenAccount.toString());
    
    // Mint tokens to source account (1,000,000 tokens with 6 decimals)
    const mintAmount = 1000000000; // 1,000,000 tokens * 10^6 decimals
    await mintTo(connection, payer, sourceMint, sourceTokenAccount, payer, mintAmount);
    
    console.log(`✓ Minted ${mintAmount / 1000000} tokens to source account`);
    
    // Verify token balances
    const sourceBalance = await getAccount(connection, sourceTokenAccount);
    const destinationBalance = await getAccount(connection, destinationTokenAccount);
    
    console.log(`✓ Source balance: ${Number(sourceBalance.amount) / 1000000} tokens`);
    console.log(`✓ Destination balance: ${Number(destinationBalance.amount) / 1000000} tokens`);
    
    console.log("✅ Token creation complete!");
    
    // Step 3: Prepare SwapArgs
    console.log("⚙️  Configuring swap parameters...");
    
    // Generate unique order ID
    orderId = new BN(Date.now());
    
    // Configure swap parameters
    const amountIn = new BN(100000000);      // 100 tokens (with 6 decimals)
    const expectAmountOut = new BN(50000000); // Expect 50 tokens out
    const minReturn = new BN(49000000);      // Min 49 tokens (2% slippage tolerance)
    
    swapArgs = {
      amountIn,
      expectAmountOut,
      minReturn,
      amounts: [amountIn],     // Single route - all tokens go through one path
      routes: [[
        {
          dexes: [{ raydiumSwap: {} }], // Use Raydium DEX
          weights: [100]                 // 100% through Raydium
        }
      ]]
    };
    
    console.log("✓ Order ID:", orderId.toString());
    console.log(`✓ Amount in: ${amountIn.toString()} (${amountIn.toNumber() / 1000000} tokens)`);
    console.log(`✓ Expected out: ${expectAmountOut.toString()} (${expectAmountOut.toNumber() / 1000000} tokens)`);
    console.log(`✓ Min return: ${minReturn.toString()} (${minReturn.toNumber() / 1000000} tokens)`);
    console.log("✓ Route: 100% through Raydium");
    console.log("✅ SwapArgs configuration complete!");
  });

  it("should have proper environment setup", async () => {
    // Verify program is loaded
    expect(program).toBeDefined();
    expect(program.programId).toBeDefined();
    
    // Verify connection is working
    const version = await connection.getVersion();
    expect(version).toBeDefined();
    
    // Verify payer has a balance
    const balance = await connection.getBalance(payer.publicKey);
    expect(balance).toBeGreaterThan(0);
    
    console.log("✅ Environment validation passed");
    console.log(`   Program ID: ${program.programId.toString()}`);
    console.log(`   Payer: ${payer.publicKey.toString()}`);
    console.log(`   Balance: ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
  });

  it("should have created test tokens properly", async () => {
    // Verify mints are created
    expect(sourceMint).toBeDefined();
    expect(destinationMint).toBeDefined();
    expect(sourceMint).not.toEqual(destinationMint);
    
    // Verify token accounts are created
    expect(sourceTokenAccount).toBeDefined();
    expect(destinationTokenAccount).toBeDefined();
    expect(sourceTokenAccount).not.toEqual(destinationTokenAccount);
    
    // Verify source account has tokens
    const sourceBalance = await getAccount(connection, sourceTokenAccount);
    expect(Number(sourceBalance.amount)).toBeGreaterThan(0);
    
    // Verify destination account is empty initially
    const destinationBalance = await getAccount(connection, destinationTokenAccount);
    expect(Number(destinationBalance.amount)).toBe(0);
    
    console.log("✅ Token creation validation passed");
    console.log(`   Source mint: ${sourceMint.toString()}`);
    console.log(`   Destination mint: ${destinationMint.toString()}`);
    console.log(`   Source balance: ${Number(sourceBalance.amount) / 1000000} tokens`);
    console.log(`   Destination balance: ${Number(destinationBalance.amount) / 1000000} tokens`);
  });

  it("should have configured SwapArgs properly", async () => {
    // Verify SwapArgs is configured
    expect(swapArgs).toBeDefined();
    expect(orderId).toBeDefined();
    
    // Verify swap parameters
    expect(swapArgs.amountIn).toBeDefined();
    expect(swapArgs.expectAmountOut).toBeDefined();
    expect(swapArgs.minReturn).toBeDefined();
    expect(swapArgs.amounts).toBeDefined();
    expect(swapArgs.routes).toBeDefined();
    
    // Verify amounts are valid
    expect(swapArgs.amountIn.toNumber()).toBeGreaterThan(0);
    expect(swapArgs.expectAmountOut.toNumber()).toBeGreaterThan(0);
    expect(swapArgs.minReturn.toNumber()).toBeGreaterThan(0);
    
    // Verify slippage tolerance (expectAmountOut >= minReturn)
    expect(swapArgs.expectAmountOut.toNumber()).toBeGreaterThanOrEqual(swapArgs.minReturn.toNumber());
    
    // Verify amounts array matches amountIn
    const totalAmounts = swapArgs.amounts.reduce((sum: number, amount: BN) => sum + amount.toNumber(), 0);
    expect(totalAmounts).toBe(swapArgs.amountIn.toNumber());
    
    // Verify routes structure
    expect(swapArgs.routes).toHaveLength(1); // Single route
    expect(swapArgs.routes[0]).toHaveLength(1); // Single hop
    expect(swapArgs.routes[0][0].dexes).toHaveLength(1); // Single DEX
    expect(swapArgs.routes[0][0].weights).toHaveLength(1); // Single weight
    expect(swapArgs.routes[0][0].weights[0]).toBe(100); // 100% through Raydium
    
    console.log("✅ SwapArgs validation passed");
    console.log(`   Order ID: ${orderId.toString()}`);
    console.log(`   Amount in: ${swapArgs.amountIn.toString()}`);
    console.log(`   Expected out: ${swapArgs.expectAmountOut.toString()}`);
    console.log(`   Min return: ${swapArgs.minReturn.toString()}`);
    console.log(`   Routes: ${swapArgs.routes.length} route(s)`);
    console.log(`   DEX: Raydium (100%)`);
  });
});
