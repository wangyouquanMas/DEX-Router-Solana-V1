目标：
1. 掌握delta_x计算方式
2. 理解delta_x 和实际传入的数量数据



内容：
1. 计算
命令：./target/release/client open-position 0.9 1.5 --is-base-0 10000

公式：
/// Gets the delta amount_0 for given liquidity and price range
///
/// # Formula
///
/// * `Δx = L * (1 / √P_lower - 1 / √P_upper)`
/// * i.e. `L * (√P_upper - √P_lower) / (√P_upper * √P_lower)`

示例：
calculate delta amount0
sqrt_ratio_a_x64: 583337266871351552
sqrt_ratio_b_x64: 713944872580414131
liquidity: 1728
round_up: true
get_delta_amount_0_unsigned result: 9997

----------------------------------------------------------------------------

公式：
/// Gets the delta amount_1 for given liquidity and price range
/// * `Δy = L (√P_upper - √P_lower)`

参数：
tick_current: -69082
sqrt_price_x64_current: 583337266871351552
tick_lower: -70140
tick_upper: -65040
liquidity_delta: 1728

结果：
sqrt_ratio_a_x64: 553254713879737396
sqrt_ratio_b_x64: 583337266871351552
liquidity: 1728
round_up: true
U256::from(liquidity): 1728
U256::from(sqrt_ratio_b_x64 - sqrt_ratio_a_x64): 30082552991614156
U256::from(fixed_point_64::Q64): 18446744073709551616
result: 3

2.区别

1) input_amount is the user's intended input (e.g., "I want to provide 1000 USDC worth of liquidity")
2) deltaX is the calculated result showing exactly how much of each token (token0 and token1) is needed to create that liquidity position