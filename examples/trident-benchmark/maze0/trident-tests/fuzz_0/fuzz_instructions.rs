use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_fuzz::fuzzing::*;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    Initialize(Initialize),
    MoveEast(MoveEast),
    MoveNorth(MoveNorth),
    MoveSouth(MoveSouth),
    MoveWest(MoveWest),
}
#[derive(Arbitrary, Debug)]
pub struct Initialize {
    pub accounts: InitializeAccounts,
    pub _data: InitializeData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeAccounts {
    pub state_author: AccountId,
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
pub struct MoveEast {
    pub accounts: MoveEastAccounts,
    pub data: MoveEastData,
}
#[derive(Arbitrary, Debug)]
pub struct MoveEastAccounts {
    pub state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug, BorshSerialize, BorshDeserialize)]
pub struct MoveEastData {
    pub p0: u64,
    pub p1: u64,
    pub p2: u64,
    pub p3: u64,
    pub p4: u64,
    pub p5: u64,
    pub p6: u64,
    pub p7: u64,
}
#[derive(Arbitrary, Debug)]
pub struct MoveNorth {
    pub accounts: MoveNorthAccounts,
    pub data: MoveNorthData,
}
#[derive(Arbitrary, Debug)]
pub struct MoveNorthAccounts {
    pub state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug, BorshSerialize, BorshDeserialize)]
pub struct MoveNorthData {
    pub p0: u64,
    pub p1: u64,
    pub p2: u64,
    pub p3: u64,
    pub p4: u64,
    pub p5: u64,
    pub p6: u64,
    pub p7: u64,
}
#[derive(Arbitrary, Debug)]
pub struct MoveSouth {
    pub accounts: MoveSouthAccounts,
    pub data: MoveSouthData,
}
#[derive(Arbitrary, Debug)]
pub struct MoveSouthAccounts {
    pub state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug, BorshSerialize, BorshDeserialize)]
pub struct MoveSouthData {
    pub p0: u64,
    pub p1: u64,
    pub p2: u64,
    pub p3: u64,
    pub p4: u64,
    pub p5: u64,
    pub p6: u64,
    pub p7: u64,
}
#[derive(Arbitrary, Debug)]
pub struct MoveWest {
    pub accounts: MoveWestAccounts,
    pub data: MoveWestData,
}
#[derive(Arbitrary, Debug)]
pub struct MoveWestAccounts {
    pub state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug, BorshSerialize, BorshDeserialize)]
pub struct MoveWestData {
    pub p0: u64,
    pub p1: u64,
    pub p2: u64,
    pub p3: u64,
    pub p4: u64,
    pub p5: u64,
    pub p6: u64,
    pub p7: u64,
}
///IxOps implementation for `Initialize` with all required functions.
impl IxOps for Initialize {
    type IxAccounts = FuzzAccounts;

    fn get_discriminator(&self) -> Vec<u8> {
        vec![175, 175, 109, 31, 13, 152, 155, 237]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
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
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let mut signers = vec![];

        {
            let state_author = fuzz_accounts.state_author.get_or_create_account(
                self.accounts.state_author,
                client,
                10 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new(state_author.pubkey(), true));
            signers.push(state_author.insecure_clone());
        }

        {
            let state = fuzz_accounts.state.get_or_create_account(
                1,
                client,
                &["state".as_bytes()],
                &self.get_program_id(),
            );

            account_metas.push(AccountMeta::new(state, false));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                solana_sdk::system_program::ID,
                false,
            ));
        }
        Ok((signers, account_metas))
    }
}
///IxOps implementation for `MoveEast` with all required functions.
impl IxOps for MoveEast {
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![220, 96, 254, 139, 6, 133, 127, 93]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data.p0).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p1).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p2).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p3).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p4).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p5).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p6).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p7).unwrap());
        }
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let signers = vec![];

        {
            let state = fuzz_accounts.state.get(1);
            account_metas.push(AccountMeta::new(state, false));
        }
        Ok((signers, account_metas))
    }
    fn tx_error_handler(
        &self,
        _e: FuzzClientErrorWithOrigin,
        _ix_data: Vec<u8>,
        _pre_ix_acc_infos: &[SnapshotAccount],
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        Ok(())
    }
}
///IxOps implementation for `MoveNorth` with all required functions.
impl IxOps for MoveNorth {
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![65, 4, 235, 142, 120, 215, 181, 131]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data.p0).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p1).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p2).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p3).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p4).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p5).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p6).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p7).unwrap());
        }
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let signers = vec![];

        {
            let state = fuzz_accounts.state.get(1);
            account_metas.push(AccountMeta::new(state, false));
        }
        Ok((signers, account_metas))
    }
    fn tx_error_handler(
        &self,
        _e: FuzzClientErrorWithOrigin,
        _ix_data: Vec<u8>,
        _pre_ix_acc_infos: &[SnapshotAccount],
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        Ok(())
    }
}
///IxOps implementation for `MoveSouth` with all required functions.
impl IxOps for MoveSouth {
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![146, 138, 196, 38, 130, 143, 149, 55]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data.p0).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p1).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p2).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p3).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p4).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p5).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p6).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p7).unwrap());
        }
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let signers = vec![];

        {
            let state = fuzz_accounts.state.get(1);
            account_metas.push(AccountMeta::new(state, false));
        }
        Ok((signers, account_metas))
    }
    fn tx_error_handler(
        &self,
        _e: FuzzClientErrorWithOrigin,
        _ix_data: Vec<u8>,
        _pre_ix_acc_infos: &[SnapshotAccount],
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        Ok(())
    }
}
///IxOps implementation for `MoveWest` with all required functions.
impl IxOps for MoveWest {
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![122, 187, 56, 38, 248, 122, 182, 106]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data.p0).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p1).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p2).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p3).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p4).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p5).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p6).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.p7).unwrap());
        }
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let signers = vec![];

        {
            let state = fuzz_accounts.state.get(1);
            account_metas.push(AccountMeta::new(state, false));
        }
        Ok((signers, account_metas))
    }
    fn tx_error_handler(
        &self,
        _e: FuzzClientErrorWithOrigin,
        _ix_data: Vec<u8>,
        _pre_ix_acc_infos: &[SnapshotAccount],
    ) -> Result<(), FuzzClientErrorWithOrigin> {
        Ok(())
    }
}
/// Use AccountsStorage<T> where T can be one of:
/// Keypair, PdaStore, TokenStore, MintStore, ProgramStore
#[derive(Default)]
pub struct FuzzAccounts {
    state: AccountsStorage<PdaStore>,
    state_author: AccountsStorage<KeypairStore>,
    // system_program: AccountsStorage<todo!()>,
}
