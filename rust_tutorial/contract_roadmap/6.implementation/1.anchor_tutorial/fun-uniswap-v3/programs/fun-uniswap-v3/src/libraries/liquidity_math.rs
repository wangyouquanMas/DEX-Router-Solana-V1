use anchor_lang::prelude::*;

use crate::errors::ErrorCode;

pub fn add_liquidity_delta(liquidity: u128, delta: i128) -> Result<u128> {
    if delta < 0 && liquidity < delta.unsigned_abs() {
        return Err(ErrorCode::LiquidityUnderflow.into());
    }
    let updated = if delta < 0 {
        liquidity - delta.unsigned_abs()
    } else {
        liquidity + delta as u128
    };
    Ok(updated)
}

