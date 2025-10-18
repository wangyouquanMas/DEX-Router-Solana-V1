#[cfg(test)]
mod tests {
    /// 基础：Box<T> 的“拆箱”（拿到 T 的所有权）与借用
    #[test]
    fn box_unpack_and_borrow() {
        #[derive(Debug, PartialEq)]
        struct Foo(u32);

        // 拆箱：消耗 Box，拿到内部值
        let b = Box::new(Foo(7));
        let foo: Foo = *b; // 等价：Box::into_inner(b)
        assert_eq!(foo, Foo(7));

        // 借用：不移动内部值，只拿引用
        let b = Box::new(123_i32);
        let r: &i32 = b.as_ref();
        println!("r: {}", r);
        assert_eq!(*r, 123);
        // 仍可使用 b，自身并未被消耗
        assert_eq!(*b, 123);
    }
}
