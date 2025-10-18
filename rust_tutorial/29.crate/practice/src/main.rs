mod utils;
mod hash;

use crate::utils::greet; // Importing the `greet` function from the `utils.rs` module
use crate::hash::hash;    // Importing the `hash` function from hash.rs

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{namespace}:{name}");
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&hash(preimage.as_bytes())[..8]);  // Use the hash function here
    sighash
}

fn main() {
    let name = "Alice";
    let greeting = greet(name);
    println!("{}", greeting); // Output: Hello, Alice!

    // Test sighash function
    let namespace = "namespace";
    let name = "name";
    let result = sighash(namespace, name);
    println!("{:?}", result);  // Output will depend on hash function logic
}