pub mod big_num;
pub mod fixed_point_64;

use big_num::{U128, MulDiv};
use fixed_point_64::Q64;

pub fn get_liquidity_from_amount_0(sqrt_ratio_a_x64: u128, sqrt_ratio_b_x64: u128, amount_0: u64) -> Option<u128> {
    // Directly calculate the smaller value
    let (smaller_ratio, larger_ratio) = if sqrt_ratio_a_x64 < sqrt_ratio_b_x64 {
        (sqrt_ratio_a_x64, sqrt_ratio_b_x64)
    } else {
        (sqrt_ratio_b_x64, sqrt_ratio_a_x64)
    };

    // Check if the ratios are equal to prevent division by zero
    if larger_ratio == smaller_ratio {
        return None;
    }

    let intermediate = U128::from(smaller_ratio)
        .mul_div_floor(U128::from(larger_ratio), U128::from(Q64))?;

    let liquidity = U128::from(amount_0)
        .mul_div_floor(intermediate, U128::from(larger_ratio - smaller_ratio))?;

    Some(liquidity.as_u128())
} 