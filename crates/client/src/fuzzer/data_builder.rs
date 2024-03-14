use anchor_client::anchor_lang::solana_program::account_info::{Account as Acc, AccountInfo};
use anchor_client::anchor_lang::solana_program::hash::Hash;
use anchor_lang::prelude::Rent;
use arbitrary::Arbitrary;
use arbitrary::Unstructured;
use solana_sdk::account::{Account, AccountSharedData};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::VersionedTransaction;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;

use crate::error::*;

pub struct FuzzData<T, U> {
    pub pre_ixs: Vec<T>,
    pub ixs: Vec<T>,
    pub post_ixs: Vec<T>,
    pub accounts: RefCell<U>,
}

pub struct FuzzDataIterator<'a, T> {
    pre_ixs_iter: std::slice::Iter<'a, T>,
    ixs_iter: std::slice::Iter<'a, T>,
    post_ixs_iter: std::slice::Iter<'a, T>,
}

impl<T, U> FuzzData<T, U> {
    pub fn iter(&self) -> FuzzDataIterator<'_, T> {
        FuzzDataIterator {
            pre_ixs_iter: self.pre_ixs.iter(),
            ixs_iter: self.ixs.iter(),
            post_ixs_iter: self.post_ixs.iter(),
        }
    }
}

impl<'a, T> Iterator for FuzzDataIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pre_ixs_iter
            .next()
            .or_else(|| self.ixs_iter.next())
            .or_else(|| self.post_ixs_iter.next())
    }
}

impl<T, U> FuzzData<T, U>
where
    T: FuzzTestExecutor<U> + Display,
{
    pub fn run_with_runtime(
        &self,
        program_id: Pubkey,
        client: &mut impl FuzzClient,
    ) -> core::result::Result<(), Box<dyn Error + 'static>> {
        // solana_logger::setup_with_default("off");
        // #[cfg(fuzzing_debug)]
        // solana_logger::setup_with_default(
        //     "solana_rbpf::vm=debug,\
        //         solana_runtime::message_processor=debug,\
        //         solana_runtime::system_instruction_processor=trace,\
        //         solana_program_test=info,\
        //         fuzz_target=info",
        // );

        #[cfg(fuzzing_debug)]
        {
            eprintln!("Instructions sequence:");
            for ix in self.iter() {
                eprintln!("{}", ix);
            }
            eprintln!("------ End of Instructions sequence ------ ");
        }

        for fuzz_ix in &mut self.iter() {
            #[cfg(fuzzing_debug)]
            eprintln!("Currently processing: {}", fuzz_ix);

            if fuzz_ix
                .run_fuzzer(program_id, &self.accounts, client)
                .is_err()
            {
                // for now skip following instructions in case of error and move to the next fuzz iteration
                return Ok(());
            }
        }
        Ok(())
    }
}

pub trait FuzzTestExecutor<T> {
    fn run_fuzzer(
        &self,
        program_id: Pubkey,
        accounts: &RefCell<T>,
        client: &mut impl FuzzClient,
    ) -> core::result::Result<(), FuzzClientErrorWithOrigin>;
}

#[allow(unused_variables)]
pub trait FuzzDataBuilder<T: for<'a> Arbitrary<'a>> {
    /// The instruction(s) executed as first, can be used for initialization.
    fn pre_ixs(u: &mut Unstructured) -> arbitrary::Result<Vec<T>> {
        Ok(vec![])
    }

    /// The main instructions for fuzzing.
    fn ixs(u: &mut Unstructured) -> arbitrary::Result<Vec<T>> {
        let v = <Vec<T>>::arbitrary(u)?;
        // Return always a vector with at least one element, othewise return error.
        if v.is_empty() {
            return Err(arbitrary::Error::NotEnoughData);
        }
        Ok(v)
    }

    /// The instuction(s) executed as last.
    fn post_ixs(u: &mut Unstructured) -> arbitrary::Result<Vec<T>> {
        Ok(vec![])
    }
}

/// A trait providing methods to prepare data and accounts for the fuzzed instructions and allowing
/// users to implement custom invariants checks and transactions error handling.
pub trait IxOps<'info> {
    /// The data to be passed as instruction data parameter
    type IxData;
    /// The accounts to be passed as instruction accounts
    type IxAccounts;
    /// The structure to which the instruction accounts will be deserialized
    type IxSnapshot;

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
    /// self.deserialize_option(pre_ix_acc_infos)
    /// ```
    #[allow(unused_variables)]
    fn tx_error_handler(
        &self,
        e: FuzzClientErrorWithOrigin,
        ix_data: Self::IxData,
        pre_ix_acc_infos: &'info mut [Option<AccountInfo<'info>>],
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        Err(e)
    }
}

