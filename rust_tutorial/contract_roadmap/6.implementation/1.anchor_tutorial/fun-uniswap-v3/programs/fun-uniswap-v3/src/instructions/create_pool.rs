use crate::errors::ErrorCode;
use crate::events::PoolCreatedEvent;
use crate::libraries::tick_math;
use crate::state::{
    AmmConfig,
    ObservationState,
    PoolState,
    TickArrayBitmapAccount,
    TickArrayBitmapExtensionAccount,
    POOL_OBSERVATION_SEED,
    POOL_STATE_SEED,
    POOL_VAULT_SEED,
    TICK_ARRAY_BITMAP_EXTENSION_SEED,
    TICK_ARRAY_BITMAP_SEED,
};
use crate::utils::{create_token_vault_account, validation};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use anchor_lang::prelude::InterfaceAccount;

/// Accounts required to create and initialize a new pool.
#[derive(Accounts)]
pub struct CreatePool<'info> {
    /// Signer paying for all newly created accounts.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Authority that owns the configuration and must approve the pool.
    pub authority: Signer<'info>,

    /// Global configuration that governs this pool.
    #[account(has_one = authority)]
    pub amm_config: Account<'info, AmmConfig>,

    /// Deterministic PDA storing core pool state data.
    #[account(
        init,
        payer = payer,
        space = PoolState::space(),
        seeds = [
            POOL_STATE_SEED.as_bytes(),
            amm_config.key().as_ref(),
            token_mint_0.key().as_ref(),
            token_mint_1.key().as_ref(),
        ],
        bump,
    )]
    pub pool_state: Account<'info, PoolState>,

    /// Token mint with the smaller public key (enforced client side).
    #[account(mint::token_program = token_program_0)]
    pub token_mint_0: InterfaceAccount<'info, Mint>,

    /// Token mint with the larger public key.
    #[account(mint::token_program = token_program_1)]
    pub token_mint_1: InterfaceAccount<'info, Mint>,

    /// PDA token account controlled by `pool_state` for mint 0.
    #[account(
        mut,
        seeds = [
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_mint_0.key().as_ref(),
        ],
        bump,
    )]
    pub token_vault_0: UncheckedAccount<'info>,

    /// PDA token account controlled by `pool_state` for mint 1.
    #[account(
        mut,
        seeds = [
            POOL_VAULT_SEED.as_bytes(),
            pool_state.key().as_ref(),
            token_mint_1.key().as_ref(),
        ],
        bump,
    )]
    pub token_vault_1: UncheckedAccount<'info>,

    /// PDA storing oracle observations for TWAP calculations.
    #[account(
        init,
        payer = payer,
        space = ObservationState::LEN,
        seeds = [
            POOL_OBSERVATION_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        bump,
    )]
    pub observation_state: AccountLoader<'info, ObservationState>,

    /// PDA storing tick-array initialization bitmap.
    #[account(
        init,
        payer = payer,
        space = TickArrayBitmapAccount::space(),
        seeds = [
            TICK_ARRAY_BITMAP_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        bump,
    )]
    pub tick_array_bitmap: Account<'info, TickArrayBitmapAccount>,

    #[account(
        init,
        payer = payer,
        space = TickArrayBitmapExtensionAccount::space(),
        seeds = [
            TICK_ARRAY_BITMAP_EXTENSION_SEED.as_bytes(),
            pool_state.key().as_ref(),
        ],
        bump,
    )]
    pub tick_array_bitmap_extension: Account<'info, TickArrayBitmapExtensionAccount>,

    /// Token program for mint_0 (supports SPL Token or Token-2022).
    pub token_program_0: Interface<'info, TokenInterface>,
    /// Token program for mint_1.
    pub token_program_1: Interface<'info, TokenInterface>,
    /// System program for account creation.
    pub system_program: Program<'info, System>,

    /// Rent sysvar required for account initialization.
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_pool(
    ctx: Context<CreatePool>,
    sqrt_price_x64: u128,
    tick_current: i32,
) -> Result<()> {
    let accounts = ctx.accounts;

    validation::validate_authority(&accounts.amm_config, &accounts.authority.key())?;
    validation::validate_mint_order(&accounts.token_mint_0.key(), &accounts.token_mint_1.key())?;
    validation::validate_mint_decimals(&accounts.token_mint_0, &accounts.token_mint_1)?;

    let pool_bump = ctx.bumps.pool_state;

    let initial_tick = tick_math::get_tick_at_sqrt_price(sqrt_price_x64)?;

    let vault_0_bump = ctx.bumps.token_vault_0;
    create_token_vault_account(
        &accounts.payer,
        &accounts.pool_state.to_account_info(),
        &accounts.token_vault_0.to_account_info(),
        &accounts.token_mint_0,
        &accounts.system_program,
        &accounts.token_program_0,
        &[
            POOL_VAULT_SEED.as_bytes(),
            accounts.pool_state.key().as_ref(),
            accounts.token_mint_0.key().as_ref(),
            &[vault_0_bump],
        ],
    )?;

    let vault_1_bump = ctx.bumps.token_vault_1;
    create_token_vault_account(
        &accounts.payer,
        &accounts.pool_state.to_account_info(),
        &accounts.token_vault_1.to_account_info(),
        &accounts.token_mint_1,
        &accounts.system_program,
        &accounts.token_program_1,
        &[
            POOL_VAULT_SEED.as_bytes(),
            accounts.pool_state.key().as_ref(),
            accounts.token_mint_1.key().as_ref(),
            &[vault_1_bump],
        ],
    )?;

    let pool_state = &mut accounts.pool_state;
    pool_state.bump = pool_bump;
    pool_state.amm_config = accounts.amm_config.key();
    pool_state.authority = accounts.authority.key();
    pool_state.token_mint_0 = accounts.token_mint_0.key();
    pool_state.token_mint_1 = accounts.token_mint_1.key();
    pool_state.token_vault_0 = accounts.token_vault_0.key();
    pool_state.token_vault_1 = accounts.token_vault_1.key();
    pool_state.observation_state = accounts.observation_state.key();
    pool_state.tick_array_bitmap = accounts.tick_array_bitmap.key();
    pool_state.tick_array_bitmap_extension = accounts.tick_array_bitmap_extension.key();
    pool_state.mint_decimals_0 = accounts.token_mint_0.decimals;
    pool_state.mint_decimals_1 = accounts.token_mint_1.decimals;
    pool_state.tick_spacing = accounts.amm_config.tick_spacing;
    pool_state.tick_current = initial_tick;
    pool_state.liquidity = 0;
    pool_state.sqrt_price_x64 = sqrt_price_x64;
    pool_state.fee_growth_global_0_x64 = 0;
    pool_state.fee_growth_global_1_x64 = 0;
    pool_state.protocol_fee_rate = accounts.amm_config.protocol_fee_rate;
    pool_state.fund_fee_rate = accounts.amm_config.fund_fee_rate;
    pool_state.trade_fee_rate = accounts.amm_config.trade_fee_rate;
    pool_state.protocol_fees_token_0 = 0;
    pool_state.protocol_fees_token_1 = 0;
    pool_state.status = 0;
    pool_state.padding0 = [0; 2];
    pool_state.reserved = [0; 32];

    validation::validate_tick_spacing(&accounts.amm_config, pool_state.tick_spacing)?;

    accounts
        .observation_state
        .load_init()?
        .initialize(pool_state.key())?;

    accounts.tick_array_bitmap.initialize(pool_state.key());
    accounts
        .tick_array_bitmap_extension
        .initialize(pool_state.key());

    let _ = tick_current;

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
        tick_array_bitmap_extension: accounts.tick_array_bitmap_extension.key(),
        sqrt_price_x64,
        tick_current: initial_tick,
    });

    Ok(())
}

