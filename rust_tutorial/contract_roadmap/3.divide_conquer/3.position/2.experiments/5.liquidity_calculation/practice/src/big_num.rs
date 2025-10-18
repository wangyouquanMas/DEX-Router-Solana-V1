use std::ops::{Add, Mul, Sub, Div, Rem};

/// Trait for calculating `val * num / denom` with different rounding modes and overflow
/// protection.
pub trait MulDiv<RHS = Self> {
    type Output;
    fn mul_div_floor(self, num: Self, denom: Self) -> Option<Self::Output>;
    fn mul_div_ceil(self, num: Self, denom: Self) -> Option<Self::Output>;
    fn to_underflow_u64(self) -> u64;
}

pub trait Upcast256 {
    fn as_u256(self) -> U256;
}

pub trait Downcast256 {
    fn as_u128(self) -> U128;
}

pub trait Upcast512 {
    fn as_u512(self) -> U512;
}

pub trait Downcast512 {
    fn as_u256(self) -> U256;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct U128([u64; 2]);

impl U128 {
    pub const MAX: U128 = U128([u64::MAX, u64::MAX]);
    
    pub fn new(value: u128) -> Self {
        U128([(value & 0xFFFFFFFFFFFFFFFF) as u64, (value >> 64) as u64])
    }

    pub fn as_u128(self) -> u128 {
        (self.0[1] as u128) << 64 | (self.0[0] as u128)
    }

    pub fn as_u64(self) -> u64 {
        self.0[0]
    }
}

impl From<u128> for U128 {
    fn from(value: u128) -> Self {
        U128::new(value)
    }
}

impl From<u64> for U128 {
    fn from(value: u64) -> Self {
        U128([value, 0])
    }
}

impl Add for U128 {
    type Output = U128;
    
    fn add(self, other: U128) -> U128 {
        let mut result = [0u64; 2];
        let mut carry = 0u64;
        
        for i in 0..2 {
            let sum = self.0[i] as u128 + other.0[i] as u128 + carry as u128;
            result[i] = (sum & 0xFFFFFFFFFFFFFFFF) as u64;
            carry = (sum >> 64) as u64;
        }
        
        U128(result)
    }
}

impl Sub for U128 {
    type Output = U128;
    
    fn sub(self, other: U128) -> U128 {
        let mut result = [0u64; 2];
        let mut borrow = 0i128;
        
        for i in 0..2 {
            let diff = self.0[i] as i128 - other.0[i] as i128 - borrow;
            result[i] = (diff & 0xFFFFFFFFFFFFFFFF) as u64;
            borrow = if diff < 0 { 1 } else { 0 };
        }
        
        U128(result)
    }
}

impl Mul for U128 {
    type Output = U128;
    
    fn mul(self, other: U128) -> U128 {
        let mut result = [0u64; 2];
        
        for i in 0..2 {
            for j in 0..2 {
                let product = self.0[i] as u128 * other.0[j] as u128;
                let pos = i + j;
                if pos < 2 {
                    let sum = result[pos] as u128 + product;
                    result[pos] = (sum & 0xFFFFFFFFFFFFFFFF) as u64;
                    if pos + 1 < 2 {
                        result[pos + 1] += (sum >> 64) as u64;
                    }
                }
            }
        }
        
        U128(result)
    }
}

impl Div for U128 {
    type Output = U128;
    
    fn div(self, other: U128) -> U128 {
        // Simple division implementation for now
        let self_val = self.as_u128();
        let other_val = other.as_u128();
        if other_val == 0 {
            return U128::default();
        }
        U128::from(self_val / other_val)
    }
}

impl Rem for U128 {
    type Output = U128;
    
    fn rem(self, other: U128) -> U128 {
        let self_val = self.as_u128();
        let other_val = other.as_u128();
        if other_val == 0 {
            return U128::default();
        }
        U128::from(self_val % other_val)
    }
}

impl Upcast256 for U128 {
    fn as_u256(self) -> U256 {
        U256([self.0[0], self.0[1], 0, 0])
    }
}

impl MulDiv for U128 {
    type Output = U128;

