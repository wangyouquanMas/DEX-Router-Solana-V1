use std::ops::{Shl, Shr};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct U1024(pub [u64; 16]);

impl U1024 {
    pub fn new() -> Self {
        U1024([0; 16])
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }

    pub fn leading_zeros(&self) -> u32 {
        let mut count = 0;
        for i in (0..16).rev() {
            if self.0[i] == 0 {
                count += 64;
            } else {
                count += self.0[i].leading_zeros();
                break;
            }
        }
        count
    }

    pub fn trailing_zeros(&self) -> u32 {
        let mut count = 0;
        for i in 0..16 {
            if self.0[i] == 0 {
                count += 64;
            } else {
                count += self.0[i].trailing_zeros();
                break;
            }
        }
        count
    }
}

impl Default for U1024 {
    fn default() -> Self {
        U1024::new()
    }
}

impl From<[u64; 16]> for U1024 {
    fn from(value: [u64; 16]) -> Self {
        U1024(value)
    }
}

impl Shl<u32> for U1024 {
    type Output = U1024;

    fn shl(self, rhs: u32) -> Self::Output {
        let mut result = [0u64; 16];
        let shift_words = (rhs / 64) as usize;
        let shift_bits = rhs % 64;

        for i in shift_words..16 {
            let src_idx = i - shift_words;
            if src_idx < 16 {
                if shift_bits == 0 {
                    result[i] = self.0[src_idx];
                } else if src_idx + 1 < 16 {
                    result[i] = (self.0[src_idx] << shift_bits) | (self.0[src_idx + 1] >> (64 - shift_bits));
                } else {
                    result[i] = self.0[src_idx] << shift_bits;
                }
            }
        }
        U1024(result)
    }
}

impl Shr<u32> for U1024 {
    type Output = U1024;

    fn shr(self, rhs: u32) -> Self::Output {
        let mut result = [0u64; 16];
        let shift_words = (rhs / 64) as usize;
        let shift_bits = rhs % 64;

        for i in 0..(16 - shift_words) {
            let src_idx = i + shift_words;
            if src_idx < 16 {
                if shift_bits == 0 {
                    result[i] = self.0[src_idx];
                } else if src_idx > 0 {
                    result[i] = (self.0[src_idx] >> shift_bits) | (self.0[src_idx - 1] << (64 - shift_bits));
                } else {
                    result[i] = self.0[src_idx] >> shift_bits;
                }
            }
        }
        U1024(result)
    }
} 