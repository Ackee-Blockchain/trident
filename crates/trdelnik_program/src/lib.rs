use anchor_lang::{InstructionData, ToAccountMetas, solana_program::pubkey::Pubkey};
pub use trdelnik_program_macros::{self, program};

pub trait FatInstruction {
    type INSTRUCTION: InstructionData + Send + 'static;
    type ACCOUNTS: ToAccountMetas + Send + 'static;

    fn new(instruction: Self::INSTRUCTION, accounts: Self::ACCOUNTS) -> Self;
    fn program() -> Pubkey;
    fn into_instruction_and_accounts(self) -> (Self::INSTRUCTION, Self::ACCOUNTS);
}
