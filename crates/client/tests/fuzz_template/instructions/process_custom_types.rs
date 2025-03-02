use crate::fuzz_transactions::FuzzAccounts;
use crate::types::*;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN")]
# [discriminator ([37u8 , 23u8 , 242u8 , 88u8 , 134u8 , 197u8 , 190u8 , 108u8 ,])]
pub struct ProcessCustomTypesInstruction {
    pub accounts: ProcessCustomTypesInstructionAccounts,
    pub data: ProcessCustomTypesInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(ProcessCustomTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct ProcessCustomTypesInstructionAccounts {
    composite_account_nested: CompositeAccountNestedAccounts,
    #[account(signer)]
    signer: TridentAccount,
    data_account_1: TridentAccount,
    data_account_2: TridentAccount,
    data_account_3: TridentAccount,
    data_account_4: TridentAccount,
    data_account_5: TridentAccount,
    data_account_6: TridentAccount,
    composite_account: CompositeAccountAccounts,
}
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(ProcessCustomTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct NestedInnerAccounts {
    some_account: TridentAccount,
    #[account(address = "11111111111111111111111111111111")]
    system_program: TridentAccount,
}
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(ProcessCustomTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct CompositeAccountNestedAccounts {
    some_account: TridentAccount,
    nested_inner: NestedInnerAccounts,
}
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(ProcessCustomTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct CompositeAccountAccounts {
    some_account: TridentAccount,
    #[account(signer)]
    signer: TridentAccount,
    data_account_1: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct ProcessCustomTypesInstructionData {
    _input_classic: ClassicStruct,
    _input_optional: OptionalFields,
    _input_tuple: TupleStruct,
    _input_enum: SimpleEnum,
    _input_data_enum: DataEnum,
    _input_multi_data_enum: MultiDataEnum,
    _input_named_fields_enum: NamedFieldsEnum,
    _input_generic_enum: GenericEnum,
    _input_unit_variants: UnitVariants,
    _input_nested: NestedStruct,
    _input_default: DefaultStruct,
    _input_generic_struct: GenericStruct,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
///
/// Docs: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/
impl InstructionHooks for ProcessCustomTypesInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_data(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        todo!()
    }
}
