use std::collections::HashMap;

use solana_sdk::account::AccountSharedData;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::traits::FuzzClient;
use crate::types::AccountId;

use super::derive_pda;
use super::AccountMetadata;
use super::PdaSeeds;

pub struct AccountsStorage {
    accounts: HashMap<AccountId, Pubkey>,
    _max_accounts: u8,
}

impl Default for AccountsStorage {
    fn default() -> Self {
        Self::new(2)
    }
}

impl AccountsStorage {
    pub fn new(max_accounts: u8) -> Self {
        let accounts: HashMap<AccountId, Pubkey> = HashMap::new();
        Self {
            accounts,
            _max_accounts: max_accounts,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }

    pub fn get_or_create(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: Option<PdaSeeds>,
        account_metadata: Option<AccountMetadata>,
    ) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(address) => *address,
            None => {
                let address = self.get_or_create_address(seeds);

                let metadata = match account_metadata {
                    Some(metadata) => metadata,
                    None => AccountMetadata {
                        lamports: 500 * LAMPORTS_PER_SOL,
                        space: 0,
                        owner: solana_sdk::system_program::ID,
                    },
                };

                // If account on the address is already initialized, we dont override it
                if client.get_account(&address) == AccountSharedData::default() {
                    let account =
                        AccountSharedData::new(metadata.lamports, metadata.space, &metadata.owner);
                    client.set_account_custom(&address, &account);
                }

                self.accounts.insert(account_id, address);
                address
            }
        }
    }

    #[cfg(feature = "token")]
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_token_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: Option<PdaSeeds>,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: bool,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(address) => *address,
            None => {
                let address = self.get_or_create_address(seeds);

                // If account on the address is already initialized, we dont override it
                if client.get_account(&address) == AccountSharedData::default() {
                    self.create_token_account(
                        client,
                        address,
                        mint,
                        owner,
                        amount,
                        delegate,
                        is_native,
                        delegated_amount,
                        close_authority,
                    );
                }

                self.accounts.insert(account_id, address);

                address
            }
        }
    }

    #[cfg(feature = "token")]
    /// Get Initialized or Create new Mint Account
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_mint_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: Option<PdaSeeds>,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(address) => *address,
            None => {
                let address = self.get_or_create_address(seeds);

                // If account on the address is already initialized, we dont override it
                if client.get_account(&address) == AccountSharedData::default() {
                    self.create_mint_account(client, address, decimals, owner, freeze_authority);
                }

                self.accounts.insert(account_id, address);

                address
            }
        }
    }

    #[cfg(feature = "stake")]
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_delegated_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: Option<PdaSeeds>,
        voter_pubkey: Pubkey,
        staker: Pubkey,
        withdrawer: Pubkey,
        stake: u64,
        activation_epoch: solana_sdk::clock::Epoch,
        deactivation_epoch: Option<solana_sdk::clock::Epoch>,
        lockup: Option<solana_stake_program::stake_state::Lockup>,
    ) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(address) => *address,
            None => {
                let address = self.get_or_create_address(seeds);

                // If account on the address is already initialized, we dont override it
                if client.get_account(&address) == AccountSharedData::default() {
                    self.create_delegated_account(
                        client,
                        address,
                        voter_pubkey,
                        staker,
                        withdrawer,
                        stake,
                        activation_epoch,
                        deactivation_epoch,
                        lockup,
                    );
                }

                self.accounts.insert(account_id, address);

                address
            }
        }
    }

    #[cfg(feature = "stake")]
    #[allow(clippy::too_many_arguments)]
    /// Get Initialized or Create new Initialized Stake Account
    pub fn get_or_create_initialized_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: Option<PdaSeeds>,
        staker: Pubkey,
        withdrawer: Pubkey,
        lockup: Option<solana_stake_program::stake_state::Lockup>,
    ) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(address) => *address,
            None => {
                let address = self.get_or_create_address(seeds);

                // If account on the address is already initialized, we dont override it
                if client.get_account(&address) == AccountSharedData::default() {
                    self.create_initialized_account(client, address, staker, withdrawer, lockup);
                }

                self.accounts.insert(account_id, address);

                address
            }
        }
    }

    #[cfg(feature = "vote")]
    /// Get Initialized or Create new Vote Account
    #[allow(clippy::too_many_arguments)]
    pub fn get_or_create_vote_account(
        &mut self,
        account_id: AccountId,
        client: &mut impl FuzzClient,
        seeds: Option<PdaSeeds>,
        node_pubkey: &Pubkey,
        authorized_voter: &Pubkey,
        authorized_withdrawer: &Pubkey,
        commission: u8,
        clock: &solana_sdk::clock::Clock,
    ) -> Pubkey {
        match self.accounts.get(&account_id) {
            Some(address) => *address,
            None => {
                let address = self.get_or_create_address(seeds);

                // If account on the address is already initialized, we dont override it
                if client.get_account(&address) == AccountSharedData::default() {
                    self.create_vote_account(
                        client,
                        address,
                        node_pubkey,
                        authorized_voter,
                        authorized_withdrawer,
                        commission,
                        clock,
                    );
                }

                self.accounts.insert(account_id, address);

                address
            }
        }
    }

    fn get_or_create_address(&self, seeds: Option<PdaSeeds>) -> Pubkey {
        match seeds {
            Some(seeds) => {
                if let Some(pubkey) = derive_pda(seeds.seeds, &seeds.program_id) {
                    pubkey
                } else {
                    Pubkey::new_unique()
                }
            }
            None => Keypair::new().pubkey(),
        }
    }
}
