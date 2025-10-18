use anchor_lang::prelude::*;

pub fn get_recent_epoch() -> Result<u64> {
    let clock = Clock::get()?;
    Ok(clock.epoch)
}


