#[test]
fn test_vector(){
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!([1,2],&vec[..]);

    for x in vec{
        print!("{x} ");
    }
}