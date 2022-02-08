use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_anchor {
    use super::*;
    pub fn set_data(ctx: Context<SetData>, data: MyAccount) -> ProgramResult {
        require!(data.data < 100, MyError::DataTooLarge);
        ctx.accounts.my_account.set_inner(data);
        Ok(())
    }
}

#[error]
pub enum MyError {
    #[msg("MyAccount may only hold data below 100")]
    DataTooLarge,
}

#[account]
#[derive(Default)]
pub struct MyAccount {
    data: u64,
    age: u8,
}

/* #[derive(AnchorSerialize, AnchorDeserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Data {
    data: u64,
    age: u64,
} */

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
    // constraints
    /* #[account(
        constraint = my_account.mint == token_account.mint,
        has_one = owner
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub owner: Signer<'info>, */
}
