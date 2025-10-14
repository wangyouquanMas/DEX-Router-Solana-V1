// Phase 2 Study Examples - SwapArgs Construction
// These are educational examples showing how to construct SwapArgs for various scenarios

use crate::instructions::common_swap::{Dex, Route, SwapArgs};

// ============================================================================
// Example 1: Simple Single-Hop, Single-DEX Swap
// ============================================================================
// Scenario: Swap 100 USDC → SOL using only Raydium
fn example_1_simple_swap() -> SwapArgs {
    SwapArgs {
        amount_in: 100_000_000,          // 100 USDC (6 decimals)
        expect_amount_out: 1_000_000_000, // Expect ~1 SOL (9 decimals)
        min_return: 980_000_000,          // Accept min 0.98 SOL (2% slippage)
        
        amounts: vec![100_000_000],       // All amount to single path
        
        routes: vec![
            vec![
                Route {
                    dexes: vec![Dex::RaydiumSwap],
                    weights: vec![100],    // 100% through Raydium
                },
            ],
        ],
    }
}

// ============================================================================
// Example 2: Single-Hop with DEX Split (Price Optimization)
// ============================================================================
// Scenario: Swap 1000 USDC → SOL split across 3 DEXs for better price
fn example_2_split_dex() -> SwapArgs {
    SwapArgs {
        amount_in: 1_000_000_000,        // 1000 USDC
        expect_amount_out: 10_000_000_000, // Expect ~10 SOL
        min_return: 9_800_000_000,        // Min 9.8 SOL (2% slippage)
        
        amounts: vec![1_000_000_000],     // All to single path
        
        routes: vec![
            vec![
                Route {
                    dexes: vec![
                        Dex::RaydiumSwap,        // 50%
                        Dex::Whirlpool,          // 30%
                        Dex::MeteoraDynamicpool, // 20%
                    ],
                    weights: vec![50, 30, 20],    // Must sum to 100
                },
            ],
        ],
    }
}

// ============================================================================
// Example 3: CHECKPOINT - 2-Hop, 3-DEX Swap
// ============================================================================
// Scenario: USDC → SOL → BONK
//   Hop 1: USDC → SOL using 3 DEXs (Raydium 50%, Whirlpool 30%, Meteora 20%)
//   Hop 2: SOL → BONK using 1 DEX (Raydium 100%)
fn example_3_checkpoint_2hop_3dex() -> SwapArgs {
    SwapArgs {
        amount_in: 1_000_000_000,          // 1000 USDC
        expect_amount_out: 50_000_000_000,  // Expect 50k BONK
        min_return: 49_000_000_000,         // Min 49k BONK (2% slippage)
        
        amounts: vec![1_000_000_000],       // All to single path
        
        routes: vec![
            vec![
                // Hop 1: USDC → SOL (3-way split)
                Route {
                    dexes: vec![
                        Dex::RaydiumSwap,
                        Dex::Whirlpool,
                        Dex::MeteoraDynamicpool,
                    ],
                    weights: vec![50, 30, 20],
                },
                // Hop 2: SOL → BONK (single DEX)
                Route {
                    dexes: vec![Dex::RaydiumSwap],
                    weights: vec![100],
                },
            ],
        ],
    }
}

// ============================================================================
// Example 4: 3-Hop Swap (USDC → SOL → RAY → BONK)
// ============================================================================
// Scenario: Complex multi-hop routing through 3 intermediate tokens
fn example_4_triple_hop() -> SwapArgs {
    SwapArgs {
        amount_in: 1_000_000_000,          // 1000 USDC
        expect_amount_out: 100_000_000_000, // Expect 100k BONK
        min_return: 95_000_000_000,         // Min 95k BONK (5% slippage)
        
        amounts: vec![1_000_000_000],
        
        routes: vec![
            vec![
                // Hop 1: USDC → SOL
                Route {
                    dexes: vec![Dex::RaydiumSwap, Dex::Whirlpool],
                    weights: vec![60, 40],
                },
                // Hop 2: SOL → RAY
                Route {
                    dexes: vec![Dex::RaydiumSwap],
                    weights: vec![100],
                },
                // Hop 3: RAY → BONK
                Route {
                    dexes: vec![Dex::RaydiumSwap, Dex::Whirlpool],
                    weights: vec![70, 30],
                },
            ],
        ],
    }
}

