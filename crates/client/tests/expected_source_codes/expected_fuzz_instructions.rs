use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    ProcessCustomTypes(ProcessCustomTypes),
    ProcessRustTypes(ProcessRustTypes),
    Initialize(Initialize),
}
#[derive(Arbitrary, Debug)]
pub struct ProcessCustomTypes {
    pub accounts: ProcessCustomTypesAccounts,
    pub data: ProcessCustomTypesData,
}
#[derive(Arbitrary, Debug)]
pub struct ProcessCustomTypesAccounts {
    pub some_account: AccountId,
    pub some_account: AccountId,
    pub signer: AccountId,
    pub data_account_1: AccountId,
    pub data_account_2: AccountId,
    pub data_account_3: AccountId,
    pub data_account_4: AccountId,
    pub data_account_5: AccountId,
    pub data_account_6: AccountId,
    pub some_account: AccountId,
    pub signer: AccountId,
    pub data_account_1: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct ProcessCustomTypesData {
    pub _input_classic: ClassicStruct,
    pub _input_optional: OptionalFields,
    pub _input_tuple: TupleStruct,
    pub _input_enum: SimpleEnum,
    pub _input_data_enum: DataEnum,
    pub _input_multi_data_enum: MultiDataEnum,
    pub _input_named_fields_enum: NamedFieldsEnum,
    pub _input_generic_enum: GenericEnum,
    pub _input_unit_variants: UnitVariants,
    pub _input_nested: NestedStruct,
    pub _input_default: DefaultStruct,
    pub _input_generic_struct: GenericStruct,
}
#[derive(Arbitrary, Debug)]
pub struct ProcessRustTypes {
    pub accounts: ProcessRustTypesAccounts,
    pub data: ProcessRustTypesData,
}
#[derive(Arbitrary, Debug)]
pub struct ProcessRustTypesAccounts {
    pub some_account: AccountId,
    pub some_account: AccountId,
    pub signer: AccountId,
    pub data_account_1: AccountId,
    pub data_account_2: AccountId,
    pub data_account_3: AccountId,
    pub data_account_4: AccountId,
    pub data_account_5: AccountId,
    pub data_account_6: AccountId,
    pub some_account: AccountId,
    pub signer: AccountId,
    pub data_account_1: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct ProcessRustTypesData {
    pub _input_u8: u8,
    pub _input_u16: u16,
    pub _input_u32: u32,
    pub _input_u64: u64,
    pub _input_i8: i8,
    pub _input_i16: i16,
    pub _input_i32: i32,
    pub _input_i64: i64,
    pub _input_i128: i128,
    pub _input_f32: f32,
    pub _input_f64: f64,
    pub _input_string: String,
    pub _input_vec: Vec<u8>,
    pub _input_vec_string: Vec<String>,
    pub _input_bool: bool,
}
#[derive(Arbitrary, Debug)]
pub struct Initialize {
    pub accounts: InitializeAccounts,
    pub data: InitializeData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeAccounts {}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct InitializeData {}
///IxOps implementation for `ProcessCustomTypes` with all required functions.
impl IxOps for ProcessCustomTypes {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![37u8, 23u8, 242u8, 88u8, 134u8, 197u8, 190u8, 108u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data._input_classic).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_optional).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_tuple).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_enum).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_data_enum).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_multi_data_enum).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_named_fields_enum).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_generic_enum).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_unit_variants).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_nested).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_default).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_generic_struct).unwrap());
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
            let some_account = todo!();
            account_metas.push(todo!());
        }
        {
            let some_account = todo!();
            account_metas.push(todo!());
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new_readonly(signer.pubkey(), true));
            signers.push(signer.insecure_clone());
        }
        {
            let data_account_1 = todo!();
            account_metas.push(todo!());
        }
        {
            let data_account_2 = todo!();
            account_metas.push(todo!());
        }
        {
            let data_account_3 = todo!();
            account_metas.push(todo!());
        }
        {
            let data_account_4 = todo!();
            account_metas.push(todo!());
        }
        {
            let data_account_5 = todo!();
            account_metas.push(todo!());
        }
        {
            let data_account_6 = todo!();
            account_metas.push(todo!());
        }
        {
            let some_account = todo!();
            account_metas.push(todo!());
        }
        {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new_readonly(signer.pubkey(), true));
            signers.push(signer.insecure_clone());
        }
        {
            let data_account_1 = todo!();
            account_metas.push(todo!());
        }
        Ok((signers, account_metas))
    }
}
///IxOps implementation for `ProcessRustTypes` with all required functions.
impl IxOps for ProcessRustTypes {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![74u8, 102u8, 18u8, 245u8, 253u8, 10u8, 252u8, 246u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data._input_u8).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_u16).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_u32).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_u64).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_i8).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_i16).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_i32).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_i64).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_i128).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_f32).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_f64).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_string).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_vec).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_vec_string).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data._input_bool).unwrap());
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
            let some_account = todo!();
            account_metas.push(todo!());
        }
        {
            let some_account = todo!();
            account_metas.push(todo!());
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new_readonly(signer.pubkey(), true));
            signers.push(signer.insecure_clone());
        }
        {
            let data_account_1 = todo!();
            account_metas.push(todo!());
        }
        {
            let data_account_2 = todo!();
            account_metas.push(todo!());
        }
        {
            let data_account_3 = todo!();
            account_metas.push(todo!());
        }
        {
            let data_account_4 = todo!();
            account_metas.push(todo!());
        }
        {
            let data_account_5 = todo!();
            account_metas.push(todo!());
        }
        {
            let data_account_6 = todo!();
            account_metas.push(todo!());
        }
        {
            let some_account = todo!();
            account_metas.push(todo!());
        }
        {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new_readonly(signer.pubkey(), true));
            signers.push(signer.insecure_clone());
        }
        {
            let data_account_1 = todo!();
            account_metas.push(todo!());
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
        pubkey!("8bPSKGoWCdAW8Hu3S1hLHPpBv8BNwse4jDyaXNrj3jWB")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
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
        Ok((signers, account_metas))
    }
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    data_account_1: AccountsStorage<todo!()>,
    data_account_2: AccountsStorage<todo!()>,
    data_account_3: AccountsStorage<todo!()>,
    data_account_4: AccountsStorage<todo!()>,
    data_account_5: AccountsStorage<todo!()>,
    data_account_6: AccountsStorage<todo!()>,
    signer: AccountsStorage<KeypairStore>,
    some_account: AccountsStorage<todo!()>,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct ClassicStruct {
    field1: u8,
    field2: u16,
    field3: AccountId,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct ClassicStructAccount {
    field1: u8,
    field2: u16,
    field3: Pubkey,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct DataAccount {
    unit_struct: UnitStruct,
    tuple_struct: TupleStruct,
    classic_struct: ClassicStruct,
    generic_struct: GenericStruct,
    optional_fields: OptionalFields,
    default_struct: DefaultStruct,
    nested_struct: NestedStruct,
    simple_enum: SimpleEnum,
    data_enum: DataEnum,
    multi_data_enum: MultiDataEnum,
    named_fields_enum: NamedFieldsEnum,
    generic_enum: GenericEnum,
    unit_variants: UnitVariants,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum DataEnum {
    Integer(i32),
    Float(f64),
    Text(String),
    Pubkey(AccountId),
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct DefaultStruct {
    field1: u8,
    field2: u16,
    field3: AccountId,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum GenericEnum {
    Value(T),
    None,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct GenericStruct {
    value: T,
    key: AccountId,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum MultiDataEnum {
    Pair(i32, i32),
    Triple(i32, i32, i32),
    Pubkey(AccountId, AccountId),
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum NamedFieldsEnum {
    Point {
        x: f64,
        y: f64,
    },
    Circle {
        radius: f64,
    },
    Pubkey {
        pubkey1: AccountId,
        pubkey2: AccountId,
    },
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct NestedStruct {
    inner: ClassicStruct,
    key: AccountId,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct NestedStructAccount {
    inner: ClassicStructAccount,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct OptionalFields {
    field1: Option<u8>,
    field2: Option<u16>,
    field3: Option<AccountId>,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct OptionalFieldsAccount {
    field1: Option<u8>,
    field2: Option<u16>,
    field3: Option<Pubkey>,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum SimpleEnum {
    Variant1,
    Variant2,
    Pubkey,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
struct TupleStruct(pub u8, pub u16, pub AccountId);
#[derive(Debug, BorshDeserialize, BorshSerialize)]
struct TupleStructAccount(pub u8, pub u16, pub Pubkey);
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct UnitStruct;
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct UnitStructAccount;
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum UnitVariants {
    VariantA,
    VariantB,
    VariantC,
}
