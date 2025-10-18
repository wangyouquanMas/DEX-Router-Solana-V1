

/// Calculate the logarithm of a number in a given base using the change of base formula.
/// 
/// The change of base formula is: log_b(x) = ln(x) / ln(b)
/// where ln is the natural logarithm.
/// 
/// # Arguments
/// * `number` - The number to find the logarithm of (must be positive)
/// * `base` - The base of the logarithm (must be positive and not equal to 1)
/// 
/// # Returns
/// * `Ok(f64)` - The logarithm value if successful
/// * `Err` - If the inputs are invalid (non-positive numbers or base = 1)
pub fn change_of_base(number: f64, base: f64) -> Result<f64> {
    // Check for valid inputs
    if number <= 0.0 {
        return Err(anyhow!("Number must be positive"));
    }
    if base <= 0.0 || base == 1.0 {
        return Err(anyhow!("Base must be positive and not equal to 1"));
    }
    
    // Apply change of base formula: log_b(x) = ln(x) / ln(b)
    let result = number.ln() / base.ln();
    Ok(result)
}

/// Alternative implementation using base-10 logarithm
/// 
/// The change of base formula is: log_b(x) = log_10(x) / log_10(b)
/// 
/// # Arguments
/// * `number` - The number to find the logarithm of (must be positive)
/// * `base` - The base of the logarithm (must be positive and not equal to 1)
/// 
/// # Returns
/// * `Ok(f64)` - The logarithm value if successful
/// * `Err` - If the inputs are invalid (non-positive numbers or base = 1)
pub fn change_of_base_log10(number: f64, base: f64) -> Result<f64> {
    // Check for valid inputs
    if number <= 0.0 {
        return Err(anyhow!("Number must be positive"));
    }
    if base <= 0.0 || base == 1.0 {
        return Err(anyhow!("Base must be positive and not equal to 1"));
    }
    
    // Apply change of base formula: log_b(x) = log_10(x) / log_10(b)
    let result = number.log10() / base.log10();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change_of_base_basic() {
        // Test log_2(8) = 3
        let result = change_of_base(8.0, 2.0).unwrap();
        assert!((result - 3.0).abs() < 1e-10);
        
        // Test log_3(27) = 3
        let result = change_of_base(27.0, 3.0).unwrap();
        assert!((result - 3.0).abs() < 1e-10);
        
        // Test log_10(1000) = 3
        let result = change_of_base(1000.0, 10.0).unwrap();
        assert!((result - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_change_of_base_log10() {
        // Test log_2(8) = 3 using log10 method
        let result = change_of_base_log10(8.0, 2.0).unwrap();
        assert!((result - 3.0).abs() < 1e-10);
        
        // Test log_5(125) = 3 using log10 method
        let result = change_of_base_log10(125.0, 5.0).unwrap();
        assert!((result - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_change_of_base_errors() {
        // Test negative number
        assert!(change_of_base(-5.0, 2.0).is_err());
        
        // Test zero number
        assert!(change_of_base(0.0, 2.0).is_err());
        
        // Test invalid base (1)
        assert!(change_of_base(10.0, 1.0).is_err());
        
        // Test negative base
        assert!(change_of_base(10.0, -2.0).is_err());
        
        // Test zero base
        assert!(change_of_base(10.0, 0.0).is_err());
    }

    #[test]
    fn test_change_of_base_precision() {
        // Test log_e(e) = 1
        let result = change_of_base(2.718281828459045, 2.718281828459045).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
        
        // Test log_2(16) = 4
        let result = change_of_base(16.0, 2.0).unwrap();
        assert!((result - 4.0).abs() < 1e-10);
    }
}
