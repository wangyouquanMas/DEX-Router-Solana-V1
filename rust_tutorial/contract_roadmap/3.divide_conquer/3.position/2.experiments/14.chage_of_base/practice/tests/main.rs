use practice::{change_of_base, change_of_base_log10};

fn main() {
    println!("Change of Base Calculator Examples\n");
    
    // Example 1: Calculate log_2(8)
    match change_of_base(8.0, 2.0) {
        Ok(result) => println!("log_2(8) = {:.6}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example 2: Calculate log_3(27)
    match change_of_base(27.0, 3.0) {
        Ok(result) => println!("log_3(27) = {:.6}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example 3: Calculate log_10(1000)
    match change_of_base(1000.0, 10.0) {
        Ok(result) => println!("log_10(1000) = {:.6}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example 4: Using the log10 version
    match change_of_base_log10(1000.0, 10.0) {
        Ok(result) => println!("log_10(1000) using log10 method = {:.6}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example 5: Calculate log_5(125)
    match change_of_base(125.0, 5.0) {
        Ok(result) => println!("log_5(125) = {:.6}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example 6: Error case - negative number
    match change_of_base(-5.0, 2.0) {
        Ok(result) => println!("log_2(-5) = {:.6}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example 7: Error case - invalid base
    match change_of_base(10.0, 1.0) {
        Ok(result) => println!("log_1(10) = {:.6}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example 8: Calculate log_e(2.718) ≈ 1
    match change_of_base(2.718, 2.718) {
        Ok(result) => println!("log_e(2.718) = {:.6}", result),
        Err(e) => println!("Error: {}", e),
    }
}
