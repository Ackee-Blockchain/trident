use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// FuzzTransactions contains all available transactions
///
/// Below, the transaction variants are defined.
/// Each variant contains a transaction struct.
/// The transaction struct contains the instruction and the accounts and data.
///
/// You can create your own transactions by adding new variants to the enum.
#[derive(Arbitrary, FuzzTestExecutor)]
pub enum FuzzTransactions {
    InitializeCalleeTransaction(InitializeCalleeTransaction),
    InitializeCallerTransaction(InitializeCallerTransaction),
}
#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct InitializeCalleeTransaction {
    pub instruction: InitializeCalleeInstruction,
}
#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct InitializeCallerTransaction {
    pub instruction: InitializeCallerInstruction,
}
/// Custom Transaction Methods
///
/// Provides hooks for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
impl TransactionCustomMethods for InitializeCalleeTransaction {}
/// Custom Transaction Methods
///
/// Provides hooks for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
impl TransactionCustomMethods for InitializeCallerTransaction {}
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("HJR1TK8bgrUWzysdpS1pBGBYKF7zi1tU9cS4qj8BW8ZL")]
# [discriminator ([164u8 , 75u8 , 79u8 , 32u8 , 57u8 , 23u8 , 116u8 , 175u8 ,])]
pub struct InitializeCalleeInstruction {
    pub accounts: InitializeCalleeInstructionAccounts,
    pub data: InitializeCalleeInstructionData,
}
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("FWtSodrkUnovFPnNRCxneP6VWh6JH6jtQZ4PHoP8Ejuz")]
# [discriminator ([204u8 , 76u8 , 200u8 , 172u8 , 185u8 , 14u8 , 99u8 , 166u8 ,])]
pub struct InitializeCallerInstruction {
    pub accounts: InitializeCallerInstructionAccounts,
    pub data: InitializeCallerInstructionData,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
impl InstructionSetters for InitializeCalleeInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {}
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
    }
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
impl InstructionSetters for InitializeCallerInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let signer = fuzz_accounts.signer.get_or_create_account(
            self.accounts.signer.account_id,
            client,
            50 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .signer
            .set_account_meta(signer.pubkey(), true, true);
    }
}
/// Account metadata for #instruction_name instruction
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InitializeCalleeInstructionAccounts {
    pub signer: TridentAccount,
}
/// Account metadata for #instruction_name instruction
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InitializeCallerInstructionAccounts {
    pub signer: TridentAccount,
    #[program("HJR1TK8bgrUWzysdpS1pBGBYKF7zi1tU9cS4qj8BW8ZL")]
    pub program: TridentAccount,
}
/// Input data structure for #instruction_name instruction
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Copy, Clone)]
pub struct InitializeCalleeInstructionData {
    pub input: u16,
}
/// Input data structure for #instruction_name instruction
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Copy, Clone)]
pub struct InitializeCallerInstructionData {
    pub input: u16,
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    signer: AccountsStorage<KeypairStore>,
}
