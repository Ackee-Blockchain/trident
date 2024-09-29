#![allow(dead_code)]

use crate::error::*;
use crate::fuzz_client::FuzzClient;
use crate::fuzz_deserialize::FuzzDeserialize;
use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::InstructionData;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::signature::Keypair;

/// A trait providing methods to prepare data and accounts for the fuzzed instructions and allowing
/// users to implement custom invariants checks and transactions error handling.
pub trait IxOps<'info> {
    /// The data to be passed as instruction data parameter
    type IxData: InstructionData;
    /// The accounts to be passed as instruction accounts
    type IxAccounts;
    /// The structure to which the instruction accounts will be deserialized
    type IxSnapshot: FuzzDeserialize<'info>;

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
    ) -> Result<Self::IxData, FuzzingError>;

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
        pre_ix: Self::IxSnapshot,
        post_ix: Self::IxSnapshot,
        ix_data: Self::IxData,
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
        e: FuzzClientErrorWithOrigin,
        ix_data: Self::IxData,
        pre_ix_acc_infos: &mut &'info [Option<AccountInfo<'info>>],
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        Err(e)
    }

    /// A method implemented for each instruction variant.
    /// This method calls the corresponding `deserialize_option`, which is defined
    /// by deriving the `AccountsSnapshot` macro.
    /// No changes are needed for this function.
    fn deserialize_accounts(
        &self,
        accounts: &mut &'info [Option<AccountInfo<'info>>],
    ) -> Result<Self::IxSnapshot, FuzzingError> {
        Self::IxSnapshot::deserialize_option(&self.get_program_id(), accounts)
    }
}