pub trait FuzzDeserialize<'info> {
    type Ix;
    // TODO return also remaining accounts

    fn deserialize_option(
        &self,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> Result<Self::Ix, FuzzingError>;
}

/// A trait providing methods to read and write (manipulate) accounts
pub trait FuzzClient {
    /// Create an empty account and add lamports to it
    fn set_account(&mut self, lamports: u64) -> Keypair;

    /// Create or overwrite a custom account, subverting normal runtime checks.
    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData);

    /// Create an SPL token account
    #[allow(clippy::too_many_arguments)]
    fn set_token_account(
        &mut self,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Pubkey;

    /// Create an SPL mint account
    fn set_mint_account(
        &mut self,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Pubkey;

    /// Get the Keypair of the client's payer account
    fn payer(&self) -> Keypair;

    /// Get the account at the given address
    fn get_account(&mut self, key: &Pubkey) -> Result<Option<Account>, FuzzClientError>;

    /// Get accounts based on the supplied meta information
    fn get_accounts(
        &mut self,
        metas: &[AccountMeta],
    ) -> Result<Vec<Option<Account>>, FuzzClientErrorWithOrigin>;

    /// Get last blockhash
    fn get_last_blockhash(&self) -> Hash;

    /// Get the cluster rent
    fn get_rent(&mut self) -> Result<Rent, FuzzClientError>;

    /// Send a transaction and return until the transaction has been finalized or rejected.
    fn process_transaction(
        &mut self,
        transaction: impl Into<VersionedTransaction>,
    ) -> Result<(), FuzzClientError>;
}

#[macro_export]
macro_rules! fuzz_trident {
    ($ix:ident: $ix_dty:ident , |$buf:ident: $dty:ident| $body:block) => {
        fuzz(|$buf| {
            let mut $buf: FuzzData<$ix_dty, _> = {
                use arbitrary::Unstructured;

                let mut buf = Unstructured::new($buf);
                if let Ok(fuzz_data) = build_ix_fuzz_data($dty {}, &mut buf) {
                    fuzz_data
                } else {
                    return;
                }
            };
            $body
        });
    };
}
/// Prints the details of a given account in a pretty-printed format.
///
/// This macro takes a single argument, which is an expression referring to the account
/// you want to print. The account data structure must implement or derive the [`Debug`]
/// trait for this macro to work, as it relies on `std::fmt::Debug` for formatting.
///
/// # Examples
///
/// ```rust,ignore
/// use trident_client::fuzzing::show_account;
///
/// #[derive(Debug)]
/// #[account]
/// struct Escrow {
///     recipeint: Pubkey,
///     id: u32,
///     balance: f64,
///     name: String,
/// }
///
/// fn check(
///     &self,
///     pre_ix: Self::IxSnapshot,
///     post_ix: Self::IxSnapshot,
///     ix_data: Self::IxData,
/// ) -> Result<(), FuzzingError> {
///     if let Some(escrow) = pre_ix.escrow{
///         show_account!(escrow);
///     }
/// }
/// ```
///
/// # Requirements
///
/// The `account` passed to `show_account!` must implement or derive the [`Debug`] trait.
/// Attempting to use this macro with a type that does not meet this requirement will
/// result in a compilation error.
#[macro_export]
macro_rules! show_account {
    ($account:expr) => {
        eprintln!("{:#?}", $account);
    };
}

pub fn build_ix_fuzz_data<U: for<'a> Arbitrary<'a>, T: FuzzDataBuilder<U>, V: Default>(
    _data_builder: T,
    u: &mut arbitrary::Unstructured,
) -> arbitrary::Result<FuzzData<U, V>> {
    Ok(FuzzData {
        pre_ixs: T::pre_ixs(u)?,
        ixs: T::ixs(u)?,
        post_ixs: T::post_ixs(u)?,
        accounts: RefCell::new(V::default()),
    })
}

/// Creates `AccountInfo`s from `Accounts` and corresponding `AccountMeta` slices.
pub fn get_account_infos_option<'info>(
    accounts: &'info mut [Option<Account>],
    metas: &'info [AccountMeta],
) -> Result<Vec<Option<AccountInfo<'info>>>, FuzzingError> {
    let iter = accounts.iter_mut().zip(metas);
    let r = iter
        .map(|(account, meta)| {
            if let Some(account) = account {
                let (lamports, data, owner, executable, rent_epoch) = account.get();
                Some(AccountInfo::new(
                    &meta.pubkey,
                    meta.is_signer,
                    meta.is_writable,
                    lamports,
                    data,
                    owner,
                    executable,
                    rent_epoch,
                ))
            } else {
                None
            }
        })
        .collect();

    Ok(r)
}
