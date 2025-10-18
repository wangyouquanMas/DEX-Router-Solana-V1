use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::libraries::tick_array_bit_map;
use crate::state::tick_array::TickArrayState;

pub const TICK_ARRAY_BITMAP_SEED: &str = "tick-array-bitmap";
pub const TICK_ARRAY_BITMAP_EXTENSION_SEED: &str = "tick-array-bitmap-extension";
const EXTENSION_BITMAP_SIZE: usize = 14;

#[account]
#[derive(Debug, Default)]
pub struct TickArrayBitmapAccount {
    pub pool: Pubkey,
    pub bitmap: [u64; 16],
}

impl TickArrayBitmapAccount {
    pub const LEN: usize = 8 + 32 + (8 * 16);

    pub const fn space() -> usize {
        Self::LEN
    }

    pub fn initialize(&mut self, pool: Pubkey) {
        self.pool = pool;
        self.bitmap = [0; 16];
    }

    pub fn set(&mut self, start_index: i32, tick_spacing: u16) -> Result<()> {
        let offset = Self::offset_in_bitmap(start_index, tick_spacing)?;
        let (byte_index, bit_index) = (offset / 64, offset % 64);
        self.bitmap[byte_index as usize] |= 1u64 << bit_index;
        Ok(())
    }

    pub fn clear(&mut self, start_index: i32, tick_spacing: u16) -> Result<()> {
        let offset = Self::offset_in_bitmap(start_index, tick_spacing)?;
        let (byte_index, bit_index) = (offset / 64, offset % 64);
        self.bitmap[byte_index as usize] &= !(1u64 << bit_index);
        Ok(())
    }

    pub fn is_set(&self, start_index: i32, tick_spacing: u16) -> Result<bool> {
        let offset = Self::offset_in_bitmap(start_index, tick_spacing)?;
        let (byte_index, bit_index) = (offset / 64, offset % 64);
        Ok((self.bitmap[byte_index as usize] & (1u64 << bit_index)) != 0)
    }

    fn offset_in_bitmap(start_index: i32, tick_spacing: u16) -> Result<i32> {
        if !TickArrayState::check_is_valid_start_index(start_index, tick_spacing) {
            return err!(ErrorCode::InvalidTickIndex);
        }
        let multiplier = TickArrayState::tick_count(tick_spacing);
        let compressed = start_index / multiplier + tick_array_bit_map::TICK_ARRAY_BITMAP_SIZE / 2;
        Ok(compressed)
    }
}

#[account]
#[derive(Debug, Default)]
pub struct TickArrayBitmapExtensionAccount {
    pub pool: Pubkey,
    pub positive_bitmap: [[u64; 8]; EXTENSION_BITMAP_SIZE],
    pub negative_bitmap: [[u64; 8]; EXTENSION_BITMAP_SIZE],
}

impl TickArrayBitmapExtensionAccount {
    pub const LEN: usize = 8 + 32 + 64 * EXTENSION_BITMAP_SIZE * 2;

    pub const fn space() -> usize {
        Self::LEN
    }

    pub fn initialize(&mut self, pool: Pubkey) {
        self.pool = pool;
        self.positive_bitmap = [[0; 8]; EXTENSION_BITMAP_SIZE];
        self.negative_bitmap = [[0; 8]; EXTENSION_BITMAP_SIZE];
    }

    pub fn set(&mut self, start_index: i32, tick_spacing: u16) -> Result<()> {
        let (bucket, offset) = self.offset_in_extension(start_index, tick_spacing)?;
        let (byte_index, bit_index) = (offset / 64, offset % 64);
        if start_index < 0 {
            self.negative_bitmap[bucket][byte_index as usize] |= 1u64 << bit_index;
        } else {
            self.positive_bitmap[bucket][byte_index as usize] |= 1u64 << bit_index;
        }
        Ok(())
    }

    pub fn clear(&mut self, start_index: i32, tick_spacing: u16) -> Result<()> {
        let (bucket, offset) = self.offset_in_extension(start_index, tick_spacing)?;
        let (byte_index, bit_index) = (offset / 64, offset % 64);
        if start_index < 0 {
            self.negative_bitmap[bucket][byte_index as usize] &= !(1u64 << bit_index);
        } else {
            self.positive_bitmap[bucket][byte_index as usize] &= !(1u64 << bit_index);
        }
        Ok(())
    }

    pub fn is_set(&self, start_index: i32, tick_spacing: u16) -> Result<bool> {
        let (bucket, offset) = self.offset_in_extension(start_index, tick_spacing)?;
        let (byte_index, bit_index) = (offset / 64, offset % 64);
        let value = if start_index < 0 {
            self.negative_bitmap[bucket][byte_index as usize]
        } else {
            self.positive_bitmap[bucket][byte_index as usize]
        };
        Ok((value & (1u64 << bit_index)) != 0)
    }

    fn offset_in_extension(&self, start_index: i32, tick_spacing: u16) -> Result<(usize, i32)> {
        let ticks_in_bitmap = tick_array_bit_map::max_tick_in_tickarray_bitmap(tick_spacing);
        if start_index.abs() < ticks_in_bitmap {
            return err!(ErrorCode::InvalidTickArrayBoundary);
        }
        let bucket = (start_index.abs() / ticks_in_bitmap - 1) as usize;
        if bucket >= EXTENSION_BITMAP_SIZE {
            return err!(ErrorCode::InvalidTickIndex);
        }
        let remaining = start_index.abs() % ticks_in_bitmap;
        let offset = remaining / TickArrayState::tick_count(tick_spacing);
        Ok((bucket, offset))
    }
}

