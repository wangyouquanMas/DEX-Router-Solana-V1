



// accepts a closure f, which takes a String and returns a String.
fn consume_and_return<F>(f: F) -> String
where F: FnOnce(String) -> String,{
    let result = f(String::from("Hello"));
    result
}


#[test]
fn test_fn_once_closure(){
    let closure = |input:String| -> String{
        input + ", world!"
    };

    let result = consume_and_return(closure);

    assert_eq!(result, "Hello, world!");

    let result = consume_and_return(closure);
    assert_eq!(result, "Hello, world!");
}

#[test]
fn test_fn_once_closure_v1(){
    let  move_closure = move |s: String|{
        println!("{}",s);
        s //Closure consume s and returns it 
    };

    let s = String::from("Hello");

    let result = move_closure(s); //First call: consumes 's' and prints it 

    // println!("{}", s);  //value borrowed here after move
    // let second_call = move_closure(s);  // ^ value used here after move
}