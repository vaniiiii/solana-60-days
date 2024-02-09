use anchor_lang::prelude::*;

declare_id!("A5SQJSj9H1nsuWgs2nRNjpkCsLTMzpMhqDGrVoJhi9pq");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!"); // **** NEW LINE HERE ****
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
