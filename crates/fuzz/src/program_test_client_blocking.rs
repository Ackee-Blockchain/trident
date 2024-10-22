use solana_program_runtime::invoke_context::BuiltinFunctionWithContext;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::account::{Account, WritableAccount};
use solana_sdk::account_info::AccountInfo;
use solana_sdk::clock::{Clock, Epoch};
use solana_sdk::entrypoint::ProgramResult;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::stake::stake_flags::StakeFlags;
use solana_sdk::stake::state::{Authorized, Delegation, Lockup, Meta, Stake, StakeStateV2};
use solana_sdk::system_program::ID as SYSTEM_PROGRAM_ID;
use solana_sdk::vote::state::{VoteInit, VoteStateVersions};
use solana_sdk::vote::{program::ID as vote_program_id, state::VoteState};
use solana_sdk::{
    account::AccountSharedData, hash::Hash, instruction::AccountMeta, program_option::COption,
    program_pack::Pack, pubkey::Pubkey, rent::Rent, signature::Keypair, signature::Signer,
    transaction::VersionedTransaction,
};
use spl_token::state::Mint;
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

/// Converts Anchor 0.29.0 and higher entrypoint into the runtime entrypoint style
///
/// Starting Anchor 0.29.0 the accounts are passed by reference https://github.com/coral-xyz/anchor/pull/2656
/// and the lifetime requirements are `accounts: &'a [AccountInfo<'a>]` instead of `accounts: &'a [AccountInfo<'b>]`.
/// The new requirements require the slice of AccountInfos and the contained Accounts to have the same lifetime but
/// the previous version is more general. The compiler implies that `'b` must live at least as long as `'a` or longer.
///
/// The transaction data is serialized and again deserialized to the `&[AccountInfo<_>]` slice just before invoking
/// the entry point and the modified account data is copied to the original accounts just after the entry point.
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
    fn set_account(&mut self, lamports: u64) -> Keypair {
        let owner = Keypair::new();
        let account = AccountSharedData::new(lamports, 0, &SYSTEM_PROGRAM_ID);
        self.ctx.set_account(&owner.pubkey(), &account);
        owner
    }

    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.ctx.set_account(address, account);
    }

    fn set_token_account(
        &mut self,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Pubkey {
        let mint_account_key = Keypair::new().pubkey();

        let delegate = match delegate {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let is_native = match is_native {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let close_authority = match close_authority {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let r = Rent::default();
        let lamports = r.minimum_balance(spl_token::state::Account::LEN);

        let mut account =
            AccountSharedData::new(lamports, spl_token::state::Account::LEN, &spl_token::id());

        let token_account = spl_token::state::Account {
            mint,
            owner,
            amount,
            delegate,
            state: spl_token::state::AccountState::Initialized,
            is_native,
            delegated_amount,
            close_authority,
        };

        let mut data = vec![0u8; spl_token::state::Account::LEN];
        spl_token::state::Account::pack(token_account, &mut data[..]).unwrap();
        account.set_data_from_slice(&data);
        self.ctx.set_account(&mint_account_key, &account);

        mint_account_key
    }

    fn set_mint_account(
        &mut self,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Pubkey {
        let mint_account = Keypair::new();

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
        self.ctx.set_account(&mint_account.pubkey(), &account);

        mint_account.pubkey()
    }

    fn set_vote_account(
        &mut self,
        node_pubkey: &Pubkey,
        authorized_voter: &Pubkey,
        authorized_withdrawer: &Pubkey,
        commission: u8,
        clock: &Clock,
    ) -> Pubkey {
        let vote_account = Keypair::new();

        let rent = Rent::default();
        let lamports = rent.minimum_balance(VoteState::size_of());
        let mut account = AccountSharedData::new(lamports, VoteState::size_of(), &vote_program_id);

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

        self.ctx.set_account(&vote_account.pubkey(), &account);

        vote_account.pubkey()
    }

    fn set_delegated_stake_account(
        &mut self,
        voter_pubkey: Pubkey, // vote account delegated to
        staker: Pubkey,
        withdrawer: Pubkey,
        stake: u64,
        activation_epoch: Epoch,
        deactivation_epoch: Option<Epoch>,
        lockup: Option<Lockup>,
    ) -> Pubkey {
        let stake_account = Keypair::new();

        let rent = Rent::default();
        let rent_exempt_lamports = rent.minimum_balance(StakeStateV2::size_of());
        let minimum_delegation = LAMPORTS_PER_SOL; // TODO: a way to get minimum delegation with feature set?
        let minimum_lamports = rent_exempt_lamports + minimum_delegation;

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

        self.ctx.set_account(&stake_account.pubkey(), &account);

        stake_account.pubkey()
    }

    fn set_initialized_stake_account(
        &mut self,
        staker: Pubkey,
        withdrawer: Pubkey,
        lockup: Option<Lockup>,
    ) -> Pubkey {
        let stake_account = Keypair::new();

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

        self.ctx.set_account(&stake_account.pubkey(), &account);

        stake_account.pubkey()
    }

    fn payer(&self) -> Keypair {
        self.ctx.payer.insecure_clone()
    }
    fn get_account(&mut self, key: &Pubkey) -> Result<Option<Account>, FuzzClientError> {
        Ok(self
            .rt
            .block_on(self.ctx.banks_client.get_account_with_commitment(
                *key,
                solana_sdk::commitment_config::CommitmentLevel::Confirmed,
            ))?)
    }

    fn get_accounts(
        &mut self,
        metas: &[AccountMeta],
    ) -> Result<Vec<Option<Account>>, FuzzClientErrorWithOrigin> {
        let result: Vec<_> = metas
            .iter()
            .map(|m| {
                self.get_account(&m.pubkey)
                    .map_err(|e| e.with_origin(Origin::Account(m.pubkey)))
            })
            .collect();
        result.into_iter().collect()
    }
    fn get_last_blockhash(&self) -> Hash {
        self.ctx.last_blockhash
    }

    fn get_rent(&mut self) -> Result<Rent, FuzzClientError> {
        Ok(self.rt.block_on(self.ctx.banks_client.get_rent())?)
    }

    fn process_transaction(
        &mut self,
        transaction: impl Into<VersionedTransaction>,
    ) -> Result<(), FuzzClientError> {
        Ok(self
            .rt
            .block_on(self.ctx.banks_client.process_transaction(transaction))?)
    }
}
