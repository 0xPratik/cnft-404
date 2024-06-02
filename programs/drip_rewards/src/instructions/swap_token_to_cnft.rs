use anchor_lang::{prelude::*, solana_program::sysvar};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use arrayref::array_ref;

use crate::{CnftClaimCoupon, HybridPoolConfig};

#[derive(Accounts)]
pub struct SwapTokenToCnft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, HybridPoolConfig>,
    #[account(constraint = pool.token == mint.key())]
    pub mint: Account<'info, Mint>,
    #[
        account(
            mut,
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

    #[account(
        init,
        space = 4,
        payer = authority,
        seeds = [b"cnft_claim_coupon".as_ref(),authority.key().as_ref()],
        bump,
    )]
    pub cnft_claim_coupon: Account<'info, CnftClaimCoupon>,
    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = sysvar::slot_hashes::id())]
    pub recent_slot_hashes: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SwapTokenToCnft>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let amount = pool.price * 10u64.pow(ctx.accounts.mint.decimals as u32);

    let transfer_accounts = Transfer {
        authority: ctx.accounts.authority.to_account_info(),
        from: ctx.accounts.authority_token_account.to_account_info(),
        to: ctx.accounts.pool_token_account.to_account_info(),
    };
    let token_program = ctx.accounts.token_program.to_account_info();
    let transfer_context = CpiContext::new(token_program, transfer_accounts);
    token::transfer(transfer_context, amount)?;

    //simple Random Generator
    let recent_slothashes = &ctx.accounts.recent_slot_hashes;
    let data = recent_slothashes.data.borrow();
    let most_recent = array_ref![data, 12, 8];

    let clock = Clock::get()?;
    let seed = u64::from_le_bytes(*most_recent).saturating_sub(clock.unix_timestamp as u64);
    let max_result = 255;
    let result = seed as u8 % max_result + 1;
    let nearest = pool
        .items
        .iter()
        .min_by_key(|&&item| (item as u8 - result as u8))
        .unwrap_or(&result);
    ctx.accounts.cnft_claim_coupon.coupon = *nearest;
    ctx.accounts.cnft_claim_coupon.bump = ctx.bumps.cnft_claim_coupon;

    Ok(())
}
