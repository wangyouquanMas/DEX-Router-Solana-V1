目标：
1. 理解计算过程中涉及到的数学概念



内容：
1. 新概念

公式：  p(i) = 1.0001^(i)  => sqrt_x64(p) = sqrt(1.0001^i) * Q64.64 = 1.0001^(i/2) * Q64.64

0）数学概念
Binary Exponentiation : 将数用二进制表示 
tick = b₀×2⁰ + b₁×2¹ + b₂×2² + ... + b₁₇×2¹⁷ + ...
Where each bᵢ is either 0 or 1 (binary representation).

The formula being calculated is:
1.0001^tick = 1.0001^(b₀×2⁰) × 1.0001^(b₁×2¹) × 1.0001^(b₂×2²) × ... × 1.0001^(b₁₇×2¹⁷)
This works because:
1.0001^(a+b) = 1.0001^a × 1.0001^b

所以这里就是利用二进制+预计算来加速计算过程。


Magic number:
/// Calculates result as a U64.64
/// Each magic factor is `2^64 / (1.0001^(2^(i - 1)))` for i in `[0, 18)`.
举例：
i = 0 
>>> result = int(2**64/1.0001**2**(-1))
>>> hex(result)
'0xfffcb933bd6fb800'


println!("abs_tick: {}", abs_tick);
abs_tick: 69098
abs_tick & 0x2: 2   i = 1 
abs_tick & 0x8: 8      i = 3
abs_tick & 0x20: 32    i = 5 
abs_tick & 0x40: 64      i = 6 
abs_tick & 0x80: 128      i = 7 
abs_tick & 0x100: 256      i = 8 
abs_tick & 0x400: 1024      i = 10 
abs_tick & 0x800: 2048      i = 11 
abs_tick & 0x10000: 65536    i = 12 


进制关系：
0x1     = 1      = 2^0  = binary: 00000000000000001
0x2     = 2      = 2^1  = binary: 00000000000000010  
0x4     = 4      = 2^2  = binary: 00000000000000100
0x8     = 8      = 2^3  = binary: 00000000000001000
0x10    = 16     = 2^4  = binary: 00000000000010000
0x20    = 32     = 2^5  = binary: 00000000000100000
0x40    = 64     = 2^6  = binary: 00000000001000000
0x80    = 128    = 2^7  = binary: 00000000010000000
0x100   = 256    = 2^8  = binary: 00000000100000000
0x200   = 512    = 2^9  = binary: 00000001000000000
0x400   = 1024   = 2^10 = binary: 00000010000000000
0x800   = 2048   = 2^11 = binary: 00000100000000000
0x1000  = 4096   = 2^12 = binary: 00001000000000000
0x2000  = 8192   = 2^13 = binary: 00010000000000000
0x4000  = 16384  = 2^14 = binary: 00100000000000000
0x8000  = 32768  = 2^15 = binary: 01000000000000000
0x10000 = 65536  = 2^16 = binary: 10000000000000000
0x20000 = 131072 = 2^17 = binary: 100000000000000000
0x40000 = 262144 = 2^18 = binary: 1000000000000000000


如果 tick <0
    1.0001^(-2) =   1/1.0001^2

如果 tick >0   【invert the result】
      // Divide to obtain 1.0001^(2^(i - 1)) * 2^32 in numerator
    if tick > 0 {
        ratio = U128::MAX / ratio;
    }



1） ratio

What is ratio?
ratio is the accumulated product of magic factors that represents the intermediate calculation of 1.0001^(abs_tick/2).


What Are Magic Factors?
Magic factors are pre-calculated constants that represent specific mathematical values:
// Each magic factor = 2^64 / (1.0001^(2^(i-1)))
Magic factor 0 = 2^64 / 1.0001^1    = 2^64 / 1.0001
Magic factor 1 = 2^64 / 1.0001^2    = 2^64 / 1.0001^2  
Magic factor 2 = 2^64 / 1.0001^4    = 2^64 / 1.0001^4
Magic factor 3 = 2^64 / 1.0001^8    = 2^64 / 1.0001^8
Magic factor 4 = 2^64 / 1.0001^16   = 2^64 / 1.0001^16
// ... and so on


2) Why This Complex Approach?
1. Performance - Avoid Expensive Operations
Traditional approach would be:
// This would be extremely slow and imprecise
let result = 1.0001_f64.powf(tick as f64 / 2.0);

Problems with traditional approach:
    Floating-point arithmetic is slow on blockchain
    Precision loss for large numbers
    Non-deterministic results across different platforms
    High gas costs for complex math operations


2. Bit Manipulation is Fast
This approach uses:
// Bit operations are extremely fast
if abs_tick & 0x1 != 0 {  // Check if bit is set
    ratio = (ratio * magic_factor) >> NUM_64  // Fast multiply and shift
}
Benefits:
    Bit operations are CPU-native and fast
    No floating-point calculations
    Deterministic results
    Low gas costs

3. The Mathematical Insight:
3.1. Power of 2 Decomposition
Any integer can be written as a sum of powers of 2:
tick = 5 = 1 + 4 = 2^0 + 2^2
tick = 13 = 1 + 4 + 8 = 2^0 + 2^2 + 2^3

3.2. Exponential Property
1.0001^(a + b) = 1.0001^a * 1.0001^b
1.0001^5 = 1.0001^(1 + 4) = 1.0001^1 * 1.0001^4


3.3 Why This Design Pattern?
1. Lookup Table Approach
Instead of calculating 1.0001^x at runtime, we:
Pre-calculate all possible values for powers of 2
Store them as magic factors
Look up and multiply only the ones we need

3.4 举例
Binary Number System Advantage
tick = 5 = 0b101
We only need magic factors for bits 0 and 2:
- Bit 0: 1.0001^1
- Bit 2: 1.0001^4
Result: 1.0001^1 * 1.0001^4 = 1.0001^5