use crate::errors::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
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
pub struct DepositCnft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut,has_one = authority)]
    pub pool: Account<'info, HybridPoolConfig>,
    #[account(
        init,
        seeds = [b"cnft".as_ref(),pool.key().as_ref(),&pos.to_le_bytes()],
        bump,
        space = 33,
        payer = authority,
    )]
    pub nft_store: Account<'info, NftStore>,
    #[account(
        seeds = [merkle_tree.key().as_ref()],
        bump,
        seeds::program = mpl_bubblegum::ID
    )]
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
pub struct AnchorMetadataArgs {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    /// nonce for easy calculation of editions, if present
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TokenStandard>,
    /// Collection
    pub collection: Option<Collection>,
    /// Uses
    pub uses: Option<Uses>,
    pub token_program_version: TokenProgramVersion,
    pub creators: Vec<Creator>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DepositCnftArgs {
    pub root: [u8; 32],
    pub data_hash: [u8; 32],
    pub creator_hash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub pos: u8,
    pub metadata: AnchorMetadataArgs,
}

pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositCnft<'info>>,
    args: DepositCnftArgs,
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
    let asset_id = get_asset_id(&ctx.accounts.merkle_tree.key(), args.nonce);
    let pool = &mut ctx.accounts.pool;
    ctx.accounts.nft_store.set_inner(NftStore {
        asset_id: asset_id,
        bump: ctx.accounts.nft_store.bump,
    });

    pool.items.push(args.pos);

    msg!("{:?}", asset_id);

    Ok(())
}
