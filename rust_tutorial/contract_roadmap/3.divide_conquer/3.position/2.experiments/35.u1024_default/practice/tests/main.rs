use uint::construct_uint;

construct_uint! {
    pub struct U1024(16);
}


#[test]
fn test_u1024_default(){
    let default = U1024::default();
    println!("default: {:?}", default);
    println!("default (decimal) = {}", default);
    println!("default (hex) = {:#x}", default);
}

