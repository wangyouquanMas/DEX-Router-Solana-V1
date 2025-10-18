目标：
1. crate定义
2. crate使用方法
3. crate和module的关系

内容：
1. create定义
    crate is the fundamental unit of code distribution. A crate is essentially a package or library in Rust

1.1 能够指向整个项目中的module 
    crate does not refer only to the current file. In Rust, crate refers to the entire current package or project, not just a specific file. It's a way to reference the root of your crate, meaning it allows access to all modules defined within your project, regardless of the specific file they are in.
    If you're in main.rs (or any other file), crate:: can be used to access modules and items that are defined anywhere in the current crate (the entire project, not just the current file).




2. crate使用方法
    crate::hash::hash refers to the hash function inside the hash module in the same crate.
    1）crate refers to the current crate (your project).
    2）hash is the module (defined inside the crate).
    3）hash (the function) is a function inside the hash module.
The use of crate::hash::hash tells Rust to look for a hash module in the current crate and find the hash function within that module.

3. crate和module的关系
Rust's module system allows for organizing code into modules, and crates are containers for these modules. You can structure your crate into multiple modules and submodules for better organization.

3.1用法
1) 如果是其它文件中的module,需要在当前文件中声明：
mod utils;  // Declares the `utils.rs` file as a module
mod hash;   // Declares the `hash.rs` file as a module

use crate::utils::greet;  // Importing the `greet` function from utils.rs
use crate::hash::hash;    // Importing the `hash` function from hash.rs

2) 如何是当前文件显式声明的mod,直接使用 use create::mod::fn;
For instance:
You might have a crate with a module for file I/O, another for hashing, and yet another for network communication.
By using crate::, you can reference these modules from anywhere inside your crate

pub mod crypto {
    pub mod hash {
        pub fn hash(input: &[u8]) -> [u8; 32] {
            let mut result = [0u8; 32];
            for (i, &byte) in input.iter().enumerate() {
                result[i % 32] = byte;  // Mock hash
            }
            result
        }
    }
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{namespace}:{name}");
    let mut sighash = [0u8; 8];

    // Accessing the hash function inside the submodule `crypto::hash`
    sighash.copy_from_slice(&crate::crypto::hash::hash(preimage.as_bytes())[..8]);
    sighash
}

