## Next Target

- **Goal:** Finish porting the tick-array bitmap state so the pool lifecycle mirrors Raydium’s implementation.

## Current Blocker

- **Problem:** The tick-array bitmap code relies on Raydium’s math utilities (`tick_math`, `tick_array_bit_map`, `big_num`, etc.). Without those libraries, the bitmap state won’t compile or behave correctly.

Next up: port the actual tick-array state and bitmap structures so they can make use of the math utilities we just imported.

### Focus for this step
- **`TickArrayState`** and **`TickState`** from Raydium’s `states/tick_array.rs` (zero-copy layout, helpers for initialization, validation, traversal, fee/reward tracking).
- Confirm the bitmap logic we just ported (`tick_array_bit_map.rs`) compiles and integrates with these state types.

Once those structs are in place, the bitmap handling will be fully wired, and we can move on to wiring `create_pool` to initialise them.

## Way Forward

- **Solution:** First port the essential math helpers from `raydium-clmm/programs/amm/src/libraries` (starting with `tick_math` and the bitmap utilities). Once those foundations are in place, we can bring over the tick-array bitmap/extension state and hook it into our code.