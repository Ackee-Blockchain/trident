use solana_sdk::account::AccountSharedData;
use solana_sdk::clock::Clock;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar::Sysvar;

use trident_config::TridentConfig;

use trident_svm::trident_svm::TridentSVM;
use trident_svm::utils::ProgramEntrypoint;
use trident_svm::utils::SBFTarget;
use trident_svm::utils::TridentAccountSharedData;

use crate::traits::FuzzClient;
use solana_sdk::transaction::TransactionError;

impl FuzzClient for TridentSVM {
    fn deploy_native_program(&mut self, program: ProgramEntrypoint) {
        trident_svm::trident_svm::TridentSVM::deploy_native_program(self, program);
    }
    fn new_client(programs: &[ProgramEntrypoint], config: &TridentConfig) -> Self {
        let sbf_programs =
            config
                .programs()
                .iter()
                .fold(Vec::new(), |mut sbf_programs, config_program| {
                    let target = SBFTarget::new(
                        config_program.address,
                        None, // TODO add authority to the config fuzzing program
                        config_program.data.clone(),
                    );

                    sbf_programs.push(target);
                    sbf_programs
                });

        let permanent_accounts =
            config
                .accounts()
                .iter()
                .fold(Vec::new(), |mut permanent_accounts, config_account| {
                    let account = TridentAccountSharedData::new(
                        config_account.pubkey,
                        config_account.account.clone(),
                    );
                    permanent_accounts.push(account);
                    permanent_accounts
                });

        TridentSVM::new_with_syscalls(programs, &sbf_programs, &permanent_accounts)
    }
    fn warp_to_epoch(&mut self, warp_epoch: u64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.epoch = warp_epoch;
        self.set_sysvar(&clock);
    }

    fn warp_to_slot(&mut self, warp_slot: u64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.slot = warp_slot;
        self.set_sysvar(&clock);
    }
    fn warp_to_timestamp(&mut self, warp_timestamp: i64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.unix_timestamp = warp_timestamp;
        self.set_sysvar(&clock);
    }

    fn forward_in_time(&mut self, seconds: i64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.unix_timestamp = clock.unix_timestamp.saturating_add(seconds);
        self.set_sysvar(&clock);
    }

    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.add_temp_account(address, account);
    }

    fn payer(&self) -> solana_sdk::signature::Keypair {
        self.get_payer()
    }

    fn get_account(&mut self, key: &Pubkey) -> AccountSharedData {
        trident_svm::trident_svm::TridentSVM::get_account(self, key).unwrap_or_default()
    }

    fn get_last_blockhash(&self) -> Hash {
        panic!("Not yet implemented for TridentSVM");
    }

    fn process_instructions(
        &mut self,
        instructions: &[Instruction],
    ) -> Result<(), TransactionError> {
        // there should be at least 1 RW fee-payer account.
        // But we do not pay for TX currently so has to be manually updated
        // tx.message.header.num_required_signatures = 1;
        // tx.message.header.num_readonly_signed_accounts = 0;
        let tx = solana_sdk::transaction::Transaction::new_with_payer(
            instructions,
            Some(&self.payer().pubkey()),
        );

        self.process_transaction_with_settle(tx)?;
        Ok(())
    }

    fn get_sysvar<T: Sysvar>(&self) -> T {
        trident_svm::trident_svm::TridentSVM::get_sysvar::<T>(self)
    }

    fn clear_accounts(&mut self) {
        self.clear_accounts();
    }
}