    fn mul_div_floor(self, num: Self, denom: Self) -> Option<Self::Output> {
        assert_ne!(denom, U128::default());
        let r = ((self.as_u256()) * (num.as_u256())) / (denom.as_u256());
        if r > U128::MAX.as_u256() {
            None
        } else {
            Some(r.as_u128())
        }
    }

    fn mul_div_ceil(self, num: Self, denom: Self) -> Option<Self::Output> {
        assert_ne!(denom, U128::default());
        let r = (self.as_u256() * num.as_u256() + (denom - U128::from(1u64)).as_u256()) / denom.as_u256();
        if r > U128::MAX.as_u256() {
            None
        } else {
            Some(r.as_u128())
        }
    }

    fn to_underflow_u64(self) -> u64 {
        if self < U128::from(u64::MAX) {
            self.as_u64()
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct U256([u64; 4]);

impl U256 {
    pub const MAX: U256 = U256([u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
    
    pub fn new(value: u256::U256) -> Self {
        // Convert from u256 crate to our U256
        // Since u256::U256 doesn't have to_le_bytes, we'll use a different approach
        let mut result = [0u64; 4];
        // For now, just use the first 64 bits
        result[0] = value.try_into().unwrap_or(0);
        U256(result)
    }

    pub fn as_u64(self) -> u64 {
        self.0[0]
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        U256([value, 0, 0, 0])
    }
}

impl From<u128> for U256 {
    fn from(value: u128) -> Self {
        let low = (value & 0xFFFFFFFFFFFFFFFF) as u64;
        let high = (value >> 64) as u64;
        U256([low, high, 0, 0])
    }
}

impl From<U128> for U256 {
    fn from(value: U128) -> Self {
        U256([value.0[0], value.0[1], 0, 0])
    }
}

impl Mul for U256 {
    type Output = U256;
    
    fn mul(self, other: U256) -> U256 {
        let mut result = [0u64; 4];
        
        for i in 0..4 {
            for j in 0..4 {
                let product = self.0[i] as u128 * other.0[j] as u128;
                let pos = i + j;
                if pos < 4 {
                    let sum = result[pos] as u128 + product;
                    result[pos] = (sum & 0xFFFFFFFFFFFFFFFF) as u64;
                    if pos + 1 < 4 {
                        result[pos + 1] += (sum >> 64) as u64;
                    }
                }
            }
        }
        
        U256(result)
    }
}

impl Div for U256 {
    type Output = U256;
    
    fn div(self, other: U256) -> U256 {
        // Check if any of the u64 values in other are zero to prevent division by zero
        if other.0.iter().all(|&x| x == 0) {
            return U256::default();
        }
        
        // For now, use a safer division approach
        // Convert to u128 for safer arithmetic
        let self_val = self.as_u128().as_u128();
        let other_val = other.as_u128().as_u128();
        
        if other_val == 0 {
            return U256::default();
        }
        
        let result = self_val / other_val;
        U256::from(result)
    }
}

impl Add for U256 {
    type Output = U256;
    
    fn add(self, other: U256) -> U256 {
        let mut result = [0u64; 4];
        let mut carry = 0u64;
        
        for i in 0..4 {
            let sum = self.0[i] as u128 + other.0[i] as u128 + carry as u128;
            result[i] = (sum & 0xFFFFFFFFFFFFFFFF) as u64;
            carry = (sum >> 64) as u64;
        }
        
        U256(result)
    }
}

impl Sub for U256 {
    type Output = U256;
    
    fn sub(self, other: U256) -> U256 {
        let mut result = [0u64; 4];
        let mut borrow = 0i128;
        
        for i in 0..4 {
            let diff = self.0[i] as i128 - other.0[i] as i128 - borrow;
            result[i] = (diff & 0xFFFFFFFFFFFFFFFF) as u64;
            borrow = if diff < 0 { 1 } else { 0 };
        }
        
        U256(result)
    }
}

impl Downcast256 for U256 {
    fn as_u128(self) -> U128 {
        U128([self.0[0], self.0[1]])
    }
}

impl Upcast512 for U256 {
    fn as_u512(self) -> U512 {
        U512([self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0])
    }
}

impl MulDiv for U256 {
    type Output = U256;

    fn mul_div_floor(self, num: Self, denom: Self) -> Option<Self::Output> {
        assert_ne!(denom, U256::default());
        let r = (self.as_u512() * num.as_u512()) / denom.as_u512();
        if r > U256::MAX.as_u512() {
            None
        } else {
            Some(r.as_u256())
        }
    }

    fn mul_div_ceil(self, num: Self, denom: Self) -> Option<Self::Output> {
        assert_ne!(denom, U256::default());
        let r = (self.as_u512() * num.as_u512() + (denom - U256::from(1u64)).as_u512()) / denom.as_u512();
        if r > U256::MAX.as_u512() {
            None
        } else {
            Some(r.as_u256())
        }
    }

    fn to_underflow_u64(self) -> u64 {
        if self < U256::from(u64::MAX) {
            self.as_u64()
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct U512([u64; 8]);

impl U512 {
    pub const MAX: U512 = U512([u64::MAX; 8]);
}

impl Downcast512 for U512 {
    fn as_u256(self) -> U256 {
        U256([self.0[0], self.0[1], self.0[2], self.0[3]])
    }
}

impl Mul for U512 {
    type Output = U512;
    
    fn mul(self, other: U512) -> U512 {
        let mut result = [0u64; 8];
        
        for i in 0..8 {
            for j in 0..8 {
                let product = self.0[i] as u128 * other.0[j] as u128;
                let pos = i + j;
                if pos < 8 {
                    let sum = result[pos] as u128 + product;
                    result[pos] = (sum & 0xFFFFFFFFFFFFFFFF) as u64;
                    if pos + 1 < 8 {
                        result[pos + 1] += (sum >> 64) as u64;
                    }
                }
            }
        }
        
        U512(result)
    }
}

impl Div for U512 {
    type Output = U512;
    
    fn div(self, other: U512) -> U512 {
        if other == U512::default() {
            return U512::default();
        }
        // Simplified division
        U512([self.0[0] / other.0[0], 0, 0, 0, 0, 0, 0, 0])
    }
}

impl Add for U512 {
    type Output = U512;
    
    fn add(self, other: U512) -> U512 {
        let mut result = [0u64; 8];
        let mut carry = 0u64;
        
        for i in 0..8 {
            let sum = self.0[i] as u128 + other.0[i] as u128 + carry as u128;
            result[i] = (sum & 0xFFFFFFFFFFFFFFFF) as u64;
            carry = (sum >> 64) as u64;
        }
        
        U512(result)
    }
}

impl Sub for U512 {
    type Output = U512;
    
    fn sub(self, other: U512) -> U512 {
        let mut result = [0u64; 8];
        let mut borrow = 0i128;
        
        for i in 0..8 {
            let diff = self.0[i] as i128 - other.0[i] as i128 - borrow;
            result[i] = (diff & 0xFFFFFFFFFFFFFFFF) as u64;
            borrow = if diff < 0 { 1 } else { 0 };
        }
        
        U512(result)
    }
}

impl MulDiv for u64 {
    type Output = u64;

    fn mul_div_floor(self, num: Self, denom: Self) -> Option<Self::Output> {
        assert_ne!(denom, 0);
        let r = (U128::from(self) * U128::from(num)) / U128::from(denom);
        if r > U128::from(u64::MAX) {
            None
        } else {
            Some(r.as_u64())
        }
    }

    fn mul_div_ceil(self, num: Self, denom: Self) -> Option<Self::Output> {
        assert_ne!(denom, 0);
        let r = (U128::from(self) * U128::from(num) + U128::from(denom - 1)) / U128::from(denom);
        if r > U128::from(u64::MAX) {
            None
        } else {
            Some(r.as_u64())
        }
    }

    fn to_underflow_u64(self) -> u64 {
        self
    }
} 