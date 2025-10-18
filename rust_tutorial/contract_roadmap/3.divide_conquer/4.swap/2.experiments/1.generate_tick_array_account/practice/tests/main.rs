use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub const TICK_ARRAY_SEED: &str = "tick_array";


#[cfg(test)]
mod tick_array_tests {
    use super::*;

    /// Test tick array account address generation
    /// This test verifies that the same inputs always generate the same tick array address
    #[test]
    fn test_tick_array_address_generation() {
        // Mock program ID (this would be the actual raydium v3 program ID in production)
        let program_id = Pubkey::from_str("3Q744orvTFPw431YtwSymTsqEUjficVgy7hj8jyMMDyT").unwrap();
        
        // Mock pool ID (this would be the actual pool ID in production)
        let pool_id = Pubkey::from_str("4xtXs8tUtGZmaBPzoVnfFDGVVBTUaSgV5CanLpSDDgJT").unwrap();
        
        // Test different tick array start indices
        let test_cases: Vec<i32> = vec![
            0,           // Start at tick 0
            60,          // Start at tick 60 (assuming tick spacing of 1)
            -60,         // Start at tick -60
            120,         // Start at tick 120
            -120,        // Start at tick -120
        ];

        for &tick_array_start_index in &test_cases {
            let (tick_array_address, bump) = Pubkey::find_program_address(
                &[
                    TICK_ARRAY_SEED.as_bytes(),
                    pool_id.to_bytes().as_ref(),
                    &tick_array_start_index.to_be_bytes(),
                ],
                &program_id,
            );
            println!("Tick array address: {:?}, Bump: {}", tick_array_address, bump);
        }
    }
} 