// DO NOT EDIT - automatically generated file (except `use` statements inside the `*_instruction` module
pub mod fuzz_example1_instruction {
    use trident_client::*;
    pub static PROGRAM_ID: Pubkey = Pubkey::new_from_array([
        102u8, 14u8, 37u8, 223u8, 213u8, 157u8, 223u8, 31u8, 244u8, 37u8, 118u8, 158u8, 9u8, 208u8,
        160u8, 222u8, 23u8, 8u8, 210u8, 4u8, 175u8, 165u8, 252u8, 222u8, 56u8, 103u8, 180u8, 216u8,
        231u8, 91u8, 28u8, 159u8,
    ]);
    pub async fn initialize(
        client: &Client,
        a_author: Pubkey,
        a_state: Pubkey,
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                fuzz_example1::instruction::Initialize {},
                fuzz_example1::accounts::Initialize {
                    author: a_author,
                    state: a_state,
                    system_program: a_system_program,
                },
                signers,
            )
            .await
    }
    pub fn initialize_ix(
        a_author: Pubkey,
        a_state: Pubkey,
        a_system_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: fuzz_example1::instruction::Initialize {}.data(),
            accounts: fuzz_example1::accounts::Initialize {
                author: a_author,
                state: a_state,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn register(
        client: &Client,
        a_project_author: Pubkey,
        a_project: Pubkey,
        a_state: Pubkey,
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                fuzz_example1::instruction::Register {},
                fuzz_example1::accounts::Register {
                    project_author: a_project_author,
                    project: a_project,
                    state: a_state,
                    system_program: a_system_program,
                },
                signers,
            )
            .await
    }
    pub fn register_ix(
        a_project_author: Pubkey,
        a_project: Pubkey,
        a_state: Pubkey,
        a_system_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: fuzz_example1::instruction::Register {}.data(),
            accounts: fuzz_example1::accounts::Register {
                project_author: a_project_author,
                project: a_project,
                state: a_state,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
    pub async fn end_registrations(
        client: &Client,
        a_author: Pubkey,
        a_state: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                fuzz_example1::instruction::EndRegistrations {},
                fuzz_example1::accounts::EndRegistration {
                    author: a_author,
                    state: a_state,
                },
                signers,
            )
            .await
    }
    pub fn end_registrations_ix(a_author: Pubkey, a_state: Pubkey) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: fuzz_example1::instruction::EndRegistrations {}.data(),
            accounts: fuzz_example1::accounts::EndRegistration {
                author: a_author,
                state: a_state,
            }
            .to_account_metas(None),
        }
    }
    pub async fn invest(
        client: &Client,
        i_amount: u64,
        a_investor: Pubkey,
        a_project: Pubkey,
        a_state: Pubkey,
        a_system_program: Pubkey,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
        client
            .send_instruction(
                PROGRAM_ID,
                fuzz_example1::instruction::Invest { amount: i_amount },
                fuzz_example1::accounts::Invest {
                    investor: a_investor,
                    project: a_project,
                    state: a_state,
                    system_program: a_system_program,
                },
                signers,
            )
            .await
    }
    pub fn invest_ix(
        i_amount: u64,
        a_investor: Pubkey,
        a_project: Pubkey,
        a_state: Pubkey,
        a_system_program: Pubkey,
    ) -> Instruction {
        Instruction {
            program_id: PROGRAM_ID,
            data: fuzz_example1::instruction::Invest { amount: i_amount }.data(),
            accounts: fuzz_example1::accounts::Invest {
                investor: a_investor,
                project: a_project,
                state: a_state,
                system_program: a_system_program,
            }
            .to_account_metas(None),
        }
    }
}
