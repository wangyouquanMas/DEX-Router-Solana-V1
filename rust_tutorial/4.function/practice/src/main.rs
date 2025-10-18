fn main(){
    println!("Hello,world!");

    another_function();
    function_with_parameters(5);
    function_with_multiple_parameters(5, 'h');
   
   let x =  five();
   println!("The value of x is:{x}");
}

fn another_function(){
    println!("Another function.");
}


fn function_with_parameters(x: i32){
    println!("The value of x is:{x}");
}

fn function_with_multiple_parameters(value:i32, unit_label:char){
    println!("The measurement is:{value}{unit_label}");
}


// return the last expression implicitly
fn five() -> i32{
    5
}
