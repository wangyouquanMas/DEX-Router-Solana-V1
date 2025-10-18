目标：
1. 掌握match用法

内容：
1. match用法
match self {
    Some(x) => f(x),
    None => None,
}
作用是对 self 进行模式匹配。self 是一个 Option 类型的值，Option 类型有两个变体：Some(T) 和 None。

其中：
Some(x)：这是匹配 Option 类型的 Some 变体。Some 用来封装一个值，这里 x 是 Some 中的值。通过 Some(x) 模式，我们提取了 Some 中的值 x，并将其传递给函数 f(x)。

None：这是匹配 Option 类型的 None 变体，表示没有值。在这种情况下，直接返回 None。