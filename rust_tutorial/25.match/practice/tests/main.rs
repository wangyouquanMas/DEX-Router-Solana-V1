fn process_option(opt: Option<i32>) -> Option<i32>{
    match opt{
        Some(x) => Some(x *2),
        None => None,
    }
}


#[test]
fn test_some_case(){
    let input = Some(5);
    let result = process_option(input);
    assert_eq!(result,Some(10));
}

#[test]
fn test_none_case(){
    let input = None;
    let result = process_option(input);
    assert_eq!(result, None);
}
