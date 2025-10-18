use anchor_lang::prelude::*;

use crate::utils::time::get_recent_epoch;

pub const OBSERVATION_SEED: &str = "observation";
pub const OBSERVATION_CAPACITY: usize = 100;
pub const OBSERVATION_UPDATE_THRESHOLD: u32 = 15;

#[account(zero_copy(unsafe))]
#[repr(C, packed)]
#[derive(Debug)]
pub struct ObservationState {
    pub initialized: bool,
    pub recent_epoch: u64,
    pub observation_index: u16,
    pub pool_id: Pubkey,
    pub observations: [Observation; OBSERVATION_CAPACITY],
    pub padding: [u64; 4],
}

impl Default for ObservationState {
    fn default() -> Self {
        Self {
            initialized: false,
            recent_epoch: 0,
            observation_index: 0,
            pool_id: Pubkey::default(),
            observations: [Observation::default(); OBSERVATION_CAPACITY],
            padding: [0; 4],
        }
    }
}

impl ObservationState {
    pub const LEN: usize = 8 + 1 + 8 + 2 + 32 + Observation::LEN * OBSERVATION_CAPACITY + 8 * 4;

    pub const fn space() -> usize {
        Self::LEN
    }

    pub fn initialize(&mut self, pool_id: Pubkey) -> Result<()> {
        *self = Self::default();
        self.recent_epoch = get_recent_epoch()?;
        self.pool_id = pool_id;
        Ok(())
    }

    pub fn update(&mut self, block_timestamp: u32, tick: i32) {
        let observation_index = self.observation_index;
        if !self.initialized {
            self.initialized = true;
            self.observations[observation_index as usize].block_timestamp = block_timestamp;
            self.observations[observation_index as usize].tick_cumulative = 0;
        } else {
            let last_observation = self.observations[observation_index as usize];
            let delta_time = block_timestamp.saturating_sub(last_observation.block_timestamp);
            if delta_time < OBSERVATION_UPDATE_THRESHOLD {
                return;
            }

            let delta_tick_cumulative = i64::from(tick).checked_mul(delta_time.into()).unwrap();
            let next_observation_index = if observation_index as usize == OBSERVATION_CAPACITY - 1 {
                0
            } else {
                observation_index + 1
            };
            self.observations[next_observation_index as usize].block_timestamp = block_timestamp;
            self.observations[next_observation_index as usize].tick_cumulative = last_observation
                .tick_cumulative
                .wrapping_add(delta_tick_cumulative);
            self.observation_index = next_observation_index;
        }
    }
}

#[zero_copy(unsafe)]
#[repr(C, packed)]
#[derive(Default, Debug)]
pub struct Observation {
    pub block_timestamp: u32,
    pub tick_cumulative: i64,
    pub padding: [u64; 4],
}

impl Observation {
    pub const LEN: usize = 4 + 8 + 8 * 4;
}
