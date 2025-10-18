目标：
1. 如何声明
    fn  snake_case_function_name(){}

2. 函数内部调用其它函数

3. 函数参数(单参数，多参数)

4. 函数返回值

5. 函数返回值Self 是什么含义？
impl AccountInfo{
    fn new(is_writable: bool) -> Self{
        Self{ is_writable}
    }
}

6. fn find_index<T: PartialEq>(arr: &[T], value: T) -> Option<usize> {}
这里的<T: PartialEq> 作用？




内容：
1. 函数返回值Self含义

Self 是一个类型别名：它指代当前 impl 块所实现的类型。在 impl 块中，Self 代表的是类型的名字，因此它等同于 AccountInfo（在这个例子中），也可以用于返回该类型的实例。

如果你写 impl AccountInfo { ... }，那么 Self 就是 AccountInfo。

如果你写 impl<T> MyStruct<T> { ... }，那么 Self 就是 MyStruct<T>。



6.  fn find_index<T: PartialEq>(arr: &[T], value: T) -> Option<usize> {}
这里的<T: PartialEq> 作用？

<T: PartialEq> 是一种 泛型约束，用于限制 T 类型的行为。在这个特定的例子中，它的作用是 要求泛型类型 T 实现了 PartialEq trait，即支持部分相等比较（== 操作符）。

举例：
fn find_index<T: PartialEq>(arr: &[T], value: T) -> Option<usize> {
    for (index, item) in arr.iter().enumerate() {
        if *item == value {  // 使用 `==` 来比较
            return Some(index);
        }
    }
    None
}
在这个函数中，<T: PartialEq> 表明：
我们可以使用 == 比较 arr 中的元素和 value，因为它们的类型 T 必须实现 PartialEq。