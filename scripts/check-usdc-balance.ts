import { Connection, PublicKey } from "@solana/web3.js";
import { getAccount, getAssociatedTokenAddress } from "@solana/spl-token";

/**
 * Check USDC balance for a wallet on localnet
 * Usage: npx ts-node scripts/check-usdc-balance.ts <WALLET_ADDRESS>
 */

async function checkUSDCBalance(walletAddress: string) {
  const connection = new Connection("http://127.0.0.1:8899", "confirmed");
  const USDC_MINT = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
  
  try {
    const walletPubkey = new PublicKey(walletAddress);
    
    console.log("🔍 Checking USDC balance...");
    console.log(`   Wallet: ${walletAddress}`);
    console.log(`   USDC Mint: ${USDC_MINT.toString()}`);
    console.log("");
    
    // Get the associated token account address
    const tokenAccountAddress = await getAssociatedTokenAddress(
      USDC_MINT,
      walletPubkey
    );
    
    console.log(`   Token Account: ${tokenAccountAddress.toString()}`);
    
    // Try to get the account
    try {
      const tokenAccount = await getAccount(connection, tokenAccountAddress);
      const balance = Number(tokenAccount.amount) / 1_000_000; // USDC has 6 decimals
      
      console.log("");
      console.log("✅ USDC Balance:");
      console.log(`   ${balance.toLocaleString()} USDC`);
      console.log(`   (${tokenAccount.amount.toString()} raw)`);
      
    } catch (accountError) {
      console.log("");
      console.log("⚠️  USDC token account not found for this wallet");
      console.log("   The wallet may not have created a USDC account yet");
    }
    
  } catch (error) {
    console.error("❌ Error:", error instanceof Error ? error.message : String(error));
  }
}

// Get wallet address from command line argument
const walletAddress = process.argv[2];

if (!walletAddress) {
  console.log("Usage: npx ts-node scripts/check-usdc-balance.ts <WALLET_ADDRESS>");
  console.log("");
  console.log("Example:");
  console.log("  npx ts-node scripts/check-usdc-balance.ts 9icRBQWH6CMWtRtNetueciSH2pknVgVZrTptrRCofG7D");
  process.exit(1);
}

checkUSDCBalance(walletAddress);


