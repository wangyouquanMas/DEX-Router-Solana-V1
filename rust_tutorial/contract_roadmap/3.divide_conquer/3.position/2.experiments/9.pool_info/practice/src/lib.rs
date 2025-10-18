use solana_program::declare_id;

// Replace with your actual program id if needed
declare_id!("11111111111111111111111111111111");

pub use solana_program::pubkey::Pubkey as ProgramPubkey;

use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;

/// Fetch logs for a given transaction signature from the provided RPC endpoint.
/// Returns an empty vector if the transaction is not found or no logs are present.
pub fn get_transaction_logs(rpc_url: &str, signature_str: &str) -> Result<Vec<String>> {
    let client = RpcClient::new(rpc_url.to_string());
    let signature = Signature::from_str(signature_str)
        .map_err(|e| anyhow!("invalid signature: {e}"))?;

    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        ..Default::default()
    };

    let tx_with_meta = match client.get_transaction_with_config(&signature, config) {
        Ok(v) => v,
        Err(e) => {
            let msg = e.to_string();
            if msg.contains("invalid type: null") {
                return Ok(Vec::new());
            }
            return Err(anyhow!("rpc get_transaction failed: {e}"));
        }
    };

    let meta = tx_with_meta
        .transaction.meta
        .ok_or_else(|| anyhow!("transaction meta not available"))?;

    let logs = match meta.log_messages {
        solana_transaction_status::option_serializer::OptionSerializer::Some(v) => v,
        _ => Vec::new(),
    };
    Ok(logs)
} 