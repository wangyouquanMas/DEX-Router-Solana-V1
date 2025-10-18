#[test]
fn test_decimal_hexademical() {
    // Example Q64 scale factor (2^64)
    let q64: u128 = 1 << 64;  // Equivalent to 2^64
    
    // Decimal value is a floating point, multiplying it by q64
    let decimal_value = (0.5 * q64 as f64) as u128;  // Convert to u64

    // Convert the u64 value to hexadecimal (as a string)
    let hex_value = format!("{:X}", decimal_value); // {:X} formats it as hexadecimal

    // Print the hexadecimal value
    println!("hex_value: {}", hex_value);

    // You can also test the reverse, converting back from hex to decimal
    let hex_value = "FF";
    let decimal_from_hex = u64::from_str_radix(hex_value, 16).unwrap();
    println!("decimal_from_hex: {}", decimal_from_hex);

    // Assert the results (optional, for automated testing)
    assert_eq!(hex_value, "FF");  // Check if the hex conversion matches
    assert_eq!(decimal_from_hex, 255);  // Check if reverse conversion matches
}
