use anchor_lang::prelude::*;

declare_id!("AvxBzjBciSWs7vjcxnXY97SDu8hYUuJXpgCeAWKN1xb5");

#[program]
pub mod aletheia_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
