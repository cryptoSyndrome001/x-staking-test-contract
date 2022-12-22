//! Crate events

use anchor_lang::prelude::*;


#[event]
pub struct TreasuryCreated {
}

#[event]
pub struct Deposited {
}

#[event]
pub struct Claimed {
}