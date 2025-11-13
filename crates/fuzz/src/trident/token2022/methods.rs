//! Token 2022 integration for Trident fuzzing framework
//!
//! This module provides comprehensive support for SPL Token 2022 program,
//! including all extensions and proper initialization order handling.

use solana_sdk::account::ReadableAccount;
use solana_sdk::instruction::Instruction;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent::Rent;
use spl_pod::optional_keys::OptionalNonZeroPubkey;
use spl_token_2022_interface::extension::BaseStateWithExtensions;
use spl_token_2022_interface::extension::ExtensionType;
use spl_token_2022_interface::extension::StateWithExtensions;
use spl_token_2022_interface::state::Account;
use spl_token_2022_interface::state::Mint;

use crate::trident::token2022::AccountExtension;
use crate::trident::token2022::MintExtension;
use crate::trident::token2022::MintExtensionData;
use crate::trident::token2022::MintWithExtensions;
use crate::trident::token2022::TokenAccountExtensionData;
use crate::trident::token2022::TokenAccountWithExtensions;
use crate::trident::Trident;

/// Account state values for DefaultAccountState extension
const ACCOUNT_STATE_UNINITIALIZED: u8 = 0;
const ACCOUNT_STATE_INITIALIZED: u8 = 1;
const ACCOUNT_STATE_FROZEN: u8 = 2;

