// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod turnstile_instruction {
    use trident_client::prelude::*;
    use trident_client::test::*;
    pub const PROGRAM_ID: Pubkey = pubkey!("Po1RaS8BEDbNcn5oXsFryAeQ6Wn8fvmE111DJaKCgPC");
    pub async fn coin(
        client: &Client,
        i_dummy_arg: String,
        a_state: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                turnstile::instruction::Coin {
                    dummy_arg: i_dummy_arg,
                },
                turnstile::accounts::UpdateState { state: a_state },
                signers,
            )
            .await
    }
    pub fn coin_ix(i_dummy_arg: String, a_state: Pubkey) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: turnstile::instruction::Coin {
                dummy_arg: i_dummy_arg,
            }
            .data(),
            accounts: turnstile::accounts::UpdateState { state: a_state }.to_account_metas(None),
        }
    }
    pub async fn initialize(
        client: &Client,
        a_state: Pubkey,
        a_user: Pubkey,
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
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
            .await
    }
    pub fn initialize_ix(a_state: Pubkey, a_user: Pubkey, a_system_program: Pubkey) -> Instruction {
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
    pub async fn push(
        client: &Client,
        a_state: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                turnstile::instruction::Push {},
                turnstile::accounts::UpdateState { state: a_state },
                signers,
            )
            .await
    }
    pub fn push_ix(a_state: Pubkey) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: turnstile::instruction::Push {}.data(),
            accounts: turnstile::accounts::UpdateState { state: a_state }.to_account_metas(None),
        }
    }
}
