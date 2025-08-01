// ---------------------------------------------------------------------------------------
// --- Constants: constants.rs
// ---------------------------------------------------------------------------------------
// programs/limitless_defi/src/constants.rs

pub const LMT_DECIMALS: u8 = 9;
pub const USD_DECIMALS: u8 = 6; // For USDC

pub const VAULT_DURATION_SECONDS: i64 = 7 * 24 * 60 * 60; // 7 days
pub const REDEMPTION_COOLDOWN_SECONDS: i64 = 7 * 24 * 60 * 60; // 7 days

pub const MAX_VAULTS: usize = 10; // Max concurrent staking vaults per user

// Seeds for PDAs
pub const GLOBAL_STATE_SEED: &[u8] = b"global_state";
pub const USER_ACCOUNT_SEED: &[u8] = b"user_account";
pub const VAULT_SEED: &[u8] = b"vault_authority";

// Price increase per minute
// 1.5% per week = 0.0001488% per minute approx.
// We use a basis points approach for precision. 1.5% = 15000 basis points
// Weekly increase in basis points: 15000
// Minutes in a week: 10080
// Increase per minute in basis points: 15000 / 10080 = 1.488...
// We'll use a fixed-point integer representation for this.
// Let's use 1488 for 0.0001488%
pub const PRICE_INCREASE_PER_MINUTE_BPS: u64 = 1488; // 0.0001488%
pub const BPS_DENOMINATOR: u64 = 1_000_000_000; // Denominator for basis points percentage
