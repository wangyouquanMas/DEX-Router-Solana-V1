目标：
1. 理解workspace定义
2. 通过一个实际问题，掌握workspace用法


内容：
1.定义 
workspace是一个方法，用于去管理多个相关的packages(crates) under one project structure. 

1.1 类比
将workspace 堪称是文件夹，它拥有多个 Rust projects（packages）,但是它们被看成一个cohesive project.

1.2 举例
raydium-clmm/
├── Cargo.toml          # Workspace root
├── tokenstream_test/   # A member package
│   └── Cargo.toml
├── another_crate/
│   └── Cargo.toml


2. 实际问题
Problem: 
    I am running cargo run from inside tokenstream_test, but Rust is treating that folder as part of the main workspace (raydium-clmm). If tokenstream_test is not registered in the root Cargo.toml under [workspace], it will cause problems.

Solution: Add it to the workspace 
In raydium-clmm/Cargo.toml, add:
[workspace]
members = [
    "tokenstream_test",
    "another_crate"  # if any
]
This tells Cargo: “Hey, treat these folders as part of my workspace.”

