// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod fuzz_example3_instruction {
    use trident_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        222u8, 219u8, 96u8, 222u8, 150u8, 129u8, 32u8, 71u8, 184u8, 221u8, 54u8, 221u8, 224u8,
        97u8, 103u8, 133u8, 11u8, 126u8, 234u8, 11u8, 186u8, 25u8, 119u8, 161u8, 48u8, 137u8, 77u8,
        249u8, 144u8, 153u8, 133u8, 92u8,
    ]);
    pub async fn init_vesting(
        client: &Client,
        i_recipient: Pubkey,
        i_amount: u64,
        i_start_at: u64,
        i_end_at: u64,
        i_interval: u64,
        a_sender: Pubkey,
        a_sender_token_account: Pubkey,
        a_escrow: Pubkey,
        a_escrow_token_account: Pubkey,
        a_mint: Pubkey,
        a_token_program: Pubkey,
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                fuzz_example3::instruction::InitVesting {
                    recipient: i_recipient,
                    amount: i_amount,
                    start_at: i_start_at,
                    end_at: i_end_at,
                    interval: i_interval,
                },
                fuzz_example3::accounts::InitVesting {
                    sender: a_sender,
                    sender_token_account: a_sender_token_account,
                    escrow: a_escrow,
                    escrow_token_account: a_escrow_token_account,
                    mint: a_mint,
                    token_program: a_token_program,
                    system_program: a_system_program,
                },
                signers,
            )
            .await
    }
    pub fn init_vesting_ix(
        i_recipient: Pubkey,
        i_amount: u64,
        i_start_at: u64,
        i_end_at: u64,
        i_interval: u64,
        a_sender: Pubkey,
        a_sender_token_account: Pubkey,
        a_escrow: Pubkey,
        a_escrow_token_account: Pubkey,
        a_mint: Pubkey,
        a_token_program: Pubkey,
        a_system_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: fuzz_example3::instruction::InitVesting {
                recipient: i_recipient,
                amount: i_amount,
                start_at: i_start_at,
                end_at: i_end_at,
                interval: i_interval,
            }
            .data(),
            accounts: fuzz_example3::accounts::InitVesting {
                sender: a_sender,
                sender_token_account: a_sender_token_account,
                escrow: a_escrow,
                escrow_token_account: a_escrow_token_account,
                mint: a_mint,
                token_program: a_token_program,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn withdraw_unlocked(
        client: &Client,
        a_recipient: Pubkey,
        a_recipient_token_account: Pubkey,
        a_escrow: Pubkey,
        a_escrow_token_account: Pubkey,
        a_escrow_pda_authority: Pubkey,
        a_mint: Pubkey,
        a_token_program: Pubkey,
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                fuzz_example3::instruction::WithdrawUnlocked {},
                fuzz_example3::accounts::WithdrawUnlocked {
                    recipient: a_recipient,
                    recipient_token_account: a_recipient_token_account,
                    escrow: a_escrow,
                    escrow_token_account: a_escrow_token_account,
                    escrow_pda_authority: a_escrow_pda_authority,
                    mint: a_mint,
                    token_program: a_token_program,
                    system_program: a_system_program,
                },
                signers,
            )
            .await
    }
    pub fn withdraw_unlocked_ix(
        a_recipient: Pubkey,
        a_recipient_token_account: Pubkey,
        a_escrow: Pubkey,
        a_escrow_token_account: Pubkey,
        a_escrow_pda_authority: Pubkey,
        a_mint: Pubkey,
        a_token_program: Pubkey,
        a_system_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: fuzz_example3::instruction::WithdrawUnlocked {}.data(),
            accounts: fuzz_example3::accounts::WithdrawUnlocked {
                recipient: a_recipient,
                recipient_token_account: a_recipient_token_account,
                escrow: a_escrow,
                escrow_token_account: a_escrow_token_account,
                escrow_pda_authority: a_escrow_pda_authority,
                mint: a_mint,
                token_program: a_token_program,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
}
