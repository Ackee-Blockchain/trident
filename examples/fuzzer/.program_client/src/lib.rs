// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod fuzzer_instruction {
    use trdelnik_client::program_client::*;
    pub static PROGRAM_ID: solana_sdk::pubkey::Pubkey =
        solana_sdk::pubkey::Pubkey::new_from_array([
            170u8, 64u8, 48u8, 229u8, 53u8, 121u8, 89u8, 247u8, 36u8, 222u8, 119u8, 168u8, 36u8,
            42u8, 8u8, 162u8, 161u8, 90u8, 85u8, 0u8, 151u8, 100u8, 169u8, 133u8, 216u8, 142u8,
            250u8, 145u8, 26u8, 46u8, 170u8, 146u8,
        ]);
    #[allow(clippy::too_many_arguments)]
    pub async fn initialize(
        client: &Client,
        a_counter: &solana_sdk::pubkey::Pubkey,
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
                fuzzer::instruction::Initialize {},
                fuzzer::accounts::Initialize {
                    counter: *a_counter,
                    user: *a_user,
                    system_program: *a_system_program,
                },
                signers,
            )
            .await
    }
    #[allow(clippy::too_many_arguments)]
    pub fn initialize_ix(
        a_counter: &solana_sdk::pubkey::Pubkey,
        a_user: &solana_sdk::pubkey::Pubkey,
        a_system_program: &solana_sdk::pubkey::Pubkey,
    ) -> solana_sdk::instruction::Instruction {
        solana_sdk::instruction::Instruction {
            program_id: PROGRAM_ID,
            data: fuzzer::instruction::Initialize {}.data(),
            accounts: fuzzer::accounts::Initialize {
                counter: *a_counter,
                user: *a_user,
                system_program: *a_system_program,
            }
            .to_account_metas(None),
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        client: &Client,
        i_input1: u8,
        i_input2: u8,
        a_counter: &solana_sdk::pubkey::Pubkey,
        a_authority: &solana_sdk::pubkey::Pubkey,
        signers: impl IntoIterator<Item = &solana_sdk::signer::keypair::Keypair> + Send,
    ) -> Result<
        solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta,
        anchor_client::ClientError,
    > {
        client
            .send_instruction(
                PROGRAM_ID,
                fuzzer::instruction::Update {
                    input1: i_input1,
                    input2: i_input2,
                },
                fuzzer::accounts::Update {
                    counter: *a_counter,
                    authority: *a_authority,
                },
                signers,
            )
            .await
    }
    #[allow(clippy::too_many_arguments)]
    pub fn update_ix(
        i_input1: u8,
        i_input2: u8,
        a_counter: &solana_sdk::pubkey::Pubkey,
        a_authority: &solana_sdk::pubkey::Pubkey,
    ) -> solana_sdk::instruction::Instruction {
        solana_sdk::instruction::Instruction {
            program_id: PROGRAM_ID,
            data: fuzzer::instruction::Update {
                input1: i_input1,
                input2: i_input2,
            }
            .data(),
            accounts: fuzzer::accounts::Update {
                counter: *a_counter,
                authority: *a_authority,
            }
            .to_account_metas(None),
        }
    }
}
