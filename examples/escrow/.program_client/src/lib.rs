// DO NOT EDIT - automatically generated file
pub mod escrow_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 215u8, 176u8, 66u8, 255u8, 47u8, 77u8, 122u8, 100u8, 249u8, 156u8, 251u8, 44u8, 92u8,
        36u8, 220u8, 226u8, 147u8, 127u8, 109u8, 198u8, 92u8, 1u8, 127u8, 95u8, 116u8, 186u8,
        180u8, 149u8, 157u8, 170u8, 34u8,
    ]);
    pub async fn initialize_escrow(
        client: &Client,
        i_initializer_amount: u64,
        i_taker_amount: u64,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer_receive_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                escrow::instruction::InitializeEscrow {
                    initializer_amount: i_initializer_amount,
                    taker_amount: i_taker_amount,
                },
                escrow::accounts::InitializeEscrow {
                    initializer: a_initializer,
                    initializer_deposit_token_account: a_initializer_deposit_token_account,
                    initializer_receive_token_account: a_initializer_receive_token_account,
                    escrow_account: a_escrow_account,
                    system_program: a_system_program,
                    token_program: a_token_program,
                },
                signers,
            )
            .await?)
    }
    pub fn initialize_escrow_ix(
        i_initializer_amount: u64,
        i_taker_amount: u64,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer_receive_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: escrow::instruction::InitializeEscrow {
                initializer_amount: i_initializer_amount,
                taker_amount: i_taker_amount,
            }
            .data(),
            accounts: escrow::accounts::InitializeEscrow {
                initializer: a_initializer,
                initializer_deposit_token_account: a_initializer_deposit_token_account,
                initializer_receive_token_account: a_initializer_receive_token_account,
                escrow_account: a_escrow_account,
                system_program: a_system_program,
                token_program: a_token_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn cancel_escrow(
        client: &Client,
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_pda_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_pda_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                escrow::instruction::CancelEscrow {},
                escrow::accounts::CancelEscrow {
                    initializer: a_initializer,
                    pda_deposit_token_account: a_pda_deposit_token_account,
                    pda_account: a_pda_account,
                    escrow_account: a_escrow_account,
                    token_program: a_token_program,
                },
                signers,
            )
            .await?)
    }
    pub fn cancel_escrow_ix(
        a_initializer: anchor_lang::solana_program::pubkey::Pubkey,
        a_pda_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_pda_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: escrow::instruction::CancelEscrow {}.data(),
            accounts: escrow::accounts::CancelEscrow {
                initializer: a_initializer,
                pda_deposit_token_account: a_pda_deposit_token_account,
                pda_account: a_pda_account,
                escrow_account: a_escrow_account,
                token_program: a_token_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn exchange(
        client: &Client,
        a_taker: anchor_lang::solana_program::pubkey::Pubkey,
        a_taker_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_taker_receive_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_pda_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer_receive_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer_main_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_pda_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                escrow::instruction::Exchange {},
                escrow::accounts::Exchange {
                    taker: a_taker,
                    taker_deposit_token_account: a_taker_deposit_token_account,
                    taker_receive_token_account: a_taker_receive_token_account,
                    pda_deposit_token_account: a_pda_deposit_token_account,
                    initializer_receive_token_account: a_initializer_receive_token_account,
                    initializer_main_account: a_initializer_main_account,
                    escrow_account: a_escrow_account,
                    pda_account: a_pda_account,
                    token_program: a_token_program,
                },
                signers,
            )
            .await?)
    }
    pub fn exchange_ix(
        a_taker: anchor_lang::solana_program::pubkey::Pubkey,
        a_taker_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_taker_receive_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_pda_deposit_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer_receive_token_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_initializer_main_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_escrow_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_pda_account: anchor_lang::solana_program::pubkey::Pubkey,
        a_token_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: escrow::instruction::Exchange {}.data(),
            accounts: escrow::accounts::Exchange {
                taker: a_taker,
                taker_deposit_token_account: a_taker_deposit_token_account,
                taker_receive_token_account: a_taker_receive_token_account,
                pda_deposit_token_account: a_pda_deposit_token_account,
                initializer_receive_token_account: a_initializer_receive_token_account,
                initializer_main_account: a_initializer_main_account,
                escrow_account: a_escrow_account,
                pda_account: a_pda_account,
                token_program: a_token_program,
            }
            .to_account_metas(None),
        }
    }
}
