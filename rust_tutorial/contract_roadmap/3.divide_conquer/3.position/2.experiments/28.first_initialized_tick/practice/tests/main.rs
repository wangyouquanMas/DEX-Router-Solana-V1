// use solana_sdk::pubkey::Pubkey;

// pub const TICK_ARRAY_SIZE_USIZE: usize = 60;
// pub const TICK_ARRAY_SIZE: i32 = 60;

// // Custom error type for testing
// #[derive(Debug)]
// pub enum ErrorCode {
//     InvalidTickArray,
// }

// // Custom Result type for testing
// pub type Result<T> = std::result::Result<T, ErrorCode>;

// #[test]
// fn test_first_initialized_tick() {
//     // Test setup with proper initialization
//     let mut tick_array = TickArrayState::default();
    
//     // Test case 1: Empty array (no initialized ticks)
//     let result = tick_array.first_initialized_tick(true);
//     assert!(result.is_err(), "Should return error for empty array");
    
//     // Test case 2: Array with some initialized ticks
//     test_with_initialized_ticks(&mut tick_array);
    
//     // Test case 3: Test both directions
//     test_both_directions(&mut tick_array);
    
//     // Test case 4: Edge cases
//     test_edge_cases(&mut tick_array);
// }

// fn test_with_initialized_ticks(tick_array: &mut TickArrayState) {
//     // Initialize some ticks in the middle of the array
//     tick_array.ticks[20].liquidity_gross = 1000;
//     tick_array.ticks[20].tick = tick_array.start_tick_index + (20 * 60); // Assuming tick_spacing = 60
    
//     tick_array.ticks[40].liquidity_gross = 2000;
//     tick_array.ticks[40].tick = tick_array.start_tick_index + (40 * 60);
    
//     tick_array.initialized_tick_count = 2;
    
//     // Test zero_for_one = true (search from right to left)
//     let result = tick_array.first_initialized_tick(true);
//     assert!(result.is_ok(), "Should find initialized tick");
//     let tick = result.unwrap();
//     assert_eq!(tick.liquidity_gross, 2000, "Should find rightmost initialized tick");
    
//     // Test zero_for_one = false (search from left to right)
//     let result = tick_array.first_initialized_tick(false);
//     assert!(result.is_ok(), "Should find initialized tick");
//     let tick = result.unwrap();
//     assert_eq!(tick.liquidity_gross, 1000, "Should find leftmost initialized tick");
// }

// fn test_both_directions(tick_array: &mut TickArrayState) {
//     // Clear previous state
//     tick_array.ticks.iter_mut().for_each(|tick| tick.liquidity_gross = 0);
    
//     // Initialize ticks at specific positions
//     tick_array.ticks[10].liquidity_gross = 500;
//     tick_array.ticks[10].tick = tick_array.start_tick_index + (10 * 60);
    
//     tick_array.ticks[50].liquidity_gross = 1500;
//     tick_array.ticks[50].tick = tick_array.start_tick_index + (50 * 60);
    
//     tick_array.initialized_tick_count = 2;
    
//     // Test zero_for_one = true (should find tick at index 50)
//     let result = tick_array.first_initialized_tick(true);
//     assert!(result.is_ok());
//     let tick = result.unwrap();
//     assert_eq!(tick.liquidity_gross, 1500);
//     assert_eq!(tick.tick, tick_array.start_tick_index + (50 * 60));
    
//     // Test zero_for_one = false (should find tick at index 10)
//     let result = tick_array.first_initialized_tick(false);
//     assert!(result.is_ok());
//     let tick = result.unwrap();
//     assert_eq!(tick.liquidity_gross, 500);
//     assert_eq!(tick.tick, tick_array.start_tick_index + (10 * 60));
// }

// fn test_edge_cases(tick_array: &mut TickArrayState) {
//     // Clear previous state
//     tick_array.ticks.iter_mut().for_each(|tick| tick.liquidity_gross = 0);
    
//     // Test case 1: Only first tick initialized
//     tick_array.ticks[0].liquidity_gross = 100;
//     tick_array.ticks[0].tick = tick_array.start_tick_index;
//     tick_array.initialized_tick_count = 1;
    
//     let result = tick_array.first_initialized_tick(false);
//     assert!(result.is_ok());
//     let tick = result.unwrap();
//     assert_eq!(tick.liquidity_gross, 100);
//     assert_eq!(tick.tick, tick_array.start_tick_index);
    
//     // Test case 2: Only last tick initialized
//     tick_array.ticks.iter_mut().for_each(|tick| tick.liquidity_gross = 0);
//     tick_array.ticks[59].liquidity_gross = 300;
//     tick_array.ticks[59].tick = tick_array.start_tick_index + (59 * 60);
//     tick_array.initialized_tick_count = 1;
    
//     let result = tick_array.first_initialized_tick(true);
//     assert!(result.is_ok());
//     let tick = result.unwrap();
//     assert_eq!(tick.liquidity_gross, 300);
//     assert_eq!(tick.tick, tick_array.start_tick_index + (59 * 60));
    
//     // Test case 3: All ticks initialized
//     tick_array.ticks.iter_mut().enumerate().for_each(|(i, tick)| {
//         tick.liquidity_gross = 100 + (i as u128);
//         tick.tick = tick_array.start_tick_index + (i as i32 * 60);
//     });
//     tick_array.initialized_tick_count = 60;
    
//     // Should find rightmost tick for zero_for_one = true
//     let result = tick_array.first_initialized_tick(true);
//     assert!(result.is_ok());
//     let tick = result.unwrap();
//     assert_eq!(tick.liquidity_gross, 159); // 100 + 59
    
//     // Should find leftmost tick for zero_for_one = false
//     let result = tick_array.first_initialized_tick(false);
//     assert!(result.is_ok());
//     let tick = result.unwrap();
//     assert_eq!(tick.liquidity_gross, 100); // 100 + 0
// }

// #[derive(Default, Debug)]
// pub struct TickArrayState {
//     pub pool_id: Pubkey,
//     pub start_tick_index: i32,
//     pub ticks: [TickState; TICK_ARRAY_SIZE_USIZE],
//     pub initialized_tick_count: u8,
// }

// impl TickArrayState {
//     /// Base on swap direction, return the first initialized tick in the tick array.
//     pub fn first_initialized_tick(&mut self, zero_for_one: bool) -> Result<&mut TickState> {
//         if zero_for_one {
//             let mut i = TICK_ARRAY_SIZE - 1;
//             while i >= 0 {
//                 if self.ticks[i as usize].is_initialized() {
//                     return Ok(self.ticks.get_mut(i as usize).unwrap());
//                 }
//                 i = i - 1;
//             }
//         } else {
//             let mut i = 0;
//             while i < TICK_ARRAY_SIZE_USIZE {
//                 if self.ticks[i].is_initialized() {
//                     return Ok(self.ticks.get_mut(i).unwrap());
//                 }
//                 i = i + 1;
//             }
//         }
//         Err(ErrorCode::InvalidTickArray)
//     }
// }

// #[derive(Default, Debug)]
// pub struct TickState {
//     pub tick: i32,
//     /// Amount of net liquidity added (subtracted) when tick is crossed from left to right (right to left)
//     pub liquidity_net: i128,
//     /// The total position liquidity that references this tick
//     pub liquidity_gross: u128,
// }

// impl TickState {
//     pub fn is_initialized(self) -> bool {
//         self.liquidity_gross != 0
//     }
// }