// token_2022_mint.rs
use crate::traits::FuzzClient;

use solana_sdk::{
    account::AccountSharedData, program_option::COption, program_pack::Pack, pubkey::Pubkey,
    rent::Rent,
};

use std::str::FromStr;

use spl_token_2022::{
    extension::ExtensionType, id as token_2022_program_id,
    solana_program::pubkey::Pubkey as Token2022Pubkey,
};

use spl_pod::optional_keys::OptionalNonZeroPubkey;

use crate::accounts_storage::{account_storage::AccountsStorage, ParamValue};

impl AccountsStorage {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn create_token_2022_mint(
        &self,
        client: &mut impl FuzzClient,
        address: Pubkey,
        decimals: u8,
        mint_authority: &Pubkey,
        freeze_authority: Option<Pubkey>,
        extension_params: Option<Vec<ParamValue>>,
    ) {
        // Determine which extensions to use
        let actual_extensions: Vec<ExtensionType> = extension_params
            .as_ref()
            .map(|params| {
                params
                    .iter()
                    .map(|param| match param {
                        ParamValue::MetadataPointer(_) => ExtensionType::MetadataPointer,
                        ParamValue::GroupMemberPointer(_) => ExtensionType::GroupMemberPointer,
                        ParamValue::TransferHook(_) => ExtensionType::TransferHook,
                        ParamValue::MintCloseAuthority(_) => ExtensionType::MintCloseAuthority,
                        ParamValue::PermanentDelegate(_) => ExtensionType::PermanentDelegate,
                        ParamValue::GroupPointer(_) => ExtensionType::GroupPointer,
                        ParamValue::DefaultAccountState(_) => ExtensionType::DefaultAccountState,
                        ParamValue::NonTransferable(_) => ExtensionType::NonTransferable,
                        ParamValue::InterestBearingConfig(_) => {
                            ExtensionType::InterestBearingConfig
                        }
                        ParamValue::TransferFeeConfig(_) => ExtensionType::TransferFeeConfig,
                        ParamValue::ConfidentialTransferMint(_) => {
                            ExtensionType::ConfidentialTransferMint
                        }
                        ParamValue::ConfidentialTransferFeeConfig(_) => {
                            ExtensionType::ConfidentialTransferFeeConfig
                        }
                        ParamValue::ConfidentialMintBurn(_) => ExtensionType::ConfidentialMintBurn,
                        _ => panic!("Unsupported ParamValue variant"),
                    })
                    .collect()
            })
            .unwrap_or_default();
        // Calculate mint size with extensions
        let mint_size = if actual_extensions.is_empty() {
            spl_token_2022::state::Mint::get_packed_len()
        } else {
            ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(
                &actual_extensions,
            )
            .expect("Failed to calculate mint size with extensions")
        };

        eprintln!("Mint size with extensions: {}", mint_size);

        // Get rent-exempt balance
        let r = Rent::default();
        let lamports = r.minimum_balance(mint_size);

        // Get program ID
        let program_id_str = token_2022_program_id().to_string();
        let program_id = Pubkey::from_str(&program_id_str).unwrap();

        // Create account
        let mut account = AccountSharedData::new(lamports, mint_size, &program_id);

        // Initialize data buffer
        let mut data = vec![0u8; mint_size];

        if !actual_extensions.is_empty() {
            // Use proper extension initialization approach
            use spl_token_2022::extension::{BaseStateWithExtensionsMut, StateWithExtensionsMut};

            let mut state =
                StateWithExtensionsMut::<spl_token_2022::state::Mint>::unpack_uninitialized(
                    &mut data,
                )
                .expect("Failed to unpack mint state");

            // Initialize account type
            state.init_account_type().unwrap();

            // Initialize each extension based on the provided parameters
            for extension_type in &actual_extensions {
                match extension_type {
                    ExtensionType::MetadataPointer => {
                        use spl_token_2022::extension::metadata_pointer::MetadataPointer;

                        let extension = state
                            .init_extension::<MetadataPointer>(true)
                            .expect("Failed to init metadata pointer extension");

                        // Get the struct directly from ParamValue
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::MetadataPointer(_)))
                        }) {
                            if let ParamValue::MetadataPointer(metadata_config) = param {
                                // Set authority
                                extension.authority =
                                    if let Some(authority) = metadata_config.authority {
                                        OptionalNonZeroPubkey::try_from(Some(
                                            Token2022Pubkey::new_from_array(authority.to_bytes()),
                                        ))
                                        .expect("Invalid metadata pointer authority")
                                    } else {
                                        OptionalNonZeroPubkey::default()
                                    };
                                eprintln!(
                                    "Setting MetadataPointer authority: {:?}",
                                    metadata_config.authority
                                );

                                // Set metadata_address
                                extension.metadata_address = if let Some(metadata_address) =
                                    metadata_config.metadata_address
                                {
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(
                                            metadata_address.to_bytes(),
                                        ),
                                    ))
                                    .expect("Invalid metadata address")
                                } else {
                                    OptionalNonZeroPubkey::default()
                                };
                                eprintln!(
                                    "Setting MetadataPointer metadata_address: {:?}",
                                    metadata_config.metadata_address
                                );
                            }
                        } else {
                            // Default to empty values if no params provided
                            extension.authority = OptionalNonZeroPubkey::default();
                            extension.metadata_address = OptionalNonZeroPubkey::default();
                            eprintln!("Setting MetadataPointer to default values");
                        }
                    }
                    ExtensionType::GroupMemberPointer => {
                        use spl_token_2022::extension::group_member_pointer::GroupMemberPointer;

                        let extension = state
                            .init_extension::<GroupMemberPointer>(true)
                            .expect("Failed to init group member pointer extension");

                        // Get the struct directly from ParamValue
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::GroupMemberPointer(_)))
                        }) {
                            if let ParamValue::GroupMemberPointer(group_config) = param {
                                // Set authority
                                extension.authority =
                                    if let Some(authority) = group_config.authority {
                                        eprintln!(
                                            "Setting GroupMemberPointer authority: {}",
                                            authority
                                        );
                                        OptionalNonZeroPubkey::try_from(Some(
                                            Token2022Pubkey::new_from_array(authority.to_bytes()),
                                        ))
                                        .expect("Invalid group member pointer authority")
                                    } else {
                                        eprintln!(
                                        "Setting GroupMemberPointer authority to default (None)"
                                    );
                                        OptionalNonZeroPubkey::default()
                                    };

                                // Set member_address
                                extension.member_address = if let Some(member_address) =
                                    group_config.member_address
                                {
                                    eprintln!(
                                        "Setting GroupMemberPointer member_address: {}",
                                        member_address
                                    );
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(member_address.to_bytes()),
                                    ))
                                    .expect("Invalid member address")
                                } else {
                                    eprintln!("Setting GroupMemberPointer member_address to default (None)");
                                    OptionalNonZeroPubkey::default()
                                };
                            }
                        } else {
                            // Default to empty values if no params provided
                            extension.authority = OptionalNonZeroPubkey::default();
                            extension.member_address = OptionalNonZeroPubkey::default();
                            eprintln!("Setting GroupMemberPointer to default values");
                        }
                    }

                    ExtensionType::TransferHook => {
                        use spl_token_2022::extension::transfer_hook::TransferHook;

                        let extension = state
                            .init_extension::<TransferHook>(true)
                            .expect("Failed to init transfer hook extension");

                        // Get the struct directly from ParamValue
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::TransferHook(_)))
                        }) {
                            if let ParamValue::TransferHook(hook_config) = param {
                                // Set authority
                                extension.authority = if let Some(authority) = hook_config.authority
                                {
                                    eprintln!("Setting TransferHook authority: {}", authority);
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(authority.to_bytes()),
                                    ))
                                    .expect("Invalid transfer hook authority")
                                } else {
                                    eprintln!("Setting TransferHook authority to default (None)");
                                    OptionalNonZeroPubkey::default()
                                };

                                // Set program_id
                                extension.program_id = if let Some(program_id) =
                                    hook_config.program_id
                                {
                                    eprintln!("Setting TransferHook program_id: {}", program_id);
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(program_id.to_bytes()),
                                    ))
                                    .expect("Invalid transfer hook program_id")
                                } else {
                                    eprintln!("Setting TransferHook program_id to default (None)");
                                    OptionalNonZeroPubkey::default()
                                };
                            }
                        } else {
                            // Default to empty values if no params provided
                            extension.authority = OptionalNonZeroPubkey::default();
                            extension.program_id = OptionalNonZeroPubkey::default();
                            eprintln!("Setting TransferHook to default values");
                        }
                    }

                    ExtensionType::MintCloseAuthority => {
                        use spl_token_2022::extension::mint_close_authority::MintCloseAuthority;

                        let extension = state
                            .init_extension::<MintCloseAuthority>(true)
                            .expect("Failed to init mint close authority extension");

                        // Get the struct directly from ParamValue
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::MintCloseAuthority(_)))
                        }) {
                            if let ParamValue::MintCloseAuthority(close_config) = param {
                                // Set close_authority
                                extension.close_authority = if let Some(close_authority) =
                                    close_config.close_authority
                                {
                                    eprintln!(
                                        "Setting MintCloseAuthority close_authority: {}",
                                        close_authority
                                    );
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(close_authority.to_bytes()),
                                    ))
                                    .expect("Invalid mint close authority")
                                } else {
                                    eprintln!("Setting MintCloseAuthority close_authority to default (None)");
                                    OptionalNonZeroPubkey::default()
                                };
                            }
                        } else {
                            // Default to empty values if no params provided
                            extension.close_authority = OptionalNonZeroPubkey::default();
                            eprintln!("Setting MintCloseAuthority to default values");
                        }
                    }

                    ExtensionType::PermanentDelegate => {
                        use spl_token_2022::extension::permanent_delegate::PermanentDelegate;

                        let extension = state
                            .init_extension::<PermanentDelegate>(true)
                            .expect("Failed to init permanent delegate extension");

                        // Get the struct directly from ParamValue
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::PermanentDelegate(_)))
                        }) {
                            if let ParamValue::PermanentDelegate(delegate_config) = param {
                                // Set delegate
                                extension.delegate = if let Some(delegate) =
                                    delegate_config.delegate
                                {
                                    eprintln!("Setting PermanentDelegate delegate: {}", delegate);
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(delegate.to_bytes()),
                                    ))
                                    .expect("Invalid permanent delegate")
                                } else {
                                    eprintln!(
                                        "Setting PermanentDelegate delegate to default (None)"
                                    );
                                    OptionalNonZeroPubkey::default()
                                };
                            }
                        } else {
                            // Default to empty values if no params provided
                            extension.delegate = OptionalNonZeroPubkey::default();
                            eprintln!("Setting PermanentDelegate to default values");
                        }
                    }

                    ExtensionType::GroupPointer => {
                        use spl_token_2022::extension::group_pointer::GroupPointer;

                        let extension = state
                            .init_extension::<GroupPointer>(true)
                            .expect("Failed to init group pointer extension");

                        // Get the struct directly from ParamValue
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::GroupPointer(_)))
                        }) {
                            if let ParamValue::GroupPointer(group_config) = param {
                                // Set authority
                                extension.authority = if let Some(authority) =
                                    group_config.authority
                                {
                                    eprintln!("Setting GroupPointer authority: {}", authority);
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(authority.to_bytes()),
                                    ))
                                    .expect("Invalid group pointer authority")
                                } else {
                                    eprintln!("Setting GroupPointer authority to default (None)");
                                    OptionalNonZeroPubkey::default()
                                };

                                // Set group_address
                                extension.group_address = if let Some(group_address) =
                                    group_config.group_address
                                {
                                    eprintln!(
                                        "Setting GroupPointer group_address: {}",
                                        group_address
                                    );
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(group_address.to_bytes()),
                                    ))
                                    .expect("Invalid group address")
                                } else {
                                    eprintln!(
                                        "Setting GroupPointer group_address to default (None)"
                                    );
                                    OptionalNonZeroPubkey::default()
                                };
                            }

                            // Validate that at least one of authority or group_address is provided
                            if Option::<Pubkey>::from(extension.authority).is_none()
                                && Option::<Pubkey>::from(extension.group_address).is_none()
                            {
                                panic!("The group pointer extension requires at least an authority or an address for initialization");
                            }
                        } else {
                            // Default to empty values if no params provided
                            extension.authority = OptionalNonZeroPubkey::default();
                            extension.group_address = OptionalNonZeroPubkey::default();
                            eprintln!("Setting GroupPointer to default values");
                        }
                    }

                    ExtensionType::DefaultAccountState => {
                        use spl_token_2022::extension::default_account_state::DefaultAccountState;

                        let extension = state
                            .init_extension::<DefaultAccountState>(true)
                            .expect("Failed to init default account state extension");

                        // Get the struct directly from ParamValue
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::DefaultAccountState(_)))
                        }) {
                            if let ParamValue::DefaultAccountState(state_config) = param {
                                // Validate that the state is not Uninitialized (0)
                                if state_config.state == 0 {
                                    panic!("Default account state cannot be Uninitialized");
                                }
                                eprintln!(
                                    "Setting DefaultAccountState state: {}",
                                    state_config.state
                                );
                                extension.state = state_config.state;
                            }
                        } else {
                            // Default to Initialized (1) if no params provided
                            extension.state = 1; // AccountState::Initialized
                            eprintln!("Setting DefaultAccountState to default Initialized state");
                        }
                    }

                    ExtensionType::InterestBearingConfig => {
                        use spl_token_2022::extension::interest_bearing_mint::InterestBearingConfig;

                        let extension = state
                            .init_extension::<InterestBearingConfig>(true)
                            .expect("Failed to init interest bearing config extension");

                        // Get the struct directly from ParamValue
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::InterestBearingConfig(_)))
                        }) {
                            if let ParamValue::InterestBearingConfig(interest_config) = param {
                                // Set rate_authority
                                extension.rate_authority = if let Some(rate_authority) =
                                    interest_config.rate_authority
                                {
                                    eprintln!(
                                        "Setting InterestBearingConfig rate_authority: {}",
                                        rate_authority
                                    );
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(rate_authority.to_bytes()),
                                    ))
                                    .expect("Invalid interest bearing rate authority")
                                } else {
                                    eprintln!("Setting InterestBearingConfig rate_authority to default (None)");
                                    OptionalNonZeroPubkey::default()
                                };

                                // Set current_rate
                                extension.current_rate = interest_config.current_rate.into();
                                eprintln!(
                                    "Setting InterestBearingConfig current_rate: {} basis points",
                                    interest_config.current_rate
                                );

                                // Set pre_update_average_rate
                                extension.pre_update_average_rate =
                                    interest_config.pre_update_average_rate.into();
                                eprintln!("Setting InterestBearingConfig pre_update_average_rate: {} basis points", interest_config.pre_update_average_rate);

                                // Set initialization_timestamp
                                extension.initialization_timestamp =
                                    interest_config.initialization_timestamp.into();
                                eprintln!(
                                    "Setting InterestBearingConfig initialization_timestamp: {}",
                                    interest_config.initialization_timestamp
                                );

                                // Set last_update_timestamp
                                extension.last_update_timestamp =
                                    interest_config.last_update_timestamp.into();
                                eprintln!(
                                    "Setting InterestBearingConfig last_update_timestamp: {}",
                                    interest_config.last_update_timestamp
                                );
                            }
                        } else {
                            // Set defaults if no params provided
                            extension.current_rate = 0.into();
                            extension.pre_update_average_rate = 0.into();
                            extension.initialization_timestamp = 0.into();
                            extension.last_update_timestamp = 0.into();
                            eprintln!("Setting InterestBearingConfig to all defaults");
                        }
                    }

                    ExtensionType::TransferFeeConfig => {
                        use spl_token_2022::extension::transfer_fee::{
                            TransferFee, TransferFeeConfig,
                        };

                        let extension = state
                            .init_extension::<TransferFeeConfig>(true)
                            .expect("Failed to init transfer fee config extension");

                        // Set parameters if provided
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::TransferFeeConfig(_)))
                        }) {
                            if let ParamValue::TransferFeeConfig(fee_config) = param {
                                // Set transfer_fee_config_authority
                                extension.transfer_fee_config_authority = if let Some(
                                    transfer_fee_config_authority,
                                ) =
                                    fee_config.transfer_fee_config_authority
                                {
                                    eprintln!("Setting TransferFeeConfig transfer_fee_config_authority: {}", transfer_fee_config_authority);
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(
                                            transfer_fee_config_authority.to_bytes(),
                                        ),
                                    ))
                                    .expect("Invalid transfer fee config authority")
                                } else {
                                    eprintln!("Setting TransferFeeConfig transfer_fee_config_authority to default None = {:?}", OptionalNonZeroPubkey::default());
                                    OptionalNonZeroPubkey::default()
                                };

                                // Set withdraw_withheld_authority
                                extension.withdraw_withheld_authority = if let Some(
                                    withdraw_withheld_authority,
                                ) =
                                    fee_config.withdraw_withheld_authority
                                {
                                    eprintln!(
                                        "Setting TransferFeeConfig withdraw_withheld_authority: {}",
                                        withdraw_withheld_authority
                                    );
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(
                                            withdraw_withheld_authority.to_bytes(),
                                        ),
                                    ))
                                    .expect("Invalid withdraw withheld authority")
                                } else {
                                    eprintln!("Setting TransferFeeConfig withdraw_withheld_authority to default None = {:?}", OptionalNonZeroPubkey::default());
                                    OptionalNonZeroPubkey::default()
                                };

                                // Set withheld_amount
                                extension.withheld_amount = fee_config.withheld_amount.into();

                                // Validate transfer_fee_basis_points in older_transfer_fee
                                if fee_config.older_transfer_fee.transfer_fee_basis_points > 10000 {
                                    panic!("Transfer fee basis points cannot exceed 10,000 (100%)");
                                }

                                // Set older_transfer_fee
                                extension.older_transfer_fee = TransferFee {
                                    epoch: fee_config.older_transfer_fee.epoch.into(),
                                    maximum_fee: fee_config.older_transfer_fee.maximum_fee.into(),
                                    transfer_fee_basis_points: fee_config
                                        .older_transfer_fee
                                        .transfer_fee_basis_points
                                        .into(),
                                };

                                // Validate transfer_fee_basis_points in newer_transfer_fee
                                if fee_config.newer_transfer_fee.transfer_fee_basis_points > 10000 {
                                    panic!("Transfer fee basis points cannot exceed 10,000 (100%)");
                                }

                                // Set newer_transfer_fee
                                extension.newer_transfer_fee = TransferFee {
                                    epoch: fee_config.newer_transfer_fee.epoch.into(),
                                    maximum_fee: fee_config.newer_transfer_fee.maximum_fee.into(),
                                    transfer_fee_basis_points: fee_config
                                        .newer_transfer_fee
                                        .transfer_fee_basis_points
                                        .into(),
                                };

                                eprintln!("Setting TransferFeeConfig: withheld_amount={}, older_fee=(epoch={}, basis_points={}, max_fee={}), newer_fee=(epoch={}, basis_points={}, max_fee={})", 
                                    fee_config.withheld_amount,
                                    fee_config.older_transfer_fee.epoch,
                                    fee_config.older_transfer_fee.transfer_fee_basis_points,
                                    fee_config.older_transfer_fee.maximum_fee,
                                    fee_config.newer_transfer_fee.epoch,
                                    fee_config.newer_transfer_fee.transfer_fee_basis_points,
                                    fee_config.newer_transfer_fee.maximum_fee);
                            }
                        } else {
                            // Set defaults if no params provided
                            extension.withheld_amount = 0.into();

                            let default_transfer_fee = TransferFee {
                                epoch: 0.into(),
                                maximum_fee: 0.into(),
                                transfer_fee_basis_points: 0.into(),
                            };

                            extension.older_transfer_fee = default_transfer_fee;
                            extension.newer_transfer_fee = default_transfer_fee;
                            eprintln!("Setting TransferFeeConfig to all defaults");
                        }
                    }

                    ExtensionType::NonTransferable => {
                        use spl_token_2022::extension::non_transferable::NonTransferable;

                        let _extension = state
                            .init_extension::<NonTransferable>(true)
                            .expect("Failed to init non transferable extension");

                        // NonTransferable is a zero-sized extension (marker only)
                        // No parameters to set - it's just a flag indicating tokens can't be transferred
                        eprintln!("Setting NonTransferable extension (no parameters needed)");
                    }

                    ExtensionType::ConfidentialTransferMint => {
                        use spl_pod::optional_keys::OptionalNonZeroElGamalPubkey;
                        use spl_token_2022::extension::confidential_transfer::ConfidentialTransferMint;

                        let extension = state
                            .init_extension::<ConfidentialTransferMint>(true)
                            .expect("Failed to init confidential transfer mint extension");

                        // Set parameters if provided
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::ConfidentialTransferMint(_)))
                        }) {
                            if let ParamValue::ConfidentialTransferMint(ct_config) = param {
                                // Set authority
                                extension.authority = if let Some(authority) = ct_config.authority {
                                    eprintln!(
                                        "Setting ConfidentialTransferMint authority: {}",
                                        authority
                                    );
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(authority.to_bytes()),
                                    ))
                                    .expect("Invalid confidential transfer mint authority")
                                } else {
                                    eprintln!("Setting ConfidentialTransferMint authority to default (None)");
                                    OptionalNonZeroPubkey::default()
                                };

                                // Set auto_approve_new_accounts (now required, not Option)
                                extension.auto_approve_new_accounts =
                                    ct_config.auto_approve_new_accounts.into();
                                eprintln!("Setting ConfidentialTransferMint auto_approve_new_accounts: {}", ct_config.auto_approve_new_accounts);

                                // Set auditor_elgamal_pubkey
                                extension.auditor_elgamal_pubkey = if let Some(auditor_pubkey) =
                                    ct_config.auditor_elgamal_pubkey
                                {
                                    use spl_pod::optional_keys::OptionalNonZeroElGamalPubkey;
                                    eprintln!(
                                        "Setting ConfidentialTransferMint auditor_elgamal_pubkey"
                                    );
                                    OptionalNonZeroElGamalPubkey::try_from(Some(auditor_pubkey))
                                        .expect("Invalid auditor ElGamal pubkey")
                                } else {
                                    eprintln!("Setting ConfidentialTransferMint auditor_elgamal_pubkey to default (None)");
                                    OptionalNonZeroElGamalPubkey::default()
                                };
                            }
                        } else {
                            // Set defaults if no params provided
                            extension.auto_approve_new_accounts = false.into();
                            eprintln!("Setting ConfidentialTransferMint to defaults");
                        }
                    }

                    ExtensionType::ConfidentialTransferFeeConfig => {
                        use spl_token_2022::extension::confidential_transfer_fee::{
                            ConfidentialTransferFeeConfig, EncryptedWithheldAmount,
                        };

                        let extension = state
                            .init_extension::<ConfidentialTransferFeeConfig>(true)
                            .expect("Failed to init confidential transfer fee config extension");

                        // Set parameters if provided
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::ConfidentialTransferFeeConfig(_)))
                        }) {
                            if let ParamValue::ConfidentialTransferFeeConfig(ctf_config) = param {
                                // Set authority
                                extension.authority = if let Some(authority) = ctf_config.authority
                                {
                                    eprintln!(
                                        "Setting ConfidentialTransferFeeConfig authority: {}",
                                        authority
                                    );
                                    OptionalNonZeroPubkey::try_from(Some(
                                        Token2022Pubkey::new_from_array(authority.to_bytes()),
                                    ))
                                    .expect("Invalid confidential transfer fee config authority")
                                } else {
                                    eprintln!("Setting ConfidentialTransferFeeConfig authority to default (None)");
                                    OptionalNonZeroPubkey::default()
                                };

                                // Set withdraw_withheld_authority_elgamal_pubkey (required field)
                                extension.withdraw_withheld_authority_elgamal_pubkey =
                                    ctf_config.withdraw_withheld_authority_elgamal_pubkey;
                                eprintln!("Setting ConfidentialTransferFeeConfig withdraw_withheld_authority_elgamal_pubkey");

                                // Set harvest_to_mint_enabled (required field)
                                extension.harvest_to_mint_enabled =
                                    ctf_config.harvest_to_mint_enabled.into();
                                eprintln!("Setting ConfidentialTransferFeeConfig harvest_to_mint_enabled: {}", ctf_config.harvest_to_mint_enabled);

                                // Set withheld_amount (required field)
                                extension.withheld_amount = ctf_config.withheld_amount;
                                eprintln!("Setting ConfidentialTransferFeeConfig withheld_amount");
                            }
                        } else {
                            // Set defaults if no params provided
                            extension.harvest_to_mint_enabled = true.into();
                            extension.withheld_amount = EncryptedWithheldAmount::default();
                            eprintln!("Setting ConfidentialTransferFeeConfig to defaults");
                        }
                    }

                    ExtensionType::ConfidentialMintBurn => {
                        use solana_zk_sdk::encryption::pod::auth_encryption::PodAeCiphertext;
                        use solana_zk_sdk::encryption::pod::elgamal::PodElGamalCiphertext;
                        use spl_token_2022::extension::confidential_mint_burn::ConfidentialMintBurn;
                        let extension = state
                            .init_extension::<ConfidentialMintBurn>(true)
                            .expect("Failed to init confidential mint burn extension");

                        // Set parameters if provided
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::ConfidentialMintBurn(_)))
                        }) {
                            if let ParamValue::ConfidentialMintBurn(cmb_config) = param {
                                // Set all required fields
                                extension.confidential_supply = cmb_config.confidential_supply;
                                eprintln!("Setting ConfidentialMintBurn confidential_supply");

                                extension.decryptable_supply = cmb_config.decryptable_supply;
                                eprintln!("Setting ConfidentialMintBurn decryptable_supply");

                                extension.supply_elgamal_pubkey = cmb_config.supply_elgamal_pubkey;
                                eprintln!("Setting ConfidentialMintBurn supply_elgamal_pubkey");
                            }
                        } else {
                            // Set defaults if no params provided
                            extension.confidential_supply = PodElGamalCiphertext::default();
                            extension.decryptable_supply = PodAeCiphertext::default();
                            // Note: supply_elgamal_pubkey would need a valid default or this branch should panic
                            eprintln!("Setting ConfidentialMintBurn to defaults");
                        }
                    }
                    _ => {
                        eprintln!(
                            "Extension type {:?} has no custom initialization logic",
                            extension_type
                        );
                    }
                }
            }

            // Initialize the base mint data
            let base = &mut state.base;
            base.mint_authority = if mint_authority != &Pubkey::default() {
                COption::Some(Token2022Pubkey::new_from_array(mint_authority.to_bytes()))
            } else {
                COption::None
            };

            base.freeze_authority = freeze_authority
                .map(|fa| Token2022Pubkey::new_from_array(fa.to_bytes()))
                .map(COption::Some)
                .unwrap_or(COption::None);

            base.decimals = decimals;
            base.is_initialized = true;
            base.supply = 0;

            // Pack all the data back
            state.pack_base();
        } else {
            // Create a basic mint without extensions
            let mint = spl_token_2022::state::Mint {
                is_initialized: true,
                mint_authority: COption::Some(Token2022Pubkey::new_from_array(
                    mint_authority.to_bytes(),
                )),
                freeze_authority: freeze_authority
                    .map(|fa| Token2022Pubkey::new_from_array(fa.to_bytes()))
                    .map(COption::Some)
                    .unwrap_or(COption::None),
                decimals,
                supply: 0,
            };

            // Pack the mint directly
            spl_token_2022::state::Mint::pack(mint, &mut data).unwrap();
        }

        // Set account data
        account.set_data_from_slice(&data);

        // Create the account
        client.set_account_custom(&address, &account);
        eprintln!("Token 2022 mint account created successfully: {}", address);
    }
}
