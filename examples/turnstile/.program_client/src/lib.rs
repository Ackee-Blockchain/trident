// DO NOT EDIT - automatically generated file
pub mod turnstile_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        5u8, 215u8, 176u8, 66u8, 255u8, 47u8, 77u8, 122u8, 100u8, 249u8, 156u8, 251u8, 44u8, 92u8,
        36u8, 220u8, 226u8, 147u8, 127u8, 109u8, 198u8, 92u8, 1u8, 127u8, 95u8, 116u8, 186u8,
        180u8, 149u8, 157u8, 170u8, 34u8,
    ]);
    pub async fn initialize(
        client: &Client,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                turnstile::instruction::Initialize {},
                turnstile::accounts::Initialize {
                    state: a_state,
                    user: a_user,
                    system_program: a_system_program,
                },
                signers,
            )
            .await?)
    }
    pub fn initialize_ix(
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        a_user: anchor_lang::solana_program::pubkey::Pubkey,
        a_system_program: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: turnstile::instruction::Initialize {}.data(),
            accounts: turnstile::accounts::Initialize {
                state: a_state,
                user: a_user,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn coin(
        client: &Client,
        i_dummy_arg: String,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                turnstile::instruction::Coin {
                    dummy_arg: i_dummy_arg,
                },
                turnstile::accounts::UpdateState { state: a_state },
                signers,
            )
            .await?)
    }
    pub fn coin_ix(
        i_dummy_arg: String,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: turnstile::instruction::Coin {
                dummy_arg: i_dummy_arg,
            }
            .data(),
            accounts: turnstile::accounts::UpdateState { state: a_state }.to_account_metas(None),
        }
    }
    pub async fn push(
        client: &Client,
        a_state: anchor_lang::solana_program::pubkey::Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransaction, ClientError> {
        Ok(client
            .send_instruction(
                PROGRAM_ID,
                turnstile::instruction::Push {},
                turnstile::accounts::UpdateState { state: a_state },
                signers,
            )
            .await?)
    }
    pub fn push_ix(a_state: anchor_lang::solana_program::pubkey::Pubkey) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: turnstile::instruction::Push {}.data(),
            accounts: turnstile::accounts::UpdateState { state: a_state }.to_account_metas(None),
        }
    }
}
