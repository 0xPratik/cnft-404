use crate::state::*;
use anchor_lang::prelude::*;
use mpl_bubblegum::instructions::{TransferCpi, TransferCpiAccounts, TransferInstructionArgs};
use mpl_bubblegum::ID;

#[derive(Accounts)]
pub struct ClaimCnft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, HybridPoolConfig>,
    #[account(mut,
        seeds = [b"cnft_claim_coupon".as_ref(),authority.key().as_ref()],
        bump = cnft_claim_coupon.bump,
        close = authority)]
    pub cnft_claim_coupon: Account<'info, CnftClaimCoupon>,
    #[account
    (mut,
        seeds = [b"cnft".as_ref(),pool.key().as_ref(),&cnft_claim_coupon.coupon.to_le_bytes()],
        bump = nft_store.bump,
        close = authority)]
    pub nft_store: Account<'info, NftStore>,
    /// CHECK: This account is neither written to nor read from.
    pub tree_authority: UncheckedAccount<'info>,
    ///CHECK: Checked in CPI
    pub merkle_tree: UncheckedAccount<'info>,
    ///CHECK: Checked in CPI
    pub log_wrapper: UncheckedAccount<'info>,
    #[account(address = ID  )]
    ///CHECK: Checked in CPI
    pub bubblegum_program: UncheckedAccount<'info>,
    /// CHECK: This account is checked in the CPI
    pub compression_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ClaimCnftArgs {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, ClaimCnft<'info>>,
    args: ClaimCnftArgs,
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let coupon = ctx.accounts.cnft_claim_coupon.coupon as usize;
    let coupon_bytes = ctx.accounts.cnft_claim_coupon.coupon.to_le_bytes();
    pool.items.remove(coupon);

    let pool_key = ctx.accounts.pool.key();
    let nft_store_seeds = &[
        b"cnft".as_ref(),
        pool_key.as_ref(),
        &coupon_bytes,
        &[ctx.accounts.nft_store.bump],
    ];
    let nft_store_signer = &[&nft_store_seeds[..]];

    TransferCpi::new(
        &ctx.accounts.bubblegum_program,
        TransferCpiAccounts {
            tree_config: &ctx.accounts.tree_authority.to_account_info(),
            leaf_owner: (&ctx.accounts.nft_store.to_account_info(), true),
            leaf_delegate: (&ctx.accounts.nft_store.to_account_info(), false),
            new_leaf_owner: &ctx.accounts.authority.to_account_info(),
            merkle_tree: &ctx.accounts.merkle_tree.to_account_info(),
            log_wrapper: &ctx.accounts.log_wrapper.to_account_info(),
            compression_program: &ctx.accounts.compression_program.to_account_info(),
            system_program: &ctx.accounts.system_program.to_account_info(),
        },
        TransferInstructionArgs {
            root: args.root,
            data_hash: args.data_hash,
            creator_hash: args.creator_hash,
            nonce: args.nonce,
            index: args.index,
        },
    )
    .invoke_signed_with_remaining_accounts(
        nft_store_signer,
        ctx.remaining_accounts
            .iter()
            .map(|account| (account, false, false))
            .collect::<Vec<_>>()
            .as_slice(),
    )?;

    Ok(())
}
