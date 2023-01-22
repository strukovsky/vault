use anchor_lang::prelude::*;
use crate::program_instructions::DepositEntry;
use crate::program_instructions::Vault;

pub fn __withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let deposit_entry = &mut ctx.accounts.deposit_entry;
    let vault = &mut ctx.accounts.vault;
    let receiver = &mut ctx.accounts.owner;
    require!(amount < deposit_entry.amount, WithdrawErrorCode::WithdrawalExceedsDeposit);

    let vault_account_info = vault.to_account_info();
    let mut vault_balance = vault_account_info.try_borrow_mut_lamports()?;

    let will_remain_rent_exempt = Rent::get().unwrap().is_exempt(
        **vault_balance - amount,
        vault_account_info.data_len());
    require!(will_remain_rent_exempt, anchor_lang::error::ErrorCode::ConstraintRentExempt);

    let receiver_account_info = receiver.to_account_info();
    let mut receiver_balance = receiver_account_info.try_borrow_mut_lamports()?;

    **vault_balance -= amount;
    **receiver_balance += amount;

    deposit_entry.amount -= amount;
    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub deposit_entry: Account<'info, DepositEntry>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[error_code(offset=100)]
pub enum WithdrawErrorCode {
    WithdrawalExceedsDeposit
}