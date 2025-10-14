use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod types;
use types::*;

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Trident client for interacting with the Solana program
    trident: Trident,
    /// Storage for all account addresses used in fuzz testing
    fuzz_accounts: AccountAddresses,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: AccountAddresses::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        // Perform any initialization here, this method will be executed
        // at the start of each iteration
    }

    #[flow]
    fn empty_extensions(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);

        let res = self
            .trident
            .create_mint_2022(&mint2022, 7, &author2022, None, &[]);

        assert!(
            res.is_ok(),
            "Empty extensions failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.is_empty());
        assert!(res.mint.freeze_authority.is_none());
        assert!(res.mint.decimals == 7);
        assert!(res.mint.supply == 0);
    }

    #[flow]
    fn pausable_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);
        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[MintExtension::Pausable {
                authority: author2022,
            }],
        );

        assert!(
            res.is_ok(),
            "Pausable extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 1);
    }

    #[flow]
    fn scaled_ui_amount_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);
        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[MintExtension::ScaledUiAmount {
                authority: Some(author2022),
                multiplier: 1.0,
            }],
        );

        assert!(
            res.is_ok(),
            "ScaledUiAmount extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 1);
    }

    #[flow]
    fn token_metadata_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);

        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[
                MintExtension::MetadataPointer {
                    authority: Some(author2022),
                    metadata_address: Some(mint2022),
                },
                MintExtension::TokenMetadata {
                    name: "Test".to_string(),
                    symbol: "Test".to_string(),
                    uri: "Test".to_string(),
                    mint: mint2022,
                    additional_metadata: vec![],
                    update_authority: Some(author2022),
                    metadata: mint2022,
                },
            ],
        );

        assert!(
            res.is_ok(),
            "MetadataPointer + TokenMetadata extensions failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 2);
        assert!(res.mint.decimals == 5);
        assert!(res.mint.supply == 0);
        assert!(res.mint.is_initialized);
    }

    #[flow]
    fn transfer_fee_config_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);
        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[MintExtension::TransferFeeConfig {
                transfer_fee_config_authority: Some(author2022),
                withdraw_withheld_authority: Some(author2022),
                transfer_fee_basis_points: 100,
                maximum_fee: 100,
            }],
        );

        assert!(
            res.is_ok(),
            "TransferFeeConfig extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 1);
    }

    #[flow]
    fn mint_authority_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);
        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[MintExtension::MintCloseAuthority {
                close_authority: Some(author2022),
            }],
        );

        assert!(
            res.is_ok(),
            "MintCloseAuthority extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 1);
    }

    #[flow]
    fn interest_bearing_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);

        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[MintExtension::InterestBearingConfig {
                rate_authority: Some(author2022),
                rate: 100,
            }],
        );

        assert!(
            res.is_ok(),
            "InterestBearingConfig extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 1);
    }

    #[flow]
    fn non_transferable_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);

        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[MintExtension::NonTransferable {}],
        );

        assert!(
            res.is_ok(),
            "NonTransferable extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 1);
    }

    #[flow]
    fn permanent_delegate_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);
        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[MintExtension::PermanentDelegate {
                delegate: author2022,
            }],
        );

        assert!(
            res.is_ok(),
            "PermanentDelegate extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 1);
    }

    #[flow]
    fn transfer_hook_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);
        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[MintExtension::TransferHook {
                authority: Some(author2022),
                program_id: Some(mint2022),
            }],
        );

        assert!(
            res.is_ok(),
            "TransferHook extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 1);
    }

    #[flow]
    fn transfer_fee_pausable_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);
        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[
                MintExtension::TransferFeeConfig {
                    transfer_fee_config_authority: Some(author2022),
                    withdraw_withheld_authority: Some(author2022),
                    transfer_fee_basis_points: 100,
                    maximum_fee: 100,
                },
                MintExtension::Pausable {
                    authority: author2022,
                },
            ],
        );

        assert!(
            res.is_ok(),
            "TransferFeeConfig + Pausable extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 2);
    }

    #[flow]
    fn transfer_fee_metadata_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);
        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[
                MintExtension::TransferFeeConfig {
                    transfer_fee_config_authority: Some(author2022),
                    withdraw_withheld_authority: Some(author2022),
                    transfer_fee_basis_points: 100,
                    maximum_fee: 100,
                },
                MintExtension::MetadataPointer {
                    authority: Some(author2022),
                    metadata_address: Some(mint2022),
                },
                MintExtension::TokenMetadata {
                    name: "Test".to_string(),
                    symbol: "Test".to_string(),
                    uri: "Test".to_string(),
                    mint: mint2022,
                    additional_metadata: vec![],
                    update_authority: Some(author2022),
                    metadata: mint2022,
                },
            ],
        );

        assert!(
            res.is_ok(),
            "TransferFeeConfig + MetadataPointer + TokenMetadata extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 3);
    }

    #[flow]
    fn group_mint_extension(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        // ===== GROUP MINT CREATION =====
        self.trident.airdrop(&author2022, 10 * LAMPORTS_PER_SOL);
        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[
                MintExtension::GroupPointer {
                    authority: Some(author2022),
                    group_address: Some(mint2022),
                },
                MintExtension::TokenGroup {
                    group: mint2022,
                    update_authority: Some(author2022),
                    max_size: 100,
                },
            ],
        );

        assert!(
            res.is_ok(),
            "GroupPointer + TokenGroup extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 2);

        // ===== MEMBER MINT CREATION =====

        let group_member_mint2022 = self
            .fuzz_accounts
            .group_member_mint2022
            .insert(&mut self.trident, None);

        let res = self.trident.create_mint_2022(
            &group_member_mint2022,
            5,
            &author2022,
            None,
            &[
                MintExtension::GroupMemberPointer {
                    authority: Some(author2022),
                    member_address: Some(group_member_mint2022),
                },
                MintExtension::TokenGroupMember {
                    group: mint2022,
                    group_update_authority: author2022,
                },
            ],
        );

        assert!(
            res.is_ok(),
            "GroupMemberPointer + TokenGroupMember extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&group_member_mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 2);
    }

    #[flow]
    fn multiple_extensions(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows

        let author2022 = self
            .fuzz_accounts
            .author2022
            .insert(&mut self.trident, None);

        let mint2022 = self.fuzz_accounts.mint2022.insert(&mut self.trident, None);

        self.trident.airdrop(&author2022, 20 * LAMPORTS_PER_SOL);

        let res = self.trident.create_mint_2022(
            &mint2022,
            5,
            &author2022,
            None,
            &[
                MintExtension::GroupPointer {
                    authority: Some(author2022),
                    group_address: Some(mint2022),
                },
                MintExtension::TokenGroup {
                    group: mint2022,
                    update_authority: Some(author2022),
                    max_size: 100,
                },
                MintExtension::MetadataPointer {
                    authority: Some(author2022),
                    metadata_address: Some(mint2022),
                },
                MintExtension::TokenMetadata {
                    name: "Test".to_string(),
                    symbol: "Test".to_string(),
                    uri: "Test".to_string(),
                    mint: mint2022,
                    additional_metadata: vec![],
                    update_authority: Some(author2022),
                    metadata: mint2022,
                },
                MintExtension::Pausable {
                    authority: author2022,
                },
                MintExtension::TransferFeeConfig {
                    transfer_fee_config_authority: Some(author2022),
                    withdraw_withheld_authority: Some(author2022),
                    transfer_fee_basis_points: 100,
                    maximum_fee: 100,
                },
                MintExtension::ScaledUiAmount {
                    authority: Some(author2022),
                    multiplier: 1.5,
                },
                MintExtension::MintCloseAuthority {
                    close_authority: Some(author2022),
                },
                MintExtension::DefaultAccountState { state: 1 },
            ],
        );

        assert!(
            res.is_ok(),
            "GroupPointer + TokenGroup + MetadataPointer + TokenMetadata + Pausable + TransferFeeConfig + ScaledUiAmount + MintCloseAuthority extension failed: {}",
            res.err().unwrap()
        );

        let mint_2022_with_extensions = self.trident.get_account(&mint2022);

        let res = self
            .trident
            .deserialize_mint_2022(&mint_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 9);

        let token_account2022_1 = self
            .fuzz_accounts
            .token_account2022
            .insert(&mut self.trident, None);

        let res = self.trident.create_token_account_2022(
            &token_account2022_1,
            &mint2022,
            &author2022,
            &[
                AccountExtension::ImmutableOwner,
                AccountExtension::CpiGuard,
                AccountExtension::MemoTransfer {
                    require_incoming_transfer_memos: true,
                },
            ],
        );

        assert!(
            res.is_ok(),
            "TokenAccount creation failed: {}",
            res.err().unwrap()
        );

        let token_account_2022_with_extensions = self.trident.get_account(&token_account2022_1);

        let res = self
            .trident
            .deserialize_token_account_2022(&token_account_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 5);

        let res = self.trident.mint_to_2022(
            &token_account2022_1,
            &mint2022,
            &author2022,
            1000000000000000000,
        );

        assert!(res.is_ok(), "Mint to failed: {}", res.err().unwrap());

        let token_account2022_2 = self
            .fuzz_accounts
            .token_account2022
            .insert(&mut self.trident, None);

        let res = self.trident.create_token_account_2022(
            &token_account2022_2,
            &mint2022,
            &author2022,
            &[AccountExtension::ImmutableOwner, AccountExtension::CpiGuard],
        );

        assert!(
            res.is_ok(),
            "TokenAccount creation failed: {}",
            res.err().unwrap()
        );

        let token_account_2022_with_extensions = self.trident.get_account(&token_account2022_2);

        let res = self
            .trident
            .deserialize_token_account_2022(&token_account_2022_with_extensions)
            .unwrap();

        assert!(res.extensions.len() == 4);

        let res = self.trident.transfer_checked_token_2022(
            &token_account2022_1,
            &token_account2022_2,
            &mint2022,
            &author2022,
            &[],
            1000000000000000000,
            5,
        );

        assert!(
            res.is_ok(),
            "Transfer checked token failed: {}",
            res.err().unwrap()
        );

        let token_account_2022_with_extensions = self.trident.get_account(&token_account2022_2);

        let res = self
            .trident
            .deserialize_token_account_2022(&token_account_2022_with_extensions)
            .unwrap();

        assert!(res.account.amount == 999999999999999900);
    }

    #[end]
    fn end(&mut self) {
        // Perform any cleanup here, this method will be executed
        // at the end of each iteration
    }
}

fn main() {
    FuzzTest::fuzz(1000, 1);
}
