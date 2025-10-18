pub const TICK_ARRAY_BITMAP_SIZE: i32 = 512;
pub const TICK_ARRAY_SIZE: i32 = 60;

#[test]
fn test_get_max_tick_in_tickarray_bitmap() {
	let zero_for_one = false;
	let last_tick_array_start_index = -3600;
	let tick_spacing = 60;
	let next_tick_array_start_index = if zero_for_one {
        last_tick_array_start_index - tick_count(tick_spacing)
    } else {
        last_tick_array_start_index + tick_count(tick_spacing)
    };
	println!("next_tick_array_start_index: {}", next_tick_array_start_index);
}


pub fn tick_count(tick_spacing: u16) -> i32 {
	TICK_ARRAY_SIZE * i32::from(tick_spacing)
}

	
