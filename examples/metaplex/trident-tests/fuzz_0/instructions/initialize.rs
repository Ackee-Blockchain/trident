use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(TridentInstruction, Default)]
#[program_id("H2XPhu8mmGDZioamVp2C5bDWXSSKn6bDdhpiUqWqPmLS")]
# [discriminator ([175u8 , 175u8 , 109u8 , 31u8 , 13u8 , 152u8 , 155u8 , 237u8 ,])]
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
        signer,
        mut,
        storage::name = signer,
        storage::account_id = (0..1),
    )]
    pub signer: TridentAccount,
    #[account(
        signer,
        mut,
        storage::name = mint,
        storage::account_id = (0..1),
    )]
    mint: TridentAccount,
    #[account(
        mut,
        storage::name = metadata_account,
        storage::account_id = (0..1),
        seeds = [b"metadata", mpl_token_metadata.as_ref(), mint.as_ref()],
        program_id = mpl_token_metadata
    )]
    metadata_account: TridentAccount,
    #[account(address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s", skip_snapshot)]
    mpl_token_metadata: TridentAccount,
    #[account(address = "11111111111111111111111111111111", skip_snapshot)]
    system_program: TridentAccount,
    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", skip_snapshot)]
    token_program: TridentAccount,
}
/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct InitializeInstructionData {
    input: u8,
    name: String,
    symbol: String,
    uri: String,
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
    fn set_data(
        &mut self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut Self::IxAccounts,
        rng: &mut TridentRng,
    ) {
        self.data.input = rng.gen_range(0..=u8::MAX);
        self.data.name = rng.gen_string(10);
        self.data.symbol = rng.gen_string(5);
        self.data.uri = rng.gen_string(25);
    }
    fn set_accounts(
        &mut self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut Self::IxAccounts,
        _rng: &mut TridentRng,
    ) {
        // nothing required here
    }
}
