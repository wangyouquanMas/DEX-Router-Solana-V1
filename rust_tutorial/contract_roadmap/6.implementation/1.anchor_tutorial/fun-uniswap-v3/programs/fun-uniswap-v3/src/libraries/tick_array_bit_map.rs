use crate::errors::ErrorCode;
use crate::state::tick_array::TickArrayState;
use anchor_lang::prelude::*;

pub const TICK_ARRAY_BITMAP_SIZE: i32 = 512;

pub fn max_tick_in_tickarray_bitmap(tick_spacing: u16) -> i32 {
    TickArrayState::ticks_per_array(tick_spacing) * TICK_ARRAY_BITMAP_SIZE
}

pub fn get_bitmap_tick_boundary(tick_array_start_index: i32, tick_spacing: u16) -> (i32, i32) {
    let ticks_in_one_bitmap = max_tick_in_tickarray_bitmap(tick_spacing);
    let mut m = tick_array_start_index.abs() / ticks_in_one_bitmap;
    if tick_array_start_index < 0 && tick_array_start_index.abs() % ticks_in_one_bitmap != 0 {
        m += 1;
    }
    let min_value = ticks_in_one_bitmap * m;
    if tick_array_start_index < 0 {
        (-min_value, -min_value + ticks_in_one_bitmap)
    } else {
        (min_value, min_value + ticks_in_one_bitmap)
    }
}

pub fn check_current_tick_array_is_initialized(
    is_set: bool,
    tick_current: i32,
    tick_spacing: u16,
) -> Result<(bool, i32)> {
    if TickArrayState::check_is_out_of_boundary(tick_current) {
        return err!(ErrorCode::InvalidTickIndex);
    }
    let multiplier = TickArrayState::tick_count(tick_spacing);
    let mut compressed = tick_current / multiplier + TICK_ARRAY_BITMAP_SIZE / 2;
    if tick_current < 0 && tick_current % multiplier != 0 {
        compressed -= 1;
    }
    let start_index = (compressed - TICK_ARRAY_BITMAP_SIZE / 2) * multiplier;
    Ok((is_set, start_index))
}

