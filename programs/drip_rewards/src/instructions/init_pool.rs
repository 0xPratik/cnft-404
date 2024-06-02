use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init,
        payer = authority,
        space = HybridPoolConfig::BASE_LEN,
        // change this seeds later keeping it simple for now
        seeds = [b"pool".as_ref(),authority.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, HybridPoolConfig>,
    #[account(
        constraint = mint.decimals != 0,
    )]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitPool>, collections: Vec<Pubkey>, price: u64) -> Result<()> {
    if collections.len() > MAX_COLLECTIONS {
        return Err(error!(ErrorCode::CannotBeMoreThanThree));
    }

    if price < 1000 {
        return Err(error!(ErrorCode::PriceTooLow));
    }

    ctx.accounts.pool.set_inner(HybridPoolConfig {
        collections,
        authority: ctx.accounts.authority.key(),
        price,
        token: ctx.accounts.mint.key(),
        bump: ctx.bumps.pool,
        initiated: false,
        items: vec![],
    });

    Ok(())
}
