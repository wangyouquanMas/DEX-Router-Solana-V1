use std::ops::{Add, Sub, Mul, Div};

/// Q64.64 定点数表示
/// 64位整数部分 + 64位小数部分
const Q64: u128 = (u64::MAX as u128) + 1; // 2^64
const RESOLUTION: u8 = 64;

/// 定点数结构体
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct FixedPoint64 {
    value: u128,
}

impl FixedPoint64 {
    /// 从浮点数创建定点数
    fn from_float(float_value: f64) -> Self {
        let fixed_value = (float_value * Q64 as f64) as u128;
        Self { value: fixed_value }
    }

    /// 转换为浮点数
    fn to_float(&self) -> f64 {
        self.value as f64 / Q64 as f64
    }

    /// 从整数和小数部分创建
    fn from_parts(integer: u64, fraction: u64) -> Self {
        let value = (integer as u128) * Q64 + (fraction as u128);
        Self { value }
    }

    /// 获取整数部分
    fn integer_part(&self) -> u64 {
        (self.value >> RESOLUTION) as u64
    }

    /// 获取小数部分
    fn fraction_part(&self) -> u64 {
        (self.value & (Q64 - 1)) as u64
    }

    /// 检查是否为零
    fn is_zero(&self) -> bool {
        self.value == 0
    }

    /// 检查是否为负数（通过比较）
    fn is_negative(&self) -> bool {
        self.value > (i64::MAX as u128) * Q64
    }
}

/// 加法运算
impl Add for FixedPoint64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

/// 减法运算
impl Sub for FixedPoint64 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

/// 乘法运算
impl Mul for FixedPoint64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            value: (self.value * other.value) / Q64,
        }
    }
}

/// 除法运算
impl Div for FixedPoint64 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            value: (self.value * Q64) / other.value,
        }
    }
}

/// 与整数相乘
impl Mul<u64> for FixedPoint64 {
    type Output = Self;

    fn mul(self, other: u64) -> Self {
        Self {
            value: self.value * other as u128,
        }
    }
}

/// 与整数相除
impl Div<u64> for FixedPoint64 {
    type Output = Self;

    fn div(self, other: u64) -> Self {
        Self {
            value: self.value / other as u128,
        }
    }
}

#[test]
fn test_fixed_point_conversion() {
    // 测试转换精度
    let original = 123.456789;
    let fp = FixedPoint64::from_float(original);
    let converted = fp.to_float();
    
    // 由于64位精度，转换后的值应该非常接近原始值
    let difference = (original - converted).abs();
    assert!(difference < 1e-15); // 误差应该很小

    println!("原始值: {}", original);
    println!("转换后: {}", converted);
    println!("误差: {}", difference);
}

