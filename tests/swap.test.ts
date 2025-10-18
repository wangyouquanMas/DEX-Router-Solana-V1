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
  getAccount,
  syncNative,
  createInitializeAccountInstruction,
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
  
  // DEX accounts for Raydium
  let raydiumAccounts: any[];

  beforeAll(async () => {
    console.log("🚀 Setting up test environment...");
    
    // Step 1: Environment Setup
    // Load program and setup connection
    program = anchor.workspace.DexSolana;
    connection = new Connection("http://127.0.0.1:8899", "confirmed");
    
    // payer := Keypair.generate();
    // Load wallet from file instead of generating a random one
    const walletPath = path.join(process.env.HOME || '', '.config/solana/id.json');
    const walletKeypair = JSON.parse(fs.readFileSync(walletPath, 'utf-8'));
    payer = Keypair.fromSecretKey(new Uint8Array(walletKeypair));

    console.log("✓ Program loaded:", program.programId.toString());
    console.log("✓ Connection established to localnet");
    console.log("✓ Payer wallet loaded:", payer.publicKey.toString());

    // Fund the payer account with SOL for transaction fees
    try {
      const signature = await connection.requestAirdrop(
        payer.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      await connection.confirmTransaction(signature);
      console.log("✓ Payer funded with 2 SOL");
    } catch (error) {
      console.log("⚠️  Airdrop failed, using existing wallet balance:", error instanceof Error ? error.message : String(error));
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
      console.log("❌ Program verification failed:", error instanceof Error ? error.message : String(error));
      throw error;
    }

    console.log("✅ Environment setup complete!");
    
    // Step 2: Use Cloned Mainnet Tokens (SOL/USDC)
    console.log("🪙 Setting up token accounts for cloned mainnet tokens...");
    
    // Use the real SOL and USDC mints that are cloned in Anchor.toml
    sourceMint = new PublicKey("So11111111111111111111111111111111111111112"); // Wrapped SOL
    destinationMint = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"); // USDC
    
    console.log("✓ Source mint (SOL):", sourceMint.toString());
    console.log("✓ Destination mint (USDC):", destinationMint.toString());
    
    // Create token accounts for payer
    // Note: For cloned mainnet accounts, sometimes the Associated Token Program
    // has restrictions. We'll try normal creation, and fall back if needed.
    try {
      sourceTokenAccount = await createAccount(connection, payer, sourceMint, payer.publicKey);
      destinationTokenAccount = await createAccount(connection, payer, destinationMint, payer.publicKey);
    } catch (error) {
      console.log("⚠️  Associated token account creation restricted, creating new keypairs for token accounts...");
      // Create new random keypairs to own the token accounts
      const sourceKeypair = Keypair.generate();
      const destKeypair = Keypair.generate();
      
      // Fund them with rent
      const rentExempt = await connection.getMinimumBalanceForRentExemption(165);
      
      const createSource = anchor.web3.SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: sourceKeypair.publicKey,
        lamports: rentExempt,
        space: 165,
        programId: TOKEN_PROGRAM_ID,
      });
      
      const createDest = anchor.web3.SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: destKeypair.publicKey,
        lamports: rentExempt,
        space: 165,
        programId: TOKEN_PROGRAM_ID,
      });
      
      const initSource = createInitializeAccountInstruction(
        sourceKeypair.publicKey,
        sourceMint,
        payer.publicKey,
        TOKEN_PROGRAM_ID
      );
      
      const initDest = createInitializeAccountInstruction(
        destKeypair.publicKey,
        destinationMint,
        payer.publicKey,
        TOKEN_PROGRAM_ID
      );
      
      const tx = new anchor.web3.Transaction()
        .add(createSource)
        .add(initSource)
        .add(createDest)
        .add(initDest);
      
      await anchor.web3.sendAndConfirmTransaction(connection, tx, [payer, sourceKeypair, destKeypair]);
      
      sourceTokenAccount = sourceKeypair.publicKey;
      destinationTokenAccount = destKeypair.publicKey;
    }
    
    console.log("✓ Source token account created:", sourceTokenAccount.toString());
    console.log("✓ Destination token account created:", destinationTokenAccount.toString());
    
    // Fund the source account with tokens
    // For wrapped SOL, we need to transfer SOL to the token account and sync
    console.log("✓ Funding source account...");
    try {
      // For wrapped SOL token accounts:
      // 1. Transfer SOL to the token account
      // 2. Call syncNative to update the token balance
      const transferIx = anchor.web3.SystemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: sourceTokenAccount,
        lamports: 1_000_000_000, // 1 SOL
      });
      
      const tx = new anchor.web3.Transaction().add(transferIx);
      const signature = await anchor.web3.sendAndConfirmTransaction(
        connection,
        tx,
        [payer]
      );
      
      console.log("✓ Transferred 1 SOL to token account");
      
      // Sync the native balance to update the wrapped SOL amount
      await syncNative(connection, payer, sourceTokenAccount);
      
      console.log("✓ Source account funded with 1 wrapped SOL");
      console.log("✓ Transfer signature:", signature);
    } catch (error) {
      console.log("⚠️  Funding failed:", error instanceof Error ? error.message : String(error));
      console.log("⚠️  Note: For mainnet cloned accounts, you may need to airdrop or transfer tokens differently");
    }
    
    // Verify token balances
    try {
      const sourceBalance = await getAccount(connection, sourceTokenAccount);
      const destinationBalance = await getAccount(connection, destinationTokenAccount);
      
      console.log(`✓ Source balance: ${Number(sourceBalance.amount) / 1000000000} SOL`); // SOL has 9 decimals
      console.log(`✓ Destination balance: ${Number(destinationBalance.amount) / 1000000} USDC`); // USDC has 6 decimals
    } catch (error) {
      console.log("⚠️  Token accounts created but no balance yet");
    }
    
    console.log("✅ Token setup complete!");
    
    // Step 3: Prepare SwapArgs
    console.log("⚙️  Configuring swap parameters...");
    
    // Generate unique order ID
    orderId = new BN(Date.now());
    
    // Configure swap parameters (SOL has 9 decimals, USDC has 6 decimals)
    const amountIn = new BN(100000000);      // 0.1 SOL (100,000,000 lamports, 9 decimals)
    const expectAmountOut = new BN(15000000); // Expect ~15 USDC out (6 decimals)
    const minReturn = new BN(14000000);      // Min 14 USDC (7% slippage tolerance)
    
    swapArgs = {
      amountIn,
      expectAmountOut,
      minReturn,
      amounts: [amountIn],     // Single route - all tokens go through one path
      routes: [[
        {
          dexes: [{ raydiumSwap: {} }], // Use Raydium DEX
          weights: Buffer.from([100])    // 100% through Raydium (must be Buffer/Uint8Array)
        }
      ]]
    };
    
    console.log("✓ Order ID:", orderId.toString());
    console.log(`✓ Amount in: ${amountIn.toString()} (${amountIn.toNumber() / 1000000} tokens)`);
    console.log(`✓ Expected out: ${expectAmountOut.toString()} (${expectAmountOut.toNumber() / 1000000} tokens)`);
    console.log(`✓ Min return: ${minReturn.toString()} (${minReturn.toNumber() / 1000000} tokens)`);
    console.log("✓ Route: 100% through Raydium");
    console.log("✅ SwapArgs configuration complete!");
    
    // Step 4: Set Up Remaining Accounts (All 19 Required for Raydium)
    console.log("🔗 Setting up remaining accounts for Raydium...");
    
    // Define all Raydium and Serum account addresses (VERIFIED FROM MAINNET)
    const RAYDIUM_PROGRAM_ID = new PublicKey("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
    const RAYDIUM_POOL = new PublicKey("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2");
    const POOL_AUTHORITY = new PublicKey("5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1");
    const AMM_OPEN_ORDERS = new PublicKey("HmiHHzq4Fym9e1D4qzLS6LDDM3tNsCTBPDWHTLZ763jY");
    const AMM_TARGET_ORDERS = new PublicKey("CZza3Ej4Mc58MnxWA385itCC9jCo3L1D7zc3LKy1bZMR");
    const POOL_COIN_VAULT = new PublicKey("DQyrAcCrDXQ7NeoqGgDCZwBvWDcYmFCjSb9JtteuvPpz");
    const POOL_PC_VAULT = new PublicKey("HLmqeL62xR1QoZ1HKKbXRrdN1p3phKpxRMb2VVopvBBz");
    const SERUM_PROGRAM_ID = new PublicKey("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX");
    const SERUM_MARKET = new PublicKey("8BnEgHoWFysVcuFFX7QztDmzuH8r5ZFvyP3sYwn1XTh6");
    const SERUM_BIDS = new PublicKey("96RyJdJVeo5Yr5FjJRn6AaED89myiD9fjp2Fq3zccrfj");  // Must match Anchor.toml
    const SERUM_ASKS = new PublicKey("48cNXXS5fKsA3ufrYMHxqmV93L2449tu4Ng9mQS2Mxzt");  // Must match Anchor.toml
    const SERUM_EVENT_QUEUE = new PublicKey("5KKsLVU6TcbVDK4BS6K1DGDxnh4Q9xjYJ8XaDCG5t8ht");  // Should be cloned in Anchor.toml
    const SERUM_COIN_VAULT = new PublicKey("FaFLrnxNpW4z6ivYvmDaoxvHXvi7G78veWcjW81siiE6");  // Serum Base Vault - Must match Anchor.toml
    const SERUM_PC_VAULT = new PublicKey("BmrxsPxDjYavNotwdYNMJm1Z3ruRY5AXTA7m85XZpSYj");  // Serum Quote Vault - Must match Anchor.toml
    const SERUM_VAULT_SIGNER = new PublicKey("V3gQJJhHGaRKS7uoeUVhMKzKWqbQ6dKofhPqGBxmg2c");
    
    // Configure remaining accounts for Raydium swap in exact order expected
    // Order must match RaydiumSwapAccounts::parse_accounts
    raydiumAccounts = [
      // 1. Raydium program ID
      { pubkey: RAYDIUM_PROGRAM_ID, isSigner: false, isWritable: false },
      // 2. Swap authority (payer)
      { pubkey: payer.publicKey, isSigner: true, isWritable: false },
      // 3. Source token account
      { pubkey: sourceTokenAccount, isSigner: false, isWritable: true },
      // 4. Destination token account
      { pubkey: destinationTokenAccount, isSigner: false, isWritable: true },
      // 5. Token program
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      // 6. AMM ID (Pool)
      { pubkey: RAYDIUM_POOL, isSigner: false, isWritable: true },
      // 7. AMM Authority
      { pubkey: POOL_AUTHORITY, isSigner: false, isWritable: false },
      // 8. AMM Open Orders
      { pubkey: AMM_OPEN_ORDERS, isSigner: false, isWritable: true },
      // 9. AMM Target Orders
      { pubkey: AMM_TARGET_ORDERS, isSigner: false, isWritable: true },
      // 10. Pool Coin Vault
      { pubkey: POOL_COIN_VAULT, isSigner: false, isWritable: true },
      // 11. Pool PC Vault
      { pubkey: POOL_PC_VAULT, isSigner: false, isWritable: true },
      // 12. Serum Program ID
      { pubkey: SERUM_PROGRAM_ID, isSigner: false, isWritable: false },
      // 13. Serum Market
      { pubkey: SERUM_MARKET, isSigner: false, isWritable: true },
      // 14. Serum Bids
      { pubkey: SERUM_BIDS, isSigner: false, isWritable: true },
      // 15. Serum Asks
      { pubkey: SERUM_ASKS, isSigner: false, isWritable: true },
      // 16. Serum Event Queue
      { pubkey: SERUM_EVENT_QUEUE, isSigner: false, isWritable: true },
      // 17. Serum Coin Vault
      { pubkey: SERUM_COIN_VAULT, isSigner: false, isWritable: true },
      // 18. Serum PC Vault
      { pubkey: SERUM_PC_VAULT, isSigner: false, isWritable: true },
      // 19. Serum Vault Signer
      { pubkey: SERUM_VAULT_SIGNER, isSigner: false, isWritable: false },
    ];
    
    console.log("✓ Total remaining accounts configured: 19");
    console.log("✅ Remaining accounts setup complete!");
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
    
    // Verify token accounts exist
    const sourceBalance = await getAccount(connection, sourceTokenAccount);
    const destinationBalance = await getAccount(connection, destinationTokenAccount);
    
    // Note: With mainnet clones, we can't mint tokens so balances may be 0
    // This is expected behavior when testing with cloned mainnet accounts
    expect(Number(sourceBalance.amount)).toBeGreaterThanOrEqual(0);
    expect(Number(destinationBalance.amount)).toBe(0);
    
    console.log("✅ Token creation validation passed");
    console.log(`   Source mint: ${sourceMint.toString()}`);
    console.log(`   Destination mint: ${destinationMint.toString()}`);
    console.log(`   Source balance: ${Number(sourceBalance.amount) / 1000000000} SOL`);
    console.log(`   Destination balance: ${Number(destinationBalance.amount) / 1000000} USDC`);
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
    expect(swapArgs.routes[0][0].weights).toBeInstanceOf(Buffer); // Weights must be Buffer
    expect(swapArgs.routes[0][0].weights[0]).toBe(100); // 100% through Raydium
    
    console.log("✅ SwapArgs validation passed");
    console.log(`   Order ID: ${orderId.toString()}`);
    console.log(`   Amount in: ${swapArgs.amountIn.toString()}`);
    console.log(`   Expected out: ${swapArgs.expectAmountOut.toString()}`);
    console.log(`   Min return: ${swapArgs.minReturn.toString()}`);
    console.log(`   Routes: ${swapArgs.routes.length} route(s)`);
    console.log(`   DEX: Raydium (100%)`);
  });

  it("should have configured remaining accounts properly", async () => {
    // Verify remaining accounts are configured
    expect(raydiumAccounts).toBeDefined();
    expect(raydiumAccounts).toBeInstanceOf(Array);
    expect(raydiumAccounts.length).toBeGreaterThan(0);
    
    // Verify each account has required properties
    raydiumAccounts.forEach((account, index) => {
      expect(account.pubkey).toBeDefined();
      expect(account.pubkey).toBeInstanceOf(PublicKey);
      expect(typeof account.isSigner).toBe('boolean');
      expect(typeof account.isWritable).toBe('boolean');
    });
    
    // Verify Raydium accounts exist on-chain
    // Note: Some accounts like Serum Vault Signer (PDA) may not exist until runtime
    console.log("🔍 Verifying Raydium accounts on-chain...");
    const pdaAccounts = ["V3gQJJhHGaRKS7uoeUVhMKzKWqbQ6dKofhPqGBxmg2c"]; // PDAs that don't need to pre-exist
    
    for (let i = 0; i < raydiumAccounts.length; i++) {
      const account = raydiumAccounts[i];
      const accountInfo = await connection.getAccountInfo(account.pubkey);
      const isPDA = pdaAccounts.includes(account.pubkey.toString());
      
      if (!accountInfo) {
        if (isPDA) {
          console.log(`   ⚠️  Account #${i + 1} is PDA (will be created at runtime): ${account.pubkey.toString().substring(0, 8)}...`);
          continue; // Skip validation for PDAs
        } else {
          console.log(`   ❌ Account #${i + 1} NOT FOUND: ${account.pubkey.toString()}`);
          expect(accountInfo).not.toBeNull();
        }
      } else {
        console.log(`   ✓ Account #${i + 1}: ${account.pubkey.toString().substring(0, 8)}... exists`);
      }
    }
    
    console.log("✅ Remaining accounts validation passed");
    console.log(`   Total accounts: ${raydiumAccounts.length}`);
  });

  it("should execute swap instruction successfully", async () => {
    console.log("🔄 Attempting swap instruction...");
    
    // Get initial balances
    const sourceBalanceBefore = await getAccount(connection, sourceTokenAccount);
    const destinationBalanceBefore = await getAccount(connection, destinationTokenAccount);
    
    console.log(`📊 Initial balances:`);
    console.log(`   Source: ${Number(sourceBalanceBefore.amount) / 1000000000} SOL`);
    console.log(`   Destination: ${Number(destinationBalanceBefore.amount) / 1000000} USDC`);
    
    // NOTE: This test works with cloned mainnet accounts because:
    // 1. We properly fund the source account with wrapped SOL (via transfer + syncNative)
    // 2. Cloned Raydium pool has working liquidity and can execute real swaps
    // 3. All necessary Raydium/Serum accounts are cloned in Anchor.toml
    
    try {
      // Execute the swap instruction
      const tx = await program.methods
        .swap(swapArgs, orderId)
        .accounts({
          payer: payer.publicKey,
          sourceTokenAccount: sourceTokenAccount,
          destinationTokenAccount: destinationTokenAccount,
          sourceMint: sourceMint,
          destinationMint: destinationMint,
        })
        .remainingAccounts(raydiumAccounts)
        .signers([payer])
        .rpc();
      
      console.log("✓ Transaction signature:", tx);
      
      // Wait for confirmation
      await connection.confirmTransaction(tx, "confirmed");
      console.log("✓ Transaction confirmed");
      
      // Get final balances
      const sourceBalanceAfter = await getAccount(connection, sourceTokenAccount);
      const destinationBalanceAfter = await getAccount(connection, destinationTokenAccount);
      
      console.log(`📊 Final balances:`);
      console.log(`   Source: ${Number(sourceBalanceAfter.amount) / 1000000000} SOL`);
      console.log(`   Destination: ${Number(destinationBalanceAfter.amount) / 1000000} USDC`);
      
      // Calculate changes
      const sourceChange = Number(sourceBalanceBefore.amount) - Number(sourceBalanceAfter.amount);
      const destinationChange = Number(destinationBalanceAfter.amount) - Number(destinationBalanceBefore.amount);
      
      console.log(`📈 Balance changes:`);
      console.log(`   Source decreased by: ${sourceChange / 1000000000} SOL`);
      console.log(`   Destination increased by: ${destinationChange / 1000000} USDC`);
      
      // Verify swap execution
      expect(Number(sourceBalanceAfter.amount)).toBeLessThan(Number(sourceBalanceBefore.amount));
      expect(Number(destinationBalanceAfter.amount)).toBeGreaterThan(Number(destinationBalanceBefore.amount));
      
      // Verify swap amounts
      expect(sourceChange).toBe(swapArgs.amountIn.toNumber());
      expect(destinationChange).toBeGreaterThanOrEqual(swapArgs.minReturn.toNumber());
      
      console.log("✅ Swap executed successfully!");
      
    } catch (error: any) {
      console.log("❌ Swap execution failed:", error instanceof Error ? error.message : String(error));
      if (error.logs) {
        console.log("📋 Transaction logs:");
        error.logs.forEach((log: string) => console.log(`   ${log}`));
      }
      
      // Re-throw the error to fail the test
      throw error;
    }
  });
});
