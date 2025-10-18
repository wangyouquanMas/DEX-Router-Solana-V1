### TS Integration Test Added

- `tests/fun-uniswap-v3.ts` now calls `initializeAmmConfig`, verifies the resulting account fields, and asserts a second invocation fails with an “already initialized” error.  
```1:63:rust_tutorial/contract_roadmap/6.implementation/1.anchor_tutorial/fun-uniswap-v3/tests/fun-uniswap-v3.ts
it("initializes an AMM config and blocks duplicates", async () => {
    const authority = anchor.web3.Keypair.generate();
    const index = Math.floor(Math.random() * 65_000);
    // ...
    const ammConfigAccount = await program.account.ammConfig.fetch(ammConfigPda);
    expect(ammConfigAccount.authority.toBase58()).to.equal(authority.publicKey.toBase58());
    // ...
    expect(duplicateMessage.toLowerCase()).to.contain("already");
});
```

### Next steps

- Run `anchor test` (or the script from `Anchor.toml`) to execute the new integration test.
- Commit once tests pass.