use std::fmt::Display;
use std::mem::{size_of, size_of_val, align_of};

// 1. 基础结构体定义
struct Person {
    name: String,
    age: u32,
    height: f64,   // 厘米
    is_student: bool,
}



fn main() {
    println!("===== Rust结构体全面指南 =====\n");
    
    // 1. 结构体实例化与初始化
    let mut alice = Person{
        name : String::from("Alice"),
        age :30,
        height: 165.5,
        is_student: false,
    };


    // 直接赋值给可变引用
    alice.name = String::from("I am actually Bob");


    // 2. 结构体字段访问
    println!("Alice's name: {}", alice.name);
    println!("Alice's age: {}", alice.age);
    println!("Alice's height: {}cm", alice.height);
    println!("Alice is a student: {}", alice.is_student);

    // 3. 结构体更新
    let mut bob = Person{
        name: String::from("Bob"),
        age: 25,
        height: 180.0,
        is_student: true,
    };

    // 4. 结构体解构
    let Person{name, age, height, is_student} = bob;

    println!("Bob's name: {}", name);
    println!("Bob's age: {}", age);
    println!("Bob's height: {}cm", height);
    println!("Bob is a student: {}", is_student);
}