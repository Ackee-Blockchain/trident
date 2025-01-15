use borsh::{BorshDeserialize, BorshSerialize};
use incorrect_ix_sequence_1::{PROJECT_SEED, STATE_SEED};
use trident_fuzz::fuzzing::*;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    EndRegistrations(EndRegistrations),
    Initialize(Initialize),
    Invest(Invest),
    Register(Register),
}
#[derive(Arbitrary, Debug)]
pub struct EndRegistrations {
    pub accounts: EndRegistrationsAccounts,
    pub data: EndRegistrationsData,
}
#[derive(Arbitrary, Debug)]
pub struct EndRegistrationsAccounts {
    pub author: AccountId,
    pub state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct EndRegistrationsData {}
#[derive(Arbitrary, Debug)]
pub struct Initialize {
    pub accounts: InitializeAccounts,
    pub data: InitializeData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeAccounts {
    pub author: AccountId,
    pub state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct InitializeData {}
#[derive(Arbitrary, Debug)]
pub struct Invest {
    pub accounts: InvestAccounts,
    pub data: InvestData,
}
#[derive(Arbitrary, Debug)]
pub struct InvestAccounts {
    pub investor: AccountId,
    pub project: AccountId,
    pub state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct InvestData {
    pub amount: u64,
}
#[derive(Arbitrary, Debug)]
pub struct Register {
    pub accounts: RegisterAccounts,
    pub data: RegisterData,
}
#[derive(Arbitrary, Debug)]
pub struct RegisterAccounts {
    pub project_author: AccountId,
    pub project: AccountId,
    pub state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct RegisterData {}
///IxOps implementation for `EndRegistrations` with all required functions.
impl IxOps for EndRegistrations {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![6u8, 118u8, 91u8, 63u8, 166u8, 46u8, 132u8, 233u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("dk5VmuCSjrG6iRVXRycKZ6mS4rDCyvBrYJvcfyqWGcU")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let args: Vec<u8> = self.get_discriminator();
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let mut signers = vec![];
        {
            let author = fuzz_accounts.author.get(self.accounts.author);
            account_metas.push(AccountMeta::new_readonly(author.pubkey(), true));
            signers.push(author.insecure_clone());
        };
        {
            let state = fuzz_accounts.state.get(self.accounts.state);
            account_metas.push(AccountMeta::new(state, false));
        }
        Ok((signers, account_metas))
    }
}
///IxOps implementation for `Initialize` with all required functions.
impl IxOps for Initialize {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("dk5VmuCSjrG6iRVXRycKZ6mS4rDCyvBrYJvcfyqWGcU")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let args: Vec<u8> = self.get_discriminator();
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let mut signers = vec![];
        let author = {
            let author = fuzz_accounts.author.get_or_create_account(
                self.accounts.author,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new(author.pubkey(), true));
            signers.push(author.insecure_clone());
            author.pubkey()
        };
        {
            let state = fuzz_accounts.state.get_or_create_account(
                self.accounts.state,
                client,
                &[author.as_ref(), STATE_SEED.as_ref()],
                &self.get_program_id(),
            );
            account_metas.push(AccountMeta::new(state, false));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        Ok((signers, account_metas))
    }
}
///IxOps implementation for `Invest` with all required functions.
impl IxOps for Invest {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![13u8, 245u8, 180u8, 103u8, 254u8, 182u8, 121u8, 4u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("dk5VmuCSjrG6iRVXRycKZ6mS4rDCyvBrYJvcfyqWGcU")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data.amount).unwrap());
        }
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let mut signers = vec![];
        {
            let investor = fuzz_accounts.investor.get_or_create_account(
                self.accounts.investor,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new(investor.pubkey(), true));
            signers.push(investor.insecure_clone());
        }
        {
            let project = fuzz_accounts.project.get(self.accounts.project);
            account_metas.push(AccountMeta::new(project, false));
        }
        {
            let state = fuzz_accounts.state.get(self.accounts.state);
            account_metas.push(AccountMeta::new(state, false));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        Ok((signers, account_metas))
    }
}
///IxOps implementation for `Register` with all required functions.
impl IxOps for Register {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![211u8, 124u8, 67u8, 15u8, 211u8, 194u8, 178u8, 240u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("dk5VmuCSjrG6iRVXRycKZ6mS4rDCyvBrYJvcfyqWGcU")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let args: Vec<u8> = self.get_discriminator();
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let mut signers = vec![];
        let project_author = {
            let project_author = fuzz_accounts.project_author.get_or_create_account(
                self.accounts.project_author,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new(project_author.pubkey(), true));
            signers.push(project_author.insecure_clone());
            project_author.pubkey()
        };
        let state = { fuzz_accounts.state.get(self.accounts.state) };
        {
            let project = fuzz_accounts.project.get_or_create_account(
                self.accounts.project,
                client,
                &[
                    project_author.as_ref(),
                    state.as_ref(),
                    PROJECT_SEED.as_ref(),
                ],
                &self.get_program_id(),
            );
            account_metas.push(AccountMeta::new(project, false));
        }
        {
            account_metas.push(AccountMeta::new(state, false));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        Ok((signers, account_metas))
    }
    fn check(
        &self,
        pre_ix: &[SnapshotAccount],
        post_ix: &[SnapshotAccount],
        _ix_data: Vec<u8>,
    ) -> Result<(), FuzzingError> {
        // This fuzz check will reveal that registrations can be performed
        // even though registration windows is not open.

        let state = State::deserialize(&mut pre_ix[2].data_no_discriminator());

        let project = Project::deserialize(&mut post_ix[1].data_no_discriminator());

        if let Ok(_project) = project {
            if let Ok(state) = state {
                let registrations_round = state.registrations_round;
                if !registrations_round {
                    return Err(FuzzingError::Custom(1));
                }
            }
        }
        Ok(())
    }
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    author: AccountsStorage<KeypairStore>,
    investor: AccountsStorage<KeypairStore>,
    project: AccountsStorage<PdaStore>,
    project_author: AccountsStorage<KeypairStore>,
    state: AccountsStorage<PdaStore>,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Project {
    project_author: Pubkey,
    invested_amount: u64,
    bump: u8,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct State {
    author: Pubkey,
    registrations_round: bool,
    total_invested: u64,
    bump: u8,
}
