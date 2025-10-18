目标：
1. 什么是所有权？
2. 所有权转移意味着什么？


内容：
1. 什么是所有权？
  所有权是一个核心概念，它描述了程序中数据的“归属”以及如何管理内存。
  Rust 使用所有权模型来管理内存，而无需垃圾回收机制，从而确保内存安全并提高性能。

2. 所有权系统的基本规则
   每个值都有一个所有者(owner): 每个变量，结构体或数据值在Rust中都有一个所有者，这个所有者通常是一个变量
   一个值只能有一个所有者： 同一时间内，数据的所有权只能归一个变量
   所有权可以转移(move): 当数据的所有者发生变化时，数据的所有权可以从一个变量转移到另一个变量
   所有权会在变量生命周期结束时自动释放： 当变量超出作用域时，它所拥有的资源会被自动释放

3. 所有权转移意味着什么？
  转移所有权后，原来的变量将无法再访问该数据，因为数据的所有权已经被转移到新的变量上。

4. 所有者意味着什么？
  所有者（Owner）是指某个数据（如变量、资源、对象等）的控制者，负责管理该数据的生命周期。

5. 所有者可以对其拥有的数据做什么？ 
  所有权（Ownership）：意味着数据归某个所有者所有，这个所有者拥有对数据的控制权，包括读取、修改、销毁等。
  5.1 控制数据的生命周期
  拥有数据：所有者负责数据的生命周期，从创建到销毁，数据在所有者的生命周期内是有效的。
  销毁数据：当所有者离开作用域时，数据会被自动销毁（通过 Rust 的自动内存管理机制），并释放内存。
  5.2 通过所有权转移管理数据
  所有权转移（Move）：Rust 允许数据的所有权从一个变量转移到另一个变量，这种转移是“一次性”的，也就是说，转移后原 始变量不再有效，无法访问数据。



借用与所有权
直接相关于: 堆栈优化
将  let global_config = &self.global_config;
  let creator = &self.creator;
改为：
  let creator_key = self.creator.key();

减少了堆栈使用量，直接解决了内存问题


问题背景：

# Rust Ownership特性在解决栈内存问题中的应用

在解决栈内存过大问题时，我们利用了Rust的以下ownership特性：

## 1. 避免不必要的引用嵌套
**原代码**:
```rust
let global_config = &self.global_config;
let creator = &self.creator;
let token = &self.token;
// ...
bonding_curve.token_mint = token.key();
```

**修改后**:
```rust
// 直接存储需要的值，不存储中间引用
let creator_key = self.creator.key();
let token_key = self.token.key();
// ...
self.bonding_curve.token_mint = token_key;
```

这减少了栈上的引用层级，每个中间引用都会占用栈空间。

## 2. 直接访问而非引用拷贝
**原代码**:
```rust
let bonding_curve = &mut self.bonding_curve;
bonding_curve.virtual_sol_reserves = global_config.initial_virtual_sol_reserves_config;
```

**修改后**:
```rust
self.bonding_curve.virtual_sol_reserves = self.global_config.initial_virtual_sol_reserves_config;
```

直接访问`self`的字段，避免了在栈上创建额外的可变引用。

## 3. 值拷贝替代大结构引用
对于小型数据（如`PublicKey`），我们使用值拷贝而非引用整个大型结构：
```rust
// 只复制需要的数据(PublicKey)，而不是引用整个Account结构
let creator_key = self.creator.key(); // 只占用一个PublicKey的大小
```

## 4. 减少日志输出中的临时对象创建
**原代码**:
```rust
msg!("  - Virtual SOL reserves: {}", bonding_curve.virtual_sol_reserves);
msg!("  - Virtual token reserves: {}", bonding_curve.virtual_token_reserves);
// 多行日志...
```

**修改后**:
移除了这些日志，避免了每次格式化字符串时在栈上创建临时对象。

## 5. 控制变量作用域
变量的声明被移到更接近使用点的位置，减少了它们在栈上的生存时间，这不是直接减少栈使用峰值，但是一种良好的内存管理实践。

这些ownership特性的巧妙应用，让我们在不改变程序功能的情况下，成功减少了栈内存使用，解决了原来超出Solana 4KB栈限制的问题。