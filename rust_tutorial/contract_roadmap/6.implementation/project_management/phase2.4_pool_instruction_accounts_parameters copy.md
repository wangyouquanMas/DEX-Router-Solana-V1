- Added `create_pool` handler in `instructions/create_pool.rs`: builds the initial `PoolState`, initializes vaults via CPI, and seeds observation/tick bitmap accounts. Imports pending helper account types (placeholders for now), leaving TODO to define them.
```5:169:contract_roadmap/6.implementation/1.anchor_tutorial/fun-uniswap-v3/programs/fun-uniswap-v3/src/instructions/create_pool.rs
pub fn create_pool(
    ctx: Context<CreatePool>,
    sqrt_price_x64: u128,
    tick_current: i32,
) -> Result<()> {
    let accounts = ctx.accounts;

    require!(
        accounts.token_mint_0.key() < accounts.token_mint_1.key(),
        ErrorCode::InvalidMintOrder
    );
    require!(
        state::tick_spacing_matches(&accounts.amm_config, accounts.amm_config.tick_spacing),
        ErrorCode::TickSpacingMismatch
    );

    let pool_bump = *ctx
        .bumps
        .get("pool_state")
        .ok_or(ErrorCode::MissingBump)?;

    let mut pool_state = accounts.pool_state.clone();
    pool_state.bump = pool_bump;
    ...
    accounts.pool_state.set_inner(pool_state);

    token::initialize_account3(...)
    token::initialize_account3(...)

    accounts.observation_state.initialize(accounts.pool_state.key())?;
    accounts
        .tick_array_bitmap
        .initialize(accounts.pool_state.key());

    Ok(())
}
```

Next steps: define `ObservationAccount` and `TickArrayBitmapAccount` structures in `state`, add new error variants, and ensure CPI initializations match whether mints use Token-2022. Run `cargo fmt`/tests once supporting modules are ready.