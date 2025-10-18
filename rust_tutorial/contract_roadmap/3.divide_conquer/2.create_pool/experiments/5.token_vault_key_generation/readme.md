目标：
1. 掌握token_vault_0/1的生成逻辑

    let (token_vault_0, vault0_bump) = Pubkey::find_program_address(
        &[
            POOL_VAULT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
            token_mint_0.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    println!("  token_vault_0: {} (bump: {})", token_vault_0, vault0_bump);
    
    let (token_vault_1, vault1_bump) = Pubkey::find_program_address(
        &[
            POOL_VAULT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
            token_mint_1.to_bytes().as_ref(),
        ],
        &program.id(),
    );