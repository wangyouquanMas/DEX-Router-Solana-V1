use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::libraries::{liquidity_math, tick_math};
use crate::utils::time::get_recent_epoch;

pub const TICK_ARRAY_SEED: &str = "tick_array";
pub const TICK_ARRAY_SIZE_USIZE: usize = 60;
pub const TICK_ARRAY_SIZE: i32 = 60;

#[account(zero_copy(unsafe))]
#[repr(C, packed)]
#[derive(Debug)]
pub struct TickArrayState {
    pub pool_id: Pubkey,
    pub start_tick_index: i32,
    pub ticks: [TickState; TICK_ARRAY_SIZE_USIZE],
    pub initialized_tick_count: u8,
    pub recent_epoch: u64,
    pub padding: [u8; 107],
}

impl Default for TickArrayState {
    fn default() -> Self {
        Self {
            pool_id: Pubkey::default(),
            start_tick_index: 0,
            ticks: [TickState::default(); TICK_ARRAY_SIZE_USIZE],
            initialized_tick_count: 0,
            recent_epoch: 0,
            padding: [0; 107],
        }
    }
}

impl TickArrayState {
    pub const LEN: usize = 8 + 32 + 4 + TickState::LEN * TICK_ARRAY_SIZE_USIZE + 1 + 115;

    pub fn ticks_per_array(tick_spacing: u16) -> i32 {
        TICK_ARRAY_SIZE * tick_spacing as i32
    }

    pub fn check_is_out_of_boundary(tick: i32) -> bool {
        TickState::check_is_out_of_boundary(tick)
    }

    pub fn initialize(
        &mut self,
        start_index: i32,
        tick_spacing: u16,
        pool_key: Pubkey,
    ) -> Result<()> {
        require!(
            Self::check_is_valid_start_index(start_index, tick_spacing),
            ErrorCode::InvalidTickIndex
        );
        self.start_tick_index = start_index;
        self.pool_id = pool_key;
        self.recent_epoch = get_recent_epoch()?;
        Ok(())
    }

    pub fn mark_tick(&mut self, tick_index: i32, tick_spacing: u16, initialized: bool) -> Result<()> {
        let tick_state = self.get_tick_state_mut(tick_index, tick_spacing)?;
        let was_initialized = tick_state.is_initialized();
        if initialized {
            tick_state.initialize(tick_index, tick_spacing)?;
            tick_state.set_initialized();
        } else {
            tick_state.clear();
        }

        match (was_initialized, initialized) {
            (false, true) => self.update_initialized_tick_count(true)?,
            (true, false) => self.update_initialized_tick_count(false)?,
            _ => {}
        }

        Ok(())
    }

    pub fn is_tick_initialized(&self, tick_index: i32, tick_spacing: u16) -> Result<bool> {
        if !Self::belongs_to_array(tick_index, self.start_tick_index, tick_spacing) {
            return Ok(false);
        }
        let offset = self.get_tick_offset_in_array(tick_index, tick_spacing)?;
        Ok(self.ticks[offset].is_initialized())
    }

    pub fn update_initialized_tick_count(&mut self, add: bool) -> Result<()> {
        if add {
            self.initialized_tick_count = self
                .initialized_tick_count
                .checked_add(1)
                .ok_or(ErrorCode::InvalidTickIndex)?;
        } else {
            self.initialized_tick_count = self
                .initialized_tick_count
                .checked_sub(1)
                .ok_or(ErrorCode::InvalidTickIndex)?;
        }
        Ok(())
    }

    pub fn get_tick_state_mut(
        &mut self,
        tick_index: i32,
        tick_spacing: u16,
    ) -> Result<&mut TickState> {
        let offset_in_array = self.get_tick_offset_in_array(tick_index, tick_spacing)?;
        Ok(&mut self.ticks[offset_in_array])
    }

    fn get_tick_offset_in_array(&self, tick_index: i32, tick_spacing: u16) -> Result<usize> {
        let start_tick_index = Self::get_array_start_index(tick_index, tick_spacing);
        require_eq!(start_tick_index, self.start_tick_index, ErrorCode::InvalidTickArray);
        let offset_in_array = ((tick_index - self.start_tick_index) / i32::from(tick_spacing))
            as usize;
        Ok(offset_in_array)
    }

    pub fn get_array_start_index(tick_index: i32, tick_spacing: u16) -> i32 {
        let ticks_in_array = Self::ticks_per_array(tick_spacing);
        let mut start = tick_index / ticks_in_array;
        if tick_index < 0 && tick_index % ticks_in_array != 0 {
            start -= 1;
        }
        start * ticks_in_array
    }

    pub fn belongs_to_array(tick_index: i32, start_index: i32, tick_spacing: u16) -> bool {
        Self::get_array_start_index(tick_index, tick_spacing) == start_index
    }

    pub fn check_is_valid_start_index(tick_index: i32, tick_spacing: u16) -> bool {
        if TickState::check_is_out_of_boundary(tick_index) {
            if tick_index > tick_math::MAX_TICK {
                return false;
            }
            let min_start_index = Self::get_array_start_index(tick_math::MIN_TICK, tick_spacing);
            return tick_index == min_start_index;
        }
        tick_index % Self::ticks_per_array(tick_spacing) == 0
    }

    pub fn tick_count(tick_spacing: u16) -> i32 {
        Self::ticks_per_array(tick_spacing)
    }
}

#[zero_copy(unsafe)]
#[repr(C, packed)]
#[derive(Debug)]
pub struct TickState {
    pub tick: i32,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_0_x64: u128,
    pub fee_growth_outside_1_x64: u128,
    pub padding: [u64; 8],
}

impl Default for TickState {
    fn default() -> Self {
        Self {
            tick: 0,
            liquidity_net: 0,
            liquidity_gross: 0,
            fee_growth_outside_0_x64: 0,
            fee_growth_outside_1_x64: 0,
            padding: [0; 8],
        }
    }
}

impl TickState {
    pub const LEN: usize = 4 + 16 + 16 + 16 + 16 + 8 * 8;

    pub fn initialize(&mut self, tick: i32, tick_spacing: u16) -> Result<()> {
        require!(tick % i32::from(tick_spacing) == 0, ErrorCode::TickAndSpacingNotMatch);
        self.tick = tick;
        Ok(())
    }

    pub fn set_initialized(&mut self) {
        if self.liquidity_gross == 0 {
            self.liquidity_gross = 1;
        }
    }

    pub fn update(&mut self, liquidity_delta: i128, upper: bool) -> Result<bool> {
        let liquidity_gross_before = self.liquidity_gross;
        let liquidity_gross_after = liquidity_math::add_liquidity_delta(
            liquidity_gross_before,
            liquidity_delta,
        )?;

        let flipped = (liquidity_gross_after == 0) != (liquidity_gross_before == 0);
        self.liquidity_gross = liquidity_gross_after;
        if upper {
            self.liquidity_net = self.liquidity_net.checked_sub(liquidity_delta).unwrap_or_default();
        } else {
            self.liquidity_net = self.liquidity_net.checked_add(liquidity_delta).unwrap_or_default();
        }
        Ok(flipped)
    }

    pub fn clear(&mut self) {
        self.liquidity_net = 0;
        self.liquidity_gross = 0;
        self.fee_growth_outside_0_x64 = 0;
        self.fee_growth_outside_1_x64 = 0;
    }

    pub fn is_initialized(&self) -> bool {
        self.liquidity_gross != 0
    }

    pub fn check_is_out_of_boundary(tick: i32) -> bool {
        tick < tick_math::MIN_TICK || tick > tick_math::MAX_TICK
    }
}

