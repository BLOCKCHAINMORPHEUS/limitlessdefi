// ---------------------------------------------------------------------------------------
// --- Instruction: initialize.rs
// ---------------------------------------------------------------------------------------
// programs/limitless_defi/src/instructions/initialize.rs
use super::*;

pub fn handler(ctx: Context<Initialize>, initial_lmt_price: u64) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    global_state.admin = *ctx.accounts.admin.key;
    global_state.treasury_wallet = *ctx.accounts.treasury_wallet.key;
    global_state.usdc_mint = *ctx.accounts.usdc_mint.key;
    global_state.sol_price_feed = *ctx.accounts.sol_price_feed.key;
    global_state.usdc_price_feed = *ctx.accounts.usdc_price_feed.key;
    global_state.initial_lmt_price = initial_lmt_price;
    global_state.launch_timestamp = Clock::get()?.unix_timestamp;
    global_state.total_lmt_supply = 0;
    global_state.fees = Fees {
        staking_fee_percent: 200, // 2.00%
        restaking_fee_percent: 150, // 1.50%
        withdrawal_fee_percent: 1500, // 15.00%
    };
    global_state.bump = *ctx.bumps.get("global_state").unwrap();
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = GlobalState::LEN,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK: This is the wallet that will receive treasury funds.
    pub treasury_wallet: AccountInfo<'info>,

    pub usdc_mint: Account<'info, Mint>,

    /// CHECK: Pyth price feed account for SOL/USD
    pub sol_price_feed: AccountInfo<'info>,
    /// CHECK: Pyth price feed account for USDC/USD
    pub usdc_price_feed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
