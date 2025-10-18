- Exposed pool helpers in `state/mod.rs`: added space utility, PDA seed constructors for pool/vault/observation/tick bitmap, and a tick-spacing matcher built atop `PoolState`.
```1:40:contract_roadmap/6.implementation/1.anchor_tutorial/fun-uniswap-v3/programs/fun-uniswap-v3/src/state/mod.rs
pub const fn pool_state_space() -> usize {
    PoolState::space()
}

pub fn pool_state_seeds<'a>(
    amm_config: &'a Pubkey,
    token_mint_0: &'a Pubkey,
    token_mint_1: &'a Pubkey,
) -> [&'a [u8]; 4] {
    PoolState::seeds(amm_config, token_mint_0, token_mint_1)
}
```

Next steps: keep these helpers in mind when building `create_pool` and tests; run `cargo fmt` if not already.