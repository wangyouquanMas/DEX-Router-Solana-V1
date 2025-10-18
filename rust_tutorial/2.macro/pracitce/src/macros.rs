// 声明宏
#[macro_export]
macro_rules! say_hello {
    () =>{
        println!("Hello, World!");
    };
    ($name:expr) =>{
        println!("Hello,{}!",$name);
    };
    ($name:expr,$greeting:expr) =>{
        println!("{},{}",$greeting,$name);
    };
}

// 计算宏
#[macro_export]
macro_rules! calculate {
    ($a:expr,+ ,$b:expr) => {
        $a + $b;
    };
}


pub use say_hello;
pub use calculate;
