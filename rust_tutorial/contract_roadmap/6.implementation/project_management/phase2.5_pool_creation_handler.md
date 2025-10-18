- Added `create_pool.rs` under `instructions/`, defining the `CreatePool` account context with all required PDAs, seed constraints, and program/sysvar dependencies, reusing helpers from `state`.
```1:78:contract_roadmap/6.implementation/1.anchor_tutorial/fun-uniswap-v3/programs/fun-uniswap-v3/src/instructions/create_pool.rs
#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub amm_config: Account<'info, AmmConfig>,
    #[account(
        init,
        payer = payer,
        space = PoolState::space(),
        seeds = pool_state_seeds(&amm_config.key(), &token_mint_0.key(), &token_mint_1.key()),
        bump,
    )]
    pub pool_state: Account<'info, PoolState>,
    #[account(mint::token_program = token_program_0)]
    pub token_mint_0: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program_1)]
    pub token_mint_1: InterfaceAccount<'info, Mint>,
    #[account(mut, seeds = pool_vault_seeds(&pool_state.key(), &token_mint_0.key()), bump)]
    pub token_vault_0: UncheckedAccount<'info>,
    #[account(mut, seeds = pool_vault_seeds(&pool_state.key(), &token_mint_1.key()), bump)]
    pub token_vault_1: UncheckedAccount<'info>,
    #[account(init, payer = payer, space = 8, seeds = observation_seeds(&pool_state.key()), bump)]
    pub observation_state: UncheckedAccount<'info>,
    #[account(init, payer = payer, space = 8, seeds = tick_array_bitmap_seeds(&pool_state.key()), bump)]
    pub tick_array_bitmap: UncheckedAccount<'info>,
    pub token_program_0: Interface<'info, TokenInterface>,
    pub token_program_1: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
```

Next steps: replace the placeholder `space` values with actual struct lengths once observation/bitmap accounts are defined, add the `create_pool` handler logic, and run `cargo fmt`/tests when ready.