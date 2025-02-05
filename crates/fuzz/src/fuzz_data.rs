#![allow(dead_code)]
#![allow(unexpected_cfgs)]
use arbitrary::Arbitrary;
use arbitrary::Unstructured;
use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;

use crate::fuzz_client::FuzzClient;
use crate::fuzz_test_executor::FuzzTestExecutor;
use crate::fuzzing::TransactionExecutor;
use trident_config::TridentConfig;

pub struct TransactionInstructions<T> {
    pub instructions: Vec<T>,
}

impl<T: Display> Display for TransactionInstructions<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, instruction) in self.instructions.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", instruction)?;
        }
        write!(f, "]")
    }
}

impl<T> From<T> for TransactionInstructions<T> {
    fn from(value: T) -> Self {
        TransactionInstructions {
            instructions: vec![value],
        }
    }
}

pub struct FuzzData<T, U> {
    pub pre_ixs: Vec<TransactionInstructions<T>>,
    pub ixs: Vec<TransactionInstructions<T>>,
    pub post_ixs: Vec<TransactionInstructions<T>>,
    pub accounts: RefCell<U>,
}

pub struct FuzzDataIterator<'a, T> {
    pre_ixs_iter: std::slice::Iter<'a, T>,
    ixs_iter: std::slice::Iter<'a, T>,
    post_ixs_iter: std::slice::Iter<'a, T>,
}

impl<T, U> FuzzData<T, U> {
    pub fn iter(&self) -> FuzzDataIterator<'_, TransactionInstructions<T>> {
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
        client: &mut impl FuzzClient,
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

        for instructions_batch in &mut self.iter() {
            // #[cfg(feature = "fuzzing_debug")]
            println!(
                "\x1b[96mCurrently processing transaction with instructions\x1b[0m: {}",
                instructions_batch
            );

            if TransactionExecutor::process_instructions_batch(
                client,
                instructions_batch,
                config,
                &self.accounts,
            )
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
pub trait FuzzDataBuilder<T: for<'a> Arbitrary<'a>> {
    /// The instruction(s) executed as first, can be used for initialization.
    fn pre_ixs(u: &mut Unstructured) -> arbitrary::Result<Vec<TransactionInstructions<T>>> {
        Ok(vec![])
    }

    /// The main instructions for fuzzing.
    fn ixs(u: &mut Unstructured) -> arbitrary::Result<Vec<TransactionInstructions<T>>> {
        let v = <Vec<T>>::arbitrary(u)?;

        // Return always a vector with at least one element, othewise return error.
        if v.is_empty() {
            return Err(arbitrary::Error::NotEnoughData);
        }
        // convert the vector of instructions to a vector of TransactionInstructions each with single instruction
        Ok(v.into_iter()
            .map(|x| TransactionInstructions::<T>::from(x))
            .collect())
    }

    /// The instuction(s) executed as last.
    fn post_ixs(u: &mut Unstructured) -> arbitrary::Result<Vec<TransactionInstructions<T>>> {
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
