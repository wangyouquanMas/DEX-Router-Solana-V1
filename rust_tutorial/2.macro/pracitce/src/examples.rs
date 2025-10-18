// 展示宏的用法
use crate::macros::*;

pub fn basic_macro_demo(){
    println!(" 基础宏使用 ");

    //无参数宏
    say_hello!();

    //计算宏
    let result1 = calculate!(10,+,5);

    println!("计算结果: {} + {} = {}",10,5,result1);
}