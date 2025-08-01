// =======================================================================================
//
//  Limitless Defi - Solana Smart Contract (Anchor Framework)
//
//  This program implements the complete logic for the Limitless Defi protocol,
//  including all custom rules for bonding, staking, and redemption.
//
//  Author: Gemini
//  Date: August 1, 2025
//
// =======================================================================================

// ---------------------------------------------------------------------------------------
// --- Program Entrypoint & Main Module: lib.rs
// ---------------------------------------------------------------------------------------
// programs/limitless_defi/src/lib.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use pyth_sdk_solana::load_price_feed_from_account_info;

// Import all our custom modules
mod state;
mod error;
mod constants;
mod instructions;

// Use our defined modules
use state::*;
use error::LimitlessError;
use constants::*;
use instructions::*;

// This is the on-chain program ID. Anchor will update this when you build.
declare_id!("LimitlessDeFiProgramId111111111111111111111");

#[program]
pub mod limitless_defi {
    use super::*;

    /// Initializes the entire protocol. This can only be called once by the admin.
    /// It sets up the global state, including the treasury wallet, fees, and initial price.
    pub fn initialize(
        ctx: Context<Initialize>,
        initial_lmt_price: u64,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, initial_lmt_price)
    }

    /// Creates a user account PDA. This must be called before a user can interact
    /// with the protocol. It's like registering for the service.
    pub fn create_user_account(ctx: Context<CreateUserAccount>) -> Result<()> {
        instructions::create_user_account::handler(ctx)
    }

    /// Bonds assets (USDC or SOL) to mint LMT. This is the only way LMT is created.
    /// It uses the Pyth oracle to get the real-time price of SOL.
    pub fn bond(ctx: Context<Bond>, amount: u64, bond_asset: BondAsset) -> Result<()> {
        instructions::bond::handler(ctx, amount, bond_asset)
    }

    /// Stakes LMT from a user's available balance into a new 7-day vault.
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake::handler(ctx, amount)
    }
    
    /// Checks all of a user's vaults and moves any matured funds (principal + rewards)
    /// back to their available balance. This is a manual-call function for the user.
    pub fn mature_vaults(ctx: Context<MatureVaults>) -> Result<()> {
        instructions::mature_vaults::handler(ctx)
    }

    /// Redeems LMT from the user's available balance for USDC from the treasury.
    /// Enforces the 50% limit and the 7-day redemption cooldown.
    pub fn redeem(ctx: Context<Redeem>, amount: u64) -> Result<()> {
        instructions::redeem::handler(ctx, amount)
    }

    // --- Admin Functions ---

    /// Allows the admin to update fee percentages.
    pub fn update_fees(ctx: Context<UpdateAdmin>, fees: Fees) -> Result<()> {
        instructions::admin::update_fees(ctx, fees)
    }

    /// Allows the admin to update the treasury wallet.
    pub fn update_treasury_wallet(ctx: Context<UpdateAdmin>, new_treasury: Pubkey) -> Result<()> {
        instructions::admin::update_treasury_wallet(ctx, new_treasury)
    }
}
