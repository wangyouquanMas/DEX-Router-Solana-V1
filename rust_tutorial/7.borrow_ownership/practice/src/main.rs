fn main(){
     //Ownership: notebook 被main 函数拥有
     let mut notebook = String::from("Meeting notes:");

     // Immutabe borrowing - 多个读者同时阅读
     let reader1 = &notebook;
     let reader2 = &notebook;

     println!("Reader 1 sees:{}",reader1);
     println!("Reader 2 sees:{}",reader2);

    //Borrowing ends here ，因为引用后续没有继续使用

    //Mutable borrowing - exclusive access
    {
        //只有一个作者可以修改notebook，在某一时刻
        let writer = &mut notebook;
        writer.push_str("\n- Discuss project timeline");
        println!("Updated content:{}",writer);


        //exclusive access，在内部，可以对notebook进行读取
        println!("Original content:{}",notebook);
        let another_reader = &notebook;

        //TODO: A reference's lifetime ends after its last usage, not necessarily at the end of its lexical scope.
        // println!("Updated content:{}",writer);

        let writer1 = &mut notebook;
        writer1.push_str("\n- Discuss project timeline");
    }

    
    let final_reader = &notebook;
    println!("Final content: {}",final_reader);

    notebook.push_str("\n-Review budgeet");

    println!("Updated content:{}",notebook);
}