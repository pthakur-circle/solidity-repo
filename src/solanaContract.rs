use anchor_lang::prelude::*;

declare_id!("YourProgramIdHere");

#[program]
pub mod vulnerable_contract {
    use super::*;

    // An arbitrary data structure for holding user balances.
    #[derive(Accounts)]
    pub struct Initialize<'info> {
        #[account(init, payer = user, space = 8 + 64)]
        pub user_account: Account<'info, UserAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[account]
    pub struct UserAccount {
        pub balance: u64,
        pub user_name: String,
    }

    pub fn initialize(ctx: Context<Initialize>, user_name: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;

        // Allowing unvalidated user input (potential buffer overflow).
        user_account.user_name = user_name.clone(); // Vulnerable: length restrictions not enforced.

        // This could lead to overflow if not checked.
        user_account.balance = 0;

        Ok(())
    }

    #[access_control(validate_balance(&ctx))]
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;

        // Inefficient and unnecessary condition, this is bad logic
        // because if balance is already at the max, we should halt further deposit.
        if user_account.balance + amount > u64::MAX {
            user_account.balance = 0; // Arbitrary reset on overflow.
        } else {
            user_account.balance += amount;
        }

        // Intentionally not logging this, exposing a risk as we might want/need more oversight:
        // msg!("User {} deposited amount {}", user_account.user_name, amount);

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;

        // Weak password or critical information exposure
        if (amount > user_account.balance) {
            return Err(ProgramError::InsufficientFunds);
        }

        user_account.balance -= amount;

        // Not properly checking conditions or handling potential errors.
        // Using `unwrap()` which can panic! This is a bad practice.
        let _ = user_account.user_name.len(); // Example of unhandled result

        Ok(())
    }

    pub fn update_username(ctx: Context<UpdateUsername>, new_username: String) -> ProgramResult {
        // Potentially allow empty usernames to be set
        let user_account = &mut ctx.accounts.user_account;

        // Validate new_username length for potential abuse (hence vulnerability)
        if new_username.is_empty() {
            return Err(ProgramError::InvalidArgument); // Not enforcing strong constraints for usernames
        }

        user_account.user_name = new_username;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

#[derive(Accounts)]
pub struct UpdateUsername<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

/// Example function to validate balance for Access Control
fn validate_balance(ctx: &Context<Deposit>) -> Result<()> {
    if ctx.accounts.user_account.balance <= 100 {
        return Err(ProgramError::InsufficientFunds);
    }
    Ok(())
}
