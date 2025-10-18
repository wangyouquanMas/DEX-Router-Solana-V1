#[test]
fn test_msb() {
    let sqrt_price_x64: u128 = 553402322211286528; 
    
    let msb: u32 = 128 - sqrt_price_x64.leading_zeros() - 1;
    println!("msb: {}", msb);

    let mut r = if msb >= 64 {
        sqrt_price_x64 >> (msb - 63)
    } else {
        sqrt_price_x64 << (63 - msb)
    };

    let msb_2: u32 = 128 - r.leading_zeros() - 1;
    println!("msb_2: {}", msb_2);

    println!("r: {}", r);
}

