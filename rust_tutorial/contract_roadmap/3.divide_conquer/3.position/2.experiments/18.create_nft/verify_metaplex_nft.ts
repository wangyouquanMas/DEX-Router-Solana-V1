import {
    findMetadataPda,
    mplTokenMetadata,
    verifyCollectionV1,
  } from "@metaplex-foundation/mpl-token-metadata";
  import {
    keypairIdentity,
    publicKey as UMIPublicKey,
  } from "@metaplex-foundation/umi";
  import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
  import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
  import {
    airdropIfRequired,
    getExplorerLink,
    getKeypairFromFile,
  } from "@solana-developers/helpers";
  import { clusterApiUrl, Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";

async function main() {
  // create a new connection to Solana's devnet cluster
  const connection = new Connection(clusterApiUrl("devnet"));
  
  // load keypair from local file system
  // assumes that the keypair is already generated using `solana-keygen new`
  const user = await getKeypairFromFile("~/.config/solana/id.json");
  console.log("Loaded user:", user.publicKey.toBase58());
  
  await airdropIfRequired(
    connection,
    user.publicKey,
    1 * LAMPORTS_PER_SOL,
    0.1 * LAMPORTS_PER_SOL,
  );
  
  const umi = createUmi(connection);
  
  // convert to umi compatible keypair
  const umiKeypair = umi.eddsa.createKeypairFromSecretKey(user.secretKey);
  // assigns a signer to our umi instance, and loads the MPL metadata program and Irys uploader plugins.
  umi
    .use(keypairIdentity(umiKeypair))
    .use(mplTokenMetadata())
    .use(irysUploader());
  
  // Substitute in your collection NFT address from create-metaplex-nft-collection.ts
  const collectionAddress = UMIPublicKey("B9JRiVNAGR1b2SaRYfQZ1xCvQx2SPiHnHD3uTXn5Q8Ux");
  
  // Substitute in your NFT address from create-metaplex-nft.ts
  const nftAddress = UMIPublicKey("CVc2VhGkXTJWvJFXgpC3g19YbxqqdBvDPpSn1kJ94SyV");

  // Verify our collection as a Certified Collection
  // See https://developers.metaplex.com/token-metadata/collections
  const metadata = findMetadataPda(umi, { mint: nftAddress });
  await verifyCollectionV1(umi, {
    metadata,
    collectionMint: collectionAddress,
    authority: umi.identity,
  }).sendAndConfirm(umi);

  let explorerLink = getExplorerLink("address", nftAddress, "devnet");
  console.log(`verified collection:  ${explorerLink}`);
  console.log("✅ Finished successfully!");
}

// Run the main function
main().catch(console.error);