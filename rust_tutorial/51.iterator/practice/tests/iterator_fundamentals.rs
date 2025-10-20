/// Rust 迭代器深度理解 - Iterator 是什么以及它如何工作
/// 
/// 本测试通过手动实现 Iterator trait 和观察迭代器行为，
/// 帮助你理解迭代器的本质和工作原理

#[cfg(test)]
mod iterator_fundamentals {
    
    /// 第一步：理解 Iterator trait 的核心
    /// Iterator 只是一个 trait，它的核心只有一个方法：next()
    struct Counter {
        count: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Counter {
            Counter { count: 0, max }
        }
    }

    // 手动实现 Iterator trait
    impl Iterator for Counter {
        type Item = u32;  // 迭代器产生的元素类型

        // next() 是迭代器的灵魂
        // 每次调用返回 Some(下一个值) 或 None(结束)
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None  // 迭代结束
            }
        }
    }

    #[test]
    fn test_iterator_fundamentals() {
        println!("\n=== 第一部分：Iterator 的本质 ===\n");
        
        // Iterator 就是一个实现了 next() 方法的类型
        // next() 返回 Option<Item>：Some(value) 或 None
        let mut counter = Counter::new(3);
        
        println!("手动调用 next() 方法：");
        println!("  第1次调用 next(): {:?}", counter.next()); // Some(1)
        println!("  第2次调用 next(): {:?}", counter.next()); // Some(2)
        println!("  第3次调用 next(): {:?}", counter.next()); // Some(3)
        println!("  第4次调用 next(): {:?}", counter.next()); // None
        println!("  第5次调用 next(): {:?}", counter.next()); // None - 始终返回 None
        
        println!("\n💡 关键理解：");
        println!("   - Iterator 维护内部状态(self.count)");
        println!("   - 每次调用 next() 都会改变状态(&mut self)");
        println!("   - 返回 None 表示迭代结束");

        println!("\n=== 第二部分：for 循环的本质 ===\n");
        
        // for 循环实际上就是不断调用 next() 直到返回 None
        let counter2 = Counter::new(3);
        println!("for 循环遍历:");
        for num in counter2 {
            println!("  获得值: {}", num);
        }
        // for 循环等价于：
        // let mut iter = counter2.into_iter();
        // while let Some(num) = iter.next() {
        //     println!("  获得值: {}", num);
        // }
        
        println!("\n💡 关键理解：for 循环是 next() 的语法糖");

        println!("\n=== 第三部分：迭代器的惰性(Lazy Evaluation) ===\n");
        
        let numbers = vec![1, 2, 3, 4, 5];
        
        // 创建迭代器，但还没有执行任何操作
        let iter = numbers.iter().map(|x| {
            println!("    map 正在处理: {}", x);
            x * 2
        });
        
        println!("已创建迭代器，但还没有输出...");
        println!("因为迭代器是惰性的！");
        println!("\n开始消费迭代器(调用 collect):");
        
        // 只有在消费时才真正执行
        let doubled: Vec<i32> = iter.collect();
        println!("结果: {:?}", doubled);
        
        println!("\n💡 关键理解：");
        println!("   - 迭代器是惰性的，创建时不执行");
        println!("   - 只有消费者(collect, for, fold等)触发时才执行");
        println!("   - 这样可以实现高效的链式调用");

        println!("\n=== 第四部分：迭代器方法链 ===\n");
        
        let result: Vec<u32> = Counter::new(10)
            .filter(|x| {
                println!("  filter 检查: {} -> {}", x, x % 2 == 0);
                x % 2 == 0  // 只要偶数
            })
            .map(|x| {
                println!("  map 转换: {} -> {}", x, x * 10);
                x * 10
            })
            .take(2)  // 只取前2个
            .collect();
        
        println!("最终结果: {:?}", result);
        
        println!("\n💡 关键理解：");
        println!("   - 每个方法返回新的迭代器");
        println!("   - 形成处理管道：数据 -> filter -> map -> take -> collect");
        println!("   - 每个元素依次通过整个管道");

        println!("\n=== 第五部分：三种迭代方式对比 ===\n");
        
        let mut data = vec![10, 20, 30];
        
        // 1. iter() - 借用，返回 &T
        println!("1. iter() - 不可变引用:");
        for item in data.iter() {
            println!("   类型: &i32, 值: {}", item);  // item 是 &i32
        }
        println!("   ✓ data 仍可用: {:?}\n", data);
        
        // 2. iter_mut() - 可变借用，返回 &mut T
        println!("2. iter_mut() - 可变引用:");
        for item in data.iter_mut() {
            *item *= 2;  // 可以修改
            println!("   类型: &mut i32, 新值: {}", item);
        }
        println!("   ✓ data 已修改: {:?}\n", data);
        
        // 3. into_iter() - 获取所有权，返回 T
        println!("3. into_iter() - 获取所有权:");
        for item in data.into_iter() {
            println!("   类型: i32, 值: {}", item);  // item 是 i32
        }
        // println!("{:?}", data); // ❌ 错误！data 已被移动
        println!("   ✗ data 已被消耗，不能再使用");

        println!("\n✅ 总结：Iterator 的核心概念");
        println!("   1. Iterator 是实现了 next() 方法的 trait");
        println!("   2. next() 维护内部状态，返回 Option<Item>");
        println!("   3. for 循环是反复调用 next() 的语法糖");
        println!("   4. 迭代器是惰性的，只在消费时才执行");
        println!("   5. 方法链形成高效的数据处理管道");
    }
}

