use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

const OBSERVATION_NUM: usize = 100;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, Default)]
struct Observation {
    /// The block timestamp of the observation
    pub block_timestamp: u32,
    /// the cumulative of tick during the duration time
    pub tick_cumulative: i64,
    /// padding for feature update
    pub padding: [u64; 4],
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct ObservationState {
    /// Whether the ObservationState is initialized
    pub initialized: bool,
    /// recent update epoch
    pub recent_epoch: u64,
    /// the most-recently updated index of the observations array
    pub observation_index: u16,
    /// belongs to which pool
    pub pool_id: Pubkey,
    /// observation array
    pub observations: [Observation; OBSERVATION_NUM],
    /// padding for feature update
    pub padding: [u64; 4],
}

#[test]
fn decode_observation() -> Result<(), Box<dyn std::error::Error>> {
    // The observation key account you provided
    let observation_key = "77BNsKe4b8BNH4iFCBP8FP3vHfAK7WQtQRKs5yNvs9LW";
    let pubkey = Pubkey::from_str(observation_key)?;
    
    // Connect to Solana RPC (you can change this to mainnet, devnet, etc.)
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:8899".to_string());
    let client = RpcClient::new(rpc_url);
    
    println!("Fetching observation account: {}", observation_key);
    // println!("RPC URL: {}", rpc_url);
    println!();
    
    // Get account info
    let account_info = client.get_account(&pubkey)?;
    
    println!("=== Observation Account Info ===");
    println!("Account Address: {}", pubkey);
    println!("Lamports: {}", account_info.lamports);
    println!("Owner: {}", account_info.owner);
    println!("Executable: {}", account_info.executable);
    println!("Rent Epoch: {}", account_info.rent_epoch);
    println!("Data Length: {} bytes", account_info.data.len());
    println!();
    
    // Check if account exists and has data
    if account_info.data.is_empty() {
        println!("❌ Account has no data or doesn't exist");
        return Ok(());
    }
    
    // Verify data size
    let expected_size = std::mem::size_of::<ObservationState>();
    if account_info.data.len() < expected_size {
        println!("❌ Account data too small. Expected at least {} bytes, got {} bytes", 
                expected_size, account_info.data.len());
        return Ok(());
    }
    
    // Parse the data (skip first 8 bytes which are Anchor discriminator)
    let data = &account_info.data[8..];
    if data.len() < expected_size {
        println!("❌ Data after discriminator too small. Expected at least {} bytes, got {} bytes", 
                expected_size, data.len());
        return Ok(());
    }
    
    // Safely copy data to avoid unaligned references
    let mut observation_state_data = [0u8; std::mem::size_of::<ObservationState>()];
    observation_state_data.copy_from_slice(&data[..expected_size]);
    
    // Use bytemuck to safely cast the bytes to our struct
    let observation_state: &ObservationState = unsafe {
        std::mem::transmute(&observation_state_data)
    };
    
    // Copy fields to local variables to avoid packed struct alignment issues
    let initialized = observation_state.initialized;
    let recent_epoch = observation_state.recent_epoch;
    let observation_index = observation_state.observation_index;
    let pool_id = observation_state.pool_id;
    
    println!("=== Observation State Data ===");
    println!("Initialized: {}", initialized);
    println!("Recent Epoch: {}", recent_epoch);
    println!("Observation Index: {}", observation_index);
    println!("Pool ID: {}", pool_id);
    println!();
    
    println!("=== Observations Array ===");
    println!("Total observation slots: {}", OBSERVATION_NUM);
    println!("Current index: {}", observation_index);
    println!();
    
    // Display the most recent observations (last 10)
    let start_idx = if observation_index >= 10 {
        observation_index - 10
    } else {
        0
    };
    
    for i in start_idx..=observation_index {
        let obs = &observation_state.observations[i as usize];
        // Copy fields to local variables to avoid alignment issues
        let block_timestamp = obs.block_timestamp;
        let tick_cumulative = obs.tick_cumulative;
        let padding = obs.padding;
        
        if block_timestamp != 0 || tick_cumulative != 0 {
            println!("Observation[{}]:", i);
            println!("  Timestamp: {} ({} seconds ago)", 
                    block_timestamp, 
                    get_time_ago(block_timestamp));
            println!("  Tick Cumulative: {}", tick_cumulative);
            println!("  Padding: {:?}", padding);
            println!();
        }
    }
    
    // Display some older observations if they exist
    if observation_index > 20 {
        println!("=== Sample of Older Observations ===");
        for i in (0..observation_index - 20).step_by(20) {
            let obs = &observation_state.observations[i as usize];
            // Copy fields to local variables to avoid alignment issues
            let block_timestamp = obs.block_timestamp;
            let tick_cumulative = obs.tick_cumulative;
            
            if block_timestamp != 0 {
                println!("Observation[{}]: timestamp={}, tick_cumulative={}", 
                        i, block_timestamp, tick_cumulative);
            }
        }
        println!();
    }
    
    // Calculate some statistics
    let mut valid_observations = 0;
    let mut total_tick_cumulative = 0i64;
    let mut min_timestamp = u32::MAX;
    let mut max_timestamp = 0u32;
    
    for (_i, obs) in observation_state.observations.iter().enumerate() {
        // Copy fields to local variables to avoid alignment issues
        let block_timestamp = obs.block_timestamp;
        let tick_cumulative = obs.tick_cumulative;
        
        if block_timestamp != 0 {
            valid_observations += 1;
            total_tick_cumulative += tick_cumulative;
            min_timestamp = min_timestamp.min(block_timestamp);
            max_timestamp = max_timestamp.max(block_timestamp);
        }
    }
    
    println!("=== Statistics ===");
    println!("Valid observations: {}/{}", valid_observations, OBSERVATION_NUM);
    if valid_observations > 0 {
        println!("Average tick cumulative: {}", total_tick_cumulative / valid_observations as i64);
        println!("Time range: {} to {} ({} seconds)", 
                min_timestamp, max_timestamp, max_timestamp - min_timestamp);
    }
    
    println!();
    println!("✅ Successfully decoded observation account data!");
    
    Ok(())
}

fn get_time_ago(timestamp: u32) -> String {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    
    if timestamp > current_time {
        return "future".to_string();
    }
    
    let diff = current_time - timestamp;
    if diff < 60 {
        format!("{} seconds", diff)
    } else if diff < 3600 {
        format!("{} minutes", diff / 60)
    } else if diff < 86400 {
        format!("{} hours", diff / 3600)
    } else {
        format!("{} days", diff / 86400)
    }
} 