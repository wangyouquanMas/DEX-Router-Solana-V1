目标：
1. 理解什么是tuple

内容
1. tuple使用

1.1 Definition 
A tuple in Rust is a collection of values of different types grouped together into a single compound data type


1.2 syntax 
let tuple = (value1,value2,valu3,...);

1.3 Example 
let person: (&str, i32) = ("Alice", 30);
//Accessing tuple elements 
let name = person.0;
let age = person.1;

