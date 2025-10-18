use solana_program::pubkey::Pubkey;
use std::str::FromStr;



// > Program log: protocol_position: bump 255
// > Program log: protocol_position: pool_id 4xtXs8tUtGZmaBPzoVnfFDGVVBTUaSgV5CanLpSDDgJT
// > Program log: protocol_position: tick_lower_index -153780
// > Program log: protocol_position: tick_upper_index -153360

//TODO: 修复
#[test]
fn protocol_position_pda_derivation_and_update() {
    let pool_id = Pubkey::from_str("4xtXs8tUtGZmaBPzoVnfFDGVVBTUaSgV5CanLpSDDgJT").unwrap();
    let tick_lower_index: i32 = -152880;
    let tick_upper_index: i32 = -152460;

    let (pda, bump) = practice::derive_protocol_position_pda(
        &practice::id(),
        &pool_id,
        tick_lower_index,
        tick_upper_index,
    );

    println!("protocol_position pda: {:?}, bump: {}", pda, bump);
}