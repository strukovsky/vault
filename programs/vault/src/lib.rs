pub mod program_instructions;

use anchor_lang::prelude::*;
use program_instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, apy_basis_points: u64) -> Result<()> {
        __initialize(ctx, apy_basis_points)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        __deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        __withdraw(ctx, amount)
    }

    pub fn supply(ctx: Context<Supply>, amount: u64) -> Result<()> {
        __supply(ctx, amount)
    }

    pub fn admin_withdraw(ctx: Context<AdminWithdraw>, amount: u64) -> Result<()> {
        __admin_withdraw(ctx, amount)
    }
}
