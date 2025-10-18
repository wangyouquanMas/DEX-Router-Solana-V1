use std::fmt;


struct Person{
    name: String,
    age: u32,
}

impl fmt::Display for Person{
    //fmt 方法格式化Person类型的输出
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        //使用 write! 宏将格式化后的字符串写入f
        write!(f,"Person {{name: {}, age: {}}}",self.name,self.age)
    }
}

#[test]
fn test_display(){
    let person = Person{
        name: String::from("Alice"),
        age: 30,
    };

    //使用Println! 宏打印Person 对象
    println!("{}",person);
}