目标：
1. 理解FnOnce的作用？
2. 掌握FnOnce的使用
3. 什么是trait bound ?

内容
1. FnOnce含义
    FnOnce是一个Trait.

2. FnOnce使用
    一般由closures或者 function pointers来实现。 
    如果一个closure 实现FnOnce,那么该closure仅可被使用一次，因为它会持有其获取的变量的
    所有权，也就以为着，这些变量在被closure调用后，后续不能再使用了。 

3. trait bound 和 generic type 关系？ 

A trait bound is a way of specifying constraints on the types that can be used in generic functions, structs, and other types. In this case, we are saying that the type F (which is a generic parameter in the function) must implement the FnOnce trait.

3.1 示例：
fn consume_and_return<F>(f: F) -> String where F: FnOnce(String) -> String,

In simpler terms, this line is saying:
The generic parameter F must be a closure (or function pointer) that can be called once and that takes a String as an argument and returns a String.

其中：
FnOnce(String) -> String: This is the trait bound.
FnOnce is a trait for closures that take ownership of their arguments and can be called only once.
String is the type of the argument that the closure must accept.
-> String indicates that the closure must return a String.