use solana_sdk::{
    pubkey::Pubkey,
};

pub const OBSERVATION_SEED: &str = "observation";

#[test]
fn test_observation_key_generation() {
    let pool_account_key = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let (observation_key, obs_bump) = Pubkey::find_program_address(
        &[
            OBSERVATION_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
        ],
        &program,
    );

    println!("observation_key: {:?}", observation_key);
    println!("obs_bump: {:?}", obs_bump);
}