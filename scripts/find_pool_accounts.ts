/**
 * Script to find all Raydium pool accounts for a given pool address
 * Run with: ts-node scripts/find_pool_accounts.ts
 */

import { Connection, PublicKey } from '@solana/web3.js';
import * as borsh from 'borsh';

const connection = new Connection('https://api.mainnet-beta.solana.com');

// Raydium AMM V4 layout (simplified)
class AmmInfo {
  status!: bigint;
  nonce!: bigint;
  orderNum!: bigint;
  depth!: bigint;
  coinDecimals!: bigint;
  pcDecimals!: bigint;
  state!: bigint;
  resetFlag!: bigint;
  minSize!: bigint;
  volMaxCutRatio!: bigint;
  amountWaveRatio!: bigint;
  coinLotSize!: bigint;
  pcLotSize!: bigint;
  minPriceMultiplier!: bigint;
  maxPriceMultiplier!: bigint;
  systemDecimalsValue!: bigint;
  minSeparateNumerator!: bigint;
  minSeparateDenominator!: bigint;
  tradeFeeNumerator!: bigint;
  tradeFeeDenominator!: bigint;
  pnlNumerator!: bigint;
  pnlDenominator!: bigint;
  swapFeeNumerator!: bigint;
  swapFeeDenominator!: bigint;
  needTakePnlCoin!: bigint;
  needTakePnlPc!: bigint;
  totalPnlPc!: bigint;
  totalPnlCoin!: bigint;
  poolOpenTime!: bigint;
  punishPcAmount!: bigint;
  punishCoinAmount!: bigint;
  orderbookToInitTime!: bigint;
  swapCoinInAmount!: Buffer;
  swapPcOutAmount!: Buffer;
  swapCoin2PcFee!: bigint;
  swapPcInAmount!: Buffer;
  swapCoinOutAmount!: Buffer;
  swapPc2CoinFee!: bigint;
  poolCoinTokenAccount!: Uint8Array;
  poolPcTokenAccount!: Uint8Array;
  coinMintAddress!: Uint8Array;
  pcMintAddress!: Uint8Array;
  lpMintAddress!: Uint8Array;
  ammOpenOrders!: Uint8Array;
  serumMarket!: Uint8Array;
  serumProgramId!: Uint8Array;
  ammTargetOrders!: Uint8Array;
  poolWithdrawQueue!: Uint8Array;
  poolTempLpTokenAccount!: Uint8Array;
  ammOwner!: Uint8Array;
  pnlOwner!: Uint8Array;
}

async function fetchPoolAccounts(poolAddress: string) {
  try {
    console.log(`\n🔍 Fetching pool info for: ${poolAddress}\n`);
    
    const poolPubkey = new PublicKey(poolAddress);
    const accountInfo = await connection.getAccountInfo(poolPubkey);
    
    if (!accountInfo) {
      console.error('❌ Pool not found!');
      return;
    }
    
    console.log(`✓ Pool found, owned by: ${accountInfo.owner.toBase58()}`);
    console.log(`✓ Data length: ${accountInfo.data.length} bytes\n`);
    
    // Parse pool data (Raydium AMM V4 structure)
    const data = accountInfo.data;
    
    // Offsets for key accounts (based on Raydium AMM V4 layout)
    const ammTargetOrders = new PublicKey(data.slice(0x150, 0x170));
    const poolCoinTokenAccount = new PublicKey(data.slice(0x0A8, 0x0C8));
    const poolPcTokenAccount = new PublicKey(data.slice(0x0C8, 0x0E8));
    const ammOpenOrders = new PublicKey(data.slice(0x120, 0x140));
    const serumMarket = new PublicKey(data.slice(0x140, 0x160));
    const serumProgramId = new PublicKey(data.slice(0x160, 0x180));
    
    console.log('📋 Pool Associated Accounts:\n');
    console.log(`AMM Open Orders: ${ammOpenOrders.toBase58()}`);
    console.log(`AMM Target Orders: ${ammTargetOrders.toBase58()}`);
    console.log(`Pool Coin Vault: ${poolCoinTokenAccount.toBase58()}`);
    console.log(`Pool PC Vault: ${poolPcTokenAccount.toBase58()}`);
    console.log(`Serum Market: ${serumMarket.toBase58()}`);
    console.log(`Serum Program: ${serumProgramId.toBase58()}\n`);
    
    // Fetch Serum market accounts
    console.log('🔍 Fetching Serum market accounts...\n');
    const marketInfo = await connection.getAccountInfo(serumMarket);
    if (marketInfo) {
      const marketData = marketInfo.data;
      
      // Serum market layout offsets (OpenBook V1)
      const bids = new PublicKey(marketData.slice(0x65, 0x85));
      const asks = new PublicKey(marketData.slice(0x85, 0xA5));
      const eventQueue = new PublicKey(marketData.slice(0xA5, 0xC5));
      const coinVault = new PublicKey(marketData.slice(0xC5, 0xE5));
      const pcVault = new PublicKey(marketData.slice(0xE5, 0x105));
      const vaultSigner = new PublicKey(marketData.slice(0x125, 0x145));
      
      console.log(`Serum Bids: ${bids.toBase58()}`);
      console.log(`Serum Asks: ${asks.toBase58()}`);
      console.log(`Serum Event Queue: ${eventQueue.toBase58()}`);
      console.log(`Serum Coin Vault: ${coinVault.toBase58()}`);
      console.log(`Serum PC Vault: ${pcVault.toBase58()}`);
      console.log(`Serum Vault Signer: ${vaultSigner.toBase58()}\n`);
      
      // Print Anchor.toml format
      console.log('📝 Add these to Anchor.toml:\n');
      console.log(`[[test.validator.clone]]`);
      console.log(`address = "${poolAddress}"  # Pool`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${ammOpenOrders.toBase58()}"  # AMM Open Orders`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${ammTargetOrders.toBase58()}"  # AMM Target Orders`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${poolCoinTokenAccount.toBase58()}"  # Pool Coin Vault`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${poolPcTokenAccount.toBase58()}"  # Pool PC Vault`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${serumMarket.toBase58()}"  # Serum Market`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${bids.toBase58()}"  # Serum Bids`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${asks.toBase58()}"  # Serum Asks`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${eventQueue.toBase58()}"  # Serum Event Queue`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${coinVault.toBase58()}"  # Serum Coin Vault`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${pcVault.toBase58()}"  # Serum PC Vault`);
      console.log(`\n[[test.validator.clone]]`);
      console.log(`address = "${vaultSigner.toBase58()}"  # Serum Vault Signer`);
    }
    
  } catch (error) {
    console.error('❌ Error:', error);
  }
}

// Main
const poolAddress = process.argv[2] || '7XawhbbxtsRcQA8KTkHT9f9nc6d69UwqCDh6U5EEbEmX';
fetchPoolAccounts(poolAddress);