impl Trident {
    /// Creates instructions to initialize a Token 2022 mint with specified extensions
    ///
    /// Generates instructions with the proper initialization order for all extensions,
    /// ensuring that pre-mint extensions are initialized before the mint itself,
    /// and post-mint extensions are initialized afterward.
    ///
    /// # Arguments
    ///
    /// * `payer` - The payer covering the rent
    /// * `mint_address` - The public key for the new mint
    /// * `decimals` - Number of decimal places for the token
    /// * `mint_authority` - Authority that can mint new tokens
    /// * `freeze_authority` - Optional authority that can freeze accounts
    /// * `extensions` - Array of extensions to enable on the mint
    ///
    /// # Returns
    ///
    /// A vector of instructions that need to be executed with `process_transaction`
    pub fn initialize_mint_2022(
        &mut self,
        payer: &Pubkey,
        mint_address: &Pubkey,
        decimals: u8,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
        extensions: &[MintExtension],
    ) -> Vec<Instruction> {
        let mut extension_types: Vec<ExtensionType> = Vec::new();
        let mut rent_top_ups: Vec<usize> = Vec::new();
        let mut extension_names: Vec<String> = Vec::new();

        for ext in extensions {
            match ext {
                // MintExtension::ConfidentialTransferMint { .. } => {
                //     extension_types.push(ExtensionType::ConfidentialTransferMint);
                //     extension_names.push(format!("{:?}", ExtensionType::ConfidentialTransferMint));
                // }
                // MintExtension::ConfidentialMintBurn => {
                //     extension_types.push(ExtensionType::ConfidentialMintBurn);
                //     extension_names.push(format!("{:?}", ExtensionType::ConfidentialMintBurn));
                // }
                MintExtension::TransferFeeConfig { .. } => {
                    extension_types.push(ExtensionType::TransferFeeConfig);
                    extension_names.push(format!("{:?}", ExtensionType::TransferFeeConfig));
                }
                MintExtension::MintCloseAuthority { .. } => {
                    extension_types.push(ExtensionType::MintCloseAuthority);
                    extension_names.push(format!("{:?}", ExtensionType::MintCloseAuthority));
                }
                MintExtension::InterestBearingConfig { .. } => {
                    extension_types.push(ExtensionType::InterestBearingConfig);
                    extension_names.push(format!("{:?}", ExtensionType::InterestBearingConfig));
                }
                MintExtension::NonTransferable => {
                    extension_types.push(ExtensionType::NonTransferable);
                    extension_names.push(format!("{:?}", ExtensionType::NonTransferable));
                }
                MintExtension::PermanentDelegate { .. } => {
                    extension_types.push(ExtensionType::PermanentDelegate);
                    extension_names.push(format!("{:?}", ExtensionType::PermanentDelegate));
                }
                MintExtension::TransferHook { .. } => {
                    extension_types.push(ExtensionType::TransferHook);
                    extension_names.push(format!("{:?}", ExtensionType::TransferHook));
                }
                MintExtension::MetadataPointer { .. } => {
                    extension_types.push(ExtensionType::MetadataPointer);
                    extension_names.push(format!("{:?}", ExtensionType::MetadataPointer));
                }
                MintExtension::GroupPointer { .. } => {
                    extension_types.push(ExtensionType::GroupPointer);
                    extension_names.push(format!("{:?}", ExtensionType::GroupPointer));
                }
                MintExtension::GroupMemberPointer { .. } => {
                    extension_types.push(ExtensionType::GroupMemberPointer);
                    extension_names.push(format!("{:?}", ExtensionType::GroupMemberPointer));
                }
                MintExtension::ScaledUiAmount { .. } => {
                    extension_types.push(ExtensionType::ScaledUiAmount);
                    extension_names.push(format!("{:?}", ExtensionType::ScaledUiAmount));
                }
                MintExtension::Pausable { .. } => {
                    extension_types.push(ExtensionType::Pausable);
                    extension_names.push(format!("{:?}", ExtensionType::Pausable));
                }
                MintExtension::DefaultAccountState { .. } => {
                    extension_types.push(ExtensionType::DefaultAccountState);
                    extension_names.push(format!("{:?}", ExtensionType::DefaultAccountState));
                }
                MintExtension::TokenMetadata {
                    mint,
                    name,
                    symbol,
                    uri,
                    additional_metadata,
                    update_authority,
                    metadata,
                } => {
                    if metadata.eq(mint_address) {
                        let token_metadata = spl_token_metadata_interface::state::TokenMetadata {
                            update_authority: OptionalNonZeroPubkey::try_from(*update_authority)
                                .unwrap_or_default(),
                            mint: *mint,
                            name: name.clone(),
                            symbol: symbol.clone(),
                            uri: uri.clone(),
                            additional_metadata: additional_metadata.clone(),
                        };

                        let metadata_len = token_metadata.tlv_size_of().unwrap();
                        rent_top_ups.push(metadata_len);
                    }

                    extension_names.push(format!("{:?}", ExtensionType::TokenMetadata));
                }
                MintExtension::TokenGroup { .. } => {
                    let token_group_len = ExtensionType::try_calculate_account_len::<Mint>(&[
                        ExtensionType::TokenGroup,
                    ])
                    .unwrap_or_default();
                    extension_names.push(format!("{:?}", ExtensionType::TokenGroup));
                    rent_top_ups.push(token_group_len);
                }
                MintExtension::TokenGroupMember { .. } => {
                    let token_group_member_len =
                        ExtensionType::try_calculate_account_len::<Mint>(&[
                            ExtensionType::TokenGroupMember,
                        ])
                        .unwrap_or_default();
                    extension_names.push(format!("{:?}", ExtensionType::TokenGroupMember));
                    rent_top_ups.push(token_group_member_len);
                }
            }
        }

        // Calculate space needed for the mint account including all extensions
        let mint_space =
            ExtensionType::try_calculate_account_len::<Mint>(&extension_types).unwrap_or(Mint::LEN);

        let total_rent_top_up = rent_top_ups.iter().sum::<usize>();

        let mut instructions = self.create_account_internal(
            mint_address,
            payer,
            mint_space,
            &spl_token_2022_interface::ID,
        );

        // Handle additional rent requirements for variable-length extensions
        if total_rent_top_up > 0 {
            let rent = Rent::default();
            let required_rent = rent.minimum_balance(total_rent_top_up.saturating_add(mint_space));
            let current_balance = self.get_account(mint_address).lamports();

            if current_balance < required_rent {
                let top_up = required_rent.saturating_sub(current_balance);
                let top_up_ix =
                    solana_sdk::system_instruction::transfer(payer, mint_address, top_up);
                instructions.push(top_up_ix);
            }
        }

        // Initialize extensions that must be set before mint initialization
        self.initialize_pre_mint_extensions(mint_address, extensions, &mut instructions);

        // Initialize the mint itself
        let initialize_mint_ix = spl_token_2022_interface::instruction::initialize_mint2(
            &spl_token_2022_interface::ID,
            mint_address,
            mint_authority,
            freeze_authority,
            decimals,
        )
        .unwrap();
        instructions.push(initialize_mint_ix);

        // Initialize extensions that must be set after mint initialization
        self.initialize_post_mint_extensions(
            mint_address,
            mint_authority,
            extensions,
            &mut instructions,
        );

        instructions
    }

