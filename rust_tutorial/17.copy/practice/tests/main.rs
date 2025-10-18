#[derive(Copy,Clone)]
struct Point{
    x: i32,
    y: i32,
}

#[test]
fn test_copy(){
    //创建一个Point 实例
    let p1 =Point{ x:10, y:20};

    //将p1 赋值给p2, 自动进行 Copy
    let p2 = p1;

    // p1仍然有效，因为 Point 实现了Copy
    println!("p1: ({},{})",p1.x, p1.y);
    println!("p2: ({},{})",p2.x,p2.y);

    // 输出两个地址
     // 方法 2：使用 ptr::addr_of!
     // p1 和 p2 是两个值相同、地址不同的结构体实例（因为发生了 Copy）。
     use std::ptr;
     println!("(ptr::addr_of) Address of p1: {:?}", ptr::addr_of!(p1));
     println!("(ptr::addr_of) Address of p2: {:?}", ptr::addr_of!(p2));
}