目标：
1. 掌握array用法


内容：
1. 用法

1） declaration
A fixed-size array, denoted[T; N], for the element type, T, and the non-negative compile-time constant size,N. 
有两个 syntactic forms for creating an array
a) A list with each element, i.e.,[x,y,z]
b) A repeat expression [expr; N] where N is how many times to repeat expr in the array. expr must either be:
A value of a type implementing the Copy trait;
A const value; 


示例：
let mut array: [i32,3] = [0; 3];