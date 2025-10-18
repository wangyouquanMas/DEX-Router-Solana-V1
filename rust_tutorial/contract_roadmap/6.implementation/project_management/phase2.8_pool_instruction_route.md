- Updated `lib.rs` to export `CreatePool` and register the `create_pool` instruction route, delegating to the handler.
```8:47:contract_roadmap/6.implementation/1.anchor_tutorial/fun-uniswap-v3/programs/fun-uniswap-v3/src/lib.rs
pub use instructions::create_pool::CreatePool;
...
pub fn create_pool(
    ctx: Context<CreatePool>,
    sqrt_price_x64: u128,
    tick_current: i32,
) -> Result<()> {
    instructions::create_pool::create_pool(ctx, sqrt_price_x64, tick_current)
}
```

Next steps: integrate handler logic once observation/tick accounts exist in state; run `cargo fmt` afterwards. Step9 (tests) remains pending.