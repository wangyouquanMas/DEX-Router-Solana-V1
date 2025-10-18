
//TODO: 把POOL tickarrays tick关系图画出来
pub const TICK_ARRAY_SIZE: i32 = 60;

#[test]
fn test_get_array_start_index() {
    let tick_index = 12;
    let tick_spacing = 4;
    let offset_in_array = get_tick_offset_in_array(tick_index, tick_spacing);
    println!("offset_in_array: {}", offset_in_array);
}



  /// Get tick's offset in current tick array, tick must be include in tick array， otherwise throw an error
fn get_tick_offset_in_array(tick_index: i32, tick_spacing: u16) -> usize {
    let start_tick_index = get_array_start_index(tick_index, tick_spacing);
    // require_eq!(
    //     start_tick_index,
    //     self.start_tick_index,
    //     ErrorCode::InvalidTickArray
    // );
    let offset_in_array =
        ((tick_index - start_tick_index) / i32::from(tick_spacing)) as usize;
    offset_in_array
}


pub fn tick_count(tick_spacing: u16) -> i32 {
    TICK_ARRAY_SIZE * i32::from(tick_spacing)
}

/// Input an arbitrary tick_index, output the start_index of the tick_array it sits on
  pub fn get_array_start_index(tick_index: i32, tick_spacing: u16) -> i32 {
    let ticks_in_array = tick_count(tick_spacing);
    println!("tick_index: {}", tick_index);
    println!("tick_spacing: {}", tick_spacing);
    println!("ticks_in_array: {}", ticks_in_array);

    let mut start = tick_index / ticks_in_array;
    println!("start: {}", start);
    if tick_index < 0 && tick_index % ticks_in_array != 0 {
        start = start - 1
    }
    println!("start: {}", start);
    start * ticks_in_array
}