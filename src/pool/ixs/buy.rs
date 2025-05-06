use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{self,Mint, Token, TokenAccount, Transfer}};
use crate::{
    constants::REAL_SOL_THRESHOLD, 
    MainState, PoolState, 
    TradeEvent, CompleteEvent, 
    error::PooFunError, 
    main_state, 
    utils::{calculate_trading_fee, close_token_account, sync_native_amount},
};

pub fn buy(ctx:Context<ABuy>, amount: u64)->Result<()>{
    let main_state = &mut ctx.accounts.main_state;
    require!(main_state.initialized.eq(&true), PooFunError::Uninitialized);
    
    let pool_state = &mut ctx.accounts.pool_state;
    require!(pool_state.complete.eq(&false), PooFunError::BondingCurveComplete);

    let buyer = ctx.accounts.buyer.to_account_info();
    let buyer_base_ata = &ctx.accounts.buyer_base_ata;
    let buyer_quote_ata = &ctx.accounts.buyer_quote_ata;
    let token_program = ctx.accounts.token_program.to_account_info();
    let system_program = ctx.accounts.system_program.to_account_info();
    
    sync_native_amount(buyer.clone(), &buyer_quote_ata, amount, system_program.clone(), token_program.clone())?;

    let fee = calculate_trading_fee(main_state.trading_fee, amount);
    let input_amount = amount - fee;
    let output_amount = pool_state.compute_receivable_amount_on_buy(input_amount);
    
    // sending fee
    let fee_transfer_cpi_account = Transfer{
        from: buyer_quote_ata.to_account_info(),
        to: ctx.accounts.fee_quote_ata.to_account_info(),
        authority: buyer.clone()
    };
    token::transfer(CpiContext::new(token_program.clone(), fee_transfer_cpi_account), fee)?;
    
    // sending input amount (sol)
    let input_amount_transfer_cpi_account = Transfer{
        from: buyer_quote_ata.to_account_info(),
        to: ctx.accounts.reserver_quote_ata.to_account_info(),
        authority: buyer.clone()
    };
    token::transfer(CpiContext::new(token_program.clone(), input_amount_transfer_cpi_account), input_amount)?;
    
    // sending tokens from reserve ata (meme)
    let output_amount_transfer_cpi_account = Transfer{
        from: ctx.accounts.reserver_base_ata.to_account_info(),
        to: buyer_base_ata.to_account_info(),
        authority: pool_state.to_account_info()
    };
    token::transfer(CpiContext::new_with_signer(token_program.clone(), output_amount_transfer_cpi_account, &[&[
        PoolState::PREFIX_SEED,
        pool_state.base_mint.as_ref(),
        pool_state.quote_mint.as_ref(),
        &[ctx.bumps["pool_state"]]
    ]]), output_amount)?;

    // unwrap sol (or closing token account)
    close_token_account(buyer.clone(), buyer_quote_ata.to_account_info(), token_program)?;

    emit!(TradeEvent {
        user: buyer.key(), 
        base_mint: pool_state.base_mint, 
        // quote_mint: pool_state.quote_mint, 
        token_amount: output_amount, 
        sol_amount: amount, 
        base_reserves: pool_state.real_base_reserves + pool_state.virt_base_reserves, 
        quote_reserves: pool_state.virt_quote_reserves + pool_state.real_quote_reserves, 
        is_buy: true, 
        timestamp: Clock::get()?.unix_timestamp,
    });

    if (pool_state.real_quote_reserves >= REAL_SOL_THRESHOLD) {
        pool_state.complete = true;
        
        emit!(CompleteEvent {
            user: buyer.key(), 
            base_mint: pool_state.base_mint, 
            // quote_mint: pool_state.quote_mint, 
            timestamp: Clock::get()?.unix_timestamp,
        });
    }

    Ok(())
}

#[derive(Accounts)]
pub struct ABuy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        seeds = [MainState::PREFIX_SEED],
        bump,
    )]
    pub main_state: Box<Account<'info, MainState>>,
    
    #[account(mut, address = main_state.fee_recipient,)]
    /// CHECK: this should be set by admin
    pub fee_recipient: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = quote_mint,
        associated_token::authority = fee_recipient,
    )]
    /// CHECK: this should be set by fee_recipient
    pub fee_quote_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [
            PoolState::PREFIX_SEED,
            base_mint.key().as_ref(), 
            quote_mint.key().as_ref(),
        ],
        bump,
    )]
    pub pool_state: Box<Account<'info, PoolState>>,

    #[account(address = pool_state.base_mint)]
    pub base_mint: Box<Account<'info, Mint>>,
    #[account(address = pool_state.quote_mint)]
    pub quote_mint: Box<Account<'info, Mint>>,
    
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = base_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_base_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = quote_mint,
        associated_token::authority = buyer,
    )]
    pub buyer_quote_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = base_mint,
        associated_token::authority = pool_state,
    )]
    pub reserver_base_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = quote_mint,
        associated_token::authority = pool_state,
    )]
    pub reserver_quote_ata: Box<Account<'info, TokenAccount>>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
