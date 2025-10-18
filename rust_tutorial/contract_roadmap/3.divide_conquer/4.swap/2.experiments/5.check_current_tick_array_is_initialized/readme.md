目标：
1. 掌握check_current_tick_array_is_initialized的创建方法


内容：
1. 代码
pub fn check_current_tick_array_is_initialized(
    bit_map: U1024,
    tick_current: i32,
    tick_spacing: u16,
) -> Result<(bool, i32)> {
    //3600
    let multiplier = i32::from(tick_spacing) * TICK_ARRAY_SIZE;
    //-1/3600 + 512 = 512 
    let mut compressed = tick_current / multiplier + 512;
    if tick_current < 0 && tick_current % multiplier != 0 {
        // round towards negative infinity
        //512 -1 = 511
        compressed -= 1;
    }
    let bit_pos: i32 = compressed.abs();
    // set current bit
    let mask = U1024::one() << bit_pos.try_into().unwrap();
    
    let masked = bit_map & mask;
    
    // check the current bit whether initialized
    let initialized = masked != U1024::default();
    if initialized {
        return Ok((true, (compressed - 512) * multiplier));
    }
    // the current bit is not initialized
    return Ok((false, (compressed - 512) * multiplier));
}

1.1 Test data 
current-tick_array_bitmap: [0, 0, 0, 0, 0, 0, 0, 9223372036854775808, 1, 0, 0, 0, 0, 0, 0, 0]
tick_current: 0
tick_spacing: 60
compressed: 512
compressed: 512
bit_pos: 512
true
