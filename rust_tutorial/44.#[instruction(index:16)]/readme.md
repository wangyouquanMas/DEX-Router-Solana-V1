目标：
1. 理解什么是#[instruction(index: u16)] 

内容:
1. 使用

Here is an intuitive example using #[instruction(index: u16)] in an Anchor program. This macro allows you to use the instruction argument index as part of the account constraints, such as PDA seed derivation.

Suppose you want to create a PDA account whose address depends on both the user's public key and an index argument passed to your instruction. You would define your accounts struct like this:

```rust
#[derive(Accounts)]
#[instruction(index: u16)]
pub struct Example<'info> {
    #[account(
        init,
        seeds = [b"example", user.key().as_ref(), &index.to_le_bytes()],
        bump,
        payer = user,
        space = 8 + 8
    )]
    pub pda_account: Account<'info, SomeAccountType>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```
[(1)](https://solana.com/developers/courses/onchain-development/anchor-pdas)

In this example:
- #[instruction(index: u16)] tells Anchor that the struct will receive an index argument from the instruction.
- The index value is then used in the seeds array to derive the PDA address, making each PDA unique for a (user, index) pair.

This pattern is useful when you want to allow users to create multiple accounts, each identified by a different index, and you want the PDA address to be predictable and derived from both the user and the index.