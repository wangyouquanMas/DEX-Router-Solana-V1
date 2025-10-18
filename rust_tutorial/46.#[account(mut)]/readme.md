目标：
1. 理解#[account(mut)]

内容:
1. 使用

The #[account(mut)] macro attribute in Anchor is used on an account field within an accounts struct to specify that the account must be mutable. This means the program is allowed to modify the account's data or its lamports (SOL balance) during the instruction execution. Without #[account(mut)], the program would only be able to read from the account, not change it[(1)](https://www.anchor-lang.com/docs/references/account-constraints)[(2)](https://www.anchor-lang.com/docs/basics/program-structure)[(3)](https://github.com/solana-foundation/anchor/blob/master/lang/derive/accounts/src/lib.rs).

Example:
```rust
#[account(mut)]
pub user: Signer<'info>,
```
This line ensures that the user account can be changed (for example, to update data or transfer lamports) during the instruction[(1)](https://www.anchor-lang.com/docs/references/account-constraints)[(2)](https://www.anchor-lang.com/docs/basics/program-structure)[(3)](https://github.com/solana-foundation/anchor/blob/master/lang/derive/accounts/src/lib.rs).