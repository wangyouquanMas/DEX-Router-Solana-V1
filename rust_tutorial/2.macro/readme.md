1.什么是宏？

2.宏有什么作用？
宏在编译时被处理，生成相应的代码。

3. 如何声明宏
在 Rust 中声明宏使用 macro_rules! 关键字：
基础语法：
    macro_rules! macro_name{
        //宏规则
    }

 4. 宏规则语法
 macro_rules! macro_name{
    (pattern) => {code};
    (pattern2) => {code2};
 }

参数类型
macro_rules! example_macro{
    //表达式
    ($x:expr) => {$x + 1};
    //标识符
    ($name:ident) => {let $name = 42;};
    //类型
    ($type:ty) => {Vec<$type>};
    //路径
    ($path:path) => {$path::new()};
    //标记树
    ($($tokens:tt)*) => { /* 任意代码 */}；
}



4. 生成相应的代码在哪里？ 
// 你写的代码
declare_id!("BjcqoRYZuWuGu5nHSrEyi5DFZKg51xNSP9RP7nEYp75j");

// 宏展开后生成的代码（简化版）
pub const ID: &str = "BjcqoRYZuWuGu5nHSrEyi5DFZKg51xNSP9RP7nEYp75j";
pub const ID_BYTES: [u8; 32] = [/* 转换后的字节数组 */];


5. 宏在Solana macro 中的作用？

   在 Solana 网络中，每个程序必须有唯一的标识符，宏帮助生成该标识符。 Solana 运行时验证调用是否来自正确的程序，防止恶意程序冒充你的程序

   以declare_id!("BjcqoRYZuWuGu5nHSrEyi5DFZKg51xNSP9RP7nEYp75j"); 宏为例，它设定了合约的ProgramID，并生成了合约的IDL文件。

   
