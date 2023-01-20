use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, apy_basis_points: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.owner = *ctx.accounts.owner.key;
        vault.apy_basis_points = apy_basis_points;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
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
        if result.is_ok() {
            let deposit_entry = &mut ctx.accounts.deposit_entry;
            deposit_entry.timestamp_start = Clock::get().unwrap().unix_timestamp as i64;
            deposit_entry.amount = amount;
            deposit_entry.owner = *ctx.accounts.owner.key;
        }
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let deposit_entry = &mut ctx.accounts.deposit_entry;
        let vault = &mut ctx.accounts.vault;
        let receiver = &mut ctx.accounts.owner;
        require!(amount < deposit_entry.amount, anchor_lang::error::ErrorCode::AccountDidNotDeserialize);

        let vault_account_info = vault.to_account_info();
        let mut vault_balance = vault_account_info.try_borrow_mut_lamports()?;

        let will_remain_rent_exempt = Rent::get().unwrap().is_exempt(
            **vault_balance - amount,
            vault_account_info.data_len());
        require!(will_remain_rent_exempt, anchor_lang::error::ErrorCode::AccountDidNotDeserialize);

        let receiver_account_info = receiver.to_account_info();
        let mut receiver_balance = receiver_account_info.try_borrow_mut_lamports()?;

        **vault_balance -= amount;
        **receiver_balance += amount;

        deposit_entry.amount -= amount;
        return Ok(());
    }

    pub fn supply(ctx: Context<Supply>, amount: u64) -> Result<()> {
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

    pub fn admin_withdraw(ctx: Context<AdminWithdraw>, amount: u64) -> Result<()> {
        let vault = ctx.accounts.vault.to_account_info();
        let account = ctx.accounts.account.to_account_info();
        require!(account.key == vault.owner, anchor_lang::error::ErrorCode::AccountDidNotDeserialize);
        let instruction = anchor_lang::solana_program::system_instruction::transfer(
            vault.key,
            account.key,
            amount,
        );
        let error = anchor_lang::solana_program::program::invoke(
            &instruction,
            &[vault, account],
        );
        require!(error.is_ok(), anchor_lang::error::ErrorCode::AccountDidNotDeserialize);
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 256)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
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

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub deposit_entry: Account<'info, DepositEntry>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct Supply<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct AdminWithdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub account: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub apy_basis_points: u64,
}

#[account]
pub struct DepositEntry {
    pub amount: u64,
    pub owner: Pubkey,
    pub timestamp_start: i64,
}
