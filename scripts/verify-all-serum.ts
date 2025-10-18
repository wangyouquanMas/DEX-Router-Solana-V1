import { Connection, PublicKey } from "@solana/web3.js";

async function verifyAll() {
  const connection = new Connection("https://api.mainnet-beta.solana.com");
  
  const accounts = {
    "Serum Market": "8BnEgHoWFysVcuFFX7QztDmzuH8r5ZFvyP3sYwn1XTh6",
    "Serum Bids": "HjhDkGuABhSzqmQ4KAdfEFYCgwAW8nLzqC4i2rNVPyQ2",
    "Serum Asks": "4sKRiR2gvBRJ6fWe6CXJuhf96uDddCmTmB6wCP29BWAs",
    "Serum Base Vault": "58g347gyj2GpFga4m3Fbta2fMw3749j8Dn8uJ4N4AX9W",
    "Serum Quote Vault": "6A5NHCj1yF6urc9wZNe6Bcjj4LVszQNj5DwAWG97yzMu",
  };
  
  console.log("Verifying Serum accounts...\n");
  
  for (const [name, address] of Object.entries(accounts)) {
    try {
      const info = await connection.getAccountInfo(new PublicKey(address));
      console.log(`${name}: ${info ? "✅ EXISTS" : "❌ NOT FOUND"} (${address})`);
      if (info) {
        console.log(`   Owner: ${info.owner.toString()}`);
        console.log(`   Size: ${info.data.length} bytes\n`);
      }
    } catch (error) {
      console.log(`${name}: ❌ ERROR - ${error}\n`);
    }
  }
  
  // Let's also check if the market ID is even correct
  console.log("\nLet's verify the pool's market ID from the pool account:");
  const poolId = new PublicKey("58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2");
  const poolInfo = await connection.getAccountInfo(poolId);
  
  if (poolInfo) {
    // Parse market ID from pool (offset 360 based on Raydium layout)
    const marketId = new PublicKey(poolInfo.data.slice(360, 392));
    console.log("Market ID from pool:", marketId.toString());
    
    const marketInfo = await connection.getAccountInfo(marketId);
    console.log("Market exists:", marketInfo ? "YES" : "NO");
  }
}

verifyAll();

