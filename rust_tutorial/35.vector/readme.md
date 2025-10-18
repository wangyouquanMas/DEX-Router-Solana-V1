目标：
1. 掌握vector用法
2. 理解Vector和Array的区别

内容：
1. 用法
1) declaration
let mut vec: Vec<i32> = Vec::new();

2) insert value 
vec.push(1);
vec.push(2);


2. 区别
1) size 
array size is fixed at compile-time.
vector is dynamically sized.