use crate::errors::ErrorCode;
use crate::state::{AmmConfig, PoolState};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

pub fn validate_mint_order(mint_0: &Pubkey, mint_1: &Pubkey) -> Result<()> {
    require!(mint_0 < mint_1, ErrorCode::InvalidMintOrder);
    Ok(())
}

pub fn validate_mint_decimals(
    mint_0: &InterfaceAccount<Mint>,
    mint_1: &InterfaceAccount<Mint>,
) -> Result<()> {
    require_eq!(mint_0.decimals, mint_1.decimals, ErrorCode::MintDecimalsMismatch);
    Ok(())
}

pub fn validate_tick_spacing(config: &AmmConfig, tick_spacing: u16) -> Result<()> {
    require_eq!(tick_spacing, config.tick_spacing, ErrorCode::TickSpacingMismatch);
    Ok(())
}

pub fn ensure_pool_uninitialized(pool_state: &Account<PoolState>) -> Result<()> {
    require!(pool_state.liquidity == 0, ErrorCode::PoolAlreadyInitialized);
    Ok(())
}

pub fn validate_authority(config: &AmmConfig, signer: &Pubkey) -> Result<()> {
    require_keys_eq!(config.authority, *signer, ErrorCode::InvalidAuthority);
    Ok(())
}

