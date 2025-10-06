use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::TransactionError;
use trident_svm::prelude::solana_svm::transaction_results::TransactionExecutionResult;
use trident_svm::processor::InstructionError;

use crate::trident::Trident;

use solana_sdk::account::AccountSharedData;
use solana_sdk::account::ReadableAccount;
use solana_sdk::account::WritableAccount;
use solana_sdk::clock::Clock;
use solana_sdk::hash::Hash;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar::Sysvar;

#[cfg(any(feature = "syscall-v1", feature = "syscall-v2"))]
use trident_svm::types::trident_entrypoint::TridentEntrypoint;
use trident_svm::types::trident_program::TridentProgram;

#[cfg(feature = "stake")]
use solana_sdk::clock::Epoch;
#[cfg(feature = "stake")]
use solana_stake_program::stake_state::Lockup;

impl Trident {
    pub fn execute(
        &mut self,
        instructions: &[Instruction],
        transaction_name: &str,
    ) -> solana_sdk::transaction::Result<()> {
        let fuzzing_metrics = std::env::var("FUZZING_METRICS");
        let fuzzing_debug = std::env::var("TRIDENT_FUZZ_DEBUG");

        if fuzzing_metrics.is_ok() {
            self.fuzzing_data.add_executed_transaction(transaction_name);
        }
        if fuzzing_debug.is_ok() {
            let tx = format!("{:#?}", instructions);
            trident_svm::prelude::trident_svm_log::log_message(
                &tx,
                trident_svm::prelude::Level::Debug,
            );
        }
        let processing_data = self.process_instructions(instructions);

        // NOTE: for now we just expect that one transaction was executed
        let tx_result = &processing_data.execution_results[0];

        self.handle_tx_result(tx_result, transaction_name, instructions)
    }

    #[cfg(any(feature = "syscall-v1", feature = "syscall-v2"))]
    pub fn deploy_entrypoint(&mut self, _program: TridentEntrypoint) {
        self.client.deploy_entrypoint_program(&_program);
    }

    pub fn deploy_program(&mut self, program: TridentProgram) {
        self.client.deploy_binary_program(&program);
    }

    pub fn warp_to_epoch(&mut self, warp_epoch: u64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.epoch = warp_epoch;
        self.client.set_sysvar(&clock);
    }

    pub fn warp_to_slot(&mut self, warp_slot: u64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.slot = warp_slot;
        self.client.set_sysvar(&clock);
    }
    pub fn warp_to_timestamp(&mut self, warp_timestamp: i64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.unix_timestamp = warp_timestamp;
        self.client.set_sysvar(&clock);
    }

    pub fn forward_in_time(&mut self, seconds: i64) {
        let mut clock = self.get_sysvar::<Clock>();

        clock.unix_timestamp = clock.unix_timestamp.saturating_add(seconds);
        self.client.set_sysvar(&clock);
    }

