use anchor_lang::prelude::*;

declare_id!("7AgtmgKkg7k8x1Vt8fp8zbsEymtn38D5RKSc3oWvuJDT");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.bump = ctx.bumps.counter;
        msg!("{}", counter.count);
        msg!("{}",counter.bump); 
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Before increment, {}",counter.count);
        counter.count = counter.count.checked_add(1).unwrap();
        msg!("After increment, {}",counter.count);
        Ok(())
    }
}
#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
    pub bump:u8, 
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init,space=8+Counter::INIT_SPACE,payer=user,seeds=[b"counter"],bump)]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut,seeds=[b"counter"],bump=counter.bump)]
    pub counter: Account<'info, Counter>,
}
