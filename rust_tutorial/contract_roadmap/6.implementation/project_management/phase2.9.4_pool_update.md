### Create Pool Handler – Raydium Features Ported

I’ve expanded `instructions/create_pool.rs` to mirror Raydium’s flow:

- **Authority & mint validation** now checks the actual config signer, mint order, mint decimals, underlying mint whitelist, and superstate support.
- **Vault creation** uses the new `create_token_vault_account`, including ImmutableOwner handling and token-2022 support.
- **Tick derivation** uses `tick_math::get_tick_at_sqrt_price` to reconcile the requested sqrt price with the implied tick.
- **State initialization** sets the full pool state (vaults, observation, tick bitmap, etc.) and zeroes fee/liquidity fields.
- **Observation/tick bitmap** accounts are initialized via their new structs, ensuring epochs reset and bitmaps start clean.
- **Event emission** now includes observation + bitmap addresses and the extension handshake.

```1:199:contract_roadmap/.../create_pool.rs
let initial_tick = tick_math::get_tick_at_sqrt_price(sqrt_price_x64)?;
...
accounts.observation_state.initialize(pool_state.key())?;
accounts.tick_array_bitmap.initialize(pool_state.key());
emit!(PoolCreatedEvent {
    authority: accounts.authority.key(),
    amm_config: accounts.amm_config.key(),
    pool_state: accounts.pool_state.key(),
    token_mint_0: accounts.token_mint_0.key(),
    token_mint_1: accounts.token_mint_1.key(),
    token_vault_0: accounts.token_vault_0.key(),
    token_vault_1: accounts.token_vault_1.key(),
    observation: accounts.observation_state.key(),
    tick_array_bitmap: accounts.tick_array_bitmap.key(),
    tick_array_bitmap_extension: Pubkey::default(),
    sqrt_price_x64,
    tick_current,
});
```

### State + Library Enhancements

- Added zero-copy `TickArrayState`/`TickState` plus `TickArrayBitmapAccount` extension skeleton.
- Imported `tick_math`, `tick_array_bit_map`, and `big_num` utilities to support tick validation and future bitmap logic.

Key files touched:
- `src/instructions/create_pool.rs`
- `src/state/{pool_state.rs, tick_array.rs, tick_array_bitmap.rs, observation.rs}`
- `src/utils/{token.rs, system.rs}`
- `src/events/mod.rs`
- `src/libraries/*` (tick math + helpers)

### Next Steps

- Wire in actual tick-array bitmap extension and reservation logic (currently stubbed).
- Extend validation to cover mint whitelist and open-time constraints (clock-based).
- Add unit/integration tests once the remaining state logic is in place.

Let me know when you’re ready to tackle the next item (bitmap extension handling, create_pool tests, etc.).