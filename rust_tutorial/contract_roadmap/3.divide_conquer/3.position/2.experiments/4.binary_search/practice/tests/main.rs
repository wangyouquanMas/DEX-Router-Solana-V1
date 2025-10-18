// To understand the working of binary search, consider the following illustration:

// Consider an array arr[] = {2, 5, 8, 12, 16, 23, 38, 56, 72, 91}, and the target = 23.


#[test]
fn test_binary_serach(){
    let arr = [2, 5, 8, 12, 16, 23, 38, 56, 72, 91];
    let target = 23;

    match binary_search(&arr, target){
        Some(index) => println!("Target {} found at index {}",target, index),
        None => println!("Target {} not found",target),
    }
}


fn binary_search(arr: &[i32], target: i32) -> Option<usize>{
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;

    while left <= right{
        let mid = left + (right - left) / 2;
        let mid_val = arr[mid as usize];

        if mid_val == target{
            return Some(mid as usize);
        }else if mid_val < target{
            left = mid + 1; 
        }else{
            right = mid -1;
        }
    }
    None 
}