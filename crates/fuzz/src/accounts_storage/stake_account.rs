use crate::traits::FuzzClient;

use solana_sdk::account::AccountSharedData;
use solana_sdk::clock::Epoch;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent::Rent;
use solana_sdk::stake::stake_flags::StakeFlags;

use solana_stake_program::stake_state::Authorized;
use solana_stake_program::stake_state::Delegation;
use solana_stake_program::stake_state::Lockup;
use solana_stake_program::stake_state::Meta;
use solana_stake_program::stake_state::Stake;
use solana_stake_program::stake_state::StakeStateV2;

use crate::accounts_storage::account_storage::AccountsStorage;

impl AccountsStorage {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn create_delegated_account(
        &self,
        client: &mut impl FuzzClient,
        address: Pubkey,
        voter_pubkey: Pubkey,
        staker: Pubkey,
        withdrawer: Pubkey,
        stake: u64,
        activation_epoch: Epoch,
        deactivation_epoch: Option<Epoch>,
        lockup: Option<Lockup>,
    ) {
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

        client.set_account_custom(&address, &account);
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn create_initialized_account(
        &self,
        client: &mut impl FuzzClient,
        address: Pubkey,
        staker: Pubkey,
        withdrawer: Pubkey,
        lockup: Option<Lockup>,
    ) {
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
        client.set_account_custom(&address, &account);
    }
}
