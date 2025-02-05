use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// Instruction Struct
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN")]
# [discriminator ([37u8 , 23u8 , 242u8 , 88u8 , 134u8 , 197u8 , 190u8 , 108u8 ,])]
pub struct ProcessCustomTypesInstruction {
    pub accounts: ProcessCustomTypesInstructionAccounts,
    pub data: ProcessCustomTypesInstructionData,
}
/// Instruction Accounts
/// Instruction accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct ProcessCustomTypesInstructionAccounts {
    pub composite_account_nested: CompositeAccountNestedAccounts,
    pub signer: TridentAccount,
    pub data_account_1: TridentAccount,
    pub data_account_2: TridentAccount,
    pub data_account_3: TridentAccount,
    pub data_account_4: TridentAccount,
    pub data_account_5: TridentAccount,
    pub data_account_6: TridentAccount,
    pub composite_account: CompositeAccountAccounts,
}
/// Composite Account Structs
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct NestedInnerAccounts {
    pub some_account: TridentAccount,
    pub system_program: TridentAccount,
}
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct CompositeAccountNestedAccounts {
    pub some_account: TridentAccount,
    pub nested_inner: NestedInnerAccounts,
}
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct CompositeAccountAccounts {
    pub some_account: TridentAccount,
    pub signer: TridentAccount,
    pub data_account_1: TridentAccount,
}
/// Instruction Data
/// Instruction data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct ProcessCustomTypesInstructionData {
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
/// Instruction Setters
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionSetters for ProcessCustomTypesInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
}
