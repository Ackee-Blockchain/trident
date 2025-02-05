#![allow(dead_code)]
#![allow(unexpected_cfgs)]
use arbitrary::Arbitrary;
use std::cell::RefCell;
use std::error::Error;
use trident_svm::trident_svm::TridentSVM;

use crate::traits::FuzzTestExecutor;

use crate::types::{FuzzerData, SequenceResult};
use trident_config::TridentConfig;

pub struct FuzzData<T, U> {
    pub start_transactions: Vec<T>,
    pub middle_transactions: Vec<T>,
    pub end_transactions: Vec<T>,
    pub phantom: std::marker::PhantomData<U>,
}

pub struct FuzzDataIterator<'a, T> {
    pre_ixs_iter: std::slice::Iter<'a, T>,
    ixs_iter: std::slice::Iter<'a, T>,
    post_ixs_iter: std::slice::Iter<'a, T>,
}

pub struct FuzzDataIteratorMut<'a, T> {
    pre_ixs_iter: std::slice::IterMut<'a, T>,
    ixs_iter: std::slice::IterMut<'a, T>,
    post_ixs_iter: std::slice::IterMut<'a, T>,
}

impl<T, U> FuzzData<T, U> {
    pub fn iter(&self) -> FuzzDataIterator<'_, T> {
        FuzzDataIterator {
            pre_ixs_iter: self.start_transactions.iter(),
            ixs_iter: self.middle_transactions.iter(),
            post_ixs_iter: self.end_transactions.iter(),
        }
    }
    pub fn iter_mut(&mut self) -> FuzzDataIteratorMut<'_, T> {
        FuzzDataIteratorMut {
            pre_ixs_iter: self.start_transactions.iter_mut(),
            ixs_iter: self.middle_transactions.iter_mut(),
            post_ixs_iter: self.end_transactions.iter_mut(),
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

impl<'a, T> Iterator for FuzzDataIteratorMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.pre_ixs_iter
            .next()
            .or_else(|| self.ixs_iter.next())
            .or_else(|| self.post_ixs_iter.next())
    }
}

impl<T: FuzzTestExecutor<U>, U: Default> FuzzData<T, U> {
    pub fn run_with_runtime(
        &mut self,
        client: &mut TridentSVM,
        config: &TridentConfig,
    ) -> core::result::Result<(), Box<dyn Error + 'static>> {
        #[cfg(feature = "fuzzing_debug")]
        {
            eprintln!("\x1b[34mInstructions sequence\x1b[0m:");
            for ix in self.iter() {
                eprintln!("{}", ix);
            }
            eprintln!("------ End of Instructions sequence ------ ");
        }

        let accounts = RefCell::new(U::default());
        for instructions_batch in self.iter_mut() {
            // #[cfg(feature = "fuzzing_debug")]
            // println!(
            //     "\x1b[96mCurrently processing transaction with instructions\x1b[0m: {}",
            //     instructions_batch
            // );

            if instructions_batch
                .process_transaction(client, config, &accounts)
                .is_err()
            {
                client.clear_accounts();
                return Ok(());
            }
        }
        client.clear_accounts();
        Ok(())
    }
}

#[allow(unused_variables)]
pub trait FuzzSequenceBuilder<T: for<'a> Arbitrary<'a>> {
    /// The instruction(s) executed as first, can be used for initialization.
    fn starting_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<T> {
        let transactions = <Vec<T>>::arbitrary(fuzzer_data)?;

        // Return always a vector with at least one element, othewise return error.
        if transactions.is_empty() {
            return Err(arbitrary::Error::NotEnoughData);
        }
        // convert the vector of instructions to a vector of TransactionInstructions each with single instruction
        Ok(transactions)
    }

    /// The main instructions for fuzzing.
    fn middle_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<T> {
        let transactions = <Vec<T>>::arbitrary(fuzzer_data)?;

        // Return always a vector with at least one element, othewise return error.
        if transactions.is_empty() {
            return Err(arbitrary::Error::NotEnoughData);
        }
        // convert the vector of instructions to a vector of TransactionInstructions each with single instruction
        Ok(transactions)
    }

    /// The instuction(s) executed as last.
    fn ending_sequence(fuzzer_data: &mut FuzzerData) -> SequenceResult<T> {
        let transactions = <Vec<T>>::arbitrary(fuzzer_data)?;

        // Return always a vector with at least one element, othewise return error.
        if transactions.is_empty() {
            return Err(arbitrary::Error::NotEnoughData);
        }
        // convert the vector of instructions to a vector of TransactionInstructions each with single instruction
        Ok(transactions)
    }
}

pub fn build_ix_fuzz_data<
    U: for<'a> Arbitrary<'a> + FuzzTestExecutor<V>,
    T: FuzzSequenceBuilder<U>,
    V: Default,
>(
    _data_builder: T,
    fuzzer_data: &mut FuzzerData,
) -> arbitrary::Result<FuzzData<U, V>> {
    if fuzzer_data.len() >= 3 {
        let data_len = fuzzer_data.len();
        let part = data_len / 3;

        //
        let mut start = arbitrary::Unstructured::new(fuzzer_data.bytes(part)?);
        let mut middle = arbitrary::Unstructured::new(fuzzer_data.bytes(part)?);
        let mut end = arbitrary::Unstructured::new(fuzzer_data.bytes(part)?);

        Ok(FuzzData {
            start_transactions: T::starting_sequence(&mut start)?,
            middle_transactions: T::middle_sequence(&mut middle)?,
            end_transactions: T::ending_sequence(&mut end)?,
            phantom: std::marker::PhantomData,
        })
    } else {
        Ok(FuzzData {
            start_transactions: T::starting_sequence(fuzzer_data)?,
            middle_transactions: T::middle_sequence(fuzzer_data)?,
            end_transactions: T::ending_sequence(fuzzer_data)?,
            phantom: std::marker::PhantomData,
        })
    }
}
