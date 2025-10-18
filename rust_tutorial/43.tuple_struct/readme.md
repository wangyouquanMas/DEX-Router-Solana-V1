目标：
1. 理解什么是tuple struct？

内容:
1. 使用

1.1 definition 
A tuple struct in Rust is a type of struct that uses unnamed fields, similar to a regular tuple, but with a distinct type

1.2 syntax
struct Name(Type1, Type2, ...);

1.3 Example 
struct Point3D(f64, f64, f64);

let p = Point3D(1.0, 2.0, 3.0);

let x = p.0; // Access the first field, which is 1.0
let y = p.1; // Access the second field, which is 2.0
let z = p.2; // Access the third field, which is 3.0
