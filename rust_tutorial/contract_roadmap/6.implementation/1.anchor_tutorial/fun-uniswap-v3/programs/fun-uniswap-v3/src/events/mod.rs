use anchor_lang::prelude::*;

#[event]
pub struct AmmConfigInitializedEvent {
    pub authority: Pubkey,
    pub index: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
    pub tick_spacing: u16,
}

#[event]
pub struct PoolCreatedEvent {
    pub authority: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation: Pubkey,
    pub tick_array_bitmap: Pubkey,
    pub tick_array_bitmap_extension: Pubkey,
    pub sqrt_price_x64: u128,
    pub tick_current: i32,
}