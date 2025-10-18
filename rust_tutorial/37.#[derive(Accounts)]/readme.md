目标：
1. 理解#[derive(Accounts)]的作用


内容：
1. 作用

In Anchor, #[derive(Accounts)] is a Rust macro applied to a struct to indicate that the struct defines the list of accounts required by an instruction. This macro automatically implements the Accounts trait for the struct, which handles account validation, deserialization, and ensures that the accounts provided to the instruction match the expected types and constraints. Each field in the struct represents an account that the instruction requires, and additional constraints can be specified using #[account(...)] attributes on each field.

1.1 举例

#[derive(Accounts)]
#[instruction(index: u16)]
pub struct CreateAmmConfig<'info> {
    /// Address to be set as protocol owner.
    #[account(
        mut,
        address = crate::admin::ID @ ErrorCode::NotApproved
    )]
    pub owner: Signer<'info>,

    /// Initialize config state account to store protocol owner address and fee rates.
    #[account(
        init,
        seeds = [
            AMM_CONFIG_SEED.as_bytes(),
            &index.to_be_bytes()
        ],
        bump,
        payer = owner,
        space = AmmConfig::LEN
    )]
    pub amm_config: Account<'info, AmmConfig>,

    pub system_program: Program<'info, System>,
}
