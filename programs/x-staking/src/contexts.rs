
use anchor_lang::prelude::*;

use anchor_spl::token::{
    Mint, 
    Token, 
    TokenAccount
};

use crate::*;
use states::*;
use constants::*;

#[derive(Accounts)]
#[instruction()]
pub struct CreateTreasury<'info> {
    #[account(
        init,
        seeds = [TREASURY_TAG, treasury_mint.key().as_ref(), authority.key().as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<Treasury>() + 8
    )]
    pub treasury: Box<Account<'info, Treasury>>,

    pub treasury_mint: Box<Account<'info, Mint>>,

    #[account(init,
        mint::decimals = treasury_mint.decimals,
        mint::authority = treasury,
        seeds = [POS_MINT_TAG, treasury.key().as_ref()],
        bump,
        payer = authority)]
    pub pos_mint: Box<Account<'info, Mint>>,

    #[account(init,
        token::mint = treasury_mint,
        token::authority = treasury,
        seeds = [TREASURY_VAULT_TAG, treasury.key().as_ref()],
        bump,
        payer = authority)]
    pub treasury_vault: Box<Account<'info, TokenAccount>>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Stake<'info> {
    #[account(
        seeds = [TREASURY_TAG, treasury.treasury_mint.as_ref(), treasury.authority.as_ref()],
        bump,
    )]
    pub treasury: Box<Account<'info, Treasury>>,
    #[account(mut,
        seeds = [POS_MINT_TAG, treasury.key().as_ref()],
        bump,
    )]
    pub pos_mint: Box<Account<'info, Mint>>,

    #[account(mut,
        seeds = [TREASURY_VAULT_TAG, treasury.key().as_ref()],
        bump
    )]
    pub treasury_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user_vault: Box<Account<'info, TokenAccount>>,

    #[account(init_if_needed,
        token::mint = pos_mint,
        token::authority = authority,
        seeds = [USER_POS_VAULT_TAG, pos_mint.key().as_ref(), authority.key().as_ref()],
        bump,
        payer = authority)]
    pub user_pos_vault:Box<Account<'info, TokenAccount>>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Redeem<'info> {
    #[account(
        seeds = [TREASURY_TAG, treasury.treasury_mint.as_ref(), treasury.authority.as_ref()],
        bump,
    )]
    pub treasury: Box<Account<'info, Treasury>>,
    #[account(mut,
        seeds = [POS_MINT_TAG, treasury.key().as_ref()],
        bump,
    )]
    pub pos_mint: Box<Account<'info, Mint>>,

    // @cryptoSyndrome
    #[account(mut, 
        seeds = [TREASURY_VAULT_TAG, treasury.key().as_ref()],
        bump
    )]
    pub treasury_vault: Box<Account<'info, TokenAccount>>,
    // @cryptoSyndrome
    #[account(mut)]
    pub user_vault: Box<Account<'info, TokenAccount>>,
    // @cryptoSyndrome
    #[account(mut)]
    pub user_pos_vault:Box<Account<'info, TokenAccount>>,
    
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
