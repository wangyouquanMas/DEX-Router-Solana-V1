目标：
1. get array start index 


内容：
1. Input an arbitrary tick_index, output the start_index of the tick_array it sits on

1.1 math formula 


1.2 代码
    /// Input an arbitrary tick_index, output the start_index of the tick_array it sits on
    pub fn get_array_start_index(tick_index: i32, tick_spacing: u16) -> i32 {
        let ticks_in_array = TickArrayState::tick_count(tick_spacing);
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