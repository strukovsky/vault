use anchor_lang::prelude::*;
use crate::program_instructions::Vault;

pub fn __supply(ctx: Context<Supply>, amount: u64) -> Result<()> {
    let vault = ctx.accounts.vault.to_account_info();
    let signer = ctx.accounts.signer.to_account_info();
    let signer_key = *signer.key;
    require!(signer_key == ctx.accounts.vault.owner, anchor_lang::error::ErrorCode::AccountDidNotDeserialize);

    let instruction = anchor_lang::solana_program::system_instruction::transfer(
        &signer_key,
        &vault.key(),
        amount,
    );

    let result = anchor_lang::solana_program::program::invoke(
        &instruction,
        &[signer, vault],
    );

    require!(result.is_ok(), anchor_lang::error::ErrorCode::AccountDidNotDeserialize);
    Ok(())
}

#[derive(Accounts)]
pub struct Supply<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
