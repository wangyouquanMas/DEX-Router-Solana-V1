#![allow(unexpected_cfgs)]

pub use core as core_;

use anchor_lang::prelude::*;
pub mod state;
pub mod libraries;
pub mod instructions;
pub mod errors;
pub mod events;
pub mod utils;

use instructions::*;
pub use instructions::create_pool::CreatePool;
pub use instructions::initialize_amm_config::InitializeAmmConfig;

declare_id!("3LyCjZDwFYcFoGBUwsAmAaV7c4qWYJMW5FRdLLb3Dtq6");

#[program]
pub mod fun_uniswap_v3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn initialize_amm_config(
        ctx: Context<InitializeAmmConfig>,
        index: u16,
        tick_spacing: u16,
        trade_fee_rate: u32,
        protocol_fee_rate: u32,
        fund_fee_rate: u32,
    ) -> Result<()> {
        instructions::initialize_amm_config::initialize_amm_config(
            ctx,
            index,
            tick_spacing,
            trade_fee_rate,
            protocol_fee_rate,
            fund_fee_rate,
        )
    }

    pub fn create_pool(
        ctx: Context<CreatePool>,
        sqrt_price_x64: u128,
        tick_current: i32,
    ) -> Result<()> {
        instructions::create_pool::create_pool(ctx, sqrt_price_x64, tick_current)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
