定义
1. 什么是lifetime ?
    A lifetime is a compile-time concept that represents the scope during which a reference is valid.

2. life作用
Lifetimes ensure references don't outlive the data they point to, preventing dangling pointers and use-after-free errors.

3. 什么是dangling pointers 
悬垂指针是指向已被释放或无效内存位置的指针。使用悬垂指针会导致未定义行为，可能引起程序崩溃、数据损坏或安全漏洞。

4. 如何判断生命周期？

词法作用域基本原则
最基本的生命周期判断基于词法作用域：
fn main() {
    let x = 5;            // ------ 'x开始
    let r = &x;           // ------ 'r开始
    println!("r: {}", r); 
}                         // ------ 'x和'r结束
引用r的生命周期不能超过被引用值x的生命周期。

非词法生命周期(NLL)
Rust 2018引入了非词法生命周期，编译器根据引用的实际使用情况判断生命周期：
fn main() {
    let mut x = 5;
    let r = &x;           // 不可变借用开始
    println!("r: {}", r); // 不可变借用在此处使用后实际结束
    
    let y = &mut x;       // 可变借用开始(因为不可变借用已结束)
    *y += 1;
}
即使从词法上看r和y的作用域重叠，但由于r的最后使用在y创建之前，编译器判定生命周期不冲突。
