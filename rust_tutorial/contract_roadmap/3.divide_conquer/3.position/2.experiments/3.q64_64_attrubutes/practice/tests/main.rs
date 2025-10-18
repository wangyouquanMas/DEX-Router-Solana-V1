// Define fixed_point_64 module locally
mod fixed_point_64 {
    pub const Q64: u128 = 1 << 64;
}

#[test]
fn test_q64_64_attributes() {
    // Initialize r as a fixed-point value (1.5 in Q64.64 format)
    //r：9453956337776142641  -》128 -》 is r more thant two 1 
    //
    let mut r: u128 = 9453956337776142641;

    // We want to square the value of r and check for wraparound.
    // let mut squared_r = r * r;

    // Check and print the bit size of r and squared_r
    println!("Size of r in bits: {}", std::mem::size_of::<u128>() * 8);
    
    // Print the binary representation of r
        // 100000011001011100010010010011100110001100001011100010000000000000
    println!("r in binary: {:b}", r);
    println!("Number of significant bits in r: {}", 128 - r.leading_zeros());


    let msb: u32 = 128 - r.leading_zeros() - 1;
    println!("msb: {}", msb);

    r >>= msb - 63;
    //1000000110010111000100100100111001100011000010111000100000000000
    println!("r after shift: {:b}", r);

    
    let price = sqrt_price_x64_to_price(r, 6, 6);
    //price: 0.2562499999999999 也就说明 log2(r) r值在[1,2) TODO: 
    println!("price: {}", price);


    r *=r;  

    // r = 103287281588899644513467040504925884921;
    //127 : 1001101101101000110100000100111001110111000101100010000110001101111100111001101111011101100101101011100101101111001000111111001
   
    // r = 187474906725940900718118175667529106449;
    //128： 10001101000010100101101010001010000001000100110110000101101101010010001111010000100101001110100110010101001000101100010000010001
    
    // Why 127: With r normalized to [1,2), r² is in [1,4). In the squared fixed-point layout, bit 127 is the threshold for 2.0, so that bit being 1 means r² ≥ 2.
    println!("r after square: {:b}", r);
    let is_r_more_than_two = r >> 127 as u32;
    println!("is_r_more_than_two:{}",is_r_more_than_two);


    // // Print the binary representation of squared_r
    // println!("squared_r in binary: {:b}", squared_r);
    // println!("Number of significant bits in squared_r: {}", 128 - squared_r.leading_zeros());
   
}

pub fn multipler(decimals: u8) -> f64 {
    (10_i32).checked_pow(decimals.try_into().unwrap()).unwrap() as f64
}


pub fn sqrt_price_x64_to_price(price: u128, decimals_0: u8, decimals_1: u8) -> f64 {
    from_x64_price(price).powi(2) * multipler(decimals_0) / multipler(decimals_1)
}

pub fn from_x64_price(price: u128) -> f64 {
    price as f64 / fixed_point_64::Q64 as f64
}


// Add this function at the end of the file
// Function to perform full 128-bit multiplication returning high and low 128 bits
fn multiply_u128(a: u128, b: u128) -> (u128, u128) {
    // Split inputs into 64-bit chunks
    let a_lo = a as u64;
    let a_hi = (a >> 64) as u64;
    let b_lo = b as u64;
    let b_hi = (b >> 64) as u64;
    
    // Perform multiplication in parts
    let lo_lo = (a_lo as u128) * (b_lo as u128);
    let hi_lo = (a_hi as u128) * (b_lo as u128);
    let lo_hi = (a_lo as u128) * (b_hi as u128);
    let hi_hi = (a_hi as u128) * (b_hi as u128);
    
    // Combine results
    let mid = hi_lo + lo_hi;
    let carry = ((mid & 0xFFFFFFFF_FFFFFFFF) + (lo_lo >> 64)) >> 64;
    
    let low = lo_lo + (mid << 64);
    let high = hi_hi + (mid >> 64) + carry;
    
    (high, low)
}
