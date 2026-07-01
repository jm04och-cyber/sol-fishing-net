use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod sol_collector {
    use super::*;

    /// Initialize the vault account
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.admin = ctx.accounts.admin.key();
        vault.total_deposited = 0;
        vault.is_paused = false;
        
        emit!(VaultInitialized {
            vault: vault.key(),
            admin: vault.admin,
        });
        
        Ok(())
    }

    /// Deposit SOL into the vault
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(!ctx.accounts.vault.is_paused, VaultError::VaultPaused);
        require!(amount > 0, VaultError::InvalidAmount);

        let vault = &mut ctx.accounts.vault;
        let user_deposit = &mut ctx.accounts.user_deposit;

        // Transfer SOL to vault
        anchor_lang::solana_program::program::invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.user.key(),
                &vault.key(),
                amount,
            ),
            &[
                ctx.accounts.user.to_account_info(),
                vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Update vault state
        vault.total_deposited = vault.total_deposited.checked_add(amount)
            .ok_or(VaultError::Overflow)?;

        // Update user deposit record
        user_deposit.user = ctx.accounts.user.key();
        user_deposit.amount = user_deposit.amount.checked_add(amount)
            .ok_or(VaultError::Overflow)?;
        user_deposit.last_deposit = Clock::get()?.unix_timestamp;

        emit!(Deposited {
            user: ctx.accounts.user.key(),
            amount,
            total_in_vault: vault.total_deposited,
        });

        Ok(())
    }

    /// Withdraw SOL from the vault (admin only)
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);
        require!(
            ctx.accounts.vault.admin == ctx.accounts.admin.key(),
            VaultError::Unauthorized
        );

        let vault = &mut ctx.accounts.vault;
        let vault_lamports = vault.to_account_info().lamports();
        
        require!(
            vault_lamports >= amount,
            VaultError::InsufficientVaultBalance
        );

        // Transfer SOL from vault to recipient
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.recipient.to_account_info().try_borrow_mut_lamports()? += amount;

        vault.total_deposited = vault.total_deposited.checked_sub(amount)
            .ok_or(VaultError::Underflow)?;

        emit!(Withdrawn {
            recipient: ctx.accounts.recipient.key(),
            amount,
            remaining_in_vault: vault.total_deposited,
        });

        Ok(())
    }

    /// Pause/unpause deposits
    pub fn toggle_pause(ctx: Context<TogglePause>) -> Result<()> {
        require!(
            ctx.accounts.vault.admin == ctx.accounts.admin.key(),
            VaultError::Unauthorized
        );

        let vault = &mut ctx.accounts.vault;
        vault.is_paused = !vault.is_paused;

        emit!(PauseToggled {
            vault: vault.key(),
            is_paused: vault.is_paused,
        });

        Ok(())
    }
}

// ========== ACCOUNTS ==========

#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub total_deposited: u64,
    pub is_paused: bool,
}

#[account]
pub struct UserDeposit {
    pub user: Pubkey,
    pub amount: u64,
    pub last_deposit: i64,
}

// ========== CONTEXTS ==========

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 1)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 32 + 8 + 8,
        seeds = [b"deposit", user.key().as_ref()],
        bump
    )]
    pub user_deposit: Account<'info, UserDeposit>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub recipient: UncheckedAccount<'info>,
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
pub struct TogglePause<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub admin: Signer<'info>,
}

// ========== EVENTS ==========

#[event]
pub struct VaultInitialized {
    pub vault: Pubkey,
    pub admin: Pubkey,
}

#[event]
pub struct Deposited {
    pub user: Pubkey,
    pub amount: u64,
    pub total_in_vault: u64,
}

#[event]
pub struct Withdrawn {
    pub recipient: Pubkey,
    pub amount: u64,
    pub remaining_in_vault: u64,
}

#[event]
pub struct PauseToggled {
    pub vault: Pubkey,
    pub is_paused: bool,
}

// ========== ERRORS ==========

#[error_code]
pub enum VaultError {
    #[msg("Vault is paused")]
    VaultPaused,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Overflow error")]
    Overflow,
    #[msg("Underflow error")]
    Underflow,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Insufficient vault balance")]
    InsufficientVaultBalance,
}
