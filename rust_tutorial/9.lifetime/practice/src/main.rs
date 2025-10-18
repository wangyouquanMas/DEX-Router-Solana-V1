fn main(){
    //尝试获取悬垂引用
    // let dangling_ref = create_dangling_pointer();

    // println!("值: {}",*dangling_ref);

    //生命周期
    let mut x = 5;
    let r = &x;           // 不可变借用开始
    println!("r: {}", r); // 不可变借用在此处使用后实际结束

    let y = &mut x;       // 可变借用开始(因为不可变借用已结束)
    *y += 1;

    // println!("r: {}", r); // 如果r在这结束, r,y生命周期就会交叉,然后报错
}


// fn create_dangling_pointer() -> &i32{
//     let local_value = 42; //局部变量

//     //尝试返回局部变量的引用
//     &local_value 
// }