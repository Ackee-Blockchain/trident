use incorrect_ix_sequence_1::{PROJECT_SEED, STATE_SEED};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_client::fuzzing::*;
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
    pub _data: EndRegistrationsData,
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
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct EndRegistrationsData {}
#[derive(Arbitrary, Debug)]
pub struct Initialize {
    pub accounts: InitializeAccounts,
    pub _data: InitializeData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeAccounts {
    pub author: AccountId,
    pub state: AccountId,
    pub _system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
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
    pub _system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct InvestData {
    pub amount: u64,
}
#[derive(Arbitrary, Debug)]
pub struct Register {
    pub accounts: RegisterAccounts,
    pub _data: RegisterData,
}
#[derive(Arbitrary, Debug)]
pub struct RegisterAccounts {
    pub project_author: AccountId,
    pub project: AccountId,
    pub state: AccountId,
    pub _system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct RegisterData {}
///IxOps implementation for `EndRegistrations` with all required functions.
impl IxOps for EndRegistrations {
    type IxData = incorrect_ix_sequence_1::instruction::EndRegistrations;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        incorrect_ix_sequence_1::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = incorrect_ix_sequence_1::instruction::EndRegistrations {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let author = fuzz_accounts.author.get_or_create_account(
            self.accounts.author,
            client,
            5 * LAMPORTS_PER_SOL,
        );
        let signers = vec![author.clone()];
        let state = fuzz_accounts.state.get_or_create_account(
            self.accounts.state,
            client,
            &[author.pubkey().as_ref(), STATE_SEED.as_ref()],
            &incorrect_ix_sequence_1::ID,
        );
        let acc_meta = incorrect_ix_sequence_1::accounts::EndRegistration {
            author: author.pubkey(),
            state,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `Initialize` with all required functions.
impl IxOps for Initialize {
    type IxData = incorrect_ix_sequence_1::instruction::Initialize;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        incorrect_ix_sequence_1::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = incorrect_ix_sequence_1::instruction::Initialize {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let author = fuzz_accounts.author.get_or_create_account(
            self.accounts.author,
            client,
            5 * LAMPORTS_PER_SOL,
        );
        let signers = vec![author.clone()];
        let state = fuzz_accounts.state.get_or_create_account(
            self.accounts.state,
            client,
            &[author.pubkey().as_ref(), STATE_SEED.as_ref()],
            &incorrect_ix_sequence_1::ID,
        );
        let acc_meta = incorrect_ix_sequence_1::accounts::Initialize {
            author: author.pubkey(),
            state,
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `Invest` with all required functions.
impl IxOps for Invest {
    type IxData = incorrect_ix_sequence_1::instruction::Invest;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        incorrect_ix_sequence_1::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = incorrect_ix_sequence_1::instruction::Invest {
            amount: self.data.amount,
        };
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let investor = fuzz_accounts.investor.get_or_create_account(
            self.accounts.investor,
            client,
            5 * LAMPORTS_PER_SOL,
        );
        let signers = vec![investor.clone()];

        let project_author = fuzz_accounts.project_author.get_or_create_account(
            self.accounts.project,
            client,
            5 * LAMPORTS_PER_SOL,
        );
        let state = fuzz_accounts.state.get_or_create_account(
            self.accounts.state,
            client,
            &[project_author.pubkey().as_ref(), STATE_SEED.as_ref()],
            &incorrect_ix_sequence_1::ID,
        );

        let project = fuzz_accounts.project.get_or_create_account(
            self.accounts.project,
            client,
            &[
                project_author.pubkey().as_ref(),
                state.as_ref(),
                PROJECT_SEED.as_ref(),
            ],
            &incorrect_ix_sequence_1::ID,
        );
        let acc_meta = incorrect_ix_sequence_1::accounts::Invest {
            investor: investor.pubkey(),
            project,
            state,
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
}
///IxOps implementation for `Register` with all required functions.
impl IxOps for Register {
    type IxData = incorrect_ix_sequence_1::instruction::Register;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        incorrect_ix_sequence_1::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = incorrect_ix_sequence_1::instruction::Register {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let project_author = fuzz_accounts.project_author.get_or_create_account(
            self.accounts.project_author,
            client,
            5 * LAMPORTS_PER_SOL,
        );
        let signers = vec![project_author.clone()];
        let state = fuzz_accounts.state.get_or_create_account(
            self.accounts.state,
            client,
            &[project_author.pubkey().as_ref(), STATE_SEED.as_ref()],
            &incorrect_ix_sequence_1::ID,
        );

        let project = fuzz_accounts.project.get_or_create_account(
            self.accounts.project,
            client,
            &[
                project_author.pubkey().as_ref(),
                state.as_ref(),
                PROJECT_SEED.as_ref(),
            ],
            &incorrect_ix_sequence_1::ID,
        );

        let acc_meta = incorrect_ix_sequence_1::accounts::Register {
            project_author: project_author.pubkey(),
            project,
            state,
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);
        Ok((signers, acc_meta))
    }
    fn check(
        &self,
        pre_ix: &[SnapshotAccount],
        post_ix: &[SnapshotAccount],
        _ix_data: Self::IxData,
    ) -> Result<(), FuzzingError> {
        // This fuzz check will reveal that registrations can be performed
        // even though registration windows is not open.

        let state =
            incorrect_ix_sequence_1::state::State::try_deserialize_unchecked(&mut pre_ix[2].data());

        let project = incorrect_ix_sequence_1::state::Project::try_deserialize_unchecked(
            &mut post_ix[1].data(),
        );

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
/// Use AccountsStorage<T> where T can be one of:
/// Keypair, PdaStore, TokenStore, MintStore, ProgramStore
#[derive(Default)]
pub struct FuzzAccounts {
    project_author: AccountsStorage<KeypairStore>,
    author: AccountsStorage<KeypairStore>,
    project: AccountsStorage<PdaStore>,
    // There is no need to fuzz the 'system_program' account.
    // system_program: AccountsStorage<ProgramStore>,
    investor: AccountsStorage<KeypairStore>,
    state: AccountsStorage<PdaStore>,
}
