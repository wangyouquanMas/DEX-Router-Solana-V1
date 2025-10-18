use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::events::AmmConfigInitializedEvent;
use crate::state::{
    AmmConfig, AMM_CONFIG_SEED, FEE_RATE_DENOMINATOR, MAX_FUND_FEE_RATE, MAX_PROTOCOL_FEE_RATE,
    MAX_TRADE_FEE_RATE,
};

/// Accounts required to initialize a new global AMM configuration.
#[derive(Accounts)]
#[instruction(index: u16)]
pub struct InitializeAmmConfig<'info> {
    /// Funds the rent-exempt allocation of `amm_config`.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Designated authority allowed to manage AMM-wide settings.
    pub authority: Signer<'info>,

    /// PDA that stores global settings shared by all pools created by `authority`.
    #[account(
        init,
        seeds = [AMM_CONFIG_SEED.as_bytes(), &index.to_be_bytes()],
        bump,
        payer = payer,
        space = AmmConfig::space(),
    )]
    pub amm_config: Account<'info, AmmConfig>,

    /// Required by Anchor to create the account using the payer's lamports.
    pub system_program: Program<'info, System>,
}

pub fn initialize_amm_config(
    ctx: Context<InitializeAmmConfig>,
    index: u16,
    tick_spacing: u16,
    trade_fee_rate: u32,
    protocol_fee_rate: u32,
    fund_fee_rate: u32,
) -> Result<()> {
    let bump = ctx.bumps.amm_config;
    initialize_config_account(
        &mut ctx.accounts.amm_config,
        bump,
        ctx.accounts.authority.key(),
        tick_spacing,
        trade_fee_rate,
        protocol_fee_rate,
        fund_fee_rate,
    )?;

    emit!(AmmConfigInitializedEvent {
        authority: ctx.accounts.authority.key(),
        index,
        trade_fee_rate,
        protocol_fee_rate,
        fund_fee_rate,
        tick_spacing,
    });

    Ok(())
}

fn initialize_config_account(
    amm_config: &mut AmmConfig,
    bump: u8,
    authority: Pubkey,
    tick_spacing: u16,
    trade_fee_rate: u32,
    protocol_fee_rate: u32,
    fund_fee_rate: u32,
) -> Result<()> {
    validate_fee_inputs(tick_spacing, trade_fee_rate, protocol_fee_rate, fund_fee_rate)?;

    amm_config.bump = bump;
    amm_config.authority = authority;
    amm_config.tick_spacing = tick_spacing;
    amm_config.trade_fee_rate = trade_fee_rate;
    amm_config.protocol_fee_rate = protocol_fee_rate;
    amm_config.fund_fee_rate = fund_fee_rate;

    Ok(())
}

fn validate_fee_inputs(
    tick_spacing: u16,
    trade_fee_rate: u32,
    protocol_fee_rate: u32,
    fund_fee_rate: u32,
) -> Result<()> {
    require!(tick_spacing > 0, ErrorCode::InvalidFeeTier);
    let total_fee_rate = trade_fee_rate
        .checked_add(protocol_fee_rate)
        .and_then(|val| val.checked_add(fund_fee_rate))
        .ok_or(ErrorCode::TotalFeeRateTooHigh)?;
    require!(total_fee_rate <= FEE_RATE_DENOMINATOR, ErrorCode::TotalFeeRateTooHigh);

    require!(trade_fee_rate <= MAX_TRADE_FEE_RATE, ErrorCode::InvalidTradeFeeRate);
    require!(protocol_fee_rate <= MAX_PROTOCOL_FEE_RATE, ErrorCode::InvalidProtocolFeeRate);
    require!(fund_fee_rate <= MAX_FUND_FEE_RATE, ErrorCode::InvalidFundFeeRate);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::error::Error;

    #[test]
    fn initialize_config_account_success() {
        let mut amm_config = AmmConfig::default();
        let authority = Pubkey::new_unique();
        let bump = 255;

        initialize_config_account(
            &mut amm_config,
            bump,
            authority,
            1,
            MAX_TRADE_FEE_RATE,
            MAX_PROTOCOL_FEE_RATE - 100,
            MAX_FUND_FEE_RATE,
        )
        .expect("should succeed");

        assert_eq!(amm_config.bump, bump);
        assert_eq!(amm_config.authority, authority);
        assert_eq!(amm_config.tick_spacing, 1);
        assert_eq!(amm_config.trade_fee_rate, MAX_TRADE_FEE_RATE);
        assert_eq!(amm_config.protocol_fee_rate, MAX_PROTOCOL_FEE_RATE - 100);
        assert_eq!(amm_config.fund_fee_rate, MAX_FUND_FEE_RATE);
    }

    #[test]
    fn initialize_config_account_fails_for_high_trade_fee() {
        let mut amm_config = AmmConfig::default();
        let authority = Pubkey::new_unique();

        let err = initialize_config_account(
            &mut amm_config,
            1,
            authority,
            1,
            MAX_TRADE_FEE_RATE + 1,
            0,
            0,
        )
        .expect_err("should fail on trade fee");

        assert_eq!(err, Error::from(ErrorCode::InvalidTradeFeeRate));
    }

    #[test]
    fn initialize_config_account_fails_when_total_fee_exceeds_denominator() {
        let mut amm_config = AmmConfig::default();
        let authority = Pubkey::new_unique();

        let err = initialize_config_account(
            &mut amm_config,
            1,
            authority,
            1,
            FEE_RATE_DENOMINATOR,
            1,
            0,
        )
        .expect_err("should fail on total fee rate");

        assert_eq!(err, Error::from(ErrorCode::TotalFeeRateTooHigh));
    }
}

