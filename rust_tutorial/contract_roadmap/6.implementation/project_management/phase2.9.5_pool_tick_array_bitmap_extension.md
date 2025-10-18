### Tick-Array Bitmap Extension Ported

I’ve brought over the core bitmap-extension logic so pools can track tick arrays beyond the default bitmap:

- `TickArrayBitmapExtensionAccount` now includes 14-slot positive/negative bitmaps and helper methods (`get_bitmap_offset`, `flip_tick_array_bit`, `check_tick_array_is_initialized`, `next_initialized_tick_array_from_one_bitmap`, etc.).
- Added the corresponding imports and constants from the math helpers so the bitmap extensions use the same `U512` wrappers and tick/spacing boundary checks as Raydium.

You can see the new structure in `state/tick_array_bitmap.rs` and the utility modules under `libraries/` that drive it.

This unblocks the next step—wiring the extension account into `create_pool` (allocating it alongside the default bitmap, storing its key in `PoolState`, and using it in validation paths). Let me know when you want to proceed with that wiring or if you’d like to add companion tests.


### Tick-Array Bitmap Extension Wiring Done

- `CreatePool` now allocates `TickArrayBitmapExtensionAccount` alongside the base bitmap and stores its key in `PoolState`.
- Event payload includes both bitmap PDAs, and both accounts are initialized during pool creation.
- Added helper `tick_array_bitmap_extension_seeds` to `state/mod.rs`.

```1:118:contract_roadmap/…/create_pool.rs
#[account(
    init,
    payer = payer,
    space = TickArrayBitmapExtensionAccount::space(),
    seeds = tick_array_bitmap_extension_seeds(&pool_state.key()),
    bump,
)]
pub tick_array_bitmap_extension: Account<'info, TickArrayBitmapExtensionAccount>,
…
accounts.tick_array_bitmap.initialize(pool_state.key());
accounts
    .tick_array_bitmap_extension
    .initialize(pool_state.key());

emit!(PoolCreatedEvent {
    …
    tick_array_bitmap: accounts.tick_array_bitmap.key(),
    tick_array_bitmap_extension: accounts.tick_array_bitmap_extension.key(),
    …
});
```

Next: we still need to use the extension during tick-crossing logic, validation paths, and remaining instructions. Tests should also confirm both bitmap accounts get created and referenced. Let me know when you want to tackle those.