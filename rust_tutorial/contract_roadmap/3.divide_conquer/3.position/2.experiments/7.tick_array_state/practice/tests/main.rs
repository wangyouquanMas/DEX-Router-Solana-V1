// tests/tick_array_read.rs
use bytemuck::{Pod, Zeroable};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

const REWARD_NUM: usize = 3;
const TICK_ARRAY_SIZE_USIZE: usize = 60;

#[repr(C, packed)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct TickState {
    tick: i32,
    liquidity_net: i128,
    liquidity_gross: u128,
    fee_growth_outside_0_x64: u128,
    fee_growth_outside_1_x64: u128,
    reward_growths_outside_x64: [u128; REWARD_NUM],
    padding: [u32; 13],
}

#[repr(C, packed)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct TickArrayStateRaw {
    pool_id: [u8; 32],
    start_tick_index: i32,
    ticks: [TickState; TICK_ARRAY_SIZE_USIZE],
    initialized_tick_count: u8,
    recent_epoch: u64,
    padding: [u8; 107],
}

fn derive_tick_array_pda(program_id: &Pubkey, pool_id: &Pubkey, start_index: i32) -> Pubkey {
    let seed = b"tick_array";
    let mut be = [0u8; 4];
    be.copy_from_slice(&start_index.to_be_bytes());
    Pubkey::find_program_address(&[seed, pool_id.as_ref(), &be], program_id).0
}

fn decode_q64_64(value: u128) -> f64 {
    // Q64.64 format: 64 bits for integer part, 64 bits for fractional part
    let integer_part = (value >> 64) as u64;
    let fractional_part = (value & 0xFFFFFFFFFFFFFFFF) as u64;
    
    // Convert fractional part to decimal using u128 to avoid overflow
    let fractional_decimal = fractional_part as f64 / (1u128 << 64) as f64;
    
    integer_part as f64 + fractional_decimal
}

fn decode_q64_64_signed(value: i128) -> f64 {
    if value >= 0 {
        decode_q64_64(value as u128)
    } else {
        -decode_q64_64((-value) as u128)
    }
}

#[test]
fn print_tick_array_state() {
    // Set these via env or hardcode for your setup.
    // Example:
    // PROGRAM_ID=2ju7CAWggBdxfW4RmuEMcf3oJYPAJatdKSk5ScXBrt37
    // POOL_ID=...
    // START_INDEX=...
    // RPC_URL=http://127.0.0.1:8899
    // let program_id = std::env::var("2ju7CAWggBdxfW4RmuEMcf3oJYPAJatdKSk5ScXBrt37").expect("PROGRAM_ID not set");
    // let pool_id = std::env::var("4xtXs8tUtGZmaBPzoVnfFDGVVBTUaSgV5CanLpSDDgJT").expect("POOL_ID not set");
    // let start_index: i32 = std::env::var("START_INDEX")
    //     .expect("START_INDEX not set")
    //     .parse()
    //     .expect("invalid START_INDEX");
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());

    // let program_id = Pubkey::from_str(&program_id).expect("bad PROGRAM_ID");
    // let pool_id = Pubkey::from_str(&pool_id).expect("bad POOL_ID");
    // let tick_array_pda = derive_tick_array_pda(&program_id, &pool_id, start_index);

    let tick_array_pda = Pubkey::from_str("DCB4z7hPT4XTdTYdNu1fU7jm26jzhJcEDz3YQ6HXFny").expect("bad TICK_ARRAY_PDA");

    let client = RpcClient::new(rpc_url);
    let data = client
        .get_account_data(&tick_array_pda)
        .expect("failed to fetch account data");

    assert!(
        data.len() >= 8 + std::mem::size_of::<TickArrayStateRaw>(),
        "account too small"
    );

    // Skip 8-byte Anchor discriminator
    let raw: &TickArrayStateRaw = bytemuck::from_bytes(&data[8..8 + std::mem::size_of::<TickArrayStateRaw>()]);

    let pool_id_read = Pubkey::new_from_array(raw.pool_id);
    let start_tick_index = raw.start_tick_index;
    let initialized_tick_count = raw.initialized_tick_count;
    let recent_epoch = raw.recent_epoch;

    println!("tick_array_pda: {:?}", tick_array_pda);
    println!("pool_id: {}", pool_id_read);
    println!("start_tick_index: {}", start_tick_index);
    println!("initialized_tick_count: {}", initialized_tick_count);
    println!("recent_epoch: {}", recent_epoch);

    // Print non-zero ticks for brevity
    for (i, t) in raw.ticks.iter().enumerate() {
        let tick = t.tick;
        let liquidity_gross = t.liquidity_gross;
        let liquidity_net = t.liquidity_net;

        // Decode liquidity values from Q64.64 format
        let liquidity_gross_decoded = decode_q64_64(liquidity_gross);
        let liquidity_net_decoded = decode_q64_64_signed(liquidity_net);

        println!(
            "tick[{}]: tick={}, liquidity_gross={} (raw: {}), liquidity_net={} (raw: {})",
            i, tick, liquidity_gross_decoded, liquidity_gross, liquidity_net_decoded, liquidity_net
        );
    }
}