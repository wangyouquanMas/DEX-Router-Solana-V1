目标：
1. 掌握next_initialized_tick_array_start_index的计算方法
2. Bitmap 是如何帮助查询next initialized tick array start index.

内容：
1. 计算方法

1) 核心要点
a. Bitmap-Based State Tracking
The core mechanism uses bitmaps to efficiently track which tick arrays are initialized:

// Primary bitmap in pool state (1024 bits)
U1024(self.tick_array_bitmap)
// Extension bitmap for additional ranges (7,168 bits)
tickarray_bitmap_extension.next_initialized_tick_array_from_one_bitmap()

Why this matters:
Instead of scanning through all possible tick arrays sequentially
Each bit represents whether a tick array at that position is initialized
O(1) lookup instead of O(n) search through all tick arrays
Enables fast discovery of the next available liquidity position


b.Directional Search with Tick Spacing Alignment
The search follows a directional pattern that respects tick array boundaries:
// Normalize to valid tick array start index
last_tick_array_start_index = TickArrayState::get_array_start_index(
    last_tick_array_start_index, 
    self.tick_spacing
);

// Search direction based on swap direction
let (is_found, start_index) = tick_array_bit_map::next_initialized_tick_array_start_index(
    bitmap,
    last_tick_array_start_index,
    self.tick_spacing,  // Ensures alignment
    zero_for_one,       // Determines search direction
);

Why this matters:
Tick arrays have fixed sizes (60 ticks per array)
Must align with tick_spacing boundaries
Directional search (left/right) based on swap direction
Predictable progression through the tick space



2) The Core Algorithm Flow 
The Core Algorithm Flow:
Check bitmap bit → Is this tick array initialized?
If yes → Return the tick array start index
If no → Move to next position in the search direction
Repeat until finding an initialized tick array or hitting boundaries
These two concepts work together to provide efficient, predictable, and scalable tick array discovery in AMM operations.



2. next initialized tick array start index 

2.1) Bit Position Compression & Decompression
Compression (Tick Index → Bit Position)

let multiplier = i32::from(tick_spacing) * TICK_ARRAY_SIZE;  // tick_spacing * 60
let mut compressed = next_tick_array_start_index / multiplier + 512;
if next_tick_array_start_index < 0 && next_tick_array_start_index % multiplier != 0 {
    compressed -= 1;  // Round towards negative infinity
}
let bit_pos = compressed.abs();

Math Concept:
    Linear mapping from tick space to bitmap space
    Offset by 512: Centers the bitmap around tick index 0
    Division by multiplier: Groups ticks into array-sized chunks
    Negative handling: Ensures proper rounding for negative tick indices

Example:
    If tick_spacing = 1, multiplier = 60
    Tick index 120 → (120 / 60) + 512 = 2 + 512 = 514
    Tick index -120 → (-120 / 60) + 512 = -2 + 512 = 510

2.2) Bit Manipulation for Directional Search
Zero-for-One (Upper to Lower)

// tick from upper to lower - find from higher bits to lower bits
let offset_bit_map = bit_map << (1024 - bit_pos - 1).try_into().unwrap();
let next_bit = most_significant_bit(offset_bit_map);
if next_bit.is_some() {
    let next_array_start_index = (bit_pos - i32::from(next_bit.unwrap()) - 512) * multiplier;
    (true, next_array_start_index)
}

Math Concept:
    Left shift by (1024 - bit_pos - 1): Aligns the search position
    Most significant bit (MSB): Finds the highest set bit in the remaining range
    Decompression: (bit_pos - bit_offset - 512) * multiplier converts back to tick space


One-for-Zero (Lower to Upper)
    // tick from lower to upper - find from lower bits to higher bits
    let offset_bit_map = bit_map >> (bit_pos).try_into().unwrap();
    let next_bit = least_significant_bit(offset_bit_map);
    if next_bit.is_some() {
        let next_array_start_index = (bit_pos + i32::from(next_bit.unwrap()) - 512) * multiplier;
        (true, next_array_start_index)
    }
Math Concept:
    Right shift by bit_pos: Aligns the search position
    Least significant bit (LSB): Finds the lowest set bit in the remaining range
    Decompression: (bit_pos + bit_offset - 512) * multiplier converts back to tick space

2.3) 实例
I'll walk through a concrete example to show how the bitmap works in finding the next initialized tick array start index.

