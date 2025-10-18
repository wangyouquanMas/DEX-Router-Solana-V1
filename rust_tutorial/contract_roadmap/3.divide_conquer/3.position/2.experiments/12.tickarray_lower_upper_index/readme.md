目标：
1. 掌握tick_array_lower/upper_start_index的计算方法


内容：
1. 计算方法

    /// Input an arbitrary tick_index, output the start_index of the tick_array it sits on
    pub fn get_array_start_index(tick_index: i32, tick_spacing: u16) -> i32 {
        let ticks_in_array = TickArrayState::tick_count(tick_spacing);
        let mut start = tick_index / ticks_in_array;
        if tick_index < 0 && tick_index % ticks_in_array != 0 {
            start = start - 1
        }
        start * ticks_in_array
    }


1.1 举例

tick_index: -70140
tick_spacing: 60
ticks_in_array: 3600
start: -19
start: -20