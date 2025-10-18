// 声明math_utils模块
mod math_utils;
mod submodule_example;

// 导入模块中的具体功能
use math_utils::{add};
// use submodule_example::{demonstrate_submodules};

fn main(){
    println!("=== Rust 模块系统实践 ===\n");

    //1. 使用数据工具模块
    let result1 = add(10,5);
    println!(" 10 + 5 = {}", result1);

    //2. 使用子模块
    println!("\n2. 子模块详细演示");
    submodule_example::demonstrate_submodules();
}