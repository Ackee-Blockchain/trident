
#[cfg(feature = "token2022")]
use solana_zk_sdk::encryption::pod::{
    auth_encryption::PodAeCiphertext,
    elgamal::{PodElGamalCiphertext, PodElGamalPubkey},
    
};
#[cfg(feature = "token2022")]
use spl_token_2022::extension::confidential_transfer_fee::EncryptedWithheldAmount;
#[cfg(feature = "token2022")]
use trident_svm::processor::Pubkey;
#[cfg(feature = "token2022")]
pub struct MetadataPointer {
    pub authority: Option<Pubkey>,
    pub metadata_address: Option<Pubkey>,
}

#[cfg(feature = "token2022")]
pub struct GroupMemberPointer {
    pub authority: Option<Pubkey>,
    pub member_address: Option<Pubkey>,
}
#[cfg(feature = "token2022")]
pub struct GroupPointer {
    pub authority: Option<Pubkey>,
    pub group_address: Option<Pubkey>,
}
#[cfg(feature = "token2022")]
pub struct TransferHook {
    pub authority: Option<Pubkey>,
    pub program_id: Option<Pubkey>,
}
#[cfg(feature = "token2022")]
pub struct MintCloseAuthority {
    pub close_authority: Option<Pubkey>,
}

#[cfg(feature = "token2022")]
pub struct PermanentDelegate {
    pub delegate: Option<Pubkey>,
}
#[cfg(feature = "token2022")]
pub struct DefaultAccountState {
    pub state: u8,
}
#[cfg(feature = "token2022")]
pub struct InterestBearingConfig {
    pub rate_authority: Option<Pubkey>,
    pub initialization_timestamp: i64,
    pub pre_update_average_rate: i16,
    pub last_update_timestamp: i64,
    pub current_rate: i16,
}
#[cfg(feature = "token2022")]
pub struct TransferFee {
    pub epoch: u64,
    pub maximum_fee: u64,
    pub transfer_fee_basis_points: u16,
}
#[cfg(feature = "token2022")]
pub struct TransferFeeConfig {
    pub transfer_fee_config_authority: Option<Pubkey>,
    pub withdraw_withheld_authority: Option<Pubkey>,
    pub withheld_amount: u64,
    pub older_transfer_fee: TransferFee,
    pub newer_transfer_fee: TransferFee,
}

#[cfg(feature = "token2022")]
pub struct ConfidentialTransferMint {
    pub authority: Option<Pubkey>,
    pub auto_approve_new_accounts: bool,
    pub auditor_elgamal_pubkey: Option<PodElGamalPubkey>,
}

#[cfg(feature = "token2022")]
pub struct ConfidentialTransferFeeConfig {
    pub authority: Option<Pubkey>,
    pub withdraw_withheld_authority_elgamal_pubkey: PodElGamalPubkey,
    pub harvest_to_mint_enabled: bool,
    pub withheld_amount: EncryptedWithheldAmount,
}
#[cfg(feature = "token2022")]
pub struct ConfidentialMintBurn {
    pub confidential_supply: PodElGamalCiphertext,
    pub decryptable_supply: PodAeCiphertext,
    pub supply_elgamal_pubkey: PodElGamalPubkey,
}

#[cfg(feature = "token2022")]
pub struct NonTransferable {}


// Token 2022 Account Extensions
#[cfg(feature = "token2022")]
pub struct ImmutableOwner {}

#[cfg(feature = "token2022")]
pub struct ConfidentialTransferAccount {
    pub approved: bool,
    pub elgamal_pubkey: PodElGamalPubkey, 
    pub pending_balance_lo: PodElGamalCiphertext,
    pub pending_balance_hi: PodElGamalCiphertext,
    pub available_balance: PodElGamalCiphertext,
    pub decryptable_available_balance: PodAeCiphertext,
    pub allow_confidential_credits: bool,
    pub allow_non_confidential_credits: bool,
    pub pending_balance_credit_counter: u64,
    pub maximum_pending_balance_credit_counter: u64,
    pub expected_pending_balance_credit_counter: u64,
    pub actual_pending_balance_credit_counter: u64,
}
#[cfg(feature = "token2022")]
pub struct TransferHookAccount {
    pub transferring: Option<bool>,
}

#[cfg(feature = "token2022")]
pub struct CpiGuard {
    pub lock_cpi: bool, 
}

#[cfg(feature = "token2022")]
pub struct MemoTransfer {
    pub require_incoming_transfer_memos: bool, 
}
#[cfg(feature = "token2022")]
pub struct TransferFeeAmount {
    pub withheld_amount: u64, 
}
#[cfg(feature = "token2022")]
pub struct ConfidentialTransferFeeAmount {
    pub withheld_amount: EncryptedWithheldAmount,
}

#[cfg(feature = "token2022")]
pub struct NonTransferableAccount {}