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
      console.log("⚠️  Airdrop failed, using existing wallet:", error.message);
      // Fallback to using the default wallet if airdrop fails
      const walletPath = "/root/.config/solana/id.json";
      if (fs.existsSync(walletPath)) {
        payer = anchor.web3.Keypair.fromSecretKey(
          new Uint8Array(JSON.parse(fs.readFileSync(walletPath, 'utf8')))
        );
        console.log("✓ Using existing wallet:", payer.publicKey.toString());
      }
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
});
