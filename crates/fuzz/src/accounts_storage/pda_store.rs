use solana_sdk::{
    account::{AccountSharedData, WritableAccount},
    clock::{Clock, Epoch},
    native_token::LAMPORTS_PER_SOL,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    stake::stake_flags::StakeFlags,
};
use solana_stake_program::stake_state::{
    Authorized, Delegation, Lockup, Meta, Stake, StakeStateV2,
};
use solana_vote_program::vote_state::{VoteInit, VoteState, VoteStateVersions};
use spl_token::state::Mint;

use crate::{fuzz_client::FuzzClient, AccountId};

use super::AccountsStorage;

pub struct PdaStore {
    pub pubkey: Pubkey,
    pub seeds: (Vec<Vec<u8>>, Pubkey),
}
impl PdaStore {
    pub fn pubkey(&self) -> Pubkey {
        self.pubkey
    }
}

impl From<Pubkey> for PdaStore {
    fn from(pubkey: Pubkey) -> Self {
        PdaStore {
            pubkey,
            seeds: (Vec::new(), Pubkey::default()), // Note: This creates empty seeds
        }
    }
}

impl AccountsStorage<PdaStore> {
    pub fn get_or_create_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: &[&[u8]],
        program_id: &Pubkey,
    ) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(v) => v.pubkey,
            None => {
                if let Some((key, _)) = Pubkey::try_find_program_address(seeds, program_id) {
                    let seeds_vec: Vec<_> = seeds.iter().map(|&s| s.to_vec()).collect();
                    let pda_store = PdaStore {
                        pubkey: key,
                        seeds: (seeds_vec, *program_id),
                    };
                    self.accounts.insert(account_id, pda_store);
                    client.set_account_custom(
                        &key,
                        &AccountSharedData::new(0, 0, &solana_sdk::system_program::ID),
                    );
                    key
                } else {
                    Pubkey::new_unique()
                }
            }
        }
    }
    /// Get Initialized or Create new Token Account
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_token_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: &[&[u8]],
        program_id: &Pubkey,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Pubkey {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let address = derive_pda(seeds, program_id);

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

            let token_account_ = spl_token::state::Account {
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
            spl_token::state::Account::pack(token_account_, &mut data[..]).unwrap();
            account.set_data_from_slice(&data);

            client.set_account_custom(&address.0, &account);

            let seeds_vec: Vec<_> = seeds.iter().map(|&s| s.to_vec()).collect();
            PdaStore {
                pubkey: address.0,
                seeds: (seeds_vec, *program_id),
            }
        });
        key.pubkey
    }
    /// Get Initialized or Create new Mint Account
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_mint_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: &[&[u8]],
        program_id: &Pubkey,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Pubkey {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let address = derive_pda(seeds, program_id);

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

            client.set_account_custom(&address.0, &account);

            let seeds_vec: Vec<_> = seeds.iter().map(|&s| s.to_vec()).collect();
            PdaStore {
                pubkey: address.0,
                seeds: (seeds_vec, *program_id),
            }
        });
        key.pubkey
    }
    /// Get Initialized or Create new Delegated Stake Account
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_delegated_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: &[&[u8]],
        program_id: &Pubkey,
        voter_pubkey: Pubkey,
        staker: Pubkey,
        withdrawer: Pubkey,
        stake: u64,
        activation_epoch: Epoch,
        deactivation_epoch: Option<Epoch>,
        lockup: Option<Lockup>,
    ) -> Pubkey {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let address = derive_pda(seeds, program_id);

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

            client.set_account_custom(&address.0, &account);

            let seeds_vec: Vec<_> = seeds.iter().map(|&s| s.to_vec()).collect();
            PdaStore {
                pubkey: address.0,
                seeds: (seeds_vec, *program_id),
            }
        });
        key.pubkey
    }
    #[allow(clippy::too_many_arguments)]
    /// Get Initialized or Create new Initialized Stake Account
    pub fn get_or_create_initialized_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: &[&[u8]],
        program_id: &Pubkey,
        staker: Pubkey,
        withdrawer: Pubkey,
        lockup: Option<Lockup>,
    ) -> Pubkey {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let address = derive_pda(seeds, program_id);

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
            client.set_account_custom(&address.0, &account);

            let seeds_vec: Vec<_> = seeds.iter().map(|&s| s.to_vec()).collect();
            PdaStore {
                pubkey: address.0,
                seeds: (seeds_vec, *program_id),
            }
        });
        key.pubkey
    }
    /// Get Initialized or Create new Vote Account
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_vote_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: &[&[u8]],
        program_id: &Pubkey,
        node_pubkey: &Pubkey,
        authorized_voter: &Pubkey,
        authorized_withdrawer: &Pubkey,
        commission: u8,
        clock: &Clock,
    ) -> Pubkey {
        let key = self.accounts.entry(account_id).or_insert_with(|| {
            let address = derive_pda(seeds, program_id);

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

            client.set_account_custom(&address.0, &account);

            let seeds_vec: Vec<_> = seeds.iter().map(|&s| s.to_vec()).collect();
            PdaStore {
                pubkey: address.0,
                seeds: (seeds_vec, *program_id),
            }
        });
        key.pubkey
    }
    pub fn get(&self, account_id: AccountId) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(v) => v.pubkey,
            None => Pubkey::new_unique(),
        }
    }
}

fn derive_pda(seeds: &[&[u8]], program_id: &Pubkey) -> (Pubkey, u8) {
    if let Some(address) = Pubkey::try_find_program_address(seeds, program_id) {
        address
    } else {
        panic!("PDA Store, seeds did not create valid PDA address")
    }
}
