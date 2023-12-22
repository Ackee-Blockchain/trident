// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod turnstile_instruction {
    use trdelnik_client::program_client::*;
    pub static PROGRAM_ID: solana_sdk::pubkey::Pubkey =
        solana_sdk::pubkey::Pubkey::new_from_array([
            5u8, 214u8, 204u8, 101u8, 166u8, 163u8, 239u8, 244u8, 13u8, 110u8, 64u8, 106u8, 230u8,
            81u8, 141u8, 186u8, 208u8, 155u8, 78u8, 83u8, 194u8, 215u8, 103u8, 17u8, 94u8, 15u8,
            137u8, 68u8, 170u8, 153u8, 74u8, 59u8,
        ]);
    #[allow(clippy::too_many_arguments)]
    pub async fn initialize(
        client: &Client,
        a_state: &solana_sdk::pubkey::Pubkey,
        a_user: &solana_sdk::pubkey::Pubkey,
        a_system_program: &solana_sdk::pubkey::Pubkey,
        signers: impl IntoIterator<Item = &solana_sdk::signer::keypair::Keypair> + Send,
    ) -> Result<
        solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta,
        anchor_client::ClientError,
    > {
        client
            .send_instruction(
                PROGRAM_ID,
                turnstile::instruction::Initialize {},
                turnstile::accounts::Initialize {
                    state: *a_state,
                    user: *a_user,
                    system_program: *a_system_program,
                },
                signers,
            )
            .await
    }
    #[allow(clippy::too_many_arguments)]
    pub fn initialize_ix(
        a_state: &solana_sdk::pubkey::Pubkey,
        a_user: &solana_sdk::pubkey::Pubkey,
        a_system_program: &solana_sdk::pubkey::Pubkey,
    ) -> solana_sdk::instruction::Instruction {
        solana_sdk::instruction::Instruction {
            program_id: PROGRAM_ID,
            data: turnstile::instruction::Initialize {}.data(),
            accounts: turnstile::accounts::Initialize {
                state: *a_state,
                user: *a_user,
                system_program: *a_system_program,
            }
            .to_account_metas(None),
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub async fn coin(
        client: &Client,
        i_dummy_arg: String,
        a_state: &solana_sdk::pubkey::Pubkey,
        signers: impl IntoIterator<Item = &solana_sdk::signer::keypair::Keypair> + Send,
    ) -> Result<
        solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta,
        anchor_client::ClientError,
    > {
        client
            .send_instruction(
                PROGRAM_ID,
                turnstile::instruction::Coin {
                    dummy_arg: i_dummy_arg,
                },
                turnstile::accounts::UpdateState { state: *a_state },
                signers,
            )
            .await
    }
    #[allow(clippy::too_many_arguments)]
    pub fn coin_ix(
        i_dummy_arg: String,
        a_state: &solana_sdk::pubkey::Pubkey,
    ) -> solana_sdk::instruction::Instruction {
        solana_sdk::instruction::Instruction {
            program_id: PROGRAM_ID,
            data: turnstile::instruction::Coin {
                dummy_arg: i_dummy_arg,
            }
            .data(),
            accounts: turnstile::accounts::UpdateState { state: *a_state }.to_account_metas(None),
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub async fn push(
        client: &Client,
        a_state: &solana_sdk::pubkey::Pubkey,
        signers: impl IntoIterator<Item = &solana_sdk::signer::keypair::Keypair> + Send,
    ) -> Result<
        solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta,
        anchor_client::ClientError,
    > {
        client
            .send_instruction(
                PROGRAM_ID,
                turnstile::instruction::Push {},
                turnstile::accounts::UpdateState { state: *a_state },
                signers,
            )
            .await
    }
    #[allow(clippy::too_many_arguments)]
    pub fn push_ix(a_state: &solana_sdk::pubkey::Pubkey) -> solana_sdk::instruction::Instruction {
        solana_sdk::instruction::Instruction {
            program_id: PROGRAM_ID,
            data: turnstile::instruction::Push {}.data(),
            accounts: turnstile::accounts::UpdateState { state: *a_state }.to_account_metas(None),
        }
    }
}
