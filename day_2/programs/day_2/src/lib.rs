use anchor_lang::prelude::*;

declare_id!("93bEzpjRYELzSUhmFZor7t18RkGvmCYFyTz92RWDCmbZ");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You sent: {} and {} and message {}", a, b, message);
        Ok(())
    }
    pub fn array(ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array: {:?}", arr);
        Ok(())
    }

    pub fn sqrt(ctx: Context<Initialize>, a: u64) -> Result<f64> {
        let result = (a as f64).sqrt();
        msg!("The square root of {} is {}", a, result);
        Ok(result)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
