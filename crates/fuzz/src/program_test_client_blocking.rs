use solana_program_runtime::invoke_context::BuiltinFunctionWithContext;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::account::Account;
use solana_sdk::account_info::AccountInfo;
use solana_sdk::clock::Clock;
use solana_sdk::entrypoint::ProgramResult;
use solana_sdk::sysvar::Sysvar;
use solana_sdk::{
    account::AccountSharedData, hash::Hash, pubkey::Pubkey, rent::Rent, signature::Keypair,
    transaction::VersionedTransaction,
};
use tokio::runtime::Builder;

use crate::config::Config;
use crate::error::*;
use crate::fuzz_client::FuzzClient;

pub type ProgramEntry = for<'info> fn(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    instruction_data: &[u8],
) -> ProgramResult;

pub struct ProgramTestClientBlocking {
    ctx: ProgramTestContext,
    rt: tokio::runtime::Runtime,
}

pub struct FuzzingProgram {
    pub program_name: String,
    pub program_id: Pubkey,
    pub entry: Option<BuiltinFunctionWithContext>,
}
impl FuzzingProgram {
    pub fn new(
        program_name: &str,
        program_id: &Pubkey,
        entry_fn: Option<BuiltinFunctionWithContext>,
    ) -> FuzzingProgram {
        Self {
            program_name: program_name.to_string(),
            program_id: *program_id,
            entry: entry_fn,
        }
    }
}

impl ProgramTestClientBlocking {
    pub fn new(program_: &[FuzzingProgram], config: &Config) -> Result<Self, FuzzClientError> {
        let mut program_test = ProgramTest::default();
        for x in program_ {
            if let Some(entry) = x.entry {
                program_test.add_builtin_program(&x.program_name, x.program_id, entry);
            }
        }
        for account in config.fuzz.accounts.iter() {
            program_test.add_account_with_base64_data(
                account.pubkey,
                account.account.lamports,
                account.account.owner,
                &account.account.data,
            )
        }

        for program in config.fuzz.programs.iter() {
            program_test.add_account(
                program.address,
                Account {
                    lamports: Rent::default().minimum_balance(program.data.len()).max(1),
                    data: program.data.clone(),
                    owner: solana_sdk::bpf_loader::id(),
                    executable: true,
                    rent_epoch: 0,
                },
            );
        }

        let rt: tokio::runtime::Runtime = Builder::new_current_thread().enable_all().build()?;

        let ctx = rt.block_on(program_test.start_with_context());
        Ok(Self { ctx, rt })
    }
}

/// Converts Anchor 0.29.0 and higher entrypoint into the runtime's entrypoint style
///
/// Starting Anchor 0.29.0 the accounts are passed by reference https://github.com/coral-xyz/anchor/pull/2656
/// and the lifetime requirements are `accounts: &'a [AccountInfo<'a>]` instead of `accounts: &'a [AccountInfo<'b>]`.
/// The new requirements require the slice of AccountInfos and the contained Accounts to have the same lifetime but
/// the previous version is more general. The compiler implies that `'b` must live at least as long as `'a` or longer.
///
/// The transaction data is serialized and again deserialized to the `&[AccountInfo<_>]` slice just before invoking
/// the entry point and the modified account data is copied to the original accounts just after the the entry point.
/// After that the `&[AccountInfo<_>]` slice goes out of scope entirely and therefore `'a` == `'b`. So it _SHOULD_ be
/// safe to do this conversion in this testing scenario.
///
/// Do not use this conversion in any on-chain programs!
#[macro_export]
macro_rules! convert_entry {
    ($entry:expr) => {
        unsafe { core::mem::transmute::<ProgramEntry, ProcessInstruction>($entry) }
    };
}

impl FuzzClient for ProgramTestClientBlocking {
    fn payer(&self) -> Keypair {
        self.ctx.payer.insecure_clone()
    }

    fn get_account(&mut self, key: &Pubkey) -> AccountSharedData {
        let account = self
            .rt
            .block_on(self.ctx.banks_client.get_account_with_commitment(
                *key,
                solana_sdk::commitment_config::CommitmentLevel::Confirmed,
            ))
            .unwrap_or_default();
        match account {
            Some(account) => account.into(),
            None => {
                let account = AccountSharedData::new(0, 0, &solana_sdk::system_program::ID);
                self.ctx.set_account(key, &account);
                account
            }
        }
    }
    fn get_last_blockhash(&self) -> Hash {
        self.ctx.last_blockhash
    }
    fn process_transaction(
        &mut self,
        transaction: impl Into<VersionedTransaction>,
    ) -> Result<(), FuzzClientError> {
        Ok(self
            .rt
            .block_on(self.ctx.banks_client.process_transaction(transaction))?)
    }

    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.ctx.set_account(address, account);
    }

    fn forward_in_time(&mut self, seconds: i64) -> Result<(), FuzzClientError> {
        // Get the current clock state from the program test context.
        let mut clock = self
            .rt
            .block_on(self.ctx.banks_client.get_sysvar::<Clock>())?;

        // Calculate the new timestamp after advancing time.
        let new_timestamp = clock.unix_timestamp.saturating_add(seconds);

        // Update the Clock instance with the new timestamp.
        clock.unix_timestamp = new_timestamp;

        // Update the sysvar in the program test context with the new Clock state.
        self.ctx.set_sysvar(&clock);
        Ok(())
    }
    fn warp_to_slot(&mut self, warp_slot: u64) {
        let _ = self.ctx.warp_to_slot(warp_slot);
    }
    fn warp_to_epoch(&mut self, warp_epoch: u64) {
        let _ = self.ctx.warp_to_epoch(warp_epoch);
    }
    fn get_sysvar<T: Sysvar>(&mut self) -> T {
        self.rt
            .block_on(self.ctx.banks_client.get_sysvar::<T>())
            .unwrap_or_default()
    }
}
