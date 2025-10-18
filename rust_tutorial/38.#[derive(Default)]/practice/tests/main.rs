extern crate arrayref;

use arrayref::array_ref;

#[derive(Default)]
struct StepComputations {
    // the price at the beginning of the step
    sqrt_price_start_x64: u128,
    // the next tick to swap to from the current tick in the swap direction
    tick_next: i32,
    // whether tick_next is initialized or not
    initialized: bool,
    // sqrt(price) for the next tick (1/0)
    sqrt_price_next_x64: u128,
    // how much is being swapped in in this step
    amount_in: u64,
    // how much is being swapped out
    amount_out: u64,
    // how much fee is being paid in
    fee_amount: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_ref_usage() {
        let mut step = StepComputations::default();
        println!("step: {:?}", step.tick_next);
        println!("initialized: {:?}", step.initialized);
    }
}
