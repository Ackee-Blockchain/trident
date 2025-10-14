//! Token 2022 integration for Trident fuzzing framework
//!
//! This module provides comprehensive support for SPL Token 2022 program,
//! including all extensions and proper initialization order handling.

use solana_sdk::pubkey::Pubkey;
use spl_token_2022_interface::extension::ExtensionType;
use spl_token_2022_interface::state::Account;
use spl_token_2022_interface::state::Mint;

mod methods;

/// A Token 2022 mint with its associated extensions
#[derive(Debug)]
pub struct MintWithExtensions {
    /// The base mint data
    pub mint: Mint,
    /// All extensions associated with this mint
    pub extensions: Vec<MintExtensionData>,
}

/// A Token 2022 account with its associated extensions
#[derive(Debug)]
pub struct TokenAccountWithExtensions {
    /// The base token account data
    pub account: Account,
    /// All extensions associated with this token account
    pub extensions: Vec<TokenAccountExtensionData>,
}

/// Extension data that can be stored on a Token 2022 mint
#[derive(Debug)]
pub enum MintExtensionData {
    TransferFeeConfig(spl_token_2022_interface::extension::transfer_fee::TransferFeeConfig),
    MintCloseAuthority(
        spl_token_2022_interface::extension::mint_close_authority::MintCloseAuthority,
    ),
    // ConfidentialTransferMint(spl_token_2022_interface::extension::confidential_transfer::ConfidentialTransferMint),
    DefaultAccountState(
        spl_token_2022_interface::extension::default_account_state::DefaultAccountState,
    ),
    NonTransferable(spl_token_2022_interface::extension::non_transferable::NonTransferable),
    InterestBearingConfig(
        spl_token_2022_interface::extension::interest_bearing_mint::InterestBearingConfig,
    ),
    PermanentDelegate(spl_token_2022_interface::extension::permanent_delegate::PermanentDelegate),
    TransferHook(spl_token_2022_interface::extension::transfer_hook::TransferHook),
    // ConfidentialTransferFeeConfig(spl_token_2022_interface::extension::confidential_transfer_fee::ConfidentialTransferFeeConfig),
    MetadataPointer(spl_token_2022_interface::extension::metadata_pointer::MetadataPointer),
    GroupPointer(spl_token_2022_interface::extension::group_pointer::GroupPointer),
    GroupMemberPointer(
        spl_token_2022_interface::extension::group_member_pointer::GroupMemberPointer,
    ),
    // ConfidentialMintBurn(spl_token_2022_interface::extension::confidential_mint_burn::ConfidentialMintBurn),
    ScaledUiAmount(spl_token_2022_interface::extension::scaled_ui_amount::ScaledUiAmountConfig),
    Pausable(spl_token_2022_interface::extension::pausable::PausableConfig),
    TokenMetadata(spl_token_metadata_interface::state::TokenMetadata),
    TokenGroup(spl_token_group_interface::state::TokenGroup),
    TokenGroupMember(spl_token_group_interface::state::TokenGroupMember),
    Unknown(ExtensionType),
}

/// Extension data that can be stored on a Token 2022 account
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum TokenAccountExtensionData {
    TransferFeeAmount(spl_token_2022_interface::extension::transfer_fee::TransferFeeAmount),
    ImmutableOwner(spl_token_2022_interface::extension::immutable_owner::ImmutableOwner),
    NonTransferableAccount(
        spl_token_2022_interface::extension::non_transferable::NonTransferableAccount,
    ),
    TransferHookAccount(spl_token_2022_interface::extension::transfer_hook::TransferHookAccount),
    // ConfidentialTransferAccount(spl_token_2022_interface::extension::confidential_transfer::ConfidentialTransferAccount),
    // ConfidentialTransferFeeAmount(spl_token_2022_interface::extension::confidential_transfer_fee::ConfidentialTransferFeeAmount),
    PausableAccount(spl_token_2022_interface::extension::pausable::PausableAccount),
    MemoTransfer(spl_token_2022_interface::extension::memo_transfer::MemoTransfer),
    CpiGuard(spl_token_2022_interface::extension::cpi_guard::CpiGuard),
    Unknown(ExtensionType),
}

/// Configuration for mint extensions during creation
#[derive(Debug, Clone)]
pub enum MintExtension {
    // ConfidentialTransferMint {
    //     authority: Option<Pubkey>,
    //     auto_approve_new_accounts: bool,
    //     auditor_elgamal_pubkey: Option<Pubkey>,
    // },
    // ConfidentialMintBurn,
    TransferFeeConfig {
        transfer_fee_config_authority: Option<Pubkey>,
        withdraw_withheld_authority: Option<Pubkey>,
        transfer_fee_basis_points: u16,
        maximum_fee: u64,
    },
    MintCloseAuthority {
        close_authority: Option<Pubkey>,
    },
    InterestBearingConfig {
        rate_authority: Option<Pubkey>,
        rate: i16,
    },
    NonTransferable,
    PermanentDelegate {
        delegate: Pubkey,
    },
    TransferHook {
        authority: Option<Pubkey>,
        program_id: Option<Pubkey>,
    },
    MetadataPointer {
        authority: Option<Pubkey>,
        metadata_address: Option<Pubkey>,
    },
    GroupPointer {
        authority: Option<Pubkey>,
        group_address: Option<Pubkey>,
    },
    GroupMemberPointer {
        authority: Option<Pubkey>,
        member_address: Option<Pubkey>,
    },
    ScaledUiAmount {
        authority: Option<Pubkey>,
        multiplier: f64,
    },
    Pausable {
        authority: Pubkey,
    },
    DefaultAccountState {
        state: u8,
    },
    TokenMetadata {
        mint: Pubkey,
        name: String,
        symbol: String,
        uri: String,
        additional_metadata: Vec<(String, String)>,
        update_authority: Option<Pubkey>,
        metadata: Pubkey,
    },
    TokenGroup {
        group: Pubkey,
        update_authority: Option<Pubkey>,
        max_size: u64,
    },
    TokenGroupMember {
        group: Pubkey,
        group_update_authority: Pubkey,
    },
}

/// Configuration for account extensions during creation
#[derive(Debug, Clone)]
pub enum AccountExtension {
    MemoTransfer {
        require_incoming_transfer_memos: bool,
    },
    ImmutableOwner,
    CpiGuard,
}
