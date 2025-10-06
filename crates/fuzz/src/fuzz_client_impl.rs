use solana_sdk::account::AccountSharedData;
use solana_sdk::account::ReadableAccount;
use solana_sdk::account::WritableAccount;
use solana_sdk::clock::Clock;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar::Sysvar;

use trident_config::TridentConfig;

use trident_svm::trident_svm::TridentSVM;
use trident_svm::types::trident_account::TridentAccountSharedData;
#[cfg(any(feature = "syscall-v1", feature = "syscall-v2"))]
use trident_svm::types::trident_entrypoint::TridentEntrypoint;
use trident_svm::types::trident_program::TridentProgram;

use crate::fuzz_client::FuzzClient;

#[cfg(feature = "stake")]
use solana_sdk::clock::Epoch;
#[cfg(feature = "stake")]
use solana_stake_program::stake_state::Lockup;

impl FuzzClient for TridentSVM {
    #[cfg(any(feature = "syscall-v1", feature = "syscall-v2"))]
    fn deploy_entrypoint(&mut self, _program: TridentEntrypoint) {
        self.deploy_entrypoint_program(&_program);
    }

    fn deploy_program(&mut self, program: TridentProgram) {
        self.deploy_binary_program(&program);
    }

    #[doc(hidden)]
    fn new_client() -> Self {
        let config = TridentConfig::new();
        let program_binaries =
            config
                .programs()
                .iter()
                .fold(Vec::new(), |mut sbf_programs, config_program| {
                    let target = TridentProgram::new(
                        config_program.address,
                        config_program.upgrade_authority,
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

        let mut svm_builder = TridentSVM::builder();
        svm_builder.with_syscalls_v1();
        svm_builder.with_syscalls_v2();
        svm_builder.with_sbf_programs(program_binaries);
        svm_builder.with_permanent_accounts(permanent_accounts);

        if std::env::var("TRIDENT_FUZZ_DEBUG_PATH").is_ok()
            && std::env::var("TRIDENT_FUZZ_DEBUG").is_ok()
        {
            let debug_path =
                std::env::var("TRIDENT_FUZZ_DEBUG_PATH").unwrap_or("trident_debug.log".to_string());
            svm_builder.with_debug_file_logs(&debug_path);
        } else if std::env::var("TRIDENT_LOG").is_ok() {
            svm_builder.with_cli_logs();
        }

        svm_builder.build()
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
        self.set_account(address, account, false);
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

    #[doc(hidden)]
    fn _process_instructions(
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

        self.process_transaction_with_settle(tx)
    }

    fn get_sysvar<T: Sysvar>(&self) -> T {
        trident_svm::trident_svm::TridentSVM::get_sysvar::<T>(self)
    }

    fn airdrop(&mut self, address: &Pubkey, amount: u64) {
        let mut account = self.get_account(address);

        account.set_lamports(account.lamports() + amount);
        self.set_account_custom(address, &account);
    }

    #[cfg(feature = "token")]
    fn create_mint_account(
        &mut self,
        address: &Pubkey,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) {
        use solana_sdk::program_option::COption;
        use solana_sdk::program_pack::Pack;
        use solana_sdk::rent::Rent;
        use spl_token::state::Mint;

        let authority = match freeze_authority {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let r = Rent::default();
        let lamports = r.minimum_balance(Mint::LEN);

        let mut account = AccountSharedData::new(lamports, Mint::LEN, &spl_token::id());

        let mint = Mint {
            is_initialized: true,
            mint_authority: COption::Some(*owner),
            freeze_authority: authority,
            decimals,
            ..Default::default()
        };

        let mut data = vec![0u8; Mint::LEN];
        Mint::pack(mint, &mut data[..]).unwrap();
        account.set_data_from_slice(&data);

        self.set_account_custom(address, &account);
    }

    #[cfg(feature = "token")]
    fn create_token_account(
        &mut self,
        address: Pubkey,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) {
        use solana_sdk::program_option::COption;
        use solana_sdk::program_pack::Pack;
        use solana_sdk::rent::Rent;

        let is_native = mint.eq(&spl_token::native_mint::id());

        let delegate = match delegate {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let close_authority = match close_authority {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let r = Rent::default();
        let rent_exempt_lamports = r.minimum_balance(spl_token::state::Account::LEN);

        let account = if is_native {
            let lamports = rent_exempt_lamports.saturating_add(amount);

            let mut account =
                AccountSharedData::new(lamports, spl_token::state::Account::LEN, &spl_token::id());

            let token_account_ = spl_token::state::Account {
                mint,
                owner,
                amount,
                delegate,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::Some(rent_exempt_lamports),
                delegated_amount,
                close_authority,
            };

            let mut data = vec![0u8; spl_token::state::Account::LEN];
            spl_token::state::Account::pack(token_account_, &mut data[..]).unwrap();
            account.set_data_from_slice(&data);

            account
        } else {
            let mut account = AccountSharedData::new(
                rent_exempt_lamports,
                spl_token::state::Account::LEN,
                &spl_token::id(),
            );

            let token_account_ = spl_token::state::Account {
                mint,
                owner,
                amount,
                delegate,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount,
                close_authority,
            };

            let mut data = vec![0u8; spl_token::state::Account::LEN];
            spl_token::state::Account::pack(token_account_, &mut data[..]).unwrap();
            account.set_data_from_slice(&data);

            account
        };

        self.set_account_custom(&address, &account);
    }

    #[cfg(feature = "stake")]
    fn create_delegated_account(
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
    fn create_initialized_account(
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
    fn create_vote_account(
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
}
