
#[test]
fn test_string(){
    //创建字符串
    let s = String::from("Hello, world!");
    println!("{}", s);


    //将一个字面量&str 转为String
    let s1 = "Hello,world!".to_string();
    println!("{}",s1);

    //往字符串中追加内容
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}",s);

    //获取字符串长度
    let s = String::from("Hello");
    println!("Length of s: {}",s.len());

    //字符串切片
    let s= String::from("Hello, world!");
    let slice = &s[0..5];
    println!("{}",slice);
}