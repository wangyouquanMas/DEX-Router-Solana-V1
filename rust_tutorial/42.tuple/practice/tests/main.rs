#[cfg(test)]
mod tests {
    #[test]
    fn test_tuple() {
        let person: (&str,i32) = ("Alice",30);

        let name = person.0;
        let age = person.1;

        println!("name: {}", name);
        println!("age: {}", age);
    }
}
