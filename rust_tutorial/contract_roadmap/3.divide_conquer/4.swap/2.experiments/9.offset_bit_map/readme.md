目标：
1. 掌握next_initialized_tick_array_start_index 的计算方法

内容
1. 原理
Data we already have : 
    tick_current: -1
    current_tick_array_start_index: -3600
    zero_for_one: true 
    next_tick_array_start_index: -7200
        let next_tick_array_start_index = if zero_for_one {
            last_tick_array_start_index - TickArrayState::tick_count(tick_spacing)
        }


Objective: next initialized tickarray_start_index
