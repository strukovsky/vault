use anchor_lang::prelude::*;
use crate::program_instructions::Vault;

pub fn __admin_withdraw(ctx: Context<AdminWithdraw>, amount: u64) -> Result<()> {
    let vault = ctx.accounts.vault.to_account_info();
    let signer = ctx.accounts.signer.to_account_info();
    require!(signer.key() == ctx.accounts.vault.owner, AdminWithdrawErrorCodes::NotAdminWithdrawal);
    let mut vault_balance = vault.try_borrow_mut_lamports()?;
    let is_rent_except_after_tx = Rent::get().unwrap().is_exempt(
        **vault_balance - amount,
        vault.data_len());
    require!(is_rent_except_after_tx, anchor_lang::error::ErrorCode::ConstraintRentExempt);
    let mut signer_balance = signer.try_borrow_mut_lamports()?;
    **vault_balance -= amount;
    **signer_balance += amount;
    Ok(())
}

#[derive(Accounts)]
pub struct AdminWithdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum AdminWithdrawErrorCodes {
    NotAdminWithdrawal,
}
