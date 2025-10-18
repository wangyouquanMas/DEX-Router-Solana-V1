use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use anchor_spl::token::spl_token;
use anchor_spl::token_2022::{get_account_data_size, GetAccountDataSize};
use anchor_spl::token_interface::{self as token_interface, Mint, TokenInterface};
use spl_token::state::Account as SplAccount;

use crate::errors::ErrorCode;
use crate::utils::create_or_allocate_account;

pub fn create_token_vault_account<'info>(
    payer: &Signer<'info>,
    pool_state: &AccountInfo<'info>,
    token_account: &AccountInfo<'info>,
    token_mint: &InterfaceAccount<'info, Mint>,
    system_program: &Program<'info, System>,
    token_program: &Interface<'info, TokenInterface>,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    let space = if token_program.key() == spl_token::ID {
        SplAccount::LEN
    } else {
        get_account_data_size(
            CpiContext::new(
                token_program.to_account_info(),
                GetAccountDataSize {
                    mint: token_mint.to_account_info(),
                },
            ),
            &[],
        )? as usize
    };

    create_or_allocate_account(
        token_program.key,
        payer.to_account_info(),
        system_program.to_account_info(),
        token_account.clone(),
        signer_seeds,
        space,
    )?;

    token_interface::initialize_account3(
        CpiContext::new(
            token_program.to_account_info(),
            token_interface::InitializeAccount3 {
                account: token_account.clone(),
                mint: token_mint.to_account_info(),
                authority: pool_state.clone(),
            },
        ),
    )
    .map_err(|_| ErrorCode::VaultBumpMismatch.into())
}

