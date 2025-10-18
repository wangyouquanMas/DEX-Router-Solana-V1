use bytemuck::{Pod, Zeroable};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[test]
fn test_account_size_comparision() {
    let mut mint0 = Pubkey::from_str("DMADF2pAJU7bE98LCHt3L5B336cRbHssyxTT3gQWktNX").unwrap();
    let mut mint1 = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();

    //help convert 7CWHJyvgu92Q9epFtV9ikcGnNWjHDPxXG815TeTob9P3 and So11111111111111111111111111111111111111112 to binary

    if mint0 > mint1 {
        // std::mem::swap(&mut mint0, &mut mint1);
        //what final price is ?
        // price = 1.0 / price;
        println!("  Tokens swapped!");
    }
    println!("mint0: {}", mint0);
    println!("mint1: {}", mint1);
}

#[test]
fn test_mint_address_comparison() {
    // The two mint addresses from your example
    let mint0_str = "DMADF2pAJU7bE98LCHt3L5B336cRbHssyxTT3gQWktNX";
    let mint1_str = "So11111111111111111111111111111111111111112";
    
    let mint0 = Pubkey::from_str(mint0_str).unwrap();
    let mint1 = Pubkey::from_str(mint1_str).unwrap();
    
    println!("=== Mint Address Binary Comparison ===");
    println!("Mint0: {}", mint0_str);
    println!("Mint1: {}", mint1_str);
    println!();
    
    // Convert to byte arrays
    let mint0_bytes = mint0.to_bytes();
    let mint1_bytes = mint1.to_bytes();
    
    println!("Mint0 bytes (hex): {:?}", mint0_bytes);
    println!("Mint1 bytes (hex): {:?}", mint1_bytes);
    println!();
    
    // Show binary representation
    println!("Mint0 binary:");
    for (i, &byte) in mint0_bytes.iter().enumerate() {
        println!("  Byte {}: {:08b} (0x{:02x})", i, byte, byte);
    }
    println!();
    
    println!("Mint1 binary:");
    for (i, &byte) in mint1_bytes.iter().enumerate() {
        println!("  Byte {}: {:08b} (0x{:02x})", i, byte, byte);
    }
    println!();
    
    // Compare byte by byte
    println!("=== Byte-by-Byte Comparison ===");
    let mut comparison_result = std::cmp::Ordering::Equal;
    
    for (i, (byte0, byte1)) in mint0_bytes.iter().zip(mint1_bytes.iter()).enumerate() {
        let order = byte0.cmp(byte1);
        let symbol = match order {
            std::cmp::Ordering::Less => "<",
            std::cmp::Ordering::Equal => "=",
            std::cmp::Ordering::Greater => ">",
        };
        
        println!("  Byte {}: 0x{:02x} {} 0x{:02x} ({:08b} {} {:08b})", 
                 i, byte0, symbol, byte1, byte0, symbol, byte1);
        
        if comparison_result == std::cmp::Ordering::Equal {
            comparison_result = order;
        }
    }
    println!();
    
    // Show the comparison result
    let comparison_symbol = match comparison_result {
        std::cmp::Ordering::Less => "<",
        std::cmp::Ordering::Equal => "=",
        std::cmp::Ordering::Greater => ">",
    };
    
    println!("=== Final Result ===");
    println!("mint0 {} mint1", comparison_symbol);
    println!("mint0.to_bytes() {} mint1.to_bytes()", comparison_symbol);
    
    if mint0 > mint1 {
        println!("✅ mint0 > mint1 - Tokens will be swapped!");
        println!("  Original mint0 becomes final mint1");
        println!("  Original mint1 becomes final mint0");
    } else if mint0 < mint1 {
        println!("✅ mint0 < mint1 - No swapping needed");
    } else {
        println!("⚠️  mint0 == mint1 - This shouldn't happen for different tokens");
    }
    
    // Demonstrate the swap logic
    let mut final_mint0 = mint0;
    let mut final_mint1 = mint1;
    let mut swapped = false;
    
    if mint0 > mint1 {
        std::mem::swap(&mut final_mint0, &mut final_mint1);
        swapped = true;
    }
    
    println!();
    println!("=== Final Token Order ===");
    println!("Final mint0: {}", final_mint0);
    println!("Final mint1: {}", final_mint1);
    if swapped {
        println!("🔄 Tokens were swapped to maintain correct ordering");
    } else {
        println!("✅ Tokens were already in correct order");
    }
}