目标：
1. AccountLoader 定义
2. 作用

内容：
1. 定义
AccountLoader<'info,T> is a special Anchor type used for zero-copy accounts in Solana programs. 

2. 作用
It allows you to efficiently access(read or modify) large account data structures by mapping the account's bytes directly into a Rust struct, instead of deserializing the entire account into memory.  This is especially useful for large accounts that use  the #[account(zero_copy)] attribute. 

When you declare a field like pub state: AccountLoader<'info, State>, you are telling Anchor to treat the state account as a zero-copy account of type State, and you can use methods like load(), load_mut(), and load_init() to read or write the account data efficiently. 

3. 什么是zero-copy account ?
Zero-copy accounts in Solana Anchor are accounts that use a special deserialization technique allowing your program to read or write account data directly from memory, without copying it into a new Rust struct. This is enabled by annotating your struct with #[account(zero_copy)]. Zero-copy accounts are especially useful for handling large accounts, as they avoid stack and heap limitations by mapping account data directly into your code’s data structures. You interact with these accounts using AccountLoader and methods like load(), load_mut(), or load_init()