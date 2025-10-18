use solana_sdk::{
    pubkey::Pubkey,
};

pub const POOL_SEED: &str = "pool";

#[test]
fn test_pool_account_key_generation() {
    let amm_config = Pubkey::new_unique();
    let token_mint_0 = Pubkey::new_unique();
    let token_mint_1 = Pubkey::new_unique();
    let program = Pubkey::new_unique();

    let (pool_account_key, pool_bump) = Pubkey::find_program_address(
        &[
            POOL_SEED.as_bytes(),
            amm_config.to_bytes().as_ref(),
            token_mint_0.to_bytes().as_ref(),
            token_mint_1.to_bytes().as_ref(),
        ],
        &program,
    );

    println!("pool_account_key: {:?}", pool_account_key);
    println!("pool_bump: {:?}", pool_bump);
}