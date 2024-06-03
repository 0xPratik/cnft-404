use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
pub mod state;

pub use error::ErrorCode;
pub use instructions::*;
pub use state::*;

declare_id!("G3A8CSd2ifSBZJym1z3LP53uqp1wZfFUHXxJp5zgtceR");

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

    pub fn swap_token_to_cnft<'info>(ctx: Context<SwapTokenToCnft>) -> Result<()> {
        swap_token_to_cnft::handler(ctx)
    }

    pub fn claim_cnft<'info>(
        ctx: Context<'_, '_, '_, 'info, ClaimCnft<'info>>,
        args: ClaimCnftArgs,
    ) -> Result<()> {
        claim_cnft::handler(ctx, args)
    }

    pub fn swap_cnft_to_token<'info>(
        ctx: Context<'_, '_, '_, 'info, SwapCnft<'info>>,
        args: SwapCnftArgs,
    ) -> Result<()> {
        swap_cnft_to_token::handler(ctx, args)
    }
}
