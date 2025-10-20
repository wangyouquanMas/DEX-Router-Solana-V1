/// Rust 迭代器学习 - iter() 用法示例
/// 
/// 这个单元测试演示了如何使用 iter() 和 enumerate() 方法
/// 类似于 DEX Router 代码中的用法

#[cfg(test)]
mod iter_tests {
    use std::fmt;

    /// 模拟路由跳转信息
    #[derive(Debug, Clone)]
    struct Route {
        dex_name: String,
        token_in: String,
        token_out: String,
        amount: u64,
    }

    impl fmt::Display for Route {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{}: {} -> {} (amount: {})",
                self.dex_name, self.token_in, self.token_out, self.amount
            )
        }
    }

    #[test]
    fn test_iter_enumerate_understanding() {
        // 模拟多跳路由场景：SOL -> USDC -> BTC
        let hops = vec![
            Route {
                dex_name: "Raydium".to_string(),
                token_in: "SOL".to_string(),
                token_out: "USDC".to_string(),
                amount: 1000,
            },
            Route {
                dex_name: "Orca".to_string(),
                token_in: "USDC".to_string(),
                token_out: "BTC".to_string(),
                amount: 50,
            },
        ];

        println!("\n=== iter() 和 enumerate() 用法演示 ===\n");

        // 核心用法：iter() + enumerate()
        // 这正是你在 common_swap.rs:523 看到的用法
        for (hop, route) in hops.iter().enumerate() {
            println!("Hop {}: {}", hop, route);
            
            // hop 是索引 (usize 类型): 0, 1, 2, ...
            // route 是对 hops 中元素的不可变引用 (&Route)
            
            // 验证：
            assert!(hop < hops.len(), "索引不应超出范围");
            
            // 因为 route 是不可变引用，我们可以读取但不能修改
            println!("  - DEX: {}", route.dex_name);
            println!("  - 交易对: {} -> {}", route.token_in, route.token_out);
            println!("  - 数量: {}\n", route.amount);
            
            // 注意：以下代码会编译失败（取消注释试试）
            // route.amount = 999; // ❌ 错误！route 是不可变引用
        }

        println!("=== 对比：三种迭代器方法 ===\n");

        // 1. iter() - 不可变引用，原集合保持所有权
        println!("1. iter() - 借用不可变引用");
        for route in hops.iter() {
            println!("   类型: &Route, DEX: {}", route.dex_name);
        }
        println!("   ✓ hops 仍然可用，因为只是借用\n");

        // 2. iter_mut() - 可变引用（需要 mut 变量）
        let mut hops_mut = hops.clone();
        println!("2. iter_mut() - 借用可变引用");
        for route in hops_mut.iter_mut() {
            route.amount += 100; // ✓ 可以修改
            println!("   类型: &mut Route, 新数量: {}", route.amount);
        }
        println!("   ✓ hops_mut 仍然可用\n");

        // 3. into_iter() - 消耗集合，获得所有权
        let hops_owned = hops.clone();
        println!("3. into_iter() - 获得所有权");
        for route in hops_owned.into_iter() {
            println!("   类型: Route (owned), DEX: {}", route.dex_name);
        }
        // println!("{:?}", hops_owned); // ❌ 错误！hops_owned 已被消耗
        println!("   ✗ hops_owned 已被消耗，不能再使用\n");

        println!("=== enumerate() 的实际应用 ===\n");
        
        // enumerate() 在多跳路由中的价值：
        // - 追踪当前是第几跳
        // - 根据跳数执行不同逻辑
        // - 记录日志和调试
        for (hop_index, route) in hops.iter().enumerate() {
            if hop_index == 0 {
                println!("🚀 第一跳 (索引{}): 从 {} 开始", hop_index, route.token_in);
            } else if hop_index == hops.len() - 1 {
                println!("🎯 最后一跳 (索引{}): 到达 {}", hop_index, route.token_out);
            } else {
                println!("🔄 中间跳 (索引{}): 经过 {}", hop_index, route.dex_name);
            }
        }

        // 最终验证
        assert_eq!(hops.len(), 2, "原始集合未被修改");
        println!("\n✅ 测试通过！你已掌握 iter() 和 enumerate() 的用法");
    }

    #[test]
    fn test_iter_additional_methods() {
        println!("\n=== 迭代器的其他常用方法 ===\n");
        
        let numbers = vec![1, 2, 3, 4, 5];

        // map: 转换每个元素
        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        println!("map 转换: {:?} -> {:?}", numbers, doubled);

        // // filter: 过滤元素
        // let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
        // println!("filter 过滤: {:?} -> {:?}", numbers, evens);

        // // fold: 累积计算
        // let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
        // println!("fold 求和: {:?} -> {}", numbers, sum);

        // // enumerate + filter: 组合使用
        // let indexed_evens: Vec<(usize, &i32)> = numbers
        //     .iter()
        //     .enumerate()
        //     .filter(|(_, x)| *x % 2 == 0)
        //     .collect();
        // println!("enumerate + filter: {:?}", indexed_evens);

        // assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
        // assert_eq!(evens, vec![&2, &4]);
        // assert_eq!(sum, 15);
        // assert_eq!(indexed_evens, vec![(1, &2), (3, &4)]);
    }
}

