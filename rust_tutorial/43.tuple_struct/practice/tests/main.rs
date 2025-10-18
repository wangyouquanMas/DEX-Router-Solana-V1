#[cfg(test)]
mod tests {
    #[test]
    fn test_tuple() {
        pub struct U1024(pub[u64;16]);

        let some_u1024 = U1024([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]);
        
        let arr = some_u1024.0;
        println!("arr: {:?}", arr);
    }
}
