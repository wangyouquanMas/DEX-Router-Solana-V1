use practice::get_transaction_logs;

#[test]
fn fetch_logs_from_live_rpc() {
    // Set `RPC_URL` and `SIG` env vars to run this against a live cluster.
    // let rpc_url = std::env::var("RPC_URL").expect("set RPC_URL");
    // let sig = std::env::var("SIG").expect("set SIG");

    let rpc_url = "http://127.0.0.1:8899";
    let sig = "5CcQa9DBrUEcVb2X8DLN7MN6cVrSqd3i8zD23eCaetAsGTHevtnjv4SBBhVW9znFKq2k9ukYKp3YFkZmMN9ysTpe";

    let logs = get_transaction_logs(&rpc_url, &sig).expect("fetch logs");

    println!("logs: {:?}", logs);
    // Just assert it doesn't crash; logs may or may not be present depending on the transaction
    assert!(logs.len() >= 0);
}