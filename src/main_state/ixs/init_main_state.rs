use crate::{
    constants::{TOTAL_SUPPLY, VIRT_SOL_RESERVE},
    error::PooFunError,
    MainState,
};
use anchor_lang::prelude::*;

pub fn init_main_state(ctx: Context<AInitMainState>) -> Result<()> {
    let state = &mut ctx.accounts.main_state;
    require!(
        state.initialized.eq(&false),
        PooFunError::AlreadyInitialized
    );

    state.initialized = true;
    state.owner = ctx.accounts.owner.key();
    state.fee_recipient = ctx.accounts.owner.key();
    state.total_token_supply = TOTAL_SUPPLY;
    state.init_real_base_reserves = state.total_token_supply * 8 / 10;
    state.init_virt_base_reserves = state.total_token_supply - state.init_real_base_reserves;
    state.init_virt_quote_reserves = VIRT_SOL_RESERVE;
    state.trading_fee = 1_000;
    Ok(())
}

#[derive(Accounts)]
pub struct AInitMainState<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [MainState::PREFIX_SEED],
        bump,
        space = 8 + MainState::MAX_SIZE
    )]
    pub main_state: Account<'info, MainState>,

    pub system_program: Program<'info, System>,
}
