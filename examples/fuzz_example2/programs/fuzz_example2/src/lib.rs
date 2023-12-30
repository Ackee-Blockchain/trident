use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("7fd3w1z9biVCPkAJ8AtLVLUCv2WQjFvDPg27Z2tJmFVe");

#[program]
pub mod fuzz_example2 {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        receiver: Pubkey,
        amount: u64,
        option: AnotherOptions,
        i_struct: MyStruct,
    ) -> Result<()> {
        _initialize(ctx, receiver, amount, option, i_struct)
    }
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        _withdraw(ctx)
    }
}
