import {
    createNft,
    findMetadataPda,
    mplTokenMetadata,
    verifyCollectionV1,
  } from "@metaplex-foundation/mpl-token-metadata";
  import {
    createGenericFile,
    generateSigner,
    keypairIdentity,
    percentAmount,
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
  import { promises as fs } from "fs";
  import * as path from "path";
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
  
  // load our plugins and signer
  umi
    .use(keypairIdentity(umiKeypair))
    .use(mplTokenMetadata())
    .use(irysUploader());

//TODO: step2: let's tell Metaplex our collection, and the NFT we want to make
// Substitute in your collection NFT address from create-metaplex-nft-collection.ts
const collectionNftAddress = UMIPublicKey("B9JRiVNAGR1b2SaRYfQZ1xCvQx2SPiHnHD3uTXn5Q8Ux");

// example data and metadata for our NFT
const nftData = {
  name: "My NFT",
  symbol: "MN",
  description: "My NFT Description",
  sellerFeeBasisPoints: 0,
  imageFile: "nft.png",
};

//TODO: step3: put out files into lrys
const imagesDir = path.join(process.cwd(), "images");
const NFTImagePath = path.join(imagesDir, "nft.png");
const buffer = await fs.readFile(NFTImagePath);
let file = createGenericFile(buffer, NFTImagePath, {
  contentType: "image/png",
});

// upload image and get image uri
const [image] = await umi.uploader.upload([file]);
console.log("image uri:", image);

// upload offchain json using irys and get metadata uri
const uri = await umi.uploader.uploadJson({
  name: "My NFT",
  symbol: "MN",
  description: "My NFT Description",
  image,
});
console.log("NFT offchain metadata URI:", uri);

//TODO: step4: create an NFT using the URL from the metadata
// generate mint keypair
const mint = generateSigner(umi);

// create and mint NFT
await createNft(umi, {
  mint,
  name: "My NFT",
  symbol: "MN",
  uri,
  updateAuthority: umi.identity.publicKey,
  sellerFeeBasisPoints: percentAmount(0),
  collection: {
    key: collectionNftAddress,
    verified: false,
  },
}).sendAndConfirm(umi, { send: { commitment: "finalized" } });

let explorerLink = getExplorerLink("address", mint.publicKey, "devnet");
console.log(`Token Mint:  ${explorerLink}`);
