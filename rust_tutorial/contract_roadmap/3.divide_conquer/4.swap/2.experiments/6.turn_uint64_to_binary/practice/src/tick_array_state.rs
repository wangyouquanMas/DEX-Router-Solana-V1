pub struct TickArrayState;

impl TickArrayState {
    pub fn check_is_valid_start_index(start_index: i32, tick_spacing: u16) -> bool {
        let tick_array_size = 60;
        let multiplier = i32::from(tick_spacing) * tick_array_size;
        start_index % multiplier == 0
    }
} 