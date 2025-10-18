use solana_program::pubkey::Pubkey;
use std::str::FromStr;

#[test]
fn test_account_size_comparision() {
    let mut mint0 = Pubkey::from_str("7CWHJyvgu92Q9epFtV9ikcGnNWjHDPxXG815TeTob9P3").unwrap();
    let mut mint1 = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    if mint0 > mint1 {
        // std::mem::swap(&mut mint0, &mut mint1);
        //what final price is ?
        // price = 1.0 / price;
        println!("  Tokens swapped!");
    }
    println!("mint0: {}", mint0);
    println!("mint1: {}", mint1);
}