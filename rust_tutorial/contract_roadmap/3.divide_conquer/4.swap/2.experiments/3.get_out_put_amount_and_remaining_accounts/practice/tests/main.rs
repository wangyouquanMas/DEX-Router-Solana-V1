use practice::{
    next_initialized_tick_array_start_index,
    U1024
};

pub const TICK_ARRAY_BITMAP_SIZE: i32 = 512;
pub const TICK_ARRAY_SIZE: i32 = 60;

#[test]
fn test_next_initialized_tick_array_start_index() {
    // Test case 1: Find next initialized tick array going upward (one_for_zero = false)
    //Breakdown:
    // 16 elements in the array: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    // Each element is a u64: 64 bits per element
    // Total bits: 16 × 64 = 1024 bits
    let bit_map = U1024([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    // Set bit at position 516 (representing tick array starting at 240)
    let mut test_bitmap = bit_map;

    //// Formula: tick_array_start_index = (bit_pos - 512) * multiplier
    // Where multiplier = tick_spacing * TICK_ARRAY_SIZE
    //Bit 516 → Tick array starting at (516 - 512) × 60 = 4 × 60 = **240**
    test_bitmap.0[8] = 1 << 4; // Set bit 516 (8*64 + 4 = 516)
    
    let (found, next_index) = next_initialized_tick_array_start_index(
        test_bitmap,
        120,  // Start from tick array 120
        1,    // tick_spacing = 1
        false // one_for_zero = false (search upward)
    );
    
    println!("found: {}", found);
    println!("next_index: {}", next_index);

    assert!(found, "Should find next initialized tick array");
    assert_eq!(next_index, 240, "Next tick array should start at 240");
    
    // Test case 2: Find next initialized tick array going downward (zero_for_one = true)
    let (found, next_index) = next_initialized_tick_array_start_index(
        test_bitmap,
        360,  // Start from tick array 360
        1,    // tick_spacing = 1
        true  // zero_for_one = true (search downward)
    );
    
    assert!(found, "Should find next initialized tick array");
    assert_eq!(next_index, 240, "Next tick array should start at 240");
}