    pub fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.client.set_account(address, account, false);
    }

    pub fn payer(&self) -> solana_sdk::signature::Keypair {
        self.client.get_payer()
    }

    pub fn get_account(&mut self, key: &Pubkey) -> AccountSharedData {
        trident_svm::trident_svm::TridentSVM::get_account(&self.client, key).unwrap_or_default()
    }

    pub fn get_last_blockhash(&self) -> Hash {
        panic!("Not yet implemented for TridentSVM");
    }

    fn process_instructions(
        &mut self,
        instructions: &[Instruction],
    ) -> trident_svm::prelude::solana_svm::transaction_processor::LoadAndExecuteSanitizedTransactionsOutput{
        // there should be at least 1 RW fee-payer account.
        // But we do not pay for TX currently so has to be manually updated
        // tx.message.header.num_required_signatures = 1;
        // tx.message.header.num_readonly_signed_accounts = 0;
        let tx = solana_sdk::transaction::Transaction::new_with_payer(
            instructions,
            Some(&self.payer().pubkey()),
        );

        self.client.process_transaction_with_settle(tx)
    }

    pub fn get_sysvar<T: Sysvar>(&self) -> T {
        trident_svm::trident_svm::TridentSVM::get_sysvar::<T>(&self.client)
    }

    pub fn airdrop(&mut self, address: &Pubkey, amount: u64) {
        let mut account = self.get_account(address);

        account.set_lamports(account.lamports() + amount);
        self.set_account_custom(address, &account);
    }

    #[cfg(feature = "token")]
    pub fn create_mint(
        &mut self,
        mint_address: &Pubkey,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> solana_sdk::transaction::Result<()> {
        let ix = spl_token::instruction::initialize_mint2(
            &spl_token::ID,
            mint_address,
            owner,
            freeze_authority,
            decimals,
        )
        .unwrap();

        let processing_data = self.process_instructions(&[ix]);

        // NOTE: for now we just expect that one transaction was executed
        let tx_result = &processing_data.execution_results[0];

        self.handle_tx_result(tx_result, "Creating Mint Account", &[ix])
    }

    #[cfg(feature = "token")]
    pub fn create_token_account(
        &mut self,
        token_account_address: Pubkey,
        mint: Pubkey,
        owner: Pubkey,
    ) -> solana_sdk::transaction::Result<()> {
        let ix = spl_token::instruction::initialize_account3(
            &spl_token::ID,
            &token_account_address,
            &mint,
            &owner,
        )
        .unwrap();

        let processing_data = self.process_instructions(&[ix]);

        // NOTE: for now we just expect that one transaction was executed
        let tx_result = &processing_data.execution_results[0];

        self.handle_tx_result(tx_result, "Creating Token Account", &[ix])
    }

    #[cfg(feature = "token")]
    pub fn mint_to(
        &mut self,
        token_account_address: &Pubkey,
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        amount: u64,
    ) -> solana_sdk::transaction::Result<()> {
        let ix = spl_token::instruction::mint_to(
            &spl_token::ID,
            mint_address,
            token_account_address,
            mint_authority,
            &[],
            amount,
        )
        .unwrap();

        let processing_data = self.process_instructions(&[ix]);

        // NOTE: for now we just expect that one transaction was executed
        let tx_result = &processing_data.execution_results[0];

        self.handle_tx_result(tx_result, "Minting to Token Account", &[ix])
    }

    #[cfg(feature = "stake")]
    pub fn create_delegated_account(
        &mut self,
        address: Pubkey,
        voter_pubkey: Pubkey,
        staker: Pubkey,
        withdrawer: Pubkey,
        stake: u64,
        activation_epoch: Epoch,
        deactivation_epoch: Option<Epoch>,
        lockup: Option<Lockup>,
    ) {
        use solana_sdk::native_token::LAMPORTS_PER_SOL;
        use solana_sdk::program_pack::Pack;
        use solana_sdk::rent::Rent;
        use solana_sdk::stake::stake_flags::StakeFlags;
        use solana_stake_program::stake_state::Authorized;
        use solana_stake_program::stake_state::Delegation;
        use solana_stake_program::stake_state::Meta;
        use solana_stake_program::stake_state::Stake;
        use solana_stake_program::stake_state::StakeStateV2;

        let rent = Rent::default();
        let rent_exempt_lamports = rent.minimum_balance(StakeStateV2::size_of());
        let minimum_delegation = LAMPORTS_PER_SOL; // TODO: a way to get minimum delegation with feature set?
        let minimum_lamports = rent_exempt_lamports.saturating_add(minimum_delegation);

        let stake_state = StakeStateV2::Stake(
            Meta {
                authorized: Authorized { staker, withdrawer },
                lockup: lockup.unwrap_or_default(),
                rent_exempt_reserve: rent_exempt_lamports,
            },
            Stake {
                delegation: Delegation {
                    stake,
                    activation_epoch,
                    voter_pubkey,
                    deactivation_epoch: if let Some(epoch) = deactivation_epoch {
                        epoch
                    } else {
                        u64::MAX
                    },
                    ..Delegation::default()
                },
                ..Stake::default()
            },
            StakeFlags::default(),
        );
        let account = AccountSharedData::new_data_with_space(
            if stake > minimum_lamports {
                stake
            } else {
                minimum_lamports
            },
            &stake_state,
            StakeStateV2::size_of(),
            &solana_sdk::stake::program::ID,
        )
        .unwrap();

        self.set_account_custom(&address, &account);
    }

    #[cfg(feature = "stake")]
    pub fn create_initialized_account(
        &mut self,
        address: Pubkey,
        staker: Pubkey,
        withdrawer: Pubkey,
        lockup: Option<Lockup>,
    ) {
        use solana_sdk::program_pack::Pack;
        use solana_sdk::rent::Rent;
        use solana_stake_program::stake_state::Authorized;
        use solana_stake_program::stake_state::Meta;
        use solana_stake_program::stake_state::StakeStateV2;

        let rent = Rent::default();
        let rent_exempt_lamports = rent.minimum_balance(StakeStateV2::size_of());

        let stake_state = StakeStateV2::Initialized(Meta {
            authorized: Authorized { staker, withdrawer },
            lockup: lockup.unwrap_or_default(),
            rent_exempt_reserve: rent_exempt_lamports,
        });
        let account = AccountSharedData::new_data_with_space(
            rent_exempt_lamports,
            &stake_state,
            StakeStateV2::size_of(),
            &solana_sdk::stake::program::ID,
        )
        .unwrap();
        self.set_account_custom(&address, &account);
    }

    #[cfg(feature = "vote")]
    pub fn create_vote_account(
        &mut self,
        address: Pubkey,
        node_pubkey: &Pubkey,
        authorized_voter: &Pubkey,
        authorized_withdrawer: &Pubkey,
        commission: u8,
        clock: &Clock,
    ) {
        use solana_sdk::program_pack::Pack;
        use solana_sdk::rent::Rent;
        use solana_sdk::vote::state::VoteInit;
        use solana_sdk::vote::state::VoteState;
        use solana_sdk::vote::state::VoteStateVersions;

        let rent = Rent::default();
        let lamports = rent.minimum_balance(VoteState::size_of());
        let mut account = AccountSharedData::new(
            lamports,
            VoteState::size_of(),
            &solana_sdk::vote::program::ID,
        );

        let vote_state = VoteState::new(
            &VoteInit {
                node_pubkey: *node_pubkey,
                authorized_voter: *authorized_voter,
                authorized_withdrawer: *authorized_withdrawer,
                commission,
            },
            clock,
        );

        VoteState::serialize(
            &VoteStateVersions::Current(Box::new(vote_state)),
            account.data_as_mut_slice(),
        )
        .unwrap();

        self.set_account_custom(&address, &account);
    }

    fn handle_tx_result(
        &mut self,
        tx_result: &TransactionExecutionResult,
        transaction_name: &str,
        instructions: &[Instruction],
    ) -> solana_sdk::transaction::Result<()> {
        let fuzzing_metrics = std::env::var("FUZZING_METRICS");
        let fuzzing_debug = std::env::var("TRIDENT_FUZZ_DEBUG");

        match tx_result {
            TransactionExecutionResult::Executed {
                details,
                programs_modified_by_tx: _,
            } => match &details.status {
                Ok(_) => {
                    // Record successful execution
                    if fuzzing_metrics.is_ok() {
                        self.fuzzing_data
                            .add_successful_transaction(transaction_name);
                    }
                    Ok(())
                }
                Err(transaction_error) => {
                    if let TransactionError::InstructionError(_error_code, instruction_error) =
                        &transaction_error
                    {
                        match instruction_error {
                            InstructionError::ProgramFailedToComplete => {
                                if fuzzing_metrics.is_ok() {
                                    if fuzzing_debug.is_ok() {
                                        trident_svm::prelude::trident_svm_log::log_message(
                                            "TRANSACTION PANICKED",
                                            trident_svm::prelude::Level::Error,
                                        );
                                    }
                                    let rng = self.rng.get_seed();

                                    // TODO format instructions
                                    let tx = format!("{:#?}", instructions);

                                    self.fuzzing_data.add_transaction_panicked(
                                        transaction_name,
                                        rng,
                                        instruction_error.to_string(),
                                        details.log_messages.clone(),
                                        tx,
                                    );
                                }
                            }
                            InstructionError::Custom(error_code) => {
                                if fuzzing_metrics.is_ok() {
                                    self.fuzzing_data.add_custom_instruction_error(
                                        transaction_name,
                                        error_code,
                                        details.log_messages.clone(),
                                    );
                                }
                            }
                            _ => {
                                if fuzzing_metrics.is_ok() {
                                    self.fuzzing_data.add_failed_transaction(
                                        transaction_name,
                                        instruction_error.to_string(),
                                        details.log_messages.clone(),
                                    );
                                }
                            }
                        }
                    } else if fuzzing_metrics.is_ok() {
                        self.fuzzing_data.add_failed_transaction(
                            transaction_name,
                            transaction_error.to_string(),
                            details.log_messages.clone(),
                        );
                    }
                    Err(transaction_error.clone())
                }
            },
            TransactionExecutionResult::NotExecuted(transaction_error) => {
                Err(transaction_error.clone())
            }
        }
    }
}
