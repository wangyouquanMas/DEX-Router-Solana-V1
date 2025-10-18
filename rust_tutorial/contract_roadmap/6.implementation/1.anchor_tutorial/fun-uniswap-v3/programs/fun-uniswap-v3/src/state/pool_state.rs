use anchor_lang::prelude::*;
use core::slice;

/// Seed for deriving the pool state PDA.
pub const POOL_STATE_SEED: &str = "pool";
/// Seed for deriving pool token vault PDAs.
pub const POOL_VAULT_SEED: &str = "pool-vault";
/// Seed for deriving the pool observation PDA.
pub const POOL_OBSERVATION_SEED: &str = "observation";
/// Seed for deriving the pool tick-array bitmap PDA.
pub const POOL_TICK_ARRAY_BITMAP_SEED: &str = "pool-tick-array-bitmap";

/// Core state persisted for every liquidity pool created from an `AmmConfig`.
#[account]
#[derive(Debug, Default)]
pub struct PoolState {
    /// PDA bump for the pool state account.
    pub bump: u8,
    /// `AmmConfig` this pool belongs to.
    pub amm_config: Pubkey,
    /// Authority allowed to administer protocol level actions for the pool.
    pub authority: Pubkey,
    /// Ordered token mints (token_mint_0 < token_mint_1).
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    /// Token vault PDAs controlled by the pool.
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    /// Observation account storing TWAP data.
    pub observation_state: Pubkey,
    /// Tick-array bitmap account tracking initialized tick arrays.
    pub tick_array_bitmap: Pubkey,
    pub tick_array_bitmap_extension: Pubkey,
    /// Cached mint decimals for price & fee math.
    pub mint_decimals_0: u8,
    pub mint_decimals_1: u8,
    /// Tick spacing copied from `AmmConfig` for quick validation.
    pub tick_spacing: u16,
    /// Current tick index of the pool.
    pub tick_current: i32,
    /// Active liquidity available inside the current tick range.
    pub liquidity: u128,
    /// Current sqrt price in Q64.64 fixed point format.
    pub sqrt_price_x64: u128,
    /// Global fee growth accumulators (Q64.64) for each token.
    pub fee_growth_global_0_x64: u128,
    pub fee_growth_global_1_x64: u128,
    /// Snapshotted fee configuration for this pool.
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
    pub trade_fee_rate: u32,
    /// Protocol fee amounts accrued but not yet withdrawn.
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    /// Bit flags controlling pool operations (e.g., swap pause).
    pub status: u8,
    /// Padding to align the structure and leave room for future flags.
    pub padding0: [u8; 2],
    /// Reserved bytes for future upgrades without breaking layout.
    pub reserved: [u8; 32],
}

impl PoolState {
    /// Discriminator (8 bytes) + serialized field lengths.
    pub const LEN: usize = 8 // discriminator
        + 1 // bump
        + 32 * 9 // pubkeys
        + 1 // mint_decimals_0
        + 1 // mint_decimals_1
        + 2 // tick_spacing
        + 4 // tick_current
        + 16 // liquidity
        + 16 // sqrt_price_x64
        + 16 // fee_growth_global_0_x64
        + 16 // fee_growth_global_1_x64
        + 4 // protocol_fee_rate
        + 4 // fund_fee_rate
        + 4 // trade_fee_rate
        + 8 // protocol_fees_token_0
        + 8 // protocol_fees_token_1
        + 1 // status
        + 2 // padding0
        + 32; // reserved

    /// Return the number of bytes required when allocating the account.
    pub const fn space() -> usize {
        Self::LEN
    }

    /// PDA seeds (without bump) for deriving a pool state account.
    pub fn seeds<'a>(
        amm_config: &'a Pubkey,
        token_mint_0: &'a Pubkey,
        token_mint_1: &'a Pubkey,
    ) -> [&'a [u8]; 4] {
        [
            POOL_STATE_SEED.as_bytes(),
            amm_config.as_ref(),
            token_mint_0.as_ref(),
            token_mint_1.as_ref(),
        ]
    }

    /// PDA seeds including bump for signing CPI calls on behalf of the pool.
    pub fn signer_seeds<'a>(&'a self) -> [&'a [u8]; 5] {
        [
            POOL_STATE_SEED.as_bytes(),
            self.amm_config.as_ref(),
            self.token_mint_0.as_ref(),
            self.token_mint_1.as_ref(),
            slice::from_ref(&self.bump),
        ]
    }
}


