use solana_sdk::{
    pubkey::Pubkey,
};

pub const POOL_TICK_ARRAY_BITMAP_SEED: &str = "pool_tick_array_bitmap_extension";

#[test]
fn test_tickarray_bitmap_extension_key_generation(){
    // Create a pool ID account from the given string
    let pool_id_account = "8kgPAKAc1UdaRmytFfacJXxD8PUvRxFm7dgk7NeSyEK9".parse::<Pubkey>().unwrap();
    let raydium_v3_program = "3Q744orvTFPw431YtwSymTsqEUjficVgy7hj8jyMMDyT".parse::<Pubkey>().unwrap();
    // Generate the tick array bitmap extension account
    let (tickarray_bitmap_extension, _bump) = Pubkey::find_program_address(
        &[
            POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
            pool_id_account.to_bytes().as_ref(),
        ],
        &raydium_v3_program,
    );
    
    println!("Pool ID account: {:?}", pool_id_account);
    println!("Output tickArray bitmap extension account: {:?}", tickarray_bitmap_extension);
    println!("Bump seed: {}", _bump);
}



