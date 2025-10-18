/// Converts a decimal fractional number to binary as a string.
fn decimal_to_binary_fraction(decimal: f64, precision: usize) -> String {
    let mut binary_fraction = String::from(".");

    let mut value = decimal;
    for _ in 0..precision {
        value *= 2.0;
        if value >= 1.0 {
            binary_fraction.push('1');
            value -= 1.0;
        } else {
            binary_fraction.push('0');
        }
    }

    binary_fraction
}

#[test]
fn test_fractional_binary_representation() {
    // Test case 1: Convert 0.625 to binary
    let binary = decimal_to_binary_fraction(0.625, 3);
    println!("binary: {}", binary);
    assert_eq!(binary, ".101");
}
