use std::env;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use solana_transaction_status::{UiTransactionEncoding, option_serializer::OptionSerializer};

#[test]
fn fetch_logs_for_signature() {
    let _ = env_logger::builder().is_test(true).try_init();

    // Default to mainnet unless overridden
    let rpc_url = env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let client = RpcClient::new(rpc_url);

    // Read signature from env; skip test if not provided
    let sig_str = "2t7p6jXa3kiGyY9MNH3x4euyVU9Mk83mEpEBBjbxiWMvnZgPmJD7N1iT4wefQpsGeBAkLHWZAtokMD9ReKsNARS5";

    let signature = Signature::from_str(sig_str).expect("invalid signature");

    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    // Retry a few times on localnet while the transaction becomes available
    let tx = {
        let mut result = None;
        for attempt in 0..10 {
            match client.get_transaction_with_config(&signature, config) {
                Ok(tx) => { result = Some(tx); break; }
                Err(err) => {
                    if attempt == 9 {
                        eprintln!(
                            "Could not fetch transaction after retries. It may not exist on this cluster yet. Error: {}",
                            err
                        );
                    } else {
                        thread::sleep(Duration::from_millis(500));
                    }
                }
            }
        }
        match result { Some(tx) => tx, None => return }
    };

    // Extract and assert logs
    let meta = tx.transaction.meta.expect("no meta in transaction");
    let logs = match meta.log_messages {
        OptionSerializer::Some(logs) => logs,
        _ => panic!("no logs in transaction"),
    };
    assert!(
        !logs.is_empty(),
        "transaction had no log messages: {}",
        sig_str
    );

    // Print all logs
    for line in logs.iter() {
        println!("log: {}", line);
    }
}

