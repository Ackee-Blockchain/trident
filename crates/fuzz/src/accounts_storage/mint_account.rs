use crate::traits::FuzzClient;

use solana_sdk::account::AccountSharedData;
use solana_sdk::program_option::COption;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent::Rent;
use spl_token::state::Mint;

use crate::accounts_storage::account_storage::AccountsStorage;

impl AccountsStorage {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn create_mint_account(
        &self,
        client: &mut impl FuzzClient,
        address: Pubkey,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) {
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

        client.set_account_custom(&address, &account);
    }
}