// ============================================================================
// Example 5: Multi-Path Swap (2 Parallel Paths)
// ============================================================================
// Scenario: Split input across 2 different routing strategies
//   Path A: Direct USDC → BONK (30% of input)
//   Path B: USDC → SOL → BONK (70% of input)
fn example_5_multi_path() -> SwapArgs {
    SwapArgs {
        amount_in: 1_000_000_000,          // 1000 USDC total
        expect_amount_out: 50_000_000_000,  // Expect 50k BONK
        min_return: 49_000_000_000,         // Min 49k BONK
        
        // Level 1 Split: 30% to Path A, 70% to Path B
        amounts: vec![
            300_000_000,  // 300 USDC to Path A
            700_000_000,  // 700 USDC to Path B
        ],
        
        routes: vec![
            // Path A: Direct route (1 hop)
            vec![
                Route {
                    dexes: vec![Dex::RaydiumSwap, Dex::Whirlpool],
                    weights: vec![60, 40],
                },
            ],
            
            // Path B: 2-hop route
            vec![
                Route {
                    dexes: vec![
                        Dex::RaydiumSwap,
                        Dex::Whirlpool,
                        Dex::MeteoraDynamicpool,
                    ],
                    weights: vec![50, 30, 20],
                },
                Route {
                    dexes: vec![Dex::RaydiumSwap],
                    weights: vec![100],
                },
            ],
        ],
    }
}

// ============================================================================
// Example 6: Complex Multi-Path with Different Hop Counts
// ============================================================================
// Scenario: 3 parallel paths with varying hop counts
//   Path A: Direct USDC → BONK (20%)
//   Path B: USDC → SOL → BONK (50%)
//   Path C: USDC → SOL → RAY → BONK (30%)
fn example_6_complex_multi_path() -> SwapArgs {
    SwapArgs {
        amount_in: 1_000_000_000,
        expect_amount_out: 50_000_000_000,
        min_return: 48_000_000_000,
        
        amounts: vec![
            200_000_000,  // 200 USDC (20%)
            500_000_000,  // 500 USDC (50%)
            300_000_000,  // 300 USDC (30%)
        ],
        
        routes: vec![
            // Path A: 1 hop
            vec![
                Route {
                    dexes: vec![Dex::RaydiumSwap],
                    weights: vec![100],
                },
            ],
            
            // Path B: 2 hops
            vec![
                Route {
                    dexes: vec![Dex::RaydiumSwap, Dex::Whirlpool],
                    weights: vec![60, 40],
                },
                Route {
                    dexes: vec![Dex::RaydiumSwap],
                    weights: vec![100],
                },
            ],
            
            // Path C: 3 hops
            vec![
                Route {
                    dexes: vec![Dex::RaydiumSwap],
                    weights: vec![100],
                },
                Route {
                    dexes: vec![Dex::Whirlpool],
                    weights: vec![100],
                },
                Route {
                    dexes: vec![Dex::RaydiumSwap, Dex::MeteoraDynamicpool],
                    weights: vec![70, 30],
                },
            ],
        ],
    }
}

// ============================================================================
// Example 7: Stablecoin Swap with Specialized DEXs
// ============================================================================
// Scenario: USDC → USDT using stablecoin-optimized pools
fn example_7_stablecoin_swap() -> SwapArgs {
    SwapArgs {
        amount_in: 10_000_000_000,         // 10,000 USDC
        expect_amount_out: 9_990_000_000,   // Expect ~9,990 USDT (low slippage)
        min_return: 9_980_000_000,          // Min 9,980 USDT (0.2% slippage)
        
        amounts: vec![10_000_000_000],
        
        routes: vec![
            vec![
                Route {
                    dexes: vec![
                        Dex::StableSwap,         // Optimized for stablecoins
                        Dex::RaydiumStableSwap,  // Raydium's stable pool
                        Dex::StabbleSwap,        // Another stable AMM
                    ],
                    weights: vec![40, 40, 20],
                },
            ],
        ],
    }
}

