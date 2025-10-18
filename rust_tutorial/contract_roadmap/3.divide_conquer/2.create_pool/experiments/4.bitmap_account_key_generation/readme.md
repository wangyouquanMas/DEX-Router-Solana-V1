目标：
1. 掌握Bitmap生成逻辑
    let (calculated_bitmap, bitmap_bump) = Pubkey::find_program_address(
        &[
            POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
        ],
        &program.id(),
    );

    