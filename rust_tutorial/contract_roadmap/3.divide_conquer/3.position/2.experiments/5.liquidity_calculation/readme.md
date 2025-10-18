目标：
1. 计算流动性

内容：
1. 计算公式

/// Computes the amount of liquidity received for a given amount of token_0 and price range
/// Calculates ΔL = Δx (√P_upper x √P_lower)/(√P_upper - √P_lower)
pub fn get_liquidity_from_amount_0(
    mut sqrt_ratio_a_x64: u128,
    mut sqrt_ratio_b_x64: u128,
    amount_0: u64,
) -> u128 {
    //output parameters
    msg!("sqrt_ratio_a_x64: {}", sqrt_ratio_a_x64);
    msg!("sqrt_ratio_b_x64: {}", sqrt_ratio_b_x64);
    msg!("amount_0: {}", amount_0);

    // sqrt_ratio_a_x64 should hold the smaller value
    if sqrt_ratio_a_x64 > sqrt_ratio_b_x64 {
        std::mem::swap(&mut sqrt_ratio_a_x64, &mut sqrt_ratio_b_x64);
    };
    let intermediate = U128::from(sqrt_ratio_a_x64)
        .mul_div_floor(
            U128::from(sqrt_ratio_b_x64),
            U128::from(fixed_point_64::Q64),
        )
        .unwrap();

    msg!("intermediate: {}", intermediate);

    let result = U128::from(amount_0)
        .mul_div_floor(
            intermediate,
            U128::from(sqrt_ratio_b_x64 - sqrt_ratio_a_x64),
        )
        .unwrap()
        .as_u128();

    msg!("result: {}", result);
    
    result
}