// ============================================================================
// Example 8: Liquid Staking Token (LST) Swap
// ============================================================================
// Scenario: SOL → mSOL (Marinade staked SOL) using Sanctum router
fn example_8_lst_swap() -> SwapArgs {
    SwapArgs {
        amount_in: 100_000_000_000,        // 100 SOL
        expect_amount_out: 95_000_000_000,  // Expect ~95 mSOL
        min_return: 94_000_000_000,         // Min 94 mSOL
        
        amounts: vec![100_000_000_000],
        
        routes: vec![
            vec![
                Route {
                    dexes: vec![
                        Dex::SanctumRouter,      // Specialized LST router
                        Dex::MeteoraLst,         // Meteora LST pool
                    ],
                    weights: vec![70, 30],
                },
            ],
        ],
    }
}

// ============================================================================
// Example 9: Meme Token Swap (Pump.fun)
// ============================================================================
// Scenario: SOL → MEME using Pump.fun bonding curve
fn example_9_meme_token() -> SwapArgs {
    SwapArgs {
        amount_in: 1_000_000_000,          // 1 SOL
        expect_amount_out: 1_000_000_000_000, // Expect 1M MEME tokens
        min_return: 900_000_000_000,        // Min 900k (10% slippage - volatile)
        
        amounts: vec![1_000_000_000],
        
        routes: vec![
            vec![
                Route {
                    dexes: vec![Dex::PumpfunBuy],
                    weights: vec![100],
                },
            ],
        ],
    }
}

// ============================================================================
// Example 10: Maximum Complexity - 4 Paths, Mixed Hops
// ============================================================================
// Scenario: Demonstration of the router's full flexibility
fn example_10_max_complexity() -> SwapArgs {
    SwapArgs {
        amount_in: 10_000_000_000,         // 10,000 USDC
        expect_amount_out: 500_000_000_000, // Expect 500k BONK
        min_return: 475_000_000_000,        // Min 475k BONK (5% slippage)
        
        // 4 different paths with varying allocations
        amounts: vec![
            2_000_000_000,  // Path A: 20%
            3_000_000_000,  // Path B: 30%
            2_500_000_000,  // Path C: 25%
            2_500_000_000,  // Path D: 25%
        ],
        
        routes: vec![
            // Path A: Direct (if pool exists)
            vec![
                Route {
                    dexes: vec![Dex::RaydiumSwap],
                    weights: vec![100],
                },
            ],
            
            // Path B: 2-hop conservative
            vec![
                Route {
                    dexes: vec![Dex::RaydiumSwap],
                    weights: vec![100],
                },
                Route {
                    dexes: vec![Dex::RaydiumSwap],
                    weights: vec![100],
                },
            ],
            
            // Path C: 2-hop with splits
            vec![
                Route {
                    dexes: vec![
                        Dex::RaydiumSwap,
                        Dex::Whirlpool,
                        Dex::MeteoraDynamicpool,
                    ],
                    weights: vec![40, 40, 20],
                },
                Route {
                    dexes: vec![Dex::RaydiumSwap, Dex::Whirlpool],
                    weights: vec![60, 40],
                },
            ],
            
            // Path D: 3-hop experimental
            vec![
                Route {
                    dexes: vec![Dex::RaydiumSwap, Dex::Whirlpool],
                    weights: vec![50, 50],
                },
                Route {
                    dexes: vec![Dex::RaydiumClmmSwap],
                    weights: vec![100],
                },
                Route {
                    dexes: vec![
                        Dex::RaydiumSwap,
                        Dex::MeteoraDlmm,
                    ],
                    weights: vec![70, 30],
                },
            ],
        ],
    }
}

