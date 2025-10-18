use std::collections::VecDeque;

#[test]
fn test_vec_deque() {
    let mut deque: VecDeque<i32> = VecDeque::new();

    // Adding elements
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);

    // Removing elements
    let back = deque.pop_back();  // Removes 2
    let front = deque.pop_front(); // Removes 0

    // In the returned result of the pop_back() and pop_front() methods, the value type is Option<T>, where T is the type of elements in the VecDeque.
    println!("Removed from back: {:?}", back.unwrap());
    println!("Removed from front: {:?}", front);
    println!("Remaining deque: {:?}", deque);

    if let Some(x) =  deque.front(){
        println!("1111front: {}", x);
    }


}
