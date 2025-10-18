
// 定义一个简单的结构体
struct Counter {
    count: u32,
    name: String,
}

impl Counter{
    //构造函数
    fn new(name: &str) -> Self{
        Counter{
            count:0,
            name:name.to_string(),
        }
    }

    //1. 使用&self(不可变引用接收器)
    fn get_count(&self) -> u32{
        println!("&self大小:{}字节",size_of_val(&self));
        println!("&self是指向Counter的引用,指向地址:{:p}",self);

        //可以读取字段
        self.count
    }

    //2. 使用&mut self (可变引用接收器
    fn increment(&mut self){
        println!("&mut self大小:{}字节",size_of_val(&self));
        println!("&mut self也是指向Counter的引用,指向地址:{:p}",self);

        //可以修改字段
        self.count +=1;
    }

    //3. 使用self (值接收器,获取所有权)
    fn consume(self) -> String{
        println!("self大小: {}字节",size_of_val(&self));
        println!("self是整个Counter结构体,地址:{:p}",&self);

        //获取所有权后可以任意处理,方法结束后结构体被销毁
        format!("Counter '{}' 已被消费，最终计数: {}", self.name, self.count)
    }
}

fn print_counter_info(counter: &Counter) {
    println!("Counter '{}': {}", counter.name, counter.count);
    println!("Counter地址: {:p}", counter);
    println!("Counter大小: {} 字节", size_of_val(counter));
}

fn main(){
    println!("===== self参数测试 =====\n");
    
    // 打印结构体相关信息
    println!("Counter结构体大小: {} 字节", std::mem::size_of::<Counter>());
    println!("&Counter引用大小: {} 字节\n", std::mem::size_of::<&Counter>());

    // 创建Counter实例
    let mut my_counter = Counter::new("计数器A");
    print_counter_info(&my_counter);
    
    // 1. 使用&self方法
    println!("\n=== 测试&self ===");
    let count = my_counter.get_count();
    println!("计数值: {}", count);
    
    // 2. 使用&mut self方法
    println!("\n=== 测试&mut self ===");
    my_counter.increment();
    println!("增加后的计数: {}", my_counter.count);
}


