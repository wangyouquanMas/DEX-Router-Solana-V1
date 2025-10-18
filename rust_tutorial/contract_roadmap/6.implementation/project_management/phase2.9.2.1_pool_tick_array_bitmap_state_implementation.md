### Progress

- Added the two zero-copy structs in `state/tick_array.rs`:
  - `TickArrayState` (pool id, start index, tick slots, init count, epoch, padding) with init/update helpers.
  - `TickState` (tick metadata, liquidity net/gross, fee growth fields) with update/clear logic.

- Brought in constants (`TICK_ARRAY_SEED`, `TICK_ARRAY_SIZE_USIZE`, etc.) and basic validation helpers (`tick_count`, `check_is_valid_start_index`) that the bitmap math relies on.

### Notes

This is an initial pass—liquidity math currently uses a simplified helper, and reward/fee tracking is stubbed (Raydium includes richer bookkeeping). Tests and integration with the ported bitmap logic still need to be wired up next.