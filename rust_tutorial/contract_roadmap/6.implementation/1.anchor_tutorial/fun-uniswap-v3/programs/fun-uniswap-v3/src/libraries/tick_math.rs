use crate::errors::ErrorCode;

use super::big_num::U128;

use anchor_lang::require;

pub const MIN_TICK: i32 = -443636;
pub const MAX_TICK: i32 = -MIN_TICK;

pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;
pub const MAX_SQRT_PRICE_X64: u128 = 79226673521066979257578248091;

const NUM_64: U128 = U128([64, 0]);
const BIT_PRECISION: u32 = 16;

pub fn get_sqrt_price_at_tick(tick: i32) -> Result<u128, anchor_lang::error::Error> {
    let abs_tick = tick.abs() as u32;
    require!(abs_tick <= MAX_TICK as u32, ErrorCode::TickUpperOverflow);

    let mut ratio = if abs_tick & 0x1 != 0 {
        U128([0xfffcb933bd6fb800, 0])
    } else {
        U128([0, 1])
    };
    if abs_tick & 0x2 != 0 {
        ratio = (ratio * U128([0xfff97272373d4000, 0])) >> NUM_64
    };
    if abs_tick & 0x4 != 0 {
        ratio = (ratio * U128([0xfff2e50f5f657000, 0])) >> NUM_64
    };
    if abs_tick & 0x8 != 0 {
        ratio = (ratio * U128([0xffe5caca7e10f000, 0])) >> NUM_64
    };
    if abs_tick & 0x10 != 0 {
        ratio = (ratio * U128([0xffcb9843d60f7000, 0])) >> NUM_64
    };
    if abs_tick & 0x20 != 0 {
        ratio = (ratio * U128([0xff973b41fa98e800, 0])) >> NUM_64
    };
    if abs_tick & 0x40 != 0 {
        ratio = (ratio * U128([0xff2ea16466c9b000, 0])) >> NUM_64
    };
    if abs_tick & 0x80 != 0 {
        ratio = (ratio * U128([0xfe5dee046a9a3800, 0])) >> NUM_64
    };
    if abs_tick & 0x100 != 0 {
        ratio = (ratio * U128([0xfcbe86c7900bb000, 0])) >> NUM_64
    };
    if abs_tick & 0x200 != 0 {
        ratio = (ratio * U128([0xf987a7253ac65800, 0])) >> NUM_64
    };
    if abs_tick & 0x400 != 0 {
        ratio = (ratio * U128([0xf3392b0822bb6000, 0])) >> NUM_64
    };
    if abs_tick & 0x800 != 0 {
        ratio = (ratio * U128([0xe7159475a2caf000, 0])) >> NUM_64
    };
    if abs_tick & 0x1000 != 0 {
        ratio = (ratio * U128([0xd097f3bdfd2f2000, 0])) >> NUM_64
    };
    if abs_tick & 0x2000 != 0 {
        ratio = (ratio * U128([0xa9f746462d9f8000, 0])) >> NUM_64
    };
    if abs_tick & 0x4000 != 0 {
        ratio = (ratio * U128([0x70d869a156f31c00, 0])) >> NUM_64
    };
    if abs_tick & 0x8000 != 0 {
        ratio = (ratio * U128([0x31be135f97ed3200, 0])) >> NUM_64
    };
    if abs_tick & 0x10000 != 0 {
        ratio = (ratio * U128([0x9aa508b5b85a500, 0])) >> NUM_64
    };
    if abs_tick & 0x20000 != 0 {
        ratio = (ratio * U128([0x5d6af8dedc582c, 0])) >> NUM_64
    };
    if abs_tick & 0x40000 != 0 {
        ratio = (ratio * U128([0x2216e584f5fa, 0])) >> NUM_64
    }

    if tick > 0 {
        ratio = U128::MAX / ratio;
    }

    Ok(ratio.as_u128())
}

pub fn get_tick_at_sqrt_price(sqrt_price_x64: u128) -> Result<i32, anchor_lang::error::Error> {
    require!(
        sqrt_price_x64 >= MIN_SQRT_PRICE_X64 && sqrt_price_x64 < MAX_SQRT_PRICE_X64,
        ErrorCode::SqrtPriceX64
    );

    let msb: u32 = 128 - sqrt_price_x64.leading_zeros() - 1;
    let log2p_integer_x32 = (msb as i128 - 64) << 32;

    let mut bit: i128 = 0x8000_0000_0000_0000i128;
    let mut precision = 0;
    let mut log2p_fraction_x64 = 0;

    let mut r = if msb >= 64 {
        sqrt_price_x64 >> (msb - 63)
    } else {
        sqrt_price_x64 << (63 - msb)
    };

    while bit > 0 && precision < BIT_PRECISION {
        r *= r;
        let is_r_more_than_two = r >> 127 as u32;
        r >>= 63 + is_r_more_than_two;
        log2p_fraction_x64 += bit * is_r_more_than_two as i128;
        bit >>= 1;
        precision += 1;
    }
    let log2p_fraction_x32 = log2p_fraction_x64 >> 32;
    let log2p_x32 = log2p_integer_x32 + log2p_fraction_x32;
    let log_sqrt_10001_x64 = log2p_x32 * 59543866431248i128;
    let tick_low = ((log_sqrt_10001_x64 - 184467440737095516i128) >> 64) as i32;
    let tick_high = ((log_sqrt_10001_x64 + 15793534762490258745i128) >> 64) as i32;

    Ok(if tick_low == tick_high {
        tick_low
    } else if get_sqrt_price_at_tick(tick_high).unwrap() <= sqrt_price_x64 {
        tick_high
    } else {
        tick_low
    })
}

