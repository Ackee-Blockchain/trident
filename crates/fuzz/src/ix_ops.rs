#![allow(dead_code)]

use crate::error::*;
use crate::fuzz_client::FuzzClient;
use crate::snapshot::SnapshotAccount;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::TransactionError;

/// A trait providing methods to prepare data and accounts for the fuzzed instructions and allowing
/// users to implement custom invariants checks and transactions error handling.
pub trait IxOps {
    /// The accounts to be passed as instruction accounts
    type IxAccounts;

    fn get_discriminator(&self) -> Vec<u8>;

    /// Specify Program ID to which the Instruction corresponds. This is particularly helpful when using multiple
    /// programs in the workspace, to differentiate between possible program calls.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey;

    /// Provides instruction data for the fuzzed instruction.
    /// It is assumed that the instruction data will be based on the fuzzer input stored in the `self.data` variable.
    /// However it is on the developer to decide and it can be also for example a hardcoded constant.
    /// You should only avoid any non-deterministic random values to preserve reproducibility of the tests.
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) -> Result<Vec<u8>, FuzzingError>;

    /// Provides accounts required for the fuzzed instruction. The method returns a tuple of signers and account metas.
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError>;

    /// A method to implement custom invariants checks for a given instruction. This method is called after each
    /// successfully executed instruction and by default does nothing. You can override this behavior by providing
    /// your own implementation. You can access the snapshots of account states before and after the transaction for comparison.
    ///
    /// If you want to detect a crash, you have to return a `FuzzingError` (or alternativelly panic).
    ///
    /// If you want to perform checks also on a failed instruction execution, you can do so using the [`tx_error_handler`](trident_client::fuzzer::data_builder::IxOps::tx_error_handler) method.
    #[allow(unused_variables)]
    fn check(
        &self,
        pre_ix: &[SnapshotAccount],
        post_ix: &[SnapshotAccount],
        ix_data: Vec<u8>,
    ) -> Result<(), FuzzingError> {
        Ok(())
    }

    /// A method to implement custom error handler for failed transactions.
    ///
    /// The fuzzer might generate a sequence of one or more instructions that are executed sequentially.
    /// By default, if the execution of one of the instructions fails, the remaining instructions are skipped
    /// and are not executed. This can be overriden by implementing this method and returning `Ok(())`
    /// instead of propagating the error.
    ///
    /// You can also check the kind of the transaction error by inspecting the `e` parameter.
    /// If you would like to detect a crash on a specific error, call `panic!()`.
    ///
    /// If your accounts are malformed and the fuzzed program is unable to deserialize it, the transaction
    /// execution will fail. In that case also the deserialization of accounts snapshot before executing
    /// the instruction would fail. You are provided with the raw account infos snapshots and you are free
    /// to deserialize the accounts by yourself and therefore also handling potential errors. To deserialize
    /// the `pre_ix_acc_infos` raw accounts to a snapshot structure, you can call:
    ///
    /// ```rust,ignore
    /// self.deserialize_accounts(pre_ix_acc_infos)
    /// ```
    #[allow(unused_variables)]
    fn tx_error_handler(
        &self,
        e: TransactionError,
        ix_data: Vec<u8>,
        pre_ix_accounts: &[SnapshotAccount],
    ) -> Result<(), TransactionError> {
        Err(e)
    }
}
