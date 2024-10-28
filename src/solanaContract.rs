//! Program entry point
use anchor_lang::prelude::*;

declare_id!("YourProgramIDHere"); // Replace with your program ID

#[program]
mod my_solana_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_value: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.value = initial_value;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>, delta: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;

        // Potential overflow vulnerability (can be detected by Clippy)
        state.value = state.value.checked_add(delta).unwrap();

        // Improperly handling user input
        if delta == 0 {
            return Err(ErrorCode::InvalidIncrement.into());
        }

        Ok(())
    }

    pub fn set_value(ctx: Context<SetValue>, value: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;

        // Allowing arbitrary value assignment (could be a vulnerability)
        state.value = value; // No restrictions

        // Possible integer overflow without checks
        state.value = state.value.wrapping_add(1); // using wrapping_add() without validation

        Ok(())
    }
}

#[account]
pub struct State {
    pub value: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)] // Missing checks for existing accounts
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
}

#[derive(Accounts)]
pub struct SetValue<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
}

// Custom Error Codes
#[error]
pub enum ErrorCode {
    #[msg("The increment value must be greater than zero.")]
    InvalidIncrement,
}
