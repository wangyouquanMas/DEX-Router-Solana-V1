# Rust 迭代器 (Iterator) 学习

## iter() 方法详解

`iter()` 是 Rust 中最常用的迭代器方法之一，用于创建一个对集合元素的**不可变引用**迭代器。

## 关键概念

1. **iter()** - 创建不可变引用迭代器 `&T`
2. **iter_mut()** - 创建可变引用迭代器 `&mut T`  
3. **into_iter()** - 消耗集合，创建所有权迭代器 `T`
4. **enumerate()** - 为迭代器添加索引，返回 `(index, value)` 元组

## 实际应用场景

在 DEX 路由代码中：
```rust
for (hop, route) in hops.iter().enumerate() {
    // hop 是索引 (0, 1, 2, ...)
    // route 是对 hops 中元素的不可变引用 &Route
}
```

运行测试查看完整示例：
```bash
cargo test --test iter_example -- --nocapture
```

