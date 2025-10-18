import {
    createNft,
    mplTokenMetadata,
  } from "@metaplex-foundation/mpl-token-metadata";
  import {
    createGenericFile,
    generateSigner,
    keypairIdentity,
    percentAmount,
  } from "@metaplex-foundation/umi";
  import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
  import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
  import {
    airdropIfRequired,
    getExplorerLink,
    getKeypairFromFile,
  } from "@solana-developers/helpers";
  import { clusterApiUrl, Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
  import { promises as fs } from "fs";
  import * as path from "path";

async function main() {
  //TODO: STEP1: Connect to devnet, load a user and Airdrop some SOL if needed:
  // create a new connection to Solana's devnet cluster
  const connection = new Connection(clusterApiUrl("devnet"));

  // load keypair from local file system
  // assumes that the keypair is already generated using `solana-keygen new`
  const user = await getKeypairFromFile("~/.config/solana/id.json");

  await airdropIfRequired(
    connection,
    user.publicKey,
    1 * LAMPORTS_PER_SOL,
    0.1 * LAMPORTS_PER_SOL,
  );

  console.log("Loaded user:", user.publicKey.toBase58());


//TODO: step2 create a new Umi instance
    const umi = createUmi(connection);
    // convert to umi compatible keypair
    const umiKeypair = umi.eddsa.createKeypairFromSecretKey(user.secretKey);
    // assigns a signer to our umi instance, and loads the MPL metadata program and Irys uploader plugins.
    umi
    .use(keypairIdentity(umiKeypair))
    .use(mplTokenMetadata())
    .use(irysUploader());

  console.log("Step2 success")

//TODO: step3: prepare assets
// collection image: https://github.com/solana-developers/professional-education/blob/main/labs/metaplex-umi/collection.png

// NFT image: https://github.com/solana-developers/professional-education/blob/main/labs/metaplex-umi/nft.png

// We will use these images as our collection and nft cover images respectively.


//TODO: step4: lrys: storage provider; 
// We will use Irys as our storage provider, and Metaplex conveniently ships the umi-uploader-irys plugin we can use to upload our files. 
// The plugin, also takes care of storage fees so that we don't have to worry about making this on our own.

//upload the offchain metadata to lrys:

const imagesDir = path.join(process.cwd(), "images");
const collectionImagePath = path.join(imagesDir, "collection.png");
const buffer = await fs.readFile(collectionImagePath);
let file = createGenericFile(buffer, collectionImagePath, {
    contentType: "image/png",
  });
const [image] = await umi.uploader.upload([file]);
console.log("image uri:", image);

// upload offchain json to Arweave using irys
const uri = await umi.uploader.uploadJson({
    name: "My Collection",
    symbol: "MC",
    description: "My Collection description",
    image,
  });
console.log("Collection offchain metadata URI:", uri);


//TODO: step5: Then actually make the collection
// generate mint keypair
const collectionMint = generateSigner(umi);

// create and mint NFT
await createNft(umi, {
  mint: collectionMint,
  name: "My Collection",
  uri,
  updateAuthority: umi.identity.publicKey,
  sellerFeeBasisPoints: percentAmount(0),
  isCollection: true,
}).sendAndConfirm(umi, { send: { commitment: "finalized" } });

let explorerLink = getExplorerLink(
  "address",
  collectionMint.publicKey,
  "devnet",
);
console.log(`Collection NFT:  ${explorerLink}`);
console.log(`Collection NFT address is:`, collectionMint.publicKey);
console.log("✅ Finished successfully!");


//TODO: step6: Creating a Metaplex NFT inside the collection
// We'll now make a Metaplex NFT that's a member of the collection we just made. Make a new file called create-metaplex-nft.ts. The setup for this will look the same as the previous file, with slightly different imports:


}

// Run the main function
main().catch(console.error);