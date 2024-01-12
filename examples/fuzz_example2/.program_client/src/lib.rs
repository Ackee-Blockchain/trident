// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod fuzz_example2_instruction {
    use trdelnik_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        99u8, 10u8, 244u8, 108u8, 95u8, 124u8, 110u8, 175u8, 5u8, 138u8, 87u8, 246u8, 183u8, 149u8,
        178u8, 242u8, 77u8, 253u8, 33u8, 157u8, 220u8, 188u8, 159u8, 48u8, 115u8, 50u8, 140u8,
        170u8, 68u8, 230u8, 108u8, 133u8,
    ]);
    pub async fn initialize(
        client: &Client,
        i_receiver: Pubkey,
        i_amount: u64,
        a_author: Pubkey,
        a_escrow: Pubkey,
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                fuzz_example2::instruction::Initialize {
                    receiver: i_receiver,
                    amount: i_amount,
                },
                fuzz_example2::accounts::Initialize {
                    author: a_author,
                    escrow: a_escrow,
                    system_program: a_system_program,
                },
                signers,
            )
            .await
    }
    pub fn initialize_ix(
        i_receiver: Pubkey,
        i_amount: u64,
        a_author: Pubkey,
        a_escrow: Pubkey,
        a_system_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: fuzz_example2::instruction::Initialize {
                receiver: i_receiver,
                amount: i_amount,
            }
            .data(),
            accounts: fuzz_example2::accounts::Initialize {
                author: a_author,
                escrow: a_escrow,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn withdraw(
        client: &Client,
        a_receiver: Pubkey,
        a_escrow: Pubkey,
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                fuzz_example2::instruction::Withdraw {},
                fuzz_example2::accounts::Withdraw {
                    receiver: a_receiver,
                    escrow: a_escrow,
                    system_program: a_system_program,
                },
                signers,
            )
            .await
    }
    pub fn withdraw_ix(
        a_receiver: Pubkey,
        a_escrow: Pubkey,
        a_system_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: fuzz_example2::instruction::Withdraw {}.data(),
            accounts: fuzz_example2::accounts::Withdraw {
                receiver: a_receiver,
                escrow: a_escrow,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
}
