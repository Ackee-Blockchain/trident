use std::str::FromStr;

use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::{solana_sdk::account, *};
use trident_fuzz::accounts_storage::{
    // Token 2022 Mint and Account extensions from extensions::*;
    extensions::*,

};
use trident_fuzz::accounts_storage::ParamValue;
use spl_token_2022::extension::confidential_transfer_fee::EncryptedWithheldAmount;
use solana_zk_sdk::encryption::{
    auth_encryption::AeKey,
    elgamal::ElGamalKeypair,
    pod::{
        auth_encryption::PodAeCiphertext,
        elgamal::{PodElGamalCiphertext, PodElGamalPubkey},
    },
};
#[derive(Arbitrary, TridentInstruction)]
#[program_id("7mic9LMCr7wpHeixUpEwQ9pVYa9HB2wQ5Jb47no9yXvx")]
# [discriminator ([76u8 , 184u8 , 50u8 , 62u8 , 162u8 , 141u8 , 47u8 , 103u8 ,])]
pub struct CreateMintAccountInstruction {
    pub accounts: CreateMintAccountInstructionAccounts,
    pub data: CreateMintAccountInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(CreateMintAccountInstructionData)]
#[storage(FuzzAccounts)]
pub struct CreateMintAccountInstructionAccounts {
    #[account(mut, signer)]
    payer: TridentAccount,
    #[account(mut, signer)]
    authority: TridentAccount,
    receiver: TridentAccount,
    mint: TridentAccount,
    mint_token_account: TridentAccount,
    extra_metas_account: TridentAccount,
    #[account(address = "11111111111111111111111111111111")]
    system_program: TridentAccount,
    #[account(address = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")]
    associated_token_program: TridentAccount,
    #[account(address = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb")]
    token_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct CreateMintAccountInstructionData {}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for CreateMintAccountInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        //todo!()
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
                let program_id = Pubkey::from_str("7mic9LMCr7wpHeixUpEwQ9pVYa9HB2wQ5Jb47no9yXvx");

        let payer = fuzz_accounts.payer.get_or_create(
            self.accounts.payer.account_id, 
            client, 
            None, 
            None
        );
        self.accounts.payer.set_address(payer);
  
        let authority = fuzz_accounts.authority.get_or_create(
            self.accounts.authority.account_id,
            client, 
            None, 
            None
        );
        self.accounts.authority.set_address(authority);


        let receiver = fuzz_accounts.receiver.get_or_create(
            self.accounts.receiver.account_id, 
            client, 
            None, 
            None
        );
        self.accounts.receiver.set_address(receiver);


        
        // Setting up Token 2022 MINT Extensions and Creatiung account

        let mut params = Vec::<ParamValue>::new();
        
        // For MetadataPointer extension
        let metadata_params = ParamValue::MetadataPointer(
            MetadataPointer {
                authority: Some(authority),
                metadata_address: Some(receiver),
            }
        );

        let group_member_params = ParamValue::GroupMemberPointer(
            GroupMemberPointer {
                authority: Some(authority),
                member_address: None,
            }
        );

        let transfer_hook_params = ParamValue::TransferHook(
            TransferHook {
                authority: Some(authority),
                program_id: None,
            }
        );

        let close_authority_params = ParamValue::MintCloseAuthority(
            MintCloseAuthority {
                close_authority: None,
            }
        );

        let permanent_delegate_params = ParamValue::PermanentDelegate(
            PermanentDelegate {
                delegate: None,
            }
        );

        let group_pointer_params = ParamValue::GroupPointer(
            GroupPointer {
                authority: Some(authority),
                group_address: None,
            }
        );

        let default_state_params = ParamValue::DefaultAccountState(
            DefaultAccountState {
                state: 2,// <- Frozen state
            }
        );

        let interest_bearing_params = ParamValue::InterestBearingConfig(
            InterestBearingConfig {
                rate_authority: Some(authority),
                current_rate: 500, // 5% APR
                initialization_timestamp: 1640995200, // Jan 1, 2022
                pre_update_average_rate: 500,
                last_update_timestamp: 1640995200, // Jan 1, 2022
            }
        );

        let transfer_fee_params = ParamValue::TransferFeeConfig(
            TransferFeeConfig {
                transfer_fee_config_authority: Some(authority),
                withdraw_withheld_authority: Some(authority),
                withheld_amount: 0,
                older_transfer_fee: TransferFee {
                    epoch: 0,
                    maximum_fee: 1000000,
                    transfer_fee_basis_points: 50, // 0.5%
                },
                newer_transfer_fee: TransferFee {
                    epoch: 1,
                    maximum_fee: 1000000,
                    transfer_fee_basis_points: 75, // 0.75%
                },
            }
        );

        let auditor_keypair = ElGamalKeypair::new_rand();
        let confidential_transfer_mint_params = ParamValue::ConfidentialTransferMint(
            ConfidentialTransferMint {
                authority: Some(authority),
                auto_approve_new_accounts: true, // Auto-approve new accounts
                auditor_elgamal_pubkey: Some(PodElGamalPubkey::from(*auditor_keypair.pubkey())),
            }
        );

