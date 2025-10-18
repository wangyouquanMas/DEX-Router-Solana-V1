pub mod amm_config;
pub mod pool_state;
pub mod observation;
pub mod tick_array_bitmap;
pub mod tick_array;

use anchor_lang::prelude::Pubkey;

pub use amm_config::*;
pub use pool_state::*;
pub use observation::*;
pub use tick_array_bitmap::*;
pub use tick_array::*;

/// Convenience helper exposing the account space required for a pool.
pub const fn pool_state_space() -> usize {
    PoolState::space()
}

/// Seeds used to derive the pool state PDA.
pub fn pool_state_seeds<'a>(
    amm_config: &'a Pubkey,
    token_mint_0: &'a Pubkey,
    token_mint_1: &'a Pubkey,
) -> [&'a [u8]; 4] {
    PoolState::seeds(amm_config, token_mint_0, token_mint_1)
}

/// Seeds used to derive a pool vault PDA for `token_mint`.
pub fn pool_vault_seeds<'a>(pool_state: &'a Pubkey, token_mint: &'a Pubkey) -> [&'a [u8]; 3] {
    [
        POOL_VAULT_SEED.as_bytes(),
        pool_state.as_ref(),
        token_mint.as_ref(),
    ]
}

/// Seeds used to derive the observation PDA tied to a pool.
pub fn observation_seeds<'a>(pool_state: &'a Pubkey) -> [&'a [u8]; 2] {
    [POOL_OBSERVATION_SEED.as_bytes(), pool_state.as_ref()]
}

/// Seeds used to derive the tick-array bitmap PDA tied to a pool.
pub fn tick_array_bitmap_seeds<'a>(pool_state: &'a Pubkey) -> [&'a [u8]; 2] {
    [
        POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
        pool_state.as_ref(),
    ]
}

pub fn tick_array_bitmap_extension_seeds<'a>(pool_state: &'a Pubkey) -> [&'a [u8]; 2] {
    [
        TICK_ARRAY_BITMAP_EXTENSION_SEED.as_bytes(),
        pool_state.as_ref(),
    ]
}

/// Returns `true` when the provided tick spacing matches the config.
pub fn tick_spacing_matches(config: &AmmConfig, tick_spacing: u16) -> bool {
    config.tick_spacing == tick_spacing
}