## **Example: Finding Next Initialized Tick Array**

### **Setup**
- **Tick spacing**: 1 (each tick = 1 unit)
- **TICK_ARRAY_SIZE**: 60 (each array covers 60 ticks)
- **Multiplier**: `1 * 60 = 60`
- **Bitmap**: 1024 bits representing tick arrays from -512×60 to +511×60

### **Scenario**
Let's say we're at **tick array start index 120** and want to find the **next initialized tick array** when swapping **token1 for token0** (one_for_zero = false, so we search upward).

### **Step-by-Step Process**

#### **1. Current Position Analysis**
```
Current tick array start index: 120
Search direction: upward (one_for_zero = false)
```

#### **2. Calculate Next Position**
```rust
next_tick_array_start_index = last_tick_array_start_index + TickArrayState::tick_count(tick_spacing)
next_tick_array_start_index = 120 + 60 = 180
```

#### **3. Compress to Bit Position**
```rust
let multiplier = 60;
let compressed = next_tick_array_start_index / multiplier + 512;
compressed = 180 / 60 + 512 = 3 + 512 = 515
let bit_pos = 515
```

**What this means:**
- Tick index 180 corresponds to bit position 515 in the bitmap
- Bit position 515 = (515 - 512) × 60 = 3 × 60 = 180 ticks

#### **4. Bitmap Search**
```rust
// Right shift to align search position
let offset_bit_map = bit_map >> 515;
let next_bit = least_significant_bit(offset_bit_map);
```

**Visual Representation:**
```
Original bitmap: [bit_0, bit_1, ..., bit_515, bit_516, bit_517, ...]
After >> 515:   [bit_515, bit_516, bit_517, ...]
```

#### **5. Find Next Set Bit**
Let's say the bitmap looks like this after shifting:
```
[0, 1, 0, 0, 1, 0, 0, 0, ...]
 ^  ^     ^
515 516   519
```

**Finding LSB:**
- `least_significant_bit()` finds the **first set bit** (bit 516)
- This means tick array starting at position 516 is initialized

#### **6. Decompress Back to Tick Space**
```rust
let next_array_start_index = (bit_pos + i32::from(next_bit.unwrap()) - 512) * multiplier;
next_array_start_index = (515 + 1 - 512) * 60 = 4 * 60 = 240
```

**Verification:**
- Bit position 516 = (516 - 512) × 60 = 4 × 60 = 240 ticks
- ✅ Correct!

### **Complete Example with Real Numbers**

#### **Initial State**
```
Current position: tick array starting at 120
Bitmap at position 515: [0, 1, 0, 0, 1, 0, 0, 0, ...]
                        ^  ^     ^
                       515 516   519
```

#### **Search Process**
1. **Move to next position**: 120 + 60 = 180
2. **Compress 180**: (180 / 60) + 512 = 515
3. **Right shift bitmap**: `bit_map >> 515`
4. **Find LSB**: First set bit is at offset 1 from position 515
5. **Decompress**: (515 + 1 - 512) × 60 = 240

#### **Result**
```
Found next initialized tick array at: 240
This tick array covers ticks: [240, 241, 242, ..., 299]
```

### **Why This is Efficient**

#### **Traditional Approach (Sequential)**
```
Check tick array 180: Not initialized
Check tick array 240: Initialized ✓
```
**Complexity**: O(n) where n = number of tick arrays to check

#### **Bitmap Approach**
```
1. Compress position: O(1) arithmetic
2. Bit shift: O(1) operation  
3. Find LSB: O(1) hardware operation
4. Decompress: O(1) arithmetic
```
**Complexity**: O(1) for each bitmap search

### **Key Insights**

1. **Compression**: Maps continuous tick space (120, 180, 240, 300...) to discrete bitmap positions (515, 516, 517, 518...)

2. **Bit Manipulation**: Uses hardware-accelerated operations to find the next set bit

3. **Directional Search**: 
   - **Upward search**: Right shift + LSB (finds next higher initialized array)
   - **Downward search**: Left shift + MSB (finds next lower initialized array)

4. **Memory Efficiency**: 1024 bits can track 1024 tick arrays, covering a massive tick range

This bitmap approach transforms what would be a **linear scan through potentially thousands of tick arrays** into a **constant-time bit manipulation operation**, making AMM operations extremely fast regardless of pool size.