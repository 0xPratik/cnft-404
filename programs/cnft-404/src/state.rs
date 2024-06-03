use anchor_lang::prelude::*;
use mpl_bubblegum::types::Collection as BubblegumCollection;
use mpl_bubblegum::types::Creator as BubblegumCreator;
use mpl_bubblegum::types::TokenProgramVersion as BubblegumTokenProgramVersion;
use mpl_bubblegum::types::TokenStandard as BubblegumTokenStandard;
use mpl_bubblegum::types::UseMethod as BubblegumUseMethod;
use mpl_bubblegum::types::Uses as BubblegumUses;

pub const MAX_COLLECTIONS: usize = 3;
pub const MAX_ITEMS_IN_POOL: usize = 256;

#[account]
#[derive(Default)]
pub struct NftStore {
    pub asset_id: Pubkey,
    pub bump: u8,
}

#[account]
pub struct CnftClaimCoupon {
    pub bump: u8,
    pub coupon: u8,
}

#[account]
#[derive(Default)]
pub struct HybridPoolConfig {
    pub authority: Pubkey,
    pub bump: u8,
    pub token: Pubkey,

    pub price: u64,
    pub collections: Vec<Pubkey>,
    pub initiated: bool,
    pub items: Vec<u8>,
}

impl HybridPoolConfig {
    pub const BASE_LEN: usize =
        8 + std::mem::size_of::<Self>() + 4 + (MAX_COLLECTIONS * 32) + 4 + (MAX_ITEMS_IN_POOL * 1);
    // change this number to something bigger if this exp gets bigger.
    pub const MIN_CNFTS: u8 = 5;
}

// things needed next would be instructions like
// 1. create pool
// 2. swap from pool
// 3. next will think about it.

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum TokenStandard {
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
}

impl TokenStandard {
    pub fn convert(&self) -> BubblegumTokenStandard {
        match self {
            TokenStandard::NonFungible => BubblegumTokenStandard::NonFungible,
            TokenStandard::FungibleAsset => BubblegumTokenStandard::FungibleAsset,
            TokenStandard::Fungible => BubblegumTokenStandard::Fungible,
            TokenStandard::NonFungibleEdition => BubblegumTokenStandard::NonFungibleEdition,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Collection {
    pub verified: bool,
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub key: Pubkey,
}

impl Collection {
    pub fn convert(&self) -> BubblegumCollection {
        BubblegumCollection {
            verified: self.verified,
            key: self.key,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Creator {
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub address: Pubkey,
    pub verified: bool,
    /// The percentage share.
    ///
    /// The value is a percentage, not basis points.
    pub share: u8,
}

impl Creator {
    pub fn convert(&self) -> BubblegumCreator {
        BubblegumCreator {
            address: self.address,
            verified: self.verified,
            share: self.share,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct Uses {
    pub use_method: UseMethod,
    pub remaining: u64,
    pub total: u64,
}

impl Uses {
    pub fn convert(&self) -> BubblegumUses {
        BubblegumUses {
            use_method: self.use_method.convert(),
            remaining: self.remaining,
            total: self.total,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

impl UseMethod {
    pub fn convert(&self) -> BubblegumUseMethod {
        match self {
            UseMethod::Burn => BubblegumUseMethod::Burn,
            UseMethod::Multiple => BubblegumUseMethod::Multiple,
            UseMethod::Single => BubblegumUseMethod::Single,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum TokenProgramVersion {
    Original,
    Token2022,
}

impl TokenProgramVersion {
    pub fn convert(&self) -> BubblegumTokenProgramVersion {
        match self {
            TokenProgramVersion::Original => BubblegumTokenProgramVersion::Original,
            TokenProgramVersion::Token2022 => BubblegumTokenProgramVersion::Token2022,
        }
    }
}
