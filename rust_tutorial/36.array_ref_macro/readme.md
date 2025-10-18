目标：
1. 掌握array_ref macro的用法


内容：
1. 用法
You can use array_ref to generate an array reference to a subset of a sliceable bit of data (which could be an array, or a slice, or a Vec).


macro_rules! array_ref {
    ($arr:expr, $offset:expr, $len:expr) => { ... };
}