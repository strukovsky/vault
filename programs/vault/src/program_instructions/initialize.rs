use anchor_lang::prelude::*;

pub fn __initialize(ctx: Context<Initialize>, apy_basis_points: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    vault.owner = *ctx.accounts.owner.key;
    vault.apy_basis_points = apy_basis_points;
    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 256)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub apy_basis_points: u64,
}
