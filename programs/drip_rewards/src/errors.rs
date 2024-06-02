use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Raffle Not Ended")]
    RaffleNotEnded,

    #[msg("Price is too Low")]
    PriceTooLow,

    #[msg("Invalid Hash")]
    InvalidDataHash,

    #[msg("Invalid Collection cnft")]
    InvalidCollection,

    #[msg("Collection Cannot be more than 3")]
    CannotBeMoreThanThree,

    #[msg("No cnfts in pool")]
    NoCnftsInPool,
}
