// Define fixed_point_64 module locally
mod fixed_point_64 {
    pub const Q64: u128 = 1 << 64;
}

mod fixed_point_32 {
    pub const Q32: u128 = 1 << 32;
}

#[test]
fn test_price_to_sqrt_price_x64() {
    let price = 0.9;
    let decimals_0 = 9;
    let decimals_1 = 6;

    let sqrt_price_x64 = price_to_sqrt_price_x64(price, decimals_0, decimals_1);
    println!("111sqrt_price_x64: {}", sqrt_price_x64);

    // state.sqrt_price_x64:583330181263634387 0.8466825204893473
    // step.sqrt_price_next_x64:521037230575395660 0.7978074153597088
    let sqrt_price_x64 = 22592555198148960256;  // 0.7978074153597088
    let price_from_x64 = sqrt_price_x64_to_price(sqrt_price_x64, decimals_0, decimals_1);
    println!("price_from_x64: {}", price_from_x64);

    assert_eq!(price, price_from_x64);
}

#[test]
fn test_sqrt_price_x64_to_price(){
    let sqrt_price_x64 = 517920522498632426;
    let decimals_0 = 6;
    let decimals_1 = 6;

    let price = sqrt_price_x64_to_price(sqrt_price_x64, decimals_0, decimals_1);
    println!("original price is: {}", price);

    // assert_eq!(price, 4.4);
}


#[test]
fn test_sqrt_price_x32_to_price(){
    let sqrt_price_x32 = 295239680;
    let decimals_0 = 6;
    let decimals_1 = 6;

    let price = sqrt_price_x32_to_price_with_no_pow(sqrt_price_x32, decimals_0, decimals_1);
    println!("x32 original price is: {}", price);

    // assert_eq!(price, 4.4);
}


pub fn sqrt_price_x64_to_price_with_no_pow(price: u128, decimals_0: u8, decimals_1: u8) -> f64 {
    from_x64_price(price)* multipler(decimals_0) / multipler(decimals_1)
}

pub fn sqrt_price_x32_to_price_with_no_pow(price: u128, decimals_0: u8, decimals_1: u8) -> f64 {
    from_x64_price(price)* multipler(decimals_0) / multipler(decimals_1)
}

pub fn from_x32_price(price: u128) -> f64 {
    price as f64 / fixed_point_32::Q32 as f64
}



#[test]
fn test_log2() {
    let x = 1.5;
    //log2(x): 0.5849625007211562
    println!("log2(x): {}", log2(x as f64));
}

#[test]
fn test_1_point_5_to_x64(){
    let x = 1.5;
    let x64 = price_to_x64(x);
    println!("x64: {}", x64);  //27670116110564327424
    println!("x64 in binary: {:b}", x64); //11000000000000000000000000000000000000000000000000000000000000000
    assert_eq!(x64, 187474906725940900718118175667529106449);
}

// We have r² (the squared value) in our calculation
// We need to know: Is this value ≥ 2.0?
// The trick: In this specific context, bit 127 is "on" only if the value is ≥ 2.0
// The code: Shifts all bits right by 127 positions, leaving only bit 127 at position 0
// The result: is_r_more_than_two becomes either 1 (if ≥ 2.0) or 0 (if < 2.0)
#[test]
fn test_Q64_64_attribute(){
    // Test cases for checking bit 127 in Q64.64 format numbers
    
    // Case 1: A value where bit 127 should be 1 (≥ 2.0 when squared)
    let r1: u128 = 187474906725940900718118175667529106449;
    let is_r1_more_than_two = (r1 >> 127) as u32;
    println!("r1: {}", r1);
    println!("r1 in binary: {:b}", r1);
    println!("is_r1_more_than_two: {}", is_r1_more_than_two);
    assert_eq!(is_r1_more_than_two, 1, "Bit 127 should be 1 for this value");
    
    // Case 2: A value where bit 127 should be 0 (< 2.0 when squared)
    let r2: u128 = 103287281588899644513467040504925884921;
    let is_r2_more_than_two = (r2 >> 127) as u32;
    println!("r2: {}", r2);
    println!("r2 in binary: {:b}", r2);
    println!("is_r2_more_than_two: {}", is_r2_more_than_two);
    assert_eq!(is_r2_more_than_two, 0, "Bit 127 should be 0 for this value");
    
  
    // Explanation of what's happening:
    // In Q64.64 format, the integer part uses the upper 64 bits, and the fractional part uses the lower 64 bits
    // When checking if a value is ≥ 2.0, we're checking if bit 127 is set
    // This is because:
    // - Bit 64 represents 2^0 = 1
    // - Bit 65 represents 2^1 = 2
    // - ...and so on
    // So bit 127 represents 2^63, which in the context of Q64.64 means the value is ≥ 2.0
}


fn log2(x: f64) -> f64 {
    x.log2()  // 在Rust中，f64类型有log2()方法
}

pub fn sqrt_price_x64_to_price(price: u128, decimals_0: u8, decimals_1: u8) -> f64 {
    let mut step = from_x64_price(price);
    println!("step: {}", step);

    let result =  from_x64_price(price).powi(2);
    println!("result: {}", result);

    result * multipler(decimals_0) / multipler(decimals_1)
}

pub fn from_x64_price(price: u128) -> f64 {
    price as f64 / fixed_point_64::Q64 as f64
}

pub fn price_to_sqrt_price_x64(price: f64, decimals_0: u8, decimals_1: u8) -> u128 {
    //add logs to check inputs
    println!("price: {}", price);
    println!("decimals_0: {}", decimals_0);
    println!("decimals_1: {}", decimals_1);

    let price_with_decimals = price * multipler(decimals_1) / multipler(decimals_0);
    println!("price_with_decimals: {}", price_with_decimals);

    println!("price_with_decimals.sqrt(): {}", price_with_decimals.sqrt());

    price_to_x64(price_with_decimals.sqrt())
}

pub fn price_to_x64(price: f64) -> u128 {
    println!("price_to_x64 : {}", price);
    (price * fixed_point_64::Q64 as f64) as u128
}

pub fn multipler(decimals: u8) -> f64 {
    (10_i32).checked_pow(decimals.try_into().unwrap()).unwrap() as f64
}
