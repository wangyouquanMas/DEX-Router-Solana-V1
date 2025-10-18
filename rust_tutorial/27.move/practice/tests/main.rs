

#[test]
fn test_move_closure(){
    let s = String::from("Hello");

    //使用 move, 闭包捕获并转移 s的所有权
    let closure = move || {
        println!("{}",s); //闭包使用 s 
    };

    closure();

    // println!("{}",s); // 错误：s 的所有权已转移到闭包，不能再使用
}