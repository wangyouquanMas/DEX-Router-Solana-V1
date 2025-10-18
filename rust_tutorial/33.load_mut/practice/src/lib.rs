#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWxTWqCkQH1fSP4TLuxpZxyvozP"); // valid 32-byte pubkey

#[account(zero_copy)]
pub struct State {
    pub counter: u64,
}

#[program]
pub mod counter {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let mut state = ctx.accounts.state.load_init()?;
        state.counter = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let mut state = ctx.accounts.state.load_mut()?; // <-- practice target
        //load_mut() gives you a mutable borrow guard to the zero-copy account data
        //what it is: a smart pointer that dereferences like &mut State. 
        state.counter = state.counter.checked_add(1).ok_or(ErrorCode::Overflow)?;
        println!("counter: {}", state.counter);
        // state.counter +=1;
        // println!("counter after: {}", state.counter);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Init<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + core::mem::size_of::<State>(),
        seeds = [b"state"],
        bump
    )]
    pub state: AccountLoader<'info, State>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds=[b"state"], bump)]
    pub state: AccountLoader<'info, State>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Counter overflowed")]
    Overflow,
}
