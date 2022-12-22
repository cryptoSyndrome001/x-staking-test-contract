
use anchor_lang::prelude::*;

declare_id!("9XrdGhMegRkXfTYPiMpBkPjHmJjxexbriLKVJo5vXDo3");

mod processors;
pub mod errors;
pub mod events;
pub mod states;
pub mod utils;
pub mod contexts;
pub mod constants ;

pub use contexts::*;
pub use utils::*;

#[program] 
pub mod x_staking {
    use super::*;

    pub fn create_treasury(
        _ctx: Context<CreateTreasury>,
    ) -> Result<()> {
        _ctx.accounts.create_treasury()
    }
    
    pub fn stake(
        _ctx: Context<Stake>,
        amount: u64,
    ) -> Result<()> {
        _ctx.accounts.stake(
            amount,
        )
    }
    pub fn redeem(
        _ctx: Context<Redeem>,
        amount: u64,
    ) -> Result<()> {
        _ctx.accounts.redeem(
            amount,
        )
    }
}
