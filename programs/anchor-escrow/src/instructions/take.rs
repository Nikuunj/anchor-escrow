use crate::Escrow;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, close_account, CloseAccount},
};

#[derive(Accounts)]
pub struct Take<'info> {

    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,


    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,


    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,


    #[account(
        mut, 
        close = maker,
        has_one = mint_a,
        has_one = maker,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), &escrow.seed.to_le_bytes()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,


    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn take(&mut self) -> Result<()> {

        let signer_seeds: &[&[&[u8]]] = &[&[b"escrow", self.maker.to_account_info().key.as_ref(), &self.escrow.seed.to_le_bytes(),  &[self.escrow.bump]]];
        let transfer_accounts_taker_to_maker_b = TransferChecked {
            mint: self.mint_b.to_account_info(),
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info()
        };

        let transfer_account_vault_to_taker_a = TransferChecked {
            mint: self.mint_a.to_account_info(),
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let transfer_taker_to_maker_b_cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts_taker_to_maker_b);
        let transfer_vault_to_taker_a_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_account_vault_to_taker_a, 
            signer_seeds
        );

        transfer_checked(transfer_taker_to_maker_b_cpi_ctx, self.escrow.receive, self.mint_b.decimals)?;
        transfer_checked(transfer_vault_to_taker_a_cpi_ctx, self.vault.amount, self.mint_a.decimals)
    }

    pub fn close(&mut self) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[b"escrow", self.maker.to_account_info().key.as_ref(), &self.escrow.seed.to_le_bytes(),  &[self.escrow.bump]]];
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            authority: self.escrow.to_account_info(),
            destination: self.maker.to_account_info()
        };

        let close_cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), close_accounts, signer_seeds);

        close_account(close_cpi_ctx)
    }
}
