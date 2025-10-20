use anchor_lang::prelude::*;

/// This function logs basic swap information to the Solana transaction logs.
/// 
/// **Purpose:**
/// - Records identifying information about a token swap operation
/// - Provides an audit trail for debugging and monitoring
/// - Helps track which tokens and accounts are involved in a swap
/// 
/// **What gets logged:**
/// 1. order_id - Unique identifier for the order (only if > 0)
/// 2. source_mint - The token being swapped FROM
/// 3. destination_mint - The token being swapped TO
/// 4. source_owner - Owner of the source token account
/// 5. destination_owner - Owner of the destination token account
/// 
/// **How Pubkey::log() works:**
/// When you call `.log()` on a Pubkey:
/// - On Solana blockchain: uses syscall `sol_log_pubkey()` to write to transaction logs
/// - In test environment: prints the pubkey to stdout
pub fn log_swap_basic_info(
    order_id: u64,
    source_mint: &Pubkey,
    destination_mint: &Pubkey,
    source_owner: &Pubkey,
    destination_owner: &Pubkey,
) {
    // Only log order_id if it's greater than 0 (0 might mean no order tracking)
    if order_id > 0 {
        msg!("order_id: {}", order_id);
    }
    
    // Log each pubkey - these calls write to the Solana transaction logs
    source_mint.log();
    destination_mint.log();
    source_owner.log();
    destination_owner.log();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_swap_basic_info() {
        // Create some example pubkeys for testing
        // In real usage, these would be actual token mint addresses and owner addresses
        let order_id = 12345u64;
        let source_mint = Pubkey::new_unique();  // e.g., USDC mint address
        let destination_mint = Pubkey::new_unique();  // e.g., SOL mint address
        let source_owner = Pubkey::new_unique();  // User's wallet address
        let destination_owner = Pubkey::new_unique();  // Could be same as source_owner

        println!("\n=== Testing log_swap_basic_info ===");
        println!("This function demonstrates logging in Solana programs");
        println!("\nCalling log_swap_basic_info with:");
        println!("  order_id: {}", order_id);
        println!("  source_mint: {}", source_mint);
        println!("  destination_mint: {}", destination_mint);
        println!("  source_owner: {}", source_owner);
        println!("  destination_owner: {}", destination_owner);
        println!("\n--- Function Output (logs) ---");

        // Call the logging function
        // In a real Solana program, this would write to transaction logs
        // In tests, it prints to stdout
        log_swap_basic_info(
            order_id,
            &source_mint,
            &destination_mint,
            &source_owner,
            &destination_owner,
        );

        println!("--- End of logs ---\n");
        
        // The function doesn't return anything, it just logs
        // In production, you'd view these logs in a Solana explorer or using `solana logs`
    }
}

