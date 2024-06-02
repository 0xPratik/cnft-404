use crate::errors::ErrorCode;
use crate::state::*;
use crate::AnchorMetadataArgs;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;
use mpl_bubblegum::hash::hash_metadata;
use mpl_bubblegum::instructions::TransferCpi;
use mpl_bubblegum::instructions::TransferCpiAccounts;
use mpl_bubblegum::instructions::TransferInstructionArgs;
use mpl_bubblegum::types::MetadataArgs;

use mpl_bubblegum::utils::get_asset_id;
use mpl_bubblegum::ID;

#[derive(Accounts)]
#[instruction(
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
    pos: u8
)]
pub struct SwapCnft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, HybridPoolConfig>,
    #[account(
        init,
        seeds = [b"cnft".as_ref(),pool.key().as_ref(),&pos.to_le_bytes()],
        bump,
        space = 33,
        payer = authority,
    )]
    pub nft_store: Account<'info, NftStore>,
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
        init_if_needed,
        payer = authority,
        associated_token::authority =  authority,
        associated_token::mint = mint,
    )]
    pub authority_token_account: Account<'info, TokenAccount>,
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
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwapCnftArgs {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub pos: u8,
    pub metadata: AnchorMetadataArgs,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, SwapCnft<'info>>,
    args: SwapCnftArgs,
) -> Result<()> {
    let collections = &ctx.accounts.pool.collections;

    let mpl_token_standard = args.metadata.token_standard.map(|ts| ts.convert());
    let mpl_collection = args.metadata.collection.map(|c| c.convert());
    let mpl_creators = args.metadata.creators.iter().map(|c| c.convert()).collect();
    let mpl_token_program_version = args.metadata.token_program_version.convert();
    let mpl_uses = args.metadata.uses.map(|u| u.convert());

    let metadata = MetadataArgs {
        name: args.metadata.name,
        symbol: args.metadata.symbol,
        uri: args.metadata.uri,
        seller_fee_basis_points: args.metadata.seller_fee_basis_points,
        primary_sale_happened: args.metadata.primary_sale_happened,
        is_mutable: args.metadata.is_mutable,
        edition_nonce: args.metadata.edition_nonce,
        token_standard: mpl_token_standard,
        collection: mpl_collection,
        uses: mpl_uses,
        token_program_version: mpl_token_program_version,
        creators: mpl_creators,
    };

    // Verification if legit
    let incoming_data_hash = hash_metadata(&metadata)?;
    if incoming_data_hash != args.data_hash {
        return Err(error!(ErrorCode::InvalidDataHash));
    }

    if let Some(collection) = &metadata.collection {
        if !collections.contains(&collection.key) {
            return Err(error!(ErrorCode::InvalidCollection));
        }
    }
    TransferCpi::new(
        &ctx.accounts.bubblegum_program,
        TransferCpiAccounts {
            tree_config: &ctx.accounts.tree_authority.to_account_info(),
            leaf_owner: (&ctx.accounts.authority.to_account_info(), true),
            leaf_delegate: (&ctx.accounts.authority.to_account_info(), false),
            new_leaf_owner: &ctx.accounts.nft_store.to_account_info(),
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
    .invoke_with_remaining_accounts(
        ctx.remaining_accounts
            .iter()
            .map(|account| (account, false, false))
            .collect::<Vec<_>>()
            .as_slice(),
    )?;

    let pool = &mut ctx.accounts.pool;
    let amount = pool.price * 10u64.pow(ctx.accounts.mint.decimals as u32);
    let pool_authority = pool.authority.key();
    let pool_seeds = &[
        b"pool".as_ref(),
        pool_authority.as_ref(),
        &[ctx.accounts.pool.bump],
    ];
    let pool_signer = &[&pool_seeds[..]];

    let transfer_accounts = Transfer {
        authority: ctx.accounts.authority.to_account_info(),
        from: ctx.accounts.pool_token_account.to_account_info(),
        to: ctx.accounts.authority_token_account.to_account_info(),
    };
    let token_program = ctx.accounts.token_program.to_account_info();
    let transfer_context =
        CpiContext::new_with_signer(token_program, transfer_accounts, pool_signer);
    token::transfer(transfer_context, amount)?;

    Ok(())
}
