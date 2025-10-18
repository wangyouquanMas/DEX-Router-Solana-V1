    use solana_sdk::{
        pubkey::Pubkey,
    };

    pub const POOL_TICK_ARRAY_BITMAP_SEED: &str = "pool_tick_array_bitmap_extension";

    #[test]
    fn test_bitmap_account_key_generation() {
        let pool_account_key = Pubkey::new_unique();
        let program = Pubkey::new_unique();
        let (calculated_bitmap, bitmap_bump) = Pubkey::find_program_address(
            &[
                POOL_TICK_ARRAY_BITMAP_SEED.as_bytes(),
                pool_account_key.to_bytes().as_ref(),
            ],
            &program,
        );

        println!("calculated_bitmap: {:?}", calculated_bitmap);
        println!("bitmap_bump: {:?}", bitmap_bump);
    }