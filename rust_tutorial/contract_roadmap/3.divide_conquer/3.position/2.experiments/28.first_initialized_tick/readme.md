目标：
1. 计算first initialized tick 


内容：
1. 计算代码

/// Base on swap directioin, return the first initialized tick in the tick array.
    pub fn first_initialized_tick(&mut self, zero_for_one: bool) -> Result<&mut TickState> {
        if zero_for_one {
            let mut i = TICK_ARRAY_SIZE - 1;
            while i >= 0 {
                if self.ticks[i as usize].is_initialized() {
                    return Ok(self.ticks.get_mut(i as usize).unwrap());
                }
                i = i - 1;
            }
        } else {
            let mut i = 0;
            while i < TICK_ARRAY_SIZE_USIZE {
                if self.ticks[i].is_initialized() {
                    return Ok(self.ticks.get_mut(i).unwrap());
                }
                i = i + 1;
            }
        }
        err!(ErrorCode::InvalidTickArray)
    }
