### Token Vault Utilities – Completed

I ported the Raydium-style vault helper into `utils/token.rs`, covering the full lifecycle:

- **Account allocation:** uses a new `utils/system::create_or_allocate_account` helper to create PDAs with the correct space/rent (pulled into `utils/mod.rs`).
- **Immutable owner:** detects Token-2022 “superstate” mints and initializes the immutable-owner extension when needed.
- **Supported mint checks:** added `support_mint_associated_is_initialized` and `is_supported_mint`, mirroring whitelist/associated-account logic; stubs currently use empty whitelist arrays but the structure matches Raydium’s approach.
- **Reusable helper:** `create_token_vault_account` now drives token-2022 account creation, extension init, and final `InitializeAccount3`.

```1:76:contract_roadmap/6.implementation/1.anchor_tutorial/fun-uniswap-v3/programs/fun-uniswap-v3/src/utils/token.rs
pub fn create_token_vault_account<'info>(
    payer: &Signer<'info>,
    pool_state: &AccountInfo<'info>,
    token_account: &AccountInfo<'info>,
    token_mint: &InterfaceAccount<'info, Mint>,
    system_program: &Program<'info, System>,
    token_program: &Interface<'info, TokenInterface>,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let immutable_owner_required = is_superstate_token(token_mint);
    let space = get_account_data_size(
        CpiContext::new(
            token_program.to_account_info(),
            GetAccountDataSize { mint: token_mint.to_account_info() },
        ),
        if immutable_owner_required { &[ExtensionType::ImmutableOwner] } else { &[] },
    )?;

    create_or_allocate_account(
        token_program.key,
        payer.to_account_info(),
        system_program.to_account_info(),
        token_account.clone(),
        signer_seeds,
        space.try_into().unwrap(),
    )?;

    if immutable_owner_required {
        token_2022::initialize_immutable_owner(...)
    }
    token_interface::initialize_account3(...)
}
```

Next steps: wire these utilities into `create_pool` (ensuring we pass the remaining accounts for mint-whitelist checks) and continue porting the bitmap/tick logic that relies on them.