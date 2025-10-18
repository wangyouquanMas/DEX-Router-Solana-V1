use bytemuck::{Pod, Zeroable};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

const REWARD_NUM: usize = 3;

#[repr(C, packed)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct RewardInfoRaw {
    reward_state: u8,
    open_time: u64,
    end_time: u64,
    last_update_time: u64,
    emissions_per_second_x64: u128,
    reward_total_emissioned: u64,
    reward_claimed: u64,
    token_mint: [u8; 32],
    token_vault: [u8; 32],
    authority: [u8; 32],
    reward_growth_global_x64: u128,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Zeroable, Pod)]
struct PoolStateRaw {
    bump: [u8; 1],
    amm_config: [u8; 32],
    owner: [u8; 32],
    token_mint_0: [u8; 32],
    token_mint_1: [u8; 32],
    token_vault_0: [u8; 32],
    token_vault_1: [u8; 32],
    observation_key: [u8; 32],
    mint_decimals_0: u8,
    mint_decimals_1: u8,
    tick_spacing: u16,
    liquidity: u128,
    sqrt_price_x64: u128,
    tick_current: i32,
    padding3: u16,
    padding4: u16,
    fee_growth_global_0_x64: u128,
    fee_growth_global_1_x64: u128,
    protocol_fees_token_0: u64,
    protocol_fees_token_1: u64,
    swap_in_amount_token_0: u128,
    swap_out_amount_token_1: u128,
    swap_in_amount_token_1: u128,
    swap_out_amount_token_0: u128,
    status: u8,
    padding: [u8; 7],
    reward_infos: [RewardInfoRaw; REWARD_NUM],
    tick_array_bitmap: [u64; 16],
    total_fees_token_0: u64,
    total_fees_claimed_token_0: u64,
    total_fees_token_1: u64,
    total_fees_claimed_token_1: u64,
    fund_fees_token_0: u64,
    fund_fees_token_1: u64,
    open_time: u64,
    recent_epoch: u64,
    padding1: [u64; 24],
    padding2: [u64; 32],
}

#[test]
fn print_pool_state() {
    // Inputs
    // Set POOL_ID and RPC_URL via env for flexibility
    // let pool_id = std::env::var("POOL_ID").expect("POOL_ID not set");
    let rpc_url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());
    let pool_id = Pubkey::from_str("4xtXs8tUtGZmaBPzoVnfFDGVVBTUaSgV5CanLpSDDgJT").expect("bad POOL_ID");

    // let pool_id = Pubkey::from_str(&pool_id).expect("bad POOL_ID");

    let client = RpcClient::new(rpc_url);
    let data = client
        .get_account_data(&pool_id)
        .expect("failed to fetch account data");

    assert!(
        data.len() >= 8 + std::mem::size_of::<PoolStateRaw>(),
        "account too small for PoolState"
    );

    let raw: &PoolStateRaw = bytemuck::from_bytes(&data[8..8 + std::mem::size_of::<PoolStateRaw>()]);

    // Copy to locals to avoid unaligned refs
    let amm_config = Pubkey::new_from_array(raw.amm_config);
    let owner = Pubkey::new_from_array(raw.owner);
    let token_mint_0 = Pubkey::new_from_array(raw.token_mint_0);
    let token_mint_1 = Pubkey::new_from_array(raw.token_mint_1);
    let token_vault_0 = Pubkey::new_from_array(raw.token_vault_0);
    let token_vault_1 = Pubkey::new_from_array(raw.token_vault_1);
    let observation_key = Pubkey::new_from_array(raw.observation_key);

    let mint_decimals_0 = raw.mint_decimals_0;
    let mint_decimals_1 = raw.mint_decimals_1;
    let tick_spacing = raw.tick_spacing;
    let liquidity = raw.liquidity;
    let sqrt_price_x64 = raw.sqrt_price_x64;
    let tick_current = raw.tick_current;
    let fee_growth_global_0_x64 = raw.fee_growth_global_0_x64;
    let fee_growth_global_1_x64 = raw.fee_growth_global_1_x64;
    let protocol_fees_token_0 = raw.protocol_fees_token_0;
    let protocol_fees_token_1 = raw.protocol_fees_token_1;
    let status = raw.status;
    let total_fees_token_0 = raw.total_fees_token_0;
    let total_fees_claimed_token_0 = raw.total_fees_claimed_token_0;
    let total_fees_token_1 = raw.total_fees_token_1;
    let total_fees_claimed_token_1 = raw.total_fees_claimed_token_1;
    let fund_fees_token_0 = raw.fund_fees_token_0;
    let fund_fees_token_1 = raw.fund_fees_token_1;
    let open_time = raw.open_time;
    let recent_epoch = raw.recent_epoch;

    println!("pool_id: {}", pool_id);
    println!("amm_config: {}", amm_config);
    println!("owner: {}", owner);
    println!("token_mint_0: {}", token_mint_0);
    println!("token_mint_1: {}", token_mint_1);
    println!("token_vault_0: {}", token_vault_0);
    println!("token_vault_1: {}", token_vault_1);
    println!("observation_key: {}", observation_key);
    println!("mint_decimals_0: {}", mint_decimals_0);
    println!("mint_decimals_1: {}", mint_decimals_1);
    println!("tick_spacing: {}", tick_spacing);
    println!("liquidity: {}", liquidity);
    println!("sqrt_price_x64: {}", sqrt_price_x64);
    println!("tick_current: {}", tick_current);
    println!("fee_growth_global_0_x64: {}", fee_growth_global_0_x64);
    println!("fee_growth_global_1_x64: {}", fee_growth_global_1_x64);
    println!("protocol_fees_token_0: {}", protocol_fees_token_0);
    println!("protocol_fees_token_1: {}", protocol_fees_token_1);
    println!("status: {}", status);
    println!("total_fees_token_0: {}", total_fees_token_0);
    println!("total_fees_claimed_token_0: {}", total_fees_claimed_token_0);
    println!("total_fees_token_1: {}", total_fees_token_1);
    println!("total_fees_claimed_token_1: {}", total_fees_claimed_token_1);
    println!("fund_fees_token_0: {}", fund_fees_token_0);
    println!("fund_fees_token_1: {}", fund_fees_token_1);
    println!("open_time: {}", open_time);
    println!("recent_epoch: {}", recent_epoch);

    for (i, r) in raw.reward_infos.iter().enumerate() {
        let token_mint = Pubkey::new_from_array(r.token_mint);
        let token_vault = Pubkey::new_from_array(r.token_vault);
        let authority = Pubkey::new_from_array(r.authority);
        let reward_state = r.reward_state;
        let open_time = r.open_time;
        let end_time = r.end_time;
        let last_update_time = r.last_update_time;
        let emissions_per_second_x64 = r.emissions_per_second_x64;
        let reward_total_emissioned = r.reward_total_emissioned;
        let reward_claimed = r.reward_claimed;
        let reward_growth_global_x64 = r.reward_growth_global_x64;
        println!(
            "reward[{}]: state={}, mint={}, vault={}, auth={}, open={}, end={}, last_update={}, emissions_x64={}, total_emitted={}, claimed={}, growth_x64={}",
            i,
            reward_state,
            token_mint,
            token_vault,
            authority,
            open_time,
            end_time,
            last_update_time,
            emissions_per_second_x64,
            reward_total_emissioned,
            reward_claimed,
            reward_growth_global_x64,
        );
    }
}