// ============================================================================
// Validation Helper Functions
// ============================================================================

/// Validates that a SwapArgs is correctly structured
fn validate_swap_args(args: &SwapArgs) -> Result<(), String> {
    // Check amounts and routes length match
    if args.amounts.len() != args.routes.len() {
        return Err(format!(
            "amounts.len() ({}) != routes.len() ({})",
            args.amounts.len(),
            args.routes.len()
        ));
    }
    
    // Check total amounts equal amount_in
    let total_amounts: u64 = args.amounts.iter().sum();
    if total_amounts != args.amount_in {
        return Err(format!(
            "Sum of amounts ({}) != amount_in ({})",
            total_amounts,
            args.amount_in
        ));
    }
    
    // Check expect_amount_out >= min_return
    if args.expect_amount_out < args.min_return {
        return Err(format!(
            "expect_amount_out ({}) < min_return ({})",
            args.expect_amount_out,
            args.min_return
        ));
    }
    
    // Check each route
    for (i, hops) in args.routes.iter().enumerate() {
        for (j, route) in hops.iter().enumerate() {
            // Check dexes and weights length match
            if route.dexes.len() != route.weights.len() {
                return Err(format!(
                    "Route[{}][{}]: dexes.len() ({}) != weights.len() ({})",
                    i, j, route.dexes.len(), route.weights.len()
                ));
            }
            
            // Check weights sum to 100
            let total_weight: u8 = route.weights.iter().sum();
            if total_weight != 100 {
                return Err(format!(
                    "Route[{}][{}]: weights sum to {} (expected 100)",
                    i, j, total_weight
                ));
            }
        }
    }
    
    Ok(())
}

/// Pretty-print a SwapArgs for debugging
fn print_swap_args(args: &SwapArgs) {
    println!("SwapArgs:");
    println!("  amount_in: {}", args.amount_in);
    println!("  expect_amount_out: {}", args.expect_amount_out);
    println!("  min_return: {}", args.min_return);
    println!("  slippage tolerance: {:.2}%", 
        (args.expect_amount_out - args.min_return) as f64 / args.expect_amount_out as f64 * 100.0
    );
    
    println!("\n  Paths ({}):", args.amounts.len());
    for (i, (amount, hops)) in args.amounts.iter().zip(&args.routes).enumerate() {
        let percentage = (*amount as f64 / args.amount_in as f64) * 100.0;
        println!("    Path {}: {} ({:.1}%)", i, amount, percentage);
        
        for (j, route) in hops.iter().enumerate() {
            println!("      Hop {}: {} DEXs", j, route.dexes.len());
            for (k, (dex, weight)) in route.dexes.iter().zip(&route.weights).enumerate() {
                println!("        [{:2}%] {:?}", weight, dex);
            }
        }
    }
}

// ============================================================================
// Test Runner
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_all_examples() {
        let examples = vec![
            ("Simple Swap", example_1_simple_swap()),
            ("Split DEX", example_2_split_dex()),
            ("Checkpoint 2-Hop 3-DEX", example_3_checkpoint_2hop_3dex()),
            ("Triple Hop", example_4_triple_hop()),
            ("Multi-Path", example_5_multi_path()),
            ("Complex Multi-Path", example_6_complex_multi_path()),
            ("Stablecoin Swap", example_7_stablecoin_swap()),
            ("LST Swap", example_8_lst_swap()),
            ("Meme Token", example_9_meme_token()),
            ("Max Complexity", example_10_max_complexity()),
        ];
        
        for (name, args) in examples {
            println!("\n{'='*60}");
            println!("Testing: {}", name);
            println!("{'='*60}");
            
            match validate_swap_args(&args) {
                Ok(_) => {
                    println!("✅ Validation PASSED");
                    print_swap_args(&args);
                },
                Err(e) => {
                    println!("❌ Validation FAILED: {}", e);
                    panic!("Example {} failed validation", name);
                }
            }
        }
    }
}

