use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    clock::{Clock, Epoch},
    native_token::LAMPORTS_PER_SOL,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    signature::Keypair,
    signer::Signer,
    stake::stake_flags::StakeFlags,
};
use solana_stake_program::stake_state::{
    Authorized, Delegation, Lockup, Meta, Stake, StakeStateV2,
};
use solana_vote_program::vote_state::{VoteInit, VoteState, VoteStateVersions};
use spl_token::state::Mint;

use crate::traits::FuzzClient;
use crate::types::AccountId;

use super::AccountsStorage;

pub struct KeypairStore {
    pub keypair: Keypair,
}

impl KeypairStore {
    pub fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }
}

// TODO Add an easy way to limit the number of created accounts
impl AccountsStorage<KeypairStore> {
    /// Get Initialized or Create new Solana Wallet
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        lamports: u64,
    ) -> Keypair {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let keypair = Keypair::new();

            let empty_account =
                AccountSharedData::new(lamports, 0, &solana_sdk::system_program::ID);
            client.set_account_custom(&keypair.pubkey(), &empty_account);
            KeypairStore {
                keypair: keypair.insecure_clone(),
            }
        });
        key.keypair.insecure_clone()
    }
    /// Get Initialized or Create new Token Account
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_token_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: bool,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Keypair {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let token_account = Keypair::new();

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

                let mut account = AccountSharedData::new(
                    lamports,
                    spl_token::state::Account::LEN,
                    &spl_token::id(),
                );

                let token_account_ = spl_token::state::Account {
                    mint,
                    owner,
                    amount: lamports,
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

            client.set_account_custom(&token_account.pubkey(), &account);

            KeypairStore {
                keypair: token_account.insecure_clone(),
            }
        });
        key.keypair.insecure_clone()
    }
    /// Get Initialized or Create new Mint Account
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_mint_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Keypair {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
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

            client.set_account_custom(&mint_account.pubkey(), &account);

            KeypairStore {
                keypair: mint_account.insecure_clone(),
            }
        });
        key.keypair.insecure_clone()
    }
    /// Get Initialized or Create new Delegated Stake Account
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_delegated_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        voter_pubkey: Pubkey,
        staker: Pubkey,
        withdrawer: Pubkey,
        stake: u64,
        activation_epoch: Epoch,
        deactivation_epoch: Option<Epoch>,
        lockup: Option<Lockup>,
    ) -> Keypair {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let stake_account = Keypair::new();

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

            client.set_account_custom(&stake_account.pubkey(), &account);

            KeypairStore {
                keypair: stake_account.insecure_clone(),
            }
        });
        key.keypair.insecure_clone()
    }
    /// Get Initialized or Create new Initialized Stake Account
    pub fn get_or_create_initialized_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        staker: Pubkey,
        withdrawer: Pubkey,
        lockup: Option<Lockup>,
    ) -> Keypair {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
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
            client.set_account_custom(&stake_account.pubkey(), &account);

            KeypairStore {
                keypair: stake_account.insecure_clone(),
            }
        });
        key.keypair.insecure_clone()
    }
    /// Get Initialized or Create new Vote Account
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_vote_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        node_pubkey: &Pubkey,
        authorized_voter: &Pubkey,
        authorized_withdrawer: &Pubkey,
        commission: u8,
        clock: &Clock,
    ) -> Keypair {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let vote_account = Keypair::new();

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

            client.set_account_custom(&vote_account.pubkey(), &account);

            KeypairStore {
                keypair: vote_account.insecure_clone(),
            }
        });
        key.keypair.insecure_clone()
    }

    pub fn get(&self, account_id: AccountId) -> Keypair {
        match self.accounts.get(&account_id) {
            Some(v) => v.keypair.insecure_clone(),
            None => Keypair::new(),
        }
    }
}
