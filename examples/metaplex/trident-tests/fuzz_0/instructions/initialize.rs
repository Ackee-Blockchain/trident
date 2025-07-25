use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("H2XPhu8mmGDZioamVp2C5bDWXSSKn6bDdhpiUqWqPmLS")]
#[discriminator([175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8])]
pub struct InitializeInstruction {
    pub accounts: InitializeInstructionAccounts,
    pub data: InitializeInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(InitializeInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeInstructionAccounts {
    #[account(
        mut,
        signer,
        storage::name = signer,
        storage::account_id = (0..1),
    )]
    pub signer: TridentAccount,

    #[account(mut, signer, storage::name = mint, storage::account_id = (0..1))]
    pub mint: TridentAccount,

    #[account(mut, storage::name = metadata_account, storage::account_id = (0..1), seeds = [b"metadata", mpl_token_metadata.as_ref(), mint.as_ref()], program_id = mpl_token_metadata)]
    pub metadata_account: TridentAccount,

    #[account(address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")]
    pub mpl_token_metadata: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,

    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
    pub token_program: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeInstructionData {
    pub input: u8,

    pub name: String,

    pub symbol: String,

    pub uri: String,
}

/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for InitializeInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts) {
        self.data.input = trident.gen_range(0..=u8::MAX);
        self.data.name = trident.gen_string(10);
        self.data.symbol = trident.gen_string(5);
        self.data.uri = trident.gen_string(25);
    }
}
