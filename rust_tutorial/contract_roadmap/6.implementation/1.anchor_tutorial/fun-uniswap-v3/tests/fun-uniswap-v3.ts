import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { FunUniswapV3 } from "../target/types/fun_uniswap_v3";

describe("fun-uniswap-v3", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.funUniswapV3 as Program<FunUniswapV3>;

  it("initializes an AMM config and blocks duplicates", async () => {
    const authority = anchor.web3.Keypair.generate();
    const index = Math.floor(Math.random() * 65_000);

    const indexBuffer = Buffer.alloc(2);
    indexBuffer.writeUInt16BE(index);
    const [ammConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("amm-config"), indexBuffer],
      program.programId,
    );

    const tickSpacing = 1;
    const tradeFeeRate = 3_000;
    const protocolFeeRate = 500;
    const fundFeeRate = 100;

    const txSig = await program.methods
      .initializeAmmConfig(index, tickSpacing, tradeFeeRate, protocolFeeRate, fundFeeRate)
      .accounts({
        payer: provider.wallet.publicKey,
        authority: authority.publicKey,
        ammConfig: ammConfigPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    console.log("initializeAmmConfig tx", txSig);

    const ammConfigAccount = await program.account.ammConfig.fetch(ammConfigPda);

    //output logs 
    console.log("ammConfigAccount.bump", ammConfigAccount.bump);
    console.log("ammConfigAccount.authority", ammConfigAccount.authority.toBase58());
    console.log("ammConfigAccount.tickSpacing", ammConfigAccount.tickSpacing);
    console.log("ammConfigAccount.tradeFeeRate", ammConfigAccount.tradeFeeRate);
    console.log("ammConfigAccount.protocolFeeRate", ammConfigAccount.protocolFeeRate);
    console.log("ammConfigAccount.fundFeeRate", ammConfigAccount.fundFeeRate);

    expect(ammConfigAccount.bump).to.be.a("number");
    expect(ammConfigAccount.authority.toBase58()).to.equal(authority.publicKey.toBase58());
    expect(ammConfigAccount.tickSpacing).to.equal(tickSpacing);
    expect(ammConfigAccount.tradeFeeRate).to.equal(tradeFeeRate);
    expect(ammConfigAccount.protocolFeeRate).to.equal(protocolFeeRate);
    expect(ammConfigAccount.fundFeeRate).to.equal(fundFeeRate);

    let duplicateError: unknown;
    try {
      await program.methods
        .initializeAmmConfig(index, tickSpacing, tradeFeeRate, protocolFeeRate, fundFeeRate)
        .accounts({
          payer: provider.wallet.publicKey,
          authority: authority.publicKey,
          ammConfig: ammConfigPda,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc();
    } catch (err) {
      duplicateError = err;
    }

    expect(duplicateError, "expected duplicate initialization to fail").to.exist;

    const duplicateMessage =
      (duplicateError as any)?.error?.errorMessage ??
      (duplicateError as any)?.toString?.() ??
      "";
    expect(duplicateMessage.toLowerCase()).to.contain("already");
  });
});
