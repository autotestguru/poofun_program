use anchor_lang::prelude::error_code;

#[error_code]
pub enum PooFunError {
    #[msg("Uninitialized")]
    Uninitialized,

    #[msg("AlreadyInitialized")]
    AlreadyInitialized,

    #[msg("Unauthorised")]
    Unauthorised,

    #[msg("Insufficient fund")]
    InsufficientFund,

    #[msg("One token should be Sol")]
    UnknownToken,

    #[msg("BondingCurve incomplete")]
    BondingCurveIncomplete,

    #[msg("BondingCurve complete")]
    BondingCurveComplete,
}
