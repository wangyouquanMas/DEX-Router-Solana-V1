1. 在减少栈空间内存中的作用?
## 直接访问而非引用拷贝
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