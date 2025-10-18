



// #[test]
// fn test_get_sqrt_price_at_tick(){
//     let tick = 69098;
//     let sqrt_price = get_sqrt_price_at_tick(tick);
//     println!("sqrt_price: {}", sqrt_price);
// }

// /// Calculates 1.0001^(tick/2) as a U64.64 number representing
// /// the square root of the ratio of the two assets (token_1/token_0)
// ///
// /// Calculates result as a U64.64
// /// Each magic factor is `2^64 / (1.0001^(2^(i - 1)))` for i in `[0, 18)`.
// ///
// /// Throws if |tick| > MAX_TICK
// ///
// /// # Arguments
// /// * `tick` - Price tick
// ///
// pub fn get_sqrt_price_at_tick(tick: i32) -> Result<u128, anchor_lang::error::Error> {
//     println!("tick: {}", tick);
//     let abs_tick = tick.abs() as u32;
//     println!("abs_tick: {}", abs_tick);
//     require!(abs_tick <= MAX_TICK as u32, ErrorCode::TickUpperOverflow);

//     // i = 0
//     let mut ratio = if abs_tick & 0x1 != 0 {
//         U128([0xfffcb933bd6fb800, 0])
//     } else {
//         // 2^64
//         U128([0, 1])
//     };
//     println!("abs_tick & 0x1: {}", abs_tick & 0x1);
//     println!("ratio: {}", ratio);

//     // i = 1
//     if abs_tick & 0x2 != 0 {
//         ratio = (ratio * U128([0xfff97272373d4000, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x2: {}", abs_tick & 0x2);
//     println!("ratio: {}", ratio);

//     // i = 2
//     if abs_tick & 0x4 != 0 {
//         ratio = (ratio * U128([0xfff2e50f5f657000, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x4: {}", abs_tick & 0x4);
//     println!("ratio: {}", ratio);

//     // i = 3
//     if abs_tick & 0x8 != 0 {
//         ratio = (ratio * U128([0xffe5caca7e10f000, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x8: {}", abs_tick & 0x8);
//     println!("ratio: {}", ratio);

//     // i = 4
//     if abs_tick & 0x10 != 0 {
//         ratio = (ratio * U128([0xffcb9843d60f7000, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x10: {}", abs_tick & 0x10);
//     println!("ratio: {}", ratio);

//     // i = 5
//     if abs_tick & 0x20 != 0 {
//         ratio = (ratio * U128([0xff973b41fa98e800, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x20: {}", abs_tick & 0x20);
//     println!("ratio: {}", ratio);

//     // i = 6
//     if abs_tick & 0x40 != 0 {
//         ratio = (ratio * U128([0xff2ea16466c9b000, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x40: {}", abs_tick & 0x40);
//     println!("ratio: {}", ratio);

//     // i = 7
//     if abs_tick & 0x80 != 0 {
//         ratio = (ratio * U128([0xfe5dee046a9a3800, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x80: {}", abs_tick & 0x80);
//     println!("ratio: {}", ratio);

//     // i = 8
//     if abs_tick & 0x100 != 0 {
//         ratio = (ratio * U128([0xfcbe86c7900bb000, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x100: {}", abs_tick & 0x100);
//     println!("ratio: {}", ratio);

//     // i = 9
//     if abs_tick & 0x200 != 0 {
//         ratio = (ratio * U128([0xf987a7253ac65800, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x200: {}", abs_tick & 0x200);
//     println!("ratio: {}", ratio);

//     // i = 10
//     if abs_tick & 0x400 != 0 {
//         ratio = (ratio * U128([0xf3392b0822bb6000, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x400: {}", abs_tick & 0x400);
//     println!("ratio: {}", ratio);

//     // i = 11
//     if abs_tick & 0x800 != 0 {
//         ratio = (ratio * U128([0xe7159475a2caf000, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x800: {}", abs_tick & 0x800);
//     println!("ratio: {}", ratio);

//     // i = 12
//     if abs_tick & 0x1000 != 0 {
//         ratio = (ratio * U128([0xd097f3bdfd2f2000, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x1000: {}", abs_tick & 0x1000);
//     println!("ratio: {}", ratio);

//     // i = 13
//     if abs_tick & 0x2000 != 0 {
//         ratio = (ratio * U128([0xa9f746462d9f8000, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x2000: {}", abs_tick & 0x2000);
//     println!("ratio: {}", ratio);

//     // i = 14
//     if abs_tick & 0x4000 != 0 {
//         ratio = (ratio * U128([0x70d869a156f31c00, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x4000: {}", abs_tick & 0x4000);
//     println!("ratio: {}", ratio);

//     // i = 15
//     if abs_tick & 0x8000 != 0 {
//         ratio = (ratio * U128([0x31be135f97ed3200, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x8000: {}", abs_tick & 0x8000);
//     println!("ratio: {}", ratio);

//     // i = 16
//     if abs_tick & 0x10000 != 0 {
//         ratio = (ratio * U128([0x9aa508b5b85a500, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x10000: {}", abs_tick & 0x10000);
//     println!("ratio: {}", ratio);


//     // i = 17
//     if abs_tick & 0x20000 != 0 {
//         ratio = (ratio * U128([0x5d6af8dedc582c, 0])) >> NUM_64
//     };
//     println!("abs_tick & 0x20000: {}", abs_tick & 0x20000);
//     println!("ratio: {}", ratio);

//     // i = 18
//     if abs_tick & 0x40000 != 0 {
//         ratio = (ratio * U128([0x2216e584f5fa, 0])) >> NUM_64
//     }
//     println!("abs_tick & 0x40000: {}", abs_tick & 0x40000);
//     println!("ratio: {}", ratio);

//     // Divide to obtain 1.0001^(2^(i - 1)) * 2^32 in numerator
//     if tick > 0 {
//         ratio = U128::MAX / ratio;
//     }
//     println!("ratio: {}", ratio);

//     Ok(ratio.as_u128())
// }
