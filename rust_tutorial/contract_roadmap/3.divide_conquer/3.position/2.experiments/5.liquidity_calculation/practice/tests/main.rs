use practice::get_liquidity_from_amount_0;

#[test]
fn test_get_liquidity_from_amount_0() {
    let sqrt_ratio_a_x64 = 583337266871351552;
    let sqrt_ratio_b_x64 = 688701230995211754;
    let amount_0 = 10000;

    match get_liquidity_from_amount_0(sqrt_ratio_a_x64, sqrt_ratio_b_x64, amount_0) {
        Some(liquidity) => println!("liquidity: {}", liquidity),
        None => println!("Error: Unable to calculate liquidity"),
    }
}
