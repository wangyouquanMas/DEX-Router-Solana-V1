目标：
1. 掌握observation_key的生成及创建逻辑

    let (observation_key, obs_bump) = Pubkey::find_program_address(
        &[
            OBSERVATION_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
        ],
        &program.id(),
    );