use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

pub fn create_or_allocate_account<'info>(
    program_id: &Pubkey,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
    account: AccountInfo<'info>,
    signer_seeds: &[&[u8]],
    space: usize,
) -> Result<()> {
    if account.lamports() > 0 {
        return Ok(());
    }

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);
    let create_ix = system_instruction::create_account(
        payer.key,
        account.key,
        lamports,
        space as u64,
        program_id,
    );

    let payer_info = payer;
    let account_info = account;
    let system_info = system_program;

    invoke_signed(
        &create_ix,
        &[payer_info, account_info, system_info],
        &[signer_seeds],
    )?;

    Ok(())
}

