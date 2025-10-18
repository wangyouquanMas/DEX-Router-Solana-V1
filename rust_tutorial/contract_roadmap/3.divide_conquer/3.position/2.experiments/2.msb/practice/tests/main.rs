//目标：
//1. 理解msb的含义？
//2. 理解msb和 log2(x) 整数部分的关系


#[test]
fn test_msb() {
    let sqrt_price_x64: u128 = 553402322211286528; 

    // Determine log_b(sqrt_ratio). First by calculating integer portion (msb)
    let leading_zeros = sqrt_price_x64.leading_zeros();
    println!("leading_zeros: {}", leading_zeros);
    let msb: u32 = 128 - leading_zeros - 1;
    let log2p_integer_x32 = (msb as i128 - 64) << 32;

    //help me output logsqrt_price_x64
    println!("log2(sqrt_price_x64): {}", log2(sqrt_price_x64 as f64));

    println!("msb: {}", msb);
    println!("log2p_integer_x32: {}", log2p_integer_x32);
}

fn log2(x: f64) -> f64 {
    x.log2()  // 在Rust中，f64类型有log2()方法
}