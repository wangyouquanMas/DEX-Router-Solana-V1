- Added validation helpers in `utils/validation.rs` and wired them into `create_pool` to enforce authority match, mint order, decimal parity, and tick-spacing checks; expanded error codes to cover new validation failures.
```1:131:contract_roadmap/6.implementation/1.anchor_tutorial/fun-uniswap-v3/programs/fun-uniswap-v3/src/instructions/create_pool.rs
validation::validate_authority(&accounts.amm_config, &accounts.authority.key())?;
validation::validate_mint_order(&accounts.token_mint_0.key(), &accounts.token_mint_1.key())?;
validation::validate_mint_decimals(&accounts.token_mint_0, &accounts.token_mint_1)?;
...
validation::validate_tick_spacing(&accounts.amm_config, pool_state)?;
```

Next steps: define observation/tick bitmap account types and remaining init logic; run `cargo fmt` and relevant tests when ready.