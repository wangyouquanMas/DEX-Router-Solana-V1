目标：
1.get_tick_offset_in_array 


内容
1. 计算方法

offset = (tick_index - self.start_tick_index) / tick_spacing 
where offset is the index of the current tick array .

1.1 代码
  /// Get tick's offset in current tick array, tick must be include in tick array， otherwise throw an error
    fn get_tick_offset_in_array(self, tick_index: i32, tick_spacing: u16) -> Result<usize> {
        let start_tick_index = TickArrayState::get_array_start_index(tick_index, tick_spacing);
        require_eq!(
            start_tick_index,
            self.start_tick_index,
            ErrorCode::InvalidTickArray
        );
        let offset_in_array =
            ((tick_index - self.start_tick_index) / i32::from(tick_spacing)) as usize;
        Ok(offset_in_array)
    }


