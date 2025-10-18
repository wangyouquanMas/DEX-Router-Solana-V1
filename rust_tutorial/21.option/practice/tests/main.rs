fn find_index<T: PartialEq>(arr: &[T], value: T) -> Option<usize>{
    for (index, item) in arr.iter().enumerate(){
        if *item == value{
            return Some(index);
        }
    }
    None
}


#[test]
fn test_find_index_found(){
    let arr = [1,2,3,4,5];
    let result = find_index(&arr,3);
    assert_eq!(result,Some(2));
}