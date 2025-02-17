use crate::traits::FuzzClient;

use solana_sdk::account::AccountSharedData;
use solana_sdk::program_option::COption;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent::Rent;

use crate::accounts_storage::account_storage::AccountsStorage;

impl AccountsStorage {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn create_token_account(
        &self,
        client: &mut impl FuzzClient,
        address: Pubkey,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: bool,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) {
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

        client.set_account_custom(&address, &account);
    }
}
