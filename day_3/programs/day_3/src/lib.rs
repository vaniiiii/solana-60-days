use anchor_lang::prelude::*;

declare_id!("4YP3z3EGEUakijwxJTd2P912PxaAxGFTWJsd9zjbPgcm");

#[program]
pub mod day_3 {
    use super::*;

    pub fn function_a(_ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }

    pub fn function_b(_ctx: Context<Empty>, _first_arg: u64) -> Result<()> {
        Ok(())
    }

    pub fn limit_range(_ctx: Context<LimitRange>, a: u64) -> Result<()> {
        if a < 10 {
            return err!(MyError::AisToSmall);
        }
        if a > 100 {
            return err!(MyError::AisToBig);
        }

        msg!("Result = {}", a);
        Ok(())
    }

    pub fn func(ctx: Context<ReturnError>) -> Result<()> {
        msg!("Will this print?");
        return err!(Day4Error::AlwaysErrors);
    }
}
#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Empty {}

#[derive(Accounts)]
pub struct LimitRange {}

#[derive(Accounts)]
pub struct ReturnError {}

#[error_code]
pub enum MyError {
    #[msg("a is too small")]
    AisToSmall,
    #[msg("a is too big")]
    AisToBig,
}

#[error_code]
pub enum Day4Error {
    #[msg("Always errors")]
    AlwaysErrors,
}
