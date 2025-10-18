use bytemuck::{Pod, Zeroable};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[repr(C, packed)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct AmmConfigRaw {
    bump: u8,
    index: u16,
    owner: [u8; 32],
    protocol_fee_rate: u32,
    trade_fee_rate: u32,
    tick_spacing: u16,
    fund_fee_rate: u32,
    padding_u32: u32,
    fund_owner: [u8; 32],
    padding: [u64; 3],
}

#[test]
fn print_amm_config() {
    // Inputs
    // Set AMM_CONFIG_ID and RPC_URL via env for flexibility
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    
    // Example AMM config address - replace with actual address for testing
    let amm_config_id = Pubkey::from_str("3LyCjZDwFYcFoGBUwsAmAaV7c4qWYJMW5FRdLLb3Dtq6")
        .expect("Invalid AMM config address");
    
    let client = RpcClient::new(rpc_url);
    
    // Fetch account data
    let account_info = client
        .get_account(&amm_config_id)
        .expect("failed to fetch account");
    
    let data = &account_info.data;
    
    println!("=== AMM Config Account Info ===");
    println!("Account Address: {}", amm_config_id);
    println!("Lamports: {}", account_info.lamports);
    println!("Owner: {}", account_info.owner);
    println!("Executable: {}", account_info.executable);
    println!("Rent Epoch: {}", account_info.rent_epoch);
    println!("Data Length: {} bytes", data.len());
    println!();

    // Verify account size
    assert!(
        data.len() >= 8 + std::mem::size_of::<AmmConfigRaw>(),
        "account too small for AmmConfig: expected at least {} bytes, got {} bytes",
        8 + std::mem::size_of::<AmmConfigRaw>(),
        data.len()
    );

    // Parse the data field (skip first 8 bytes which are Anchor discriminator)
    let raw: &AmmConfigRaw = bytemuck::from_bytes(&data[8..8 + std::mem::size_of::<AmmConfigRaw>()]);

    // Convert byte arrays to Pubkeys
    let owner = Pubkey::new_from_array(raw.owner);
    let fund_owner = Pubkey::new_from_array(raw.fund_owner);

    // Copy fields to local variables to avoid packed struct alignment issues
    let bump = raw.bump;
    let index = raw.index;
    let protocol_fee_rate = raw.protocol_fee_rate;
    let trade_fee_rate = raw.trade_fee_rate;
    let tick_spacing = raw.tick_spacing;
    let fund_fee_rate = raw.fund_fee_rate;
    let padding_u32 = raw.padding_u32;
    let padding = raw.padding;

    println!("=== AMM Config Data Fields ===");
    println!("Bump: {}", bump);
    println!("Index: {}", index);
    println!("Owner: {}", owner);
    println!("Protocol Fee Rate: {} ({}%)", 
        protocol_fee_rate, 
        (protocol_fee_rate as f64 / 1_000_000.0) * 100.0
    );
    println!("Trade Fee Rate: {} ({}%)", 
        trade_fee_rate, 
        (trade_fee_rate as f64 / 1_000_000.0) * 100.0
    );
    println!("Tick Spacing: {}", tick_spacing);
    println!("Fund Fee Rate: {} ({}%)", 
        fund_fee_rate, 
        (fund_fee_rate as f64 / 1_000_000.0) * 100.0
    );
    println!("Padding U32: {}", padding_u32);
    println!("Fund Owner: {}", fund_owner);
    println!("Padding: {:?}", padding);
    println!();

    // Calculate expected size based on the LEN constant from the struct
    let expected_size = 8 + 1 + 2 + 32 + 4 + 4 + 2 + 4 + 32 + 24; // 8 + bump + index + owner + protocol_fee + trade_fee + tick_spacing + fund_fee + padding_u32 + fund_owner + padding
    println!("=== Size Information ===");
    println!("Expected size (calculated): {} bytes", expected_size);
    println!("Expected size (from struct LEN): {} bytes", 8 + 64); // 8 + AmmConfig::LEN
    println!("Actual data size: {} bytes", data.len());
    println!("Raw struct size: {} bytes", std::mem::size_of::<AmmConfigRaw>());
    
    // Verify the parsed values make sense
    assert!(bump > 0, "Bump should be greater than 0");
    assert!(protocol_fee_rate <= 1_000_000, "Protocol fee rate should not exceed 100%");
    assert!(trade_fee_rate <= 1_000_000, "Trade fee rate should not exceed 100%");
    assert!(fund_fee_rate <= 1_000_000, "Fund fee rate should not exceed 100%");
    assert!(tick_spacing > 0, "Tick spacing should be greater than 0");
    
    println!("✅ All validation checks passed!");
}

#[test]
fn test_amm_config_parsing_edge_cases() {
    // Test with minimal valid data
    let minimal_data = vec![
        0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, // Anchor discriminator
        1u8, // bump
        0u8, 1u8, // index (256)
    ];
    
    // Add owner (32 bytes)
    let mut data = minimal_data;
    data.extend_from_slice(&vec![0u8; 32]); // owner (32 bytes)
    
    // Add remaining fields
    data.extend_from_slice(&[
        0u8, 0u8, 0u8, 0u8, // protocol_fee_rate (0)
        0u8, 0u8, 0u8, 0u8, // trade_fee_rate (0)
        1u8, 0u8, // tick_spacing (1)
        0u8, 0u8, 0u8, 0u8, // fund_fee_rate (0)
        0u8, 0u8, 0u8, 0u8, // padding_u32 (0)
    ]);
    
    // Add fund_owner (32 bytes)
    data.extend_from_slice(&vec![0u8; 32]); // fund_owner (32 bytes)
    
    // Add padding (convert u64 to bytes)
    let padding_bytes = [0u64, 0u64, 0u64];
    for &padding_value in &padding_bytes {
        data.extend_from_slice(&padding_value.to_le_bytes());
    }
    
    assert_eq!(data.len(), 8 + std::mem::size_of::<AmmConfigRaw>());
    
    let raw: &AmmConfigRaw = bytemuck::from_bytes(&data[8..8 + std::mem::size_of::<AmmConfigRaw>()]);
    
    // Copy fields to local variables to avoid packed struct alignment issues
    let bump = raw.bump;
    let index = raw.index;
    let protocol_fee_rate = raw.protocol_fee_rate;
    let trade_fee_rate = raw.trade_fee_rate;
    let tick_spacing = raw.tick_spacing;
    let fund_fee_rate = raw.fund_fee_rate;
    let padding_u32 = raw.padding_u32;
    let padding = raw.padding;
    
    assert_eq!(bump, 1);
    assert_eq!(index, 256);
    assert_eq!(protocol_fee_rate, 0);
    assert_eq!(trade_fee_rate, 0);
    assert_eq!(tick_spacing, 1);
    assert_eq!(fund_fee_rate, 0);
    assert_eq!(padding_u32, 0);
    assert_eq!(padding, [0u64, 0u64, 0u64]);
    
    println!("✅ Edge case parsing test passed!");
}
