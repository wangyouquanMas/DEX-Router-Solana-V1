目标：
1. 理解 
#[account(
        init,
        seeds = [AMM_CONFIG_SEED.as_bytes(), &index.to_be_bytes()],
        bump,
        payer = payer,
        space = AmmConfig::space(),
    )]
    pub amm_config: Account<'info, AmmConfig>, 用法

内容：
1. 内容
The reason there are multiple elements inside the #[account(...)] attribute is that each element is a constraint or instruction for how Anchor should handle and validate the account during the instruction:

- init: Instructs Anchor to initialize (create) the account if it does not exist.
- seeds: Specifies the seeds used to derive the Program Derived Address (PDA) for this account.
- bump: Tells Anchor to use the canonical bump when deriving the PDA, ensuring security and uniqueness.
- payer: Indicates which account will pay for the rent and creation fees for this new account.
- space: Specifies how much space to allocate for the account data on-chain.

Each of these constraints has a specific purpose, and together they define how the account should be created and validated in the context of the instruction. This pattern is common when initializing PDA accounts using Anchor, ensuring the account is properly created, funded, and uniquely associated with the provided seeds and bump[(1)](https://solana.com/developers/courses/onchain-development/anchor-pdas)[(2)](https://solana.com/docs/core/pda)[(3)](https://www.anchor-lang.com/docs/references/account-constraints).