// ---------------------------------------------------------------------------------------
// --- Custom Errors: error.rs
// ---------------------------------------------------------------------------------------
// programs/limitless_defi/src/error.rs

use anchor_lang::prelude::*;

#[error_code]
pub enum LimitlessError {
    #[msg("Vault is still locked.")]
    VaultLocked,
    #[msg("Redemption is on cooldown.")]
    RedemptionOnCooldown,
    #[msg("Cannot redeem more than 50% of available balance.")]
    RedemptionAmountExceedsLimit,
    #[msg("Cannot stake zero tokens.")]
    ZeroStakeAmount,
    #[msg("Not enough LMT in available balance.")]
    InsufficientLmtBalance,
    #[msg("Maximum number of staking vaults reached.")]
    MaxVaultsReached,
    #[msg("Invalid bond asset specified.")]
    InvalidBondAsset,
    #[msg("Oracle price is stale.")]
    OraclePriceStale,
    #[msg("Arithmetic overflow occurred.")]
    ArithmeticOverflow,
}
