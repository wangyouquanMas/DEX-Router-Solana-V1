目标:
1. 理解结构体中泛型的含义
struct Account<T>{
    info: AccountInfo,
    account: T,
}
2. 理解impl 结构



内容：
1. 什么是泛型
泛型 是一种允许我们在定义类型、函数、方法或结构体时不指定具体类型的特性
通过泛型，Rust 允许你编写更通用、更灵活的代码，使得类型在编译时确定，而不是在编写代码时固定


示例：
struct Account<T> {
    info: AccountInfo,
    account: T,
}
struct Account<T> 是一个带有泛型参数 T 的结构体。这个结构体的含义是它可以保存任何类型的 account，而具体的类型将会在创建实例时确定。

2. 理解impl<T>
impl<T> Account<T>：这里的 impl<T> 块为结构体 Account<T> 提供方法。T 是一个占位符，表示 Account 可以是任何类型
 泛型方法与普通方法的区别
泛型方法：如果一个方法需要操作泛型类型（如 T），我们必须在 impl 前面加上泛型标注，例如 impl<T>。这使得方法可以在实例化时处理不同类型的数据。

普通方法：对于普通的结构体方法（没有泛型参数），我们只需使用 impl 定义，不需要 <T>。