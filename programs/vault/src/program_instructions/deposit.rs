use anchor_lang::prelude::*;
use crate::program_instructions::Vault;

pub fn __deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let from = ctx.accounts.owner.to_account_info();
    let to = ctx.accounts.vault.to_account_info();
    let instruction = anchor_lang::solana_program::system_instruction::transfer(
        from.key,
        to.key,
        amount,
    );
    let result = anchor_lang::solana_program::program::invoke(
        &instruction,
        &[from, to],
    );
    require!(result.is_ok(), DepositErrorCodes::TransferFailed);
    let deposit_entry = &mut ctx.accounts.deposit_entry;
    deposit_entry.timestamp_start = Clock::get().unwrap().unix_timestamp;
    deposit_entry.amount = amount;
    deposit_entry.owner = *ctx.accounts.owner.key;
    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(init, payer = owner, space = 256)]
    pub deposit_entry: Account<'info, DepositEntry>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct DepositEntry {
    pub amount: u64,
    pub owner: Pubkey,
    pub timestamp_start: i64,
}

#[error_code]
pub enum DepositErrorCodes {
    NotAdminSupplement,
    TransferFailed,
}
