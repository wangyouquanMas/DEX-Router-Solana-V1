mod examples; 
mod macros;

use macros::*;
use examples::*;

fn main(){
    println!(" Rust 宏实践 \n");

    //1.基础宏演示
    println!("1. 基础宏演示");
    basic_macro_demo();

    //2. 单参数宏
    say_hello!("张三");

    //3. 多参数宏
    say_hello!("李四","你好");
}

