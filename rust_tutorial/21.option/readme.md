目标：
1. 理解Option定义
2. 掌握Option作用


内容：
1. Option定义

enum Option<T>{
    Some(T), //存在一个值，类型为T
    None, //没有值
}
Option<T> 是 Rust 的枚举类型，表示"可能有值，也可能没有值"
有两个变体：Some(T) 和 None

2. Some作用
Some 的作用是包装一个存在的值。它将一个值（类型 T）放入 Option 枚举中，以便处理值的存在性

举例：
fn find_index<T: PartialEq>(arr: &[T], value: T) -> Option<usize>{
    for (index, item) in arr.iter().enumerate(){
        if *item == value{
            return Some(index);
        }
    }
    None
}
