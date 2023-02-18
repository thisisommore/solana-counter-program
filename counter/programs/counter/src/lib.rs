use anchor_lang::prelude::*;

declare_id!("DdxQvrgHG1UQFs4u9sZw3Zz4rqU5RM4gaiigJPTiQnJf");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.value = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        // Add it to the gif_list vector.
        base_account.value += 1;
        Ok(())
    }

    pub fn decrement(ctx: Context<Increment>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        // Add it to the gif_list vector.
        base_account.value -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, CounterData>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub base_account: Account<'info, CounterData>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, CounterData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct CounterData {
    pub value: i32,
}
