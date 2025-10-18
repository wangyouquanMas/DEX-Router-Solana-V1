use std::mem::{size_of, size_of_val};
use std::ptr;


//基本引用
fn main(){
 // 创建一个原始值
 let x = 5;

 //  创建一个不可变引用
 let r = &x;

 println!("值：{}",x); //直接访问值
 println!("引用: {}",r); //自动解引用打印
 println!("解引用: {}", *r); //显示解引用

 // 尝试通过不可变引用来修改值 (编译会不通过)
//  *r = 10;  //`r` is a `&` reference, so the data it refers to cannot be written


 //可变引用
 mutable_reference();


 //中间引用
 // 创建测试数据
 let mut ctx = Context {
    accounts: vec![
        Account { pubkey: [1; 32], is_signer: true, is_writable: false, data: vec![0; 100] },
        Account { pubkey: [2; 32], is_signer: false, is_writable: true, data: vec![0; 200] },
        Account { pubkey: [3; 32], is_signer: true, is_writable: true, data: vec![0; 300] },
    ],
    program_id: [0; 32],
 };

    // 打印基本信息
    println!("基本大小信息:");
    println!("Account结构体大小: {} 字节", size_of::<Account>());
    println!("引用大小: {} 字节", size_of::<&Account>());
    println!("可变引用大小: {} 字节", size_of::<&mut Account>());
    println!("布尔值大小: {} 字节", size_of::<bool>());

    // 执行测试
    process_instruction(&mut ctx);

    // 栈使用模拟
    println!("\n栈使用模拟:");
    println!("假设一个复杂函数有20个中间引用，总占用: {} 字节", 20 * size_of::<&Account>());


//输出地址
print_memory_address();

}


fn mutable_reference(){

    //创建一个可变的原始值   [x必须被显示声明为mutable，不然就是unmutable]
    let mut x = 5; 

    //创建一个可变引用 
    let r = &mut x;

    //通过可变引用修改值
    *r +=1;

    println!("修改后的值: {}", *r); //输出6
}



// 模拟复杂结构体，类似Solana中的账户结构
struct Account {
    pubkey: [u8; 32],      // 模拟PublicKey
    is_signer: bool,
    is_writable: bool,
    data: Vec<u8>,         // 账户数据
}

// 模拟程序上下文
struct Context {
    accounts: Vec<Account>,
    program_id: [u8; 32],
}


// 模拟指令处理函数
// ctx 是一个可变引用(&mut Context)，它指向 Context 结构体实例。
//TODO: 从内存视角看： ctx 自身是一个指针，占用 8 字节栈空间，指向堆上的 Context 结构体
fn process_instruction(ctx: &mut Context) {
    // ----- 方法1: 创建中间引用 -----
    println!("\n方法1: 使用中间引用");
    
    // 使用块作用域来解决借用问题
    {
        // 创建多个不可变中间引用
        // account0 是一个不可变的中间引用(&Account)，它指向 ctx.accounts 向量中的一个元素。
        //TODO: 内存视角：account0 也是一个指针，占用另外 8 字节栈空间，指向 Context 内部 accounts 向量的第一个元素
        let account0 = &ctx.accounts[0];
        let account1 = &ctx.accounts[1];
        
        // 计算和打印中间引用的大小
        let ref_size = size_of_val(&account0);
        println!("单个账户引用大小: {} 字节", ref_size);
        
        // 计算所有中间引用占用的总空间
        let total_ref_size = size_of_val(&account0) + size_of_val(&account1);
        println!("不可变中间引用总占用: {} 字节", total_ref_size);
        
        // 使用中间引用
        println!("账户0是否可签名: {}", account0.is_signer);
        println!("账户1是否可签名: {}", account1.is_signer);
    }
    
    // 单独的可变引用作用域
    {
        let account2 = &mut ctx.accounts[2];
        println!("可变引用大小: {} 字节", size_of_val(&account2));
        account2.is_writable = true;
    }
    
    // 如果需要多个可变引用，可以使用split_at_mut
    {
        let (first_part, second_part) = ctx.accounts.split_at_mut(2);
        let account0 = &mut first_part[0];
        let account2 = &mut second_part[0]; // 实际是accounts[2]
        
        println!("两个可变引用总大小: {} 字节", size_of_val(&account0) + size_of_val(&account2));
    }
    
    // ----- 方法2: 直接访问，无中间引用 -----
    println!("\n方法2: 直接访问，无中间引用");
    
    // 直接访问，不创建中间引用
    println!("账户0是否可签名: {}", ctx.accounts[0].is_signer);
    println!("账户1是否可签名: {}", ctx.accounts[1].is_signer);
    ctx.accounts[2].is_writable = true;
    
    // 不占用额外栈空间
    println!("无额外栈空间占用");
    
    // ----- 方法3: 复制实际需要的值 -----
    println!("\n方法3: 只复制需要的值");
    
    // 只复制我们实际需要的数据，而不是整个引用
    let is_signer0 = ctx.accounts[0].is_signer;
    let is_signer1 = ctx.accounts[1].is_signer;
    
    // 计算复制值的大小
    let copy_size = size_of_val(&is_signer0) + size_of_val(&is_signer1);
    println!("复制值占用空间: {} 字节", copy_size);
    
    // 使用复制的值
    println!("账户0是否可签名: {}", is_signer0);
    println!("账户1是否可签名: {}", is_signer1);
    
    // ----- 比较方法 -----
    println!("\n比较:");
    println!("方法1 (两个不可变中间引用) 占用: {} 字节", 2 * size_of::<&Account>());
    println!("方法3 (复制两个布尔值) 占用: {} 字节", 2 * size_of::<bool>());
    println!("节省: {} 字节", 2 * (size_of::<&Account>() - size_of::<bool>()));
}

//打印内存地址
fn print_memory_address(){

    let mut value = 42;

    //创建引用
    let immut_ref = &value;

    //打印内存地址
    println!("value 的内存地址:{:p}",ptr::addr_of!(value));
    println!("immut_ref 的值:{}", immut_ref);
    println!("immut_ref 指针本身的内存地址: {:p}",&immut_ref);
    println!("immut_ref 指向的内存地址: {:p}", immut_ref);
}