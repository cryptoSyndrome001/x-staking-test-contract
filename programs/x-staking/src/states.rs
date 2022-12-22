
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Treasury {
    pub authority: Pubkey,
    pub treasury_mint: Pubkey,
    pub pos_mint: Pubkey,
    pub treasury_vault: Pubkey,
}
