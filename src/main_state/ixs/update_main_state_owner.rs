use crate::{error::PooFunError, MainState};
use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Copy)]
pub struct UpdateMainStateInput {
    owner: Pubkey,
    fee_recipient: Pubkey,
    trading_fee: u64,
}

pub fn update_main_state(
    ctx: Context<AUpdateMainState>,
    input: UpdateMainStateInput,
) -> Result<()> {
    let state = &mut ctx.accounts.main_state;
    require!(state.initialized.eq(&true), PooFunError::Uninitialized);

    state.owner = input.owner;
    state.fee_recipient = input.fee_recipient;
    state.trading_fee = input.trading_fee;

    Ok(())
}

#[derive(Accounts)]
pub struct AUpdateMainState<'info> {
    #[account(mut, address = main_state.owner @ PooFunError::Unauthorised)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [MainState::PREFIX_SEED],
        bump,
        has_one = owner,
    )]
    pub main_state: Account<'info, MainState>,
}
