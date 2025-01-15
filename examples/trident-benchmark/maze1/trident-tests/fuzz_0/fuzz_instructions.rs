use borsh::{BorshDeserialize, BorshSerialize};
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
    pub _state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct InitializeData {}
#[derive(Arbitrary, Debug)]
pub struct MoveEast {
    pub _accounts: MoveEastAccounts,
    pub data: MoveEastData,
}
#[derive(Arbitrary, Debug)]
pub struct MoveEastAccounts {
    pub _state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
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
    pub _accounts: MoveNorthAccounts,
    pub data: MoveNorthData,
}
#[derive(Arbitrary, Debug)]
pub struct MoveNorthAccounts {
    pub _state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
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
    pub _accounts: MoveSouthAccounts,
    pub data: MoveSouthData,
}
#[derive(Arbitrary, Debug)]
pub struct MoveSouthAccounts {
    pub _state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
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
    pub _accounts: MoveWestAccounts,
    pub data: MoveWestData,
}
#[derive(Arbitrary, Debug)]
pub struct MoveWestAccounts {
    pub _state: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
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
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")
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
        {
            let state_author = fuzz_accounts.state_author.get_or_create_account(
                self.accounts.state_author,
                client,
                500 * LAMPORTS_PER_SOL,
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
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        Ok((signers, account_metas))
    }
}
///IxOps implementation for `MoveEast` with all required functions.
impl IxOps for MoveEast {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![220u8, 96u8, 254u8, 139u8, 6u8, 133u8, 127u8, 93u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")
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
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
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
}
///IxOps implementation for `MoveNorth` with all required functions.
impl IxOps for MoveNorth {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![65u8, 4u8, 235u8, 142u8, 120u8, 215u8, 181u8, 131u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")
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
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
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
}
///IxOps implementation for `MoveSouth` with all required functions.
impl IxOps for MoveSouth {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![146u8, 138u8, 196u8, 38u8, 130u8, 143u8, 149u8, 55u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")
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
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
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
}
///IxOps implementation for `MoveWest` with all required functions.
impl IxOps for MoveWest {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![122u8, 187u8, 56u8, 38u8, 248u8, 122u8, 182u8, 106u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5e554BrmQN7a2nbKrSUUxP8PMbq55rMntnkoCPmwr3Aq")
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
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
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
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    state: AccountsStorage<PdaStore>,
    state_author: AccountsStorage<KeypairStore>,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct State {
    x: u64,
    y: u64,
}
