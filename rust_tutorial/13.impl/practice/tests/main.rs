trait Speak{
    fn speak(&self);
}


struct Person{
    name: String,
}

impl Speak for Person{
    fn speak(&self){
        println!("My name is {}",self.name);
    }
}

#[test]
fn Test_impl(){
    let person = Person{name: String::from("Alice")};
    person.speak();
}