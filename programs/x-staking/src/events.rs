//! Crate events

use anchor_lang::prelude::*;

//@cryptoSyndrome
#[event]
pub struct TreasuryCreated {
  pub authority: Pubkey,
  pub treasury: Pubkey,
  pub treasury_mint: Pubkey,
  pub pos_mint: Pubkey,
  pub treasury_vault: Pubkey
}

#[event]
pub struct Deposited {
  pub user: Pubkey,
  pub treasury: Pubkey,
  pub treasury_mint: Pubkey,
  pub deposit_amount: u64
}

#[event]
pub struct Claimed {
  pub user: Pubkey,
  pub treasury: Pubkey,
  pub treasury_mint: Pubkey,
  pub claim_amount: u64
}