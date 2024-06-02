use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct DepositTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,has_one = authority)]
    pub pool: Account<'info, HybridPoolConfig>,
    #[account(constraint = pool.token == mint.key())]
    pub mint: Account<'info, Mint>,
    #[
        account(
            init,
            payer = authority,
            associated_token::mint = mint,
            associated_token::authority = pool,
    )]
    pub pool_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::authority =  authority,
        associated_token::mint = mint,
    )]
    pub authority_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<DepositTokens>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    if pool.items.len() == 0 {
        return Err(error!(ErrorCode::NoCnftsInPool));
    }

    let total_amount: u64 =
        pool.price * pool.items.len() as u64 * 10u64.pow(ctx.accounts.mint.decimals as u32);
    let transfer_accounts = Transfer {
        authority: ctx.accounts.authority.to_account_info(),
        from: ctx.accounts.authority_token_account.to_account_info(),
        to: ctx.accounts.pool_token_account.to_account_info(),
    };
    let token_program = ctx.accounts.token_program.to_account_info();
    let transfer_context = CpiContext::new(token_program, transfer_accounts);
    token::transfer(transfer_context, total_amount)?;

    pool.initiated = true;

    Ok(())
}