        let confidential_transfer_fee_params = ParamValue::ConfidentialTransferFeeConfig(
            ConfidentialTransferFeeConfig {
                authority: Some(authority),
                withdraw_withheld_authority_elgamal_pubkey: PodElGamalPubkey::from(*ElGamalKeypair::new_rand().pubkey()),
                harvest_to_mint_enabled: true,
                withheld_amount: EncryptedWithheldAmount::default(),
            }
        );

        let confidential_mint_burn_params = ParamValue::ConfidentialMintBurn(
            ConfidentialMintBurn {
                supply_elgamal_pubkey: PodElGamalPubkey::from(*ElGamalKeypair::new_rand().pubkey()),
                decryptable_supply: PodAeCiphertext::from(AeKey::new_rand().encrypt(0u64)),
                confidential_supply: PodElGamalCiphertext::from(ElGamalKeypair::new_rand().pubkey().encrypt(0u64)),
            }
        );

        // Use ExtensionTypeWrapper instead of raw ExtensionType
        params.push(metadata_params);
        params.push(group_member_params);
        params.push(transfer_hook_params);
        params.push(close_authority_params);
        params.push(permanent_delegate_params);
        params.push(group_pointer_params);
        params.push(default_state_params);
        params.push(interest_bearing_params);
        params.push(transfer_fee_params);
        params.push(confidential_transfer_mint_params);
        params.push(confidential_transfer_fee_params);
        params.push(confidential_mint_burn_params);



        let mint = fuzz_accounts.mint.get_or_create_token_2022_mint(
                self.accounts.mint.account_id,
                client,
                None,
                9,
                &authority,
                Some(authority),
                Some(params)
        );

     
        self.accounts.mint.set_address(mint);

                eprintln!("\r\nMint: {:?}\r\n", mint);


        // Setting up Token 2022 Account and Account Extensions

        let mut params2 = Vec::<ParamValue>::new();

        // For TransferHook extension
        let transfer_hook_account_params = ParamValue::TransferHookAccount(
            TransferHookAccount {
                transferring: None,
            }
        );

        // For CpiGuard extension
        let cpi_guard_params = ParamValue::CpiGuard(
            CpiGuard {
                lock_cpi: true,
            }
        );

        // For MemoTransfer extension
        let memo_transfer_params = ParamValue::MemoTransfer(
            MemoTransfer {
                require_incoming_transfer_memos: true,
            }
        );

        // For TransferFeeAmount extension
        let transfer_fee_amount_params = ParamValue::TransferFeeAmount(
            TransferFeeAmount {
                withheld_amount: 1000,
            }
        );

        // For ConfidentialTransferAccount extension
        let elgamal_keypair = ElGamalKeypair::new_rand();
        let aes_key = AeKey::new_rand();

        let confidential_transfer_params = ParamValue::ConfidentialTransferAccount(
            ConfidentialTransferAccount {
                approved: true,
                elgamal_pubkey: PodElGamalPubkey::from(*elgamal_keypair.pubkey()),
                pending_balance_lo: PodElGamalCiphertext::default(),
                pending_balance_hi: PodElGamalCiphertext::default(),
                available_balance: PodElGamalCiphertext::default(),
                decryptable_available_balance: PodAeCiphertext::from(aes_key.encrypt(0u64)),
                allow_confidential_credits: true,
                allow_non_confidential_credits: true,
                pending_balance_credit_counter: 0,
                maximum_pending_balance_credit_counter: 100000,
                expected_pending_balance_credit_counter: 0,
                actual_pending_balance_credit_counter: 0,
            }
        );

        let immutable_owner_params = ParamValue::ImmutableOwner(
            ImmutableOwner {}
        );

        let non_transferable_account = ParamValue::NonTransferableAccount(
            NonTransferableAccount {}
        );

        let confidential_trasnfer_amount = ParamValue::ConfidentialTransferFeeAmount(
            ConfidentialTransferFeeAmount {
                withheld_amount: EncryptedWithheldAmount::default(),
            }
        );

        params2.push(transfer_hook_account_params);
        params2.push(cpi_guard_params);
        params2.push(memo_transfer_params);
        params2.push(transfer_fee_amount_params);
        params2.push(confidential_transfer_params);
        params2.push(immutable_owner_params);
        params2.push(non_transferable_account);
        params2.push(confidential_trasnfer_amount);


        let mint_token_account = fuzz_accounts
            .mint_token_account 
            .get_or_create_token2022_account( 
                self.accounts.mint_token_account.account_id,
                client,
                None,
                self.accounts.mint.pubkey(),
                receiver,  
                0,
                None,
                false,
                0,
                None,
                Some(params2),
            );
    

        self.accounts.mint_token_account.set_address(mint_token_account);



        let seeds = &[&b"extra-account-metas"[..]];
        let extra_metas_account = fuzz_accounts.extra_metas_account.get_or_create(
            self.accounts.extra_metas_account.account_id, 
            client, 
            None,
            None,
        );
        
        self.accounts.extra_metas_account.set_address(extra_metas_account);

    }
}
