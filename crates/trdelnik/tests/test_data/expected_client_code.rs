// DO NOT EDIT - automatically generated file
pub mod turnstile_instruction {
    use trdelnik::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        216u8, 55u8, 200u8, 93u8, 189u8, 81u8, 94u8, 109u8, 14u8, 249u8, 244u8, 106u8, 68u8, 214u8,
        222u8, 190u8, 9u8, 25u8, 199u8, 75u8, 79u8, 230u8, 94u8, 137u8, 51u8, 187u8, 193u8, 48u8,
        87u8, 222u8, 175u8, 163u8,
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
