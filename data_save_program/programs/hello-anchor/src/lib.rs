use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("GsX4b44N2vkDjnZPLucGV7ou5qxADN2N6BZ7zU8vnJ1X");

#[program]
mod hello_anchor {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>, slope : f64, intercept : f64) -> Result<()> {
        let new_account = &mut ctx.accounts.new_account;
         // Ensure the array is not full
        require!(new_account.array_length < MAX_ARRAY_SIZE as u64, CustomError::ArrayFull);

        // Append the new slope and intercept
        let index = new_account.array_length as usize;
        new_account.slopes[index] = slope;
        new_account.intercepts[index] = intercept;

        // Increment the array length
        new_account.array_length += 1;



        msg!("Added new slope and intercept at index {}!", index);
        Ok(())
    }
    pub fn save_data(ctx: Context<SaveData>, slope : f64, intercept : f64) -> Result<()> {
        let new_account = &mut ctx.accounts.new_account;
         // Ensure the array is not full
        require!(new_account.array_length < MAX_ARRAY_SIZE as u64, CustomError::ArrayFull);

        // Append the new slope and intercept
        let index = new_account.array_length as usize;
        new_account.slopes[index] = slope;
        new_account.intercepts[index] = intercept;

        // Increment the array length
        new_account.array_length += 1;

        msg!("Added new slope and intercept at index {}!", index);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account.
    #[account(init, payer = signer, space = 8 + (8 * MAX_ARRAY_SIZE as usize * 2) + 8, seeds = [b"final_seed".as_ref(), signer.key().as_ref()], bump)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SaveData<'info> {
    #[account(mut, seeds = [b"final_seed".as_ref(), signer.key().as_ref()], bump)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
}
#[account]
pub struct NewAccount {
    pub slopes: [f64; MAX_ARRAY_SIZE],
    pub intercepts: [f64; MAX_ARRAY_SIZE],
    pub array_length: u64, // To keep track of how many elements are stored
}

// Define a constant for the maximum array size
const MAX_ARRAY_SIZE: usize = 80;

// Custom errors for the program
#[error_code]
pub enum CustomError {
    #[msg("Array is full. Cannot add more slopes and intercepts.")]
    ArrayFull,
}