# 迭代器深度理解 - Iterator 的本质与工作原理

## 🎯 学习目标

理解 Rust 迭代器的本质：
- Iterator 不是魔法，只是一个实现了 `next()` 方法的 trait
- 迭代器如何维护内部状态
- 为什么迭代器是惰性的(lazy)
- 迭代器方法链如何工作
- `iter()` vs `iter_mut()` vs `into_iter()` 的区别

## 📚 核心概念

### Iterator Trait 的本质

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ... 其他方法都基于 next() 实现
}
```

就这么简单！所有迭代器的魔法都来自这个简单的 `next()` 方法。

## 🚀 运行测试

```bash
cd practice
cargo test -- --nocapture
```

## 💡 关键理解

1. **Iterator 是一个状态机**：每次调用 `next()` 都改变内部状态
2. **for 循环是语法糖**：本质上就是反复调用 `next()`
3. **惰性求值**：创建迭代器不执行，只有消费时才执行
4. **零成本抽象**：编译器会将迭代器优化成高效的循环
5. **方法链**：每个方法返回新迭代器，形成处理管道

## 🔗 相关链接

- [Iterator trait 文档](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [The Rust Book - Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)

