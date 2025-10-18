use solana_program::pubkey::Pubkey;
use std::str::FromStr;

pub const TICK_ARRAY_SIZE: i32 = 60;


// > Program log: protocol_position: bump 255
// > Program log: protocol_position: pool_id 4xtXs8tUtGZmaBPzoVnfFDGVVBTUaSgV5CanLpSDDgJT
// > Program log: protocol_position: tick_lower_index -153780
// > Program log: protocol_position: tick_upper_index -153360

/// Input an arbitrary tick_index, output the start_index of the tick_array it sits on
#[test]
fn test_get_array_start_index(){
    let tick_index = -70140;
    let tick_spacing = 60;
    let start_index = get_array_start_index(tick_index, tick_spacing);
    println!("start_index: {}", start_index);
}

pub fn get_array_start_index(tick_index: i32, tick_spacing: u16) -> i32 {
        let ticks_in_array = tick_count(tick_spacing);
        let mut start = tick_index / ticks_in_array;
        if tick_index < 0 && tick_index % ticks_in_array != 0 {
            start = start - 1
        }
        start * ticks_in_array
}

pub fn tick_count(tick_spacing: u16) -> i32 {
    TICK_ARRAY_SIZE * i32::from(tick_spacing)
}