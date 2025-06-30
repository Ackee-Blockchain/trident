// token_2022_account.rs
use crate::traits::FuzzClient;

use solana_sdk::{
    account::AccountSharedData, account::ReadableAccount, program_option::COption,
    program_pack::Pack, pubkey::Pubkey, rent::Rent,
};

use std::str::FromStr;

use spl_token_2022::{
    extension::ExtensionType, id as token_2022_program_id,
    solana_program::pubkey::Pubkey as Token2022Pubkey,
};

use crate::accounts_storage::{account_storage::AccountsStorage, ParamValue};

impl AccountsStorage {
    // Implementation for Token-2022
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn create_token_2022_account(
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

        extension_params: Option<Vec<ParamValue>>,
    ) {
        // Determine which extensions to use
        let actual_extensions: Vec<ExtensionType> = extension_params
            .as_ref()
            .map(|params| {
                params
                    .iter()
                    .map(|param| match param {
                        ParamValue::ConfidentialTransferAccount(_) => {
                            ExtensionType::ConfidentialTransferAccount
                        }
                        ParamValue::TransferHookAccount(_) => ExtensionType::TransferHookAccount,
                        ParamValue::CpiGuard(_) => ExtensionType::CpiGuard,
                        ParamValue::MemoTransfer(_) => ExtensionType::MemoTransfer,
                        ParamValue::TransferFeeAmount(_) => ExtensionType::TransferFeeAmount,
                        ParamValue::ImmutableOwner(_) => ExtensionType::ImmutableOwner,
                        ParamValue::NonTransferableAccount(_) => {
                            ExtensionType::NonTransferableAccount
                        }
                        ParamValue::ConfidentialTransferFeeAmount(_) => {
                            ExtensionType::ConfidentialTransferFeeAmount
                        }
                        _ => panic!("Unsupported account ParamValue variant"),
                    })
                    .collect()
            })
            .unwrap_or_default();

        //let filtered_extensions = filter_account_extensions(&actual_extensions);

        // Calculate account size with extensions
        let account_size = if actual_extensions.is_empty() {
            spl_token_2022::state::Account::get_packed_len()
        } else {
            ExtensionType::try_calculate_account_len::<spl_token_2022::state::Account>(
                &actual_extensions,
            )
            .expect("Failed to calculate account size with extensions")
        };

        eprintln!("Account size with extensions: {}", account_size);

        // Get rent-exempt balance
        let r = Rent::default();
        let rent_exempt_lamports = r.minimum_balance(account_size);

        // Calculate total lamports
        let total_lamports = if is_native {
            rent_exempt_lamports.saturating_add(amount)
        } else {
            rent_exempt_lamports
        };

        // Get program ID
        let program_id_str = token_2022_program_id().to_string();
        let program_id = Pubkey::from_str(&program_id_str).unwrap();

        // Create account
        let mut account = AccountSharedData::new(total_lamports, account_size, &program_id);

        // Initialize data buffer
        let mut data = vec![0u8; account_size];

        if !actual_extensions.is_empty() {
            // Use proper extension initialization approach
            use spl_token_2022::extension::{BaseStateWithExtensionsMut, StateWithExtensionsMut};

            let mut state =
                StateWithExtensionsMut::<spl_token_2022::state::Account>::unpack_uninitialized(
                    &mut data,
                )
                .expect("Failed to unpack account state");

            // Initialize account type
            state.init_account_type().unwrap();

            // Initialize each extension based on the provided parameters
            for extension_type in &actual_extensions {
                match extension_type {
                    ExtensionType::ConfidentialTransferAccount => {
                        use solana_zk_sdk::encryption::pod::auth_encryption::PodAeCiphertext;
                        use spl_token_2022::extension::confidential_transfer::{
                            ConfidentialTransferAccount, EncryptedBalance,
                        };
                        let extension = state
                            .init_extension::<ConfidentialTransferAccount>(true)
                            .expect("Failed to init confidential transfer account extension");

                        // Set parameters if provided
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::ConfidentialTransferAccount(_)))
                        }) {
                            if let ParamValue::ConfidentialTransferAccount(ct_config) = param {
                                // Set all required fields
                                extension.approved = ct_config.approved.into();
                                eprintln!(
                                    "Setting ConfidentialTransferAccount approved: {}",
                                    ct_config.approved
                                );

                                extension.elgamal_pubkey = ct_config.elgamal_pubkey;
                                eprintln!("Setting ConfidentialTransferAccount elgamal_pubkey");

                                extension.pending_balance_lo = ct_config.pending_balance_lo;
                                eprintln!("Setting ConfidentialTransferAccount pending_balance_lo");

                                extension.pending_balance_hi = ct_config.pending_balance_hi;
                                eprintln!("Setting ConfidentialTransferAccount pending_balance_hi");

                                extension.available_balance = ct_config.available_balance;
                                eprintln!("Setting ConfidentialTransferAccount available_balance");

                                extension.decryptable_available_balance =
                                    ct_config.decryptable_available_balance;
                                eprintln!("Setting ConfidentialTransferAccount decryptable_available_balance");

                                extension.allow_confidential_credits =
                                    ct_config.allow_confidential_credits.into();
                                eprintln!("Setting ConfidentialTransferAccount allow_confidential_credits: {}", ct_config.allow_confidential_credits);

                                extension.allow_non_confidential_credits =
                                    ct_config.allow_non_confidential_credits.into();
                                eprintln!("Setting ConfidentialTransferAccount allow_non_confidential_credits: {}", ct_config.allow_non_confidential_credits);

                                extension.pending_balance_credit_counter =
                                    ct_config.pending_balance_credit_counter.into();
                                eprintln!("Setting ConfidentialTransferAccount pending_balance_credit_counter: {}", ct_config.pending_balance_credit_counter);

                                extension.maximum_pending_balance_credit_counter =
                                    ct_config.maximum_pending_balance_credit_counter.into();
                                eprintln!("Setting ConfidentialTransferAccount maximum_pending_balance_credit_counter: {}", ct_config.maximum_pending_balance_credit_counter);

                                extension.expected_pending_balance_credit_counter =
                                    ct_config.expected_pending_balance_credit_counter.into();
                                eprintln!("Setting ConfidentialTransferAccount expected_pending_balance_credit_counter: {}", ct_config.expected_pending_balance_credit_counter);

                                extension.actual_pending_balance_credit_counter =
                                    ct_config.actual_pending_balance_credit_counter.into();
                                eprintln!("Setting ConfidentialTransferAccount actual_pending_balance_credit_counter: {}", ct_config.actual_pending_balance_credit_counter);
                            }
                        } else {
                            // Set defaults if no params provided
                            extension.approved = false.into();
                            extension.allow_confidential_credits = true.into();
                            extension.allow_non_confidential_credits = true.into();
                            extension.pending_balance_lo = EncryptedBalance::default();
                            extension.pending_balance_hi = EncryptedBalance::default();
                            extension.available_balance = EncryptedBalance::default();
                            extension.decryptable_available_balance = PodAeCiphertext::default();
                            extension.pending_balance_credit_counter = 0u64.into();
                            extension.maximum_pending_balance_credit_counter = 65536u64.into();
                            extension.expected_pending_balance_credit_counter = 0u64.into();
                            extension.actual_pending_balance_credit_counter = 0u64.into();
                            // Note: elgamal_pubkey needs a valid key - this else branch might need to panic or require params
                            eprintln!("Setting ConfidentialTransferAccount to defaults");
                        }
                    }
                    ExtensionType::NonTransferableAccount => {
                        use spl_token_2022::extension::non_transferable::NonTransferableAccount;

                        let _extension = state
                            .init_extension::<NonTransferableAccount>(true)
                            .expect("Failed to init non transferable account extension");

                        // NonTransferableAccount is a zero-sized extension (marker only)
                        // No parameters to set - it's just a flag indicating this account can't transfer tokens
                        eprintln!(
                            "Setting NonTransferableAccount extension (no parameters needed)"
                        );
                    }
                    ExtensionType::TransferFeeAmount => {
                        use spl_token_2022::extension::transfer_fee::TransferFeeAmount;

                        let extension = state
                            .init_extension::<TransferFeeAmount>(true)
                            .expect("Failed to init transfer fee amount extension");

                        // Set parameters if provided
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::TransferFeeAmount(_)))
                        }) {
                            if let ParamValue::TransferFeeAmount(fee_amount_config) = param {
                                // Set withheld_amount directly (no longer Option)
                                extension.withheld_amount =
                                    fee_amount_config.withheld_amount.into();
                                eprintln!(
                                    "Setting TransferFeeAmount withheld_amount: {}",
                                    fee_amount_config.withheld_amount
                                );
                            }
                        } else {
                            // Default to 0 if no params provided
                            extension.withheld_amount = 0.into();
                            eprintln!("Setting TransferFeeAmount withheld_amount to default 0");
                        }
                    }

                    ExtensionType::MemoTransfer => {
                        use spl_token_2022::extension::memo_transfer::MemoTransfer;

                        let extension = state
                            .init_extension::<MemoTransfer>(true)
                            .expect("Failed to init memo transfer extension");

                        // Set parameters if provided
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::MemoTransfer(_)))
                        }) {
                            if let ParamValue::MemoTransfer(memo_config) = param {
                                // Set require_incoming_transfer_memos directly (no longer Option)
                                extension.require_incoming_transfer_memos =
                                    memo_config.require_incoming_transfer_memos.into();
                                eprintln!(
                                    "Setting MemoTransfer require_incoming_transfer_memos: {}",
                                    memo_config.require_incoming_transfer_memos
                                );
                            }
                        } else {
                            // Default to false if no params provided
                            extension.require_incoming_transfer_memos = false.into();
                            eprintln!("Setting MemoTransfer require_incoming_transfer_memos to default false");
                        }
                    }

                    ExtensionType::CpiGuard => {
                        use spl_token_2022::extension::cpi_guard::CpiGuard;

                        let extension = state
                            .init_extension::<CpiGuard>(true)
                            .expect("Failed to init CPI guard extension");

                        // Set parameters if provided
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params.iter().find(|p| matches!(p, ParamValue::CpiGuard(_)))
                        }) {
                            if let ParamValue::CpiGuard(cpi_config) = param {
                                // Set lock_cpi directly (no longer Option)
                                extension.lock_cpi = cpi_config.lock_cpi.into();
                                eprintln!("Setting CpiGuard lock_cpi: {}", cpi_config.lock_cpi);
                            }
                        } else {
                            // Default to false if no params provided
                            extension.lock_cpi = false.into();
                            eprintln!("Setting CpiGuard lock_cpi to default false");
                        }
                    }
                    ExtensionType::ImmutableOwner => {
                        use spl_token_2022::extension::immutable_owner::ImmutableOwner;

                        let _extension = state
                            .init_extension::<ImmutableOwner>(true)
                            .expect("Failed to init immutable owner extension");

                        // ImmutableOwner is a zero-sized extension (marker only)
                        // No parameters to set - it's just a flag indicating the owner cannot be changed
                        eprintln!("Setting ImmutableOwner extension (no parameters needed)");
                    }

                    ExtensionType::ConfidentialTransferFeeAmount => {
                        use spl_token_2022::extension::confidential_transfer_fee::{
                            ConfidentialTransferFeeAmount, EncryptedWithheldAmount,
                        };

                        let extension = state
                            .init_extension::<ConfidentialTransferFeeAmount>(true)
                            .expect("Failed to init confidential transfer fee amount extension");

                        // Set parameters if provided
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::ConfidentialTransferFeeAmount(_)))
                        }) {
                            if let ParamValue::ConfidentialTransferFeeAmount(fee_amount_config) =
                                param
                            {
                                // Set withheld_amount
                                extension.withheld_amount = fee_amount_config.withheld_amount;
                                eprintln!("Setting ConfidentialTransferFeeAmount withheld_amount");
                            }
                        } else {
                            // Default to zero if no params provided
                            extension.withheld_amount = EncryptedWithheldAmount::default();
                            eprintln!("Setting ConfidentialTransferFeeAmount withheld_amount to default (zero)");
                        }
                    }

                    ExtensionType::TransferHookAccount => {
                        use spl_token_2022::extension::transfer_hook::TransferHookAccount;

                        let extension = state
                            .init_extension::<TransferHookAccount>(true)
                            .expect("Failed to init transfer hook account extension");

                        // Set parameters if provided
                        if let Some(param) = extension_params.as_ref().and_then(|params| {
                            params
                                .iter()
                                .find(|p| matches!(p, ParamValue::TransferHookAccount(_)))
                        }) {
                            if let ParamValue::TransferHookAccount(hook_config) = param {
                                // Set transferring flag if provided
                                if let Some(transferring) = hook_config.transferring {
                                    extension.transferring = transferring.into();
                                    eprintln!(
                                        "Setting TransferHookAccount transferring: {}",
                                        transferring
                                    );
                                } else {
                                    // Default to false (not transferring)
                                    extension.transferring = false.into();
                                    eprintln!(
                                        "Setting TransferHookAccount transferring to default false"
                                    );
                                }
                            }
                        } else {
                            // Default to false if no params provided
                            extension.transferring = false.into();
                            eprintln!("Setting TransferHookAccount transferring to default false");
                        }
                    }
                    _ => {
                        eprintln!(
                            "Extension type {:?} has no custom initialization logic yet",
                            extension_type
                        );
                    }
                }
            }

            // Initialize the base account data
            let base = &mut state.base;
            base.mint = Token2022Pubkey::new_from_array(mint.to_bytes());
            base.owner = Token2022Pubkey::new_from_array(owner.to_bytes());
            base.amount = if is_native { total_lamports } else { amount };
            base.delegate = delegate
                .map(|d| Token2022Pubkey::new_from_array(d.to_bytes()))
                .map(COption::Some)
                .unwrap_or(COption::None);
            base.state = spl_token_2022::state::AccountState::Initialized;
            base.is_native = if is_native {
                COption::Some(rent_exempt_lamports)
            } else {
                COption::None
            };
            base.delegated_amount = delegated_amount;
            base.close_authority = close_authority
                .map(|ca| Token2022Pubkey::new_from_array(ca.to_bytes()))
                .map(COption::Some)
                .unwrap_or(COption::None);

            // Pack all the data back
            state.pack_base();
        } else {
            // Create a basic account without extensions
            let token_account = spl_token_2022::state::Account {
                mint: Token2022Pubkey::new_from_array(mint.to_bytes()),
                owner: Token2022Pubkey::new_from_array(owner.to_bytes()),
                amount: if is_native { total_lamports } else { amount },
                delegate: delegate
                    .map(|d| Token2022Pubkey::new_from_array(d.to_bytes()))
                    .map(COption::Some)
                    .unwrap_or(COption::None),
                state: spl_token_2022::state::AccountState::Initialized,
                is_native: if is_native {
                    COption::Some(rent_exempt_lamports)
                } else {
                    COption::None
                },
                delegated_amount,
                close_authority: close_authority
                    .map(|ca| Token2022Pubkey::new_from_array(ca.to_bytes()))
                    .map(COption::Some)
                    .unwrap_or(COption::None),
            };

            // Pack the account directly
            spl_token_2022::state::Account::pack(token_account, &mut data).unwrap();
        }

        // Set account data
        account.set_data_from_slice(&data);

        // Create the account
        client.set_account_custom(&address, &account);
        eprintln!("Token 2022 account created successfully: {}", address);
    }
}
