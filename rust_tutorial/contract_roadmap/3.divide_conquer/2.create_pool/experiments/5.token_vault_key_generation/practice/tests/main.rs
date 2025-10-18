use solana_sdk::{
    pubkey::Pubkey,
};

pub const POOL_VAULT_SEED: &str = "pool_vault";

#[test]
fn test_token_vault_key_generation() {
    let pool_account_key = Pubkey::new_unique();
    let token_mint_0 = Pubkey::new_unique();
    let program = Pubkey::new_unique();
    let (token_vault_0, vault0_bump) = Pubkey::find_program_address(
        &[
            POOL_VAULT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
            token_mint_0.to_bytes().as_ref(),
        ],
        &program,
    );

    println!("token_vault_0: {:?}", token_vault_0);
    println!("vault0_bump: {:?}", vault0_bump);
}