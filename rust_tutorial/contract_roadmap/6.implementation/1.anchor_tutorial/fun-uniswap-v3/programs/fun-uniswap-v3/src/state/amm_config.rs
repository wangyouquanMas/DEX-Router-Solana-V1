use anchor_lang::prelude::*;

pub const AMM_CONFIG_SEED: &str = "amm-config";
pub const FEE_RATE_DENOMINATOR: u32 = 1_000_000;

/// Upper bounds (in 10^-6 precision) for the configurable fee fields.
pub const MAX_TRADE_FEE_RATE: u32 = 100_000; // 10%
pub const MAX_PROTOCOL_FEE_RATE: u32 = 500_000; // 50%
pub const MAX_FUND_FEE_RATE: u32 = 100_000; // 10%

/// Global AMM configuration shared by every pool created under the same authority.
#[account]
#[derive(Debug, Default)]
pub struct AmmConfig {
    /// PDA bump for `AMM_CONFIG_SEED`.
    pub bump: u8,
    /// Program authority allowed to update configuration or create pools.
    pub authority: Pubkey,
    /// Trade fee taken on each swap (10^-6 precision, e.g. 3000 = 0.3%).
    pub trade_fee_rate: u32,
    /// Protocol fee portion forwarded to protocol vaults (10^-6 precision).
    pub protocol_fee_rate: u32,
    /// Liquidity tick spacing (must divide ticks per price range).
    pub tick_spacing: u16,
    /// Optional additional fee directed to treasury/fund (10^-6 precision).
    pub fund_fee_rate: u32,
    /// Reserved bytes for future upgrades without breaking account layout.
    pub padding: [u64; 4],
}

impl AmmConfig {
    /// Anchor discriminator (8) + all field sizes.
    pub const LEN: usize = 8  // discriminator
        + 1                  // bump
        + 32                 // authority
        + 4                  // trade_fee_rate
        + 4                  // protocol_fee_rate
        + 2                  // tick_spacing
        + 4                  // fund_fee_rate
        + 8 * 4;             // padding

    /// Return the number of bytes required to allocate this account.
    pub const fn space() -> usize {
        Self::LEN
    }
}


