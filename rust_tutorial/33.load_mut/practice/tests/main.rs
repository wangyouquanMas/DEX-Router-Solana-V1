use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{signer::Signer, system_program, transaction::Transaction};

// Use Anchor’s re-export to avoid adding a new crate dep
use anchor_lang::solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey,
};

use counter::{accounts, id, instruction};

// Shim with fully general lifetimes that ProgramTest expects.
// Inside, we narrow the slice lifetime to match Anchor's `entry` signature.
fn process_instruction<'a, 'b, 'c, 'd>(
    program_id: &'a Pubkey,
    accounts: &'b [AccountInfo<'c>],
    data: &'d [u8],
) -> ProgramResult {
    // SAFETY: We only forward `accounts` directly to `entry` and don't store it.
    // This narrows the slice reference lifetime to `'c` to match `&'c [AccountInfo<'c>]`.
    let accounts_c: &'c [AccountInfo<'c>] = unsafe { std::mem::transmute(accounts) };
    counter::entry(program_id, accounts_c, data).map_err(Into::into)
}

#[tokio::test]
async fn increments_with_load_mut() {
    let pt = ProgramTest::new("counter", id(), processor!(process_instruction));
    let (mut banks, payer, bh) = pt.start().await;

    // PDA for State
    let (state_pda, _bump) = Pubkey::find_program_address(&[b"state"], &id());

    // 1) init
    let init_ix = solana_sdk::instruction::Instruction {
        program_id: id(),
        accounts: accounts::Init {
            state: state_pda,
            payer: payer.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: instruction::Init {}.data(),
    };
    let mut init_tx = Transaction::new_with_payer(&[init_ix], Some(&payer.pubkey()));
    init_tx.sign(&[&payer], bh);
    banks.process_transaction(init_tx).await.unwrap();

    // 2) increment
    let ix = solana_sdk::instruction::Instruction {
        program_id: id(),
        accounts: accounts::Increment { state: state_pda }.to_account_metas(None),
        data: instruction::Increment {}.data(),
    };
    let latest = banks.get_latest_blockhash().await.unwrap();
    let mut tx = Transaction::new_with_payer(&[ix], Some(&payer.pubkey()));
    tx.sign(&[&payer], latest);
    banks.process_transaction(tx).await.unwrap();

    // 3) assert counter == 1  (skip 8-byte Anchor discriminator)
    let acc = banks.get_account(state_pda).await.unwrap().expect("state exists");
    let mut ctr = [0u8; 8];
    ctr.copy_from_slice(&acc.data[8..16]);
    assert_eq!(u64::from_le_bytes(ctr), 1);
}
