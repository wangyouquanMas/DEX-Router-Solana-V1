use solana_program::declare_id;

// Replace with your actual program id if needed
declare_id!("11111111111111111111111111111111");

pub use solana_program::pubkey::Pubkey as ProgramPubkey;
use solana_program::pubkey::Pubkey;

use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;

pub const POSITION_SEED: &str = "position";

/// Derive the PDA for `ProtocolPositionState` using the same seeds as the Anchor account macro.
pub fn derive_protocol_position_pda(
    program_id: &Pubkey,
    pool_state: &Pubkey,
    tick_lower_index: i32,
    tick_upper_index: i32,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            POSITION_SEED.as_bytes(),
            pool_state.as_ref(),
            &tick_lower_index.to_be_bytes(),
            &tick_upper_index.to_be_bytes(),
        ],
        program_id,
    )
}
