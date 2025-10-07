use solana_sdk::pubkey::Pubkey;

use crate::trident::Trident;

use solana_sdk::account::AccountSharedData;

// #[cfg(feature = "stake")]
use solana_sdk::clock::Epoch;
// #[cfg(feature = "stake")]
use solana_stake_program::stake_state::Lockup;

impl Trident {
    // #[cfg(feature = "stake")]
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

    // #[cfg(feature = "stake")]
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
}
