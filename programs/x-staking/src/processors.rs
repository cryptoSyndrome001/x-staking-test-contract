use anchor_lang::prelude::*;
use anchor_spl::token::{self,  Transfer, MintTo, Burn};
use crate::*;
use constants::*;
use utils::*;
use events::*;
use errors::*;

impl<'info> CreateTreasury<'info>{
    pub fn create_treasury(&mut self, 
    ) -> Result<()> {

        let treasury = &mut self.treasury;
        treasury.authority = self.authority.key();
        treasury.treasury_mint = self.treasury_mint.key();
        treasury.treasury_vault = self.treasury_vault.key();
        treasury.pos_mint = self.pos_mint.key();

        //@cryptoSyndrome
        emit!(TreasuryCreated {
            authority: self.authority.key(),
            treasury: self.treasury.key(),
            treasury_mint: self.treasury_mint.key(),
            treasury_vault: self.treasury_vault.key(),
            pos_mint: self.pos_mint.key()
        });
        Ok(())
    }    
}

impl<'info> Stake<'info>{
    pub fn stake(&mut self, 
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, XError::NotAllowed);

        let treasury = &mut self.treasury;

        let bump = get_bump(&[TREASURY_TAG,treasury.treasury_mint.as_ref(), treasury.authority.as_ref()], &crate::ID);
        let signer_seeds = &[
            TREASURY_TAG, 
            treasury.treasury_mint.as_ref(), 
            treasury.authority.as_ref(),
            &[bump],
        ];
        let signer = &[&signer_seeds[..]];

        // transfer from user to pool
        let cpi_accounts = Transfer {
            from: self.user_vault.to_account_info(),
            to: self.treasury_vault.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        // mint pos token to user
        let cpi_program_pos = self.token_program.to_account_info();
        let cpi_accounts_pos = MintTo {
            mint: self.pos_mint.to_account_info(),
            to: self.user_pos_vault.to_account_info(),
            authority: treasury.to_account_info(),
        };
        let cpi_ctx_pos = CpiContext::new_with_signer(cpi_program_pos, cpi_accounts_pos, signer);
        token::mint_to(cpi_ctx_pos, amount)?;

        //@cryptoSyndrome
        emit!(Deposited {
            user: self.authority.key(),
            treasury: self.treasury.key(),
            treasury_mint: self.treasury.treasury_mint.key(),
            deposit_amount: amount
        });

        Ok(())
    }
}

impl<'info> Redeem<'info>{
    pub fn redeem(&mut self, 
        amount: u64, 
    ) -> Result<()> {
        require!(amount > 0, XError::NotAllowed);

        let treasury = &mut self.treasury;

        let bump = get_bump(&[TREASURY_TAG,treasury.treasury_mint.as_ref(), treasury.authority.as_ref()], &crate::ID);
        let signer_seeds = &[
            TREASURY_TAG, 
            treasury.treasury_mint.as_ref(), 
            treasury.authority.as_ref(),
            &[bump],
        ];
        let signer = &[&signer_seeds[..]];

        // burn pos token
        let cpi_accounts_pos = Burn {
            mint: self.pos_mint.to_account_info(),
            from: self.user_pos_vault.to_account_info(),
            authority: self.authority.to_account_info(),
        };

        let cpi_program_pos = self.token_program.to_account_info();
        let cpi_ctx_pos = CpiContext::new_with_signer(cpi_program_pos, cpi_accounts_pos, signer);
        token::burn(cpi_ctx_pos, amount)?;

        // transfer pool to user
        let cpi_accounts = Transfer {
            from: self.treasury_vault.to_account_info(),
            to: self.user_vault.to_account_info(),
            authority: treasury.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, amount)?;

        //@cryptoSyndrome
        emit!(Claimed {
            user: self.authority.key(),
            treasury: self.treasury.key(),
            treasury_mint: self.treasury.treasury_mint.key(),
            claim_amount: amount
        });

        Ok(())
    }
}
