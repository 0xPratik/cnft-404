use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;

pub use error::ErrorCode;
pub use instructions::*;
pub use state::*;

declare_id!("FChXDk5krMuZUcYoYQq7KxDcwwzSAHwJwq6kMTvN9Kbc");

#[program]
pub mod drip_rewards {

    use super::*;

    pub fn init_pool(ctx: Context<InitPool>, collections: Vec<Pubkey>, price: u64) -> Result<()> {
        init_pool::handler(ctx, collections, price)
    }

    pub fn deposit_tokens(ctx: Context<DepositTokens>) -> Result<()> {
        deposit_token::handler(ctx)
    }

    pub fn deposit_cnft<'info>(
        ctx: Context<'_, '_, '_, 'info, DepositCnft<'info>>,
        args: DepositCnftArgs,
    ) -> Result<()> {
        deposit_cnft::handler(ctx, args)
    }
}
