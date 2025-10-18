// 数学工具模块
// 这个模块展示了如何组织相关的函数和结构

// 公开的数学函数
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn divide(a: i32, b: i32) -> f64 {
    if b == 0 {
        panic!("除数不能为零!");
    }
    a as f64 / b as f64
}

// 私有函数 - 只能在模块内部使用
fn validate_number(num: i32) -> bool {
    num >= 0
}

// 公开的结构体
#[derive(Debug)]
pub struct Calculator {
    pub name: String,
    pub version: String,
}

impl Calculator {
    pub fn new(name: &str) -> Self {
        Calculator {
            name: name.to_string(),
            version: "1.0".to_string(),
        }
    }
    
    pub fn calculate(&self, operation: &str, a: i32, b: i32) -> f64 {
        match operation {
            "add" => add(a, b) as f64,
            "multiply" => multiply(a, b) as f64,
            "divide" => divide(a, b),
            _ => panic!("未知操作: {}", operation),
        }
    }
}

// 模块信息展示函数
pub fn show_module_info() {
    println!("   📊 数学工具模块");
    println!("   - 提供基础数学运算功能");
    println!("   - 包含公开函数: add, multiply, divide");
    println!("   - 包含私有函数: validate_number");
    println!("   - 包含结构体: Calculator");
} 