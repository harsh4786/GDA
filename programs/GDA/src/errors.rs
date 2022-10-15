use anchor_lang::prelude::*;

#[error_code]
pub enum AuctioneerError {
    // 6000
    #[msg("Bump seed not in hash map")]
    BumpSeedNotInHashMap,

    // 6001
    #[msg("Auction has not started yet")]
    AuctionNotStarted,

    // 6002
    #[msg("Auction has ended")]
    AuctionEnded,

    // 6003
    #[msg("Auction has not ended yet")]
    AuctionActive,

    // 6004
    #[msg("The signer must be the Auction House authority")]
    SignerNotAuth,

    // 6005
    #[msg("We're all sold out!")]
    SoldOut,
}