    /// Initialize mint extensions that must be set before the mint is created
    ///
    /// These extensions modify the mint's structure and must be initialized
    /// before calling `initialize_mint2`.
    fn initialize_pre_mint_extensions(
        &self,
        mint_address: &Pubkey,
        extensions: &[MintExtension],
        instructions: &mut Vec<solana_sdk::instruction::Instruction>,
    ) {
        for extension in extensions {
            match extension {
                // MintExtension::ConfidentialTransferMint { .. } => {
                //     // TODO: Confidential transfer mint initialization - requires ElGamal keys
                // }
                // MintExtension::ConfidentialMintBurn => {
                //     // TODO: Confidential mint-burn initialization - requires ElGamal keys
                // }
                MintExtension::TransferFeeConfig {
                    transfer_fee_config_authority,
                    withdraw_withheld_authority,
                    transfer_fee_basis_points,
                    maximum_fee,
                } => {
                    if let Ok(ix) = spl_token_2022_interface::extension::transfer_fee::instruction::initialize_transfer_fee_config(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        transfer_fee_config_authority.as_ref(),
                        withdraw_withheld_authority.as_ref(),
                        *transfer_fee_basis_points,
                        *maximum_fee,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::MintCloseAuthority { close_authority } => {
                    if let Ok(ix) = spl_token_2022_interface::instruction::initialize_mint_close_authority(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        close_authority.as_ref(),
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::InterestBearingConfig { rate_authority, rate } => {
                    if let Ok(ix) = spl_token_2022_interface::extension::interest_bearing_mint::instruction::initialize(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        *rate_authority,
                        *rate,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::NonTransferable => {
                    if let Ok(ix) = spl_token_2022_interface::instruction::initialize_non_transferable_mint(
                        &spl_token_2022_interface::ID,
                        mint_address,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::PermanentDelegate { delegate } => {
                    if let Ok(ix) = spl_token_2022_interface::instruction::initialize_permanent_delegate(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        delegate,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::TransferHook { authority, program_id } => {
                    if let Ok(ix) = spl_token_2022_interface::extension::transfer_hook::instruction::initialize(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        *authority,
                        *program_id,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::MetadataPointer { authority, metadata_address } => {
                    if let Ok(ix) = spl_token_2022_interface::extension::metadata_pointer::instruction::initialize(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        *authority,
                        *metadata_address,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::GroupPointer { authority, group_address } => {
                    if let Ok(ix) = spl_token_2022_interface::extension::group_pointer::instruction::initialize(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        *authority,
                        *group_address,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::GroupMemberPointer { authority, member_address } => {
                    if let Ok(ix) = spl_token_2022_interface::extension::group_member_pointer::instruction::initialize(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        *authority,
                        *member_address,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::ScaledUiAmount { authority, multiplier } => {
                    if let Ok(ix) = spl_token_2022_interface::extension::scaled_ui_amount::instruction::initialize(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        *authority,
                        *multiplier,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::Pausable { authority } => {
                    if let Ok(ix) = spl_token_2022_interface::extension::pausable::instruction::initialize(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        authority,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::DefaultAccountState { state } => {
                    let account_state = match *state {
                        ACCOUNT_STATE_UNINITIALIZED => spl_token_2022_interface::state::AccountState::Uninitialized,
                        ACCOUNT_STATE_INITIALIZED => spl_token_2022_interface::state::AccountState::Initialized,
                        ACCOUNT_STATE_FROZEN => spl_token_2022_interface::state::AccountState::Frozen,
                        _ => unreachable!(
                            "Invalid account state: {}. Only {} (Uninitialized), {} (Initialized), and {} (Frozen) are allowed",
                            state, ACCOUNT_STATE_UNINITIALIZED, ACCOUNT_STATE_INITIALIZED, ACCOUNT_STATE_FROZEN
                        ),
                    };
                    if let Ok(ix) = spl_token_2022_interface::extension::default_account_state::instruction::initialize_default_account_state(
                        &spl_token_2022_interface::ID,
                        mint_address,
                        &account_state,
                    ) {
                        instructions.push(ix);
                    }
                }
                MintExtension::TokenMetadata { .. } => {
                    // TokenMetadata is handled in post-mint phase - skip here
                }
                MintExtension::TokenGroup { .. } => {
                    // TokenGroup is initialized in post-mint phase - skip here
                }
                MintExtension::TokenGroupMember { .. } => {
                    // TokenGroupMember is initialized in post-mint phase - skip here
                }
            }
        }
    }

    /// Initialize mint extensions that must be set after the mint is created
    ///
    /// These extensions require the mint to exist before they can be initialized.
    fn initialize_post_mint_extensions(
        &self,
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        extensions: &[MintExtension],
        instructions: &mut Vec<solana_sdk::instruction::Instruction>,
    ) {
        for extension in extensions {
            match extension {
                MintExtension::TokenMetadata {
                    mint: _,
                    name,
                    symbol,
                    uri,
                    additional_metadata: _,
                    update_authority,
                    metadata,
                } => {
                    let metadata_ix = spl_token_metadata_interface::instruction::initialize(
                        &spl_token_2022_interface::ID,
                        metadata,
                        &update_authority.unwrap_or_default(),
                        mint_address,
                        mint_authority,
                        name.clone(),
                        symbol.clone(),
                        uri.clone(),
                    );
                    instructions.push(metadata_ix);
                }
                MintExtension::TokenGroup {
                    group,
                    update_authority,
                    max_size,
                } => {
                    let token_group_ix = spl_token_group_interface::instruction::initialize_group(
                        &spl_token_2022_interface::ID,
                        group,
                        mint_address,
                        mint_authority,
                        *update_authority,
                        *max_size,
                    );

                    instructions.push(token_group_ix);
                }
                MintExtension::TokenGroupMember {
                    group,
                    group_update_authority,
                } => {
                    let token_group_member_ix =
                        spl_token_group_interface::instruction::initialize_member(
                            &spl_token_2022_interface::ID,
                            mint_address,
                            mint_address,
                            mint_authority,
                            group,
                            group_update_authority,
                        );

                    instructions.push(token_group_member_ix);
                }
                _ => {
                    // Other extensions are handled in pre-mint phase - skip here
                }
            }
        }
    }

    /// Initialize account extensions that must be set before the account is created
    ///
    /// These extensions modify the account's structure and must be initialized
    /// before calling `initialize_account3`.
    fn initialize_pre_account_extensions(
        &self,
        token_account_address: &Pubkey,
        extensions: &[AccountExtension],
        instructions: &mut Vec<solana_sdk::instruction::Instruction>,
    ) {
        for extension in extensions {
            match extension {
                AccountExtension::ImmutableOwner => {
                    if let Ok(ix) =
                        spl_token_2022_interface::instruction::initialize_immutable_owner(
                            &spl_token_2022_interface::ID,
                            token_account_address,
                        )
                    {
                        instructions.push(ix);
                    }
                }
                AccountExtension::MemoTransfer { .. } => {
                    // MemoTransfer is handled in post-account phase - skip here
                }
                AccountExtension::CpiGuard => {
                    // CpiGuard is handled in post-account phase - skip here
                }
            }
        }
    }

    /// Initialize account extensions that must be set after the account is created
    ///
    /// These extensions require the token account to exist before they can be initialized.
    fn initialize_post_account_extensions(
        &self,
        token_account_address: &Pubkey,
        owner: &Pubkey,
        extensions: &[AccountExtension],
        instructions: &mut Vec<solana_sdk::instruction::Instruction>,
    ) {
        for extension in extensions {
            match extension {
                AccountExtension::ImmutableOwner => {
                    // ImmutableOwner is handled in pre-account phase - skip here
                }
                AccountExtension::MemoTransfer {
                    require_incoming_transfer_memos,
                } => {
                    // Only enable memo transfers if explicitly requested
                    if *require_incoming_transfer_memos {
                        if let Ok(ix) = spl_token_2022_interface::extension::memo_transfer::instruction::enable_required_transfer_memos(
                            &spl_token_2022_interface::ID,
                            token_account_address,
                            owner,
                            &[],
                        ) {
                            instructions.push(ix);
                        }
                    }
                }
                AccountExtension::CpiGuard => {
                    if let Ok(ix) = spl_token_2022_interface::extension::cpi_guard::instruction::enable_cpi_guard(
                        &spl_token_2022_interface::ID,
                        token_account_address,
                        owner,
                        &[],
                    ) {
                        instructions.push(ix);
                    }
                }
            }
        }
    }

    /// Creates instructions to initialize a Token 2022 token account with specified extensions
    ///
    /// Generates instructions with the proper initialization order for all extensions,
    /// ensuring that pre-account extensions are initialized before the account itself,
    /// and post-account extensions are initialized afterward.
    ///
    /// # Arguments
    ///
    /// * `payer` - The payer covering the rent
    /// * `token_account_address` - The public key for the new token account
    /// * `mint` - The mint this account will hold tokens for
    /// * `owner` - The owner of the token account
    /// * `extensions` - Array of extensions to enable on the account
    ///
    /// # Returns
    ///
    /// A vector of instructions that need to be executed with `process_transaction`
    pub fn initialize_token_account_2022(
        &mut self,
        payer: &Pubkey,
        token_account_address: &Pubkey,
        mint: &Pubkey,
        owner: &Pubkey,
        extensions: &[AccountExtension],
    ) -> Vec<Instruction> {
        let mint_account = self.get_account(mint);
        let state_with_extensions = StateWithExtensions::<Mint>::unpack(mint_account.data())
            .expect("Mint account does not exist");
        let mint_extension_types = state_with_extensions
            .get_extension_types()
            .unwrap_or_default();
        let required_extensions =
            ExtensionType::get_required_init_account_extensions(&mint_extension_types);

        let mut extension_names: Vec<String> = Vec::default();

        // Collect extension types and names for space calculation and logging
        let extension_types: Vec<ExtensionType> = extensions
            .iter()
            .map(|ext| match ext {
                AccountExtension::ImmutableOwner => {
                    extension_names.push(format!("{:?}", ExtensionType::ImmutableOwner));
                    ExtensionType::ImmutableOwner
                }
                AccountExtension::MemoTransfer { .. } => {
                    extension_names.push(format!("{:?}", ExtensionType::MemoTransfer));
                    ExtensionType::MemoTransfer
                }
                AccountExtension::CpiGuard => {
                    extension_names.push(format!("{:?}", ExtensionType::CpiGuard));
                    ExtensionType::CpiGuard
                }
            })
            .chain(required_extensions)
            .collect();

        // Calculate space needed for the account including all extensions
        let account_space = ExtensionType::try_calculate_account_len::<Account>(&extension_types)
            .unwrap_or(Account::LEN);

        let mut instructions = self.create_account_internal(
            token_account_address,
            payer,
            account_space,
            &spl_token_2022_interface::ID,
        );

        // Initialize pre-account extensions (before token account initialization)
        self.initialize_pre_account_extensions(
            token_account_address,
            extensions,
            &mut instructions,
        );

        // Initialize the token account
        let initialize_account_ix = spl_token_2022_interface::instruction::initialize_account3(
            &spl_token_2022_interface::ID,
            token_account_address,
            mint,
            owner,
        )
        .unwrap();
        instructions.push(initialize_account_ix);

        // Initialize post-account extensions (after token account initialization)
        self.initialize_post_account_extensions(
            token_account_address,
            owner,
            extensions,
            &mut instructions,
        );

        instructions
    }

    /// Creates an instruction to mint tokens to a Token 2022 account
    ///
    /// # Arguments
    ///
    /// * `token_account_address` - The account to mint tokens to
    /// * `mint_address` - The mint to mint tokens from
    /// * `mint_authority` - The authority allowed to mint tokens
    /// * `amount` - The number of tokens to mint (in base units)
    ///
    /// # Returns
    ///
    /// An instruction that needs to be executed with `process_transaction`
    pub fn mint_to_2022(
        &mut self,
        token_account_address: &Pubkey,
        mint_address: &Pubkey,
        mint_authority: &Pubkey,
        amount: u64,
    ) -> Instruction {
        spl_token_2022_interface::instruction::mint_to(
            &spl_token_2022_interface::ID,
            mint_address,
            token_account_address,
            mint_authority,
            &[],
            amount,
        )
        .unwrap()
    }

    /// Deserializes a Token 2022 mint account with all its extensions
    ///
    /// # Arguments
    ///
    /// * `account` - The account data to deserialize
    ///
    /// # Returns
    ///
    /// Returns a `MintWithExtensions` containing the mint data and all extensions,
    /// or an error if deserialization fails.
    pub fn get_mint_2022(
        &mut self,
        account: Pubkey,
    ) -> Result<MintWithExtensions, Box<dyn std::error::Error>> {
        let account_data = self.get_account(&account);
        let state_with_extensions = StateWithExtensions::<Mint>::unpack(account_data.data())?;
        let extension_types = state_with_extensions.get_extension_types()?;

        let mut extensions = Vec::new();

        for ext_type in &extension_types {
            match ext_type {
                ExtensionType::TransferFeeConfig => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::transfer_fee::TransferFeeConfig>() {
                        extensions.push(MintExtensionData::TransferFeeConfig(*config));
                    }
                },
                ExtensionType::MintCloseAuthority => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::mint_close_authority::MintCloseAuthority>() {
                        extensions.push(MintExtensionData::MintCloseAuthority(*config));
                    }
                },
                // ExtensionType::ConfidentialTransferMint => {
                //     // TODO: Confidential transfer mint deserialization
                // }
                ExtensionType::DefaultAccountState => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::default_account_state::DefaultAccountState>() {
                        extensions.push(MintExtensionData::DefaultAccountState(*config));
                    }
                },
                ExtensionType::NonTransferable => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::non_transferable::NonTransferable>() {
                        extensions.push(MintExtensionData::NonTransferable(*config));
                    }
                },
                ExtensionType::InterestBearingConfig => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::interest_bearing_mint::InterestBearingConfig>() {
                        extensions.push(MintExtensionData::InterestBearingConfig(*config));
                    }
                },
                ExtensionType::PermanentDelegate => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::permanent_delegate::PermanentDelegate>() {
                        extensions.push(MintExtensionData::PermanentDelegate(*config));
                    }
                },
                ExtensionType::TransferHook => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::transfer_hook::TransferHook>() {
                        extensions.push(MintExtensionData::TransferHook(*config));
                    }
                },
                // ExtensionType::ConfidentialTransferFeeConfig => {
                //     if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::confidential_transfer_fee::ConfidentialTransferFeeConfig>() {
                //         extensions.push(MintExtensionData::ConfidentialTransferFeeConfig(*config));
                //     }
                // }
                ExtensionType::MetadataPointer => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::metadata_pointer::MetadataPointer>() {
                        extensions.push(MintExtensionData::MetadataPointer(*config));
                    }
                },
                ExtensionType::GroupPointer => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::group_pointer::GroupPointer>() {
                        extensions.push(MintExtensionData::GroupPointer(*config));
                    }
                },
                ExtensionType::GroupMemberPointer => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::group_member_pointer::GroupMemberPointer>() {
                        extensions.push(MintExtensionData::GroupMemberPointer(*config));
                    }
                },
                // ExtensionType::ConfidentialMintBurn => {
                //     if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::confidential_mint_burn::ConfidentialMintBurn>() {
                //         extensions.push(MintExtensionData::ConfidentialMintBurn(*config));
                //     }
                // }
                ExtensionType::ScaledUiAmount => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::scaled_ui_amount::ScaledUiAmountConfig>() {
                        extensions.push(MintExtensionData::ScaledUiAmount(*config));
                    }
                },
                ExtensionType::Pausable => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::pausable::PausableConfig>() {
                        extensions.push(MintExtensionData::Pausable(*config));
                    }
                },
                ExtensionType::TokenMetadata => {
                    if let Ok(metadata) = state_with_extensions.get_variable_len_extension::<spl_token_metadata_interface::state::TokenMetadata>() {
                        extensions.push(MintExtensionData::TokenMetadata(metadata));
                    }
                },
                ExtensionType::TokenGroup => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_group_interface::state::TokenGroup>() {
                        extensions.push(MintExtensionData::TokenGroup(*config));
                    }
                },
                ExtensionType::TokenGroupMember => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_group_interface::state::TokenGroupMember>() {
                        extensions.push(MintExtensionData::TokenGroupMember(*config));
                    }
                },
                _ => {
                    extensions.push(MintExtensionData::Unknown(*ext_type));
                }
            }
        }

        Ok(MintWithExtensions {
            mint: state_with_extensions.base,
            extensions,
        })
    }

    /// Deserializes a Token 2022 token account with all its extensions
    ///
    /// # Arguments
    ///
    /// * `account` - The account data to deserialize
    ///
    /// # Returns
    ///
    /// Returns a `TokenAccountWithExtensions` containing the account data and all extensions,
    /// or an error if deserialization fails.
    pub fn get_token_account_2022(
        &mut self,
        account: Pubkey,
    ) -> Result<TokenAccountWithExtensions, Box<dyn std::error::Error>> {
        let account_data = self.get_account(&account);
        let state_with_extensions = StateWithExtensions::<Account>::unpack(account_data.data())?;
        let extension_types = state_with_extensions.get_extension_types()?;

        let mut extensions = Vec::new();

        for ext_type in &extension_types {
            match ext_type {
                ExtensionType::ImmutableOwner => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::immutable_owner::ImmutableOwner>() {
                        extensions.push(TokenAccountExtensionData::ImmutableOwner(*config));
                    }
                },
                ExtensionType::TransferFeeAmount => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::transfer_fee::TransferFeeAmount>() {
                        extensions.push(TokenAccountExtensionData::TransferFeeAmount(*config));
                    }
                },
                // ExtensionType::ConfidentialTransferAccount => {
                //     // TODO: Confidential transfer account deserialization
                // }
                ExtensionType::MemoTransfer => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::memo_transfer::MemoTransfer>() {
                        extensions.push(TokenAccountExtensionData::MemoTransfer(*config));
                    }
                },
                ExtensionType::NonTransferableAccount => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::non_transferable::NonTransferableAccount>() {
                        extensions.push(TokenAccountExtensionData::NonTransferableAccount(*config));
                    }
                },
                ExtensionType::TransferHookAccount => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::transfer_hook::TransferHookAccount>() {
                        extensions.push(TokenAccountExtensionData::TransferHookAccount(*config));
                    }
                },
                ExtensionType::CpiGuard => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::cpi_guard::CpiGuard>() {
                        extensions.push(TokenAccountExtensionData::CpiGuard(*config));
                    }
                },
                // ExtensionType::ConfidentialTransferFeeAmount => {
                //     if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::confidential_transfer_fee::ConfidentialTransferFeeAmount>() {
                //         extensions.push(TokenAccountExtensionData::ConfidentialTransferFeeAmount(*config));
                //     }
                // }
                ExtensionType::PausableAccount => {
                    if let Ok(config) = state_with_extensions.get_extension::<spl_token_2022_interface::extension::pausable::PausableAccount>() {
                        extensions.push(TokenAccountExtensionData::PausableAccount(*config));
                    }
                },
                _ => {
                    extensions.push(TokenAccountExtensionData::Unknown(*ext_type));
                }
            }
        }

        Ok(TokenAccountWithExtensions {
            account: state_with_extensions.base,
            extensions,
        })
    }

    /// Creates an instruction to transfer tokens between Token 2022 accounts with verification
    ///
    /// Generates a checked transfer instruction that verifies both the amount and decimals
    /// to prevent errors due to incorrect decimal places.
    ///
    /// # Arguments
    ///
    /// * `source` - The source token account
    /// * `destination` - The destination token account  
    /// * `mint` - The mint of the tokens being transferred
    /// * `authority` - The authority allowed to transfer from the source account
    /// * `signers` - Additional signers if using multisig
    /// * `amount` - The number of tokens to transfer (in base units)
    /// * `decimals` - The number of decimals for the mint (for verification)
    ///
    /// # Returns
    ///
    /// An instruction that needs to be executed with `process_transaction`
    #[allow(clippy::too_many_arguments)]
    pub fn transfer_checked(
        &mut self,
        source: &Pubkey,
        destination: &Pubkey,
        mint: &Pubkey,
        authority: &Pubkey,
        signers: &[&Pubkey],
        amount: u64,
        decimals: u8,
    ) -> Instruction {
        spl_token_2022_interface::instruction::transfer_checked(
            &spl_token_2022_interface::ID,
            source,
            mint,
            destination,
            authority,
            signers,
            amount,
            decimals,
        )
        .unwrap()
    }

    /// Creates instructions to initialize an associated Token 2022 account with specified extensions
    ///
    /// Generates instructions using the associated token account program to handle initial funding,
    /// account creation, and any mint-required extensions automatically. It then reallocates
    /// space and initializes any additional user-requested extensions.
    ///
    /// The associated token account program automatically handles extensions required by the mint
    /// (such as NonTransferableAccount for NonTransferable mints), so you only need to specify
    /// additional extensions you want to add.
    ///
    /// # Arguments
    ///
    /// * `payer` - The payer covering the rent
    /// * `mint` - The mint this account will hold tokens for
    /// * `owner` - The owner of the token account
    /// * `extensions` - Array of additional extensions to enable on the account
    ///
    /// # Returns
    ///
    /// A vector of instructions that need to be executed with `process_transaction`
    pub fn initialize_associated_token_account_2022(
        &mut self,
        payer: &Pubkey,
        mint: &Pubkey,
        owner: &Pubkey,
        extensions: &[AccountExtension],
    ) -> Vec<Instruction> {
        let address = spl_associated_token_account_interface::address::get_associated_token_address_with_program_id(
            owner, mint, &spl_token_2022_interface::ID,
        );

        let mut instructions = Vec::new();

        // Create the basic associated token account first
        let create_ix =
            spl_associated_token_account_interface::instruction::create_associated_token_account(
                payer,
                owner,
                mint,
                &spl_token_2022_interface::ID, // Use Token 2022 program ID
            );
        instructions.push(create_ix);

        // If we have extensions, we need to reallocate and initialize them
        if !extensions.is_empty() {
            // Collect user-requested extension types only
            let extension_types: Vec<ExtensionType> = extensions
                .iter()
                .map(|ext| match ext {
                    AccountExtension::ImmutableOwner => ExtensionType::ImmutableOwner,
                    AccountExtension::MemoTransfer { .. } => ExtensionType::MemoTransfer,
                    AccountExtension::CpiGuard => ExtensionType::CpiGuard,
                })
                .collect();

            // Reallocate the account to accommodate extensions
            let reallocate_ix = spl_token_2022_interface::instruction::reallocate(
                &spl_token_2022_interface::ID,
                &address,
                payer, // payer (pays for reallocation)
                owner, // owner
                &[],   // signer_pubkeys (empty for single signature)
                &extension_types,
            )
            .unwrap();
            instructions.push(reallocate_ix);

            // Initialize post-account extensions (like MemoTransfer, CpiGuard)
            self.initialize_post_account_extensions(&address, owner, extensions, &mut instructions);
        }

        instructions
    }
}
