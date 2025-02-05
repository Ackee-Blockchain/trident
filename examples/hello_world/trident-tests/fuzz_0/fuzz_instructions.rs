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
    InitializeFnTransaction(InitializeFnTransaction),
}
#[derive(Arbitrary, Debug, TridentTransaction)]
pub struct InitializeFnTransaction {
    pub instruction: InitializeFnInstruction,
}
/// Custom Transaction Methods
///
/// Provides hooks for customizing transaction behavior:
/// - `pre_transaction`: Execute custom logic before transaction execution
/// - `transaction_invariant_check`: Validate transaction-specific invariants
/// - `transaction_error_handler`: Custom handling of transaction errors
/// - `post_transaction`: Execute custom logic after transaction execution
impl TransactionCustomMethods for InitializeFnTransaction {}
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit")]
# [discriminator ([18u8 , 187u8 , 169u8 , 213u8 , 94u8 , 180u8 , 86u8 , 152u8 ,])]
pub struct InitializeFnInstruction {
    pub accounts: InitializeFnInstructionAccounts,
    pub data: InitializeFnInstructionData,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
impl InstructionSetters for InitializeFnInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let author = fuzz_accounts.author.get_or_create_account(
            self.accounts.author.account_id,
            client,
            50 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .author
            .set_account_meta(author.pubkey(), true, true);

        let hello_world_account = fuzz_accounts.hello_world_account.get_or_create_account(
            self.accounts.hello_world_account.account_id,
            client,
            &[b"hello_world_seed"],
            &self.get_program_id(),
        );
        self.accounts
            .hello_world_account
            .set_account_meta(hello_world_account, false, true);
    }
}
/// Account metadata for #instruction_name instruction
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InitializeFnInstructionAccounts {
    pub author: TridentAccount,
    pub hello_world_account: TridentAccount,
    #[skip_snapshot]
    #[program("11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}
/// Input data structure for #instruction_name instruction
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Copy, Clone)]
pub struct InitializeFnInstructionData {
    pub input: u8,
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    author: AccountsStorage<KeypairStore>,
    hello_world_account: AccountsStorage<PdaStore>,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct StoreHelloWorld {
    input: u8,
}
