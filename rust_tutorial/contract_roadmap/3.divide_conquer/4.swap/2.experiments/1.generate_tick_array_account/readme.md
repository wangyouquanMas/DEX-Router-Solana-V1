
目标：
1. 掌握tick array account address计算方法

内容：
1. 计算
Pubkey::find_program_address(
    &[
        raydium_amm_v3::states::TICK_ARRAY_SEED.as_bytes(),        // Seed 1: "tick_array"
        pool_config.pool_id_account.unwrap().to_bytes().as_ref(),   // Seed 2: Pool ID
        &current_vaild_tick_array_start_index.to_be_bytes(),        // Seed 3: Tick array start index
    ],
    &pool_config.raydium_v3_program,                               // Program ID
)
