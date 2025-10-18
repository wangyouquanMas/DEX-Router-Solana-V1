#[test]
fn test_right_left_shift_binary() {
    // Test case 1: Convert 0.625 to binary
    let mut y = 8;
    y >>=1;
    println!("y: {}", y);
    y <<= 1;
    println!("y: {}", y);

    let msb = y>>3;
    println!("msb: {}", msb);
}
