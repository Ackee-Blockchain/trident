#![allow(dead_code)]

use anchor_lang::solana_program::account_info::{Account as AccountTrait, AccountInfo};
use anchor_lang::solana_program::hash::Hash;
use arbitrary::Arbitrary;
use arbitrary::Unstructured;
use solana_sdk::account::Account;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use crate::error::*;
use crate::fuzz_client::FuzzClient;
use crate::fuzz_test_executor::FuzzTestExecutor;

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
            eprintln!("\x1b[34mInstructions sequence\x1b[0m:");
            for ix in self.iter() {
                eprintln!("{}", ix);
            }
            eprintln!("------ End of Instructions sequence ------ ");
        }

        let mut sent_txs: HashMap<Hash, ()> = HashMap::new();

        for fuzz_ix in &mut self.iter() {
            #[cfg(fuzzing_debug)]
            eprintln!("\x1b[34mCurrently processing\x1b[0m: {}", fuzz_ix);

            if fuzz_ix
                .run_fuzzer(program_id, &self.accounts, client, &mut sent_txs)
                .is_err()
            {
                // for now skip following instructions in case of error and move to the next fuzz iteration
                return Ok(());
            }
        }
        Ok(())
    }
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
