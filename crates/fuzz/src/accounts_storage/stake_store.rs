use solana_sdk::{
    account::AccountSharedData, clock::Epoch, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
    rent::Rent, signature::Keypair, signer::Signer, stake::stake_flags::StakeFlags,
};
use solana_stake_program::stake_state::{
    Authorized, Delegation, Lockup, Meta, Stake, StakeStateV2,
};

use crate::{fuzz_client::FuzzClient, AccountId};

use super::AccountsStorage;

pub struct StakeStore {
    pub pubkey: Pubkey,
}
impl From<Pubkey> for StakeStore {
    fn from(pubkey: Pubkey) -> Self {
        StakeStore { pubkey }
    }
}

impl AccountsStorage<StakeStore> {
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
    ) -> Pubkey {
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

            StakeStore {
                pubkey: stake_account.pubkey(),
            }
        });
        key.pubkey
    }
    pub fn get_or_create_initialized_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        staker: Pubkey,
        withdrawer: Pubkey,
        lockup: Option<Lockup>,
    ) -> Pubkey {
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

            StakeStore {
                pubkey: stake_account.pubkey(),
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
