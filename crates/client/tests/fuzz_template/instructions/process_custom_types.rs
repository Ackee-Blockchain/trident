use crate::fuzz_accounts::FuzzAccounts;
use crate::types::*;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use trident_fuzz::fuzzing::*;

#[derive(TridentInstruction, Default)]
#[program_id("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN")]
#[discriminator([37u8, 23u8, 242u8, 88u8, 134u8, 197u8, 190u8, 108u8])]
pub struct ProcessCustomTypesInstruction {
    pub accounts: ProcessCustomTypesInstructionAccounts,
    pub data: ProcessCustomTypesInstructionData,
}

/// Instruction Accounts
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ProcessCustomTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct ProcessCustomTypesInstructionAccounts {
    pub composite_account_nested: CompositeAccountNestedAccounts,

    #[account(mut, signer)]
    pub signer: TridentAccount,

    pub data_account_1: TridentAccount,

    pub data_account_2: TridentAccount,

    pub data_account_3: TridentAccount,

    pub data_account_4: TridentAccount,

    pub data_account_5: TridentAccount,

    pub data_account_6: TridentAccount,

    pub composite_account: CompositeAccountAccounts,
}

/// Composite Account: composite_account_nested
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ProcessCustomTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct CompositeAccountNestedAccounts {
    #[account(mut)]
    pub some_account: TridentAccount,

    pub nested_inner: NestedInnerAccounts,
}

#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ProcessCustomTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct NestedInnerAccounts {
    pub some_account: TridentAccount,

    #[account(address = "11111111111111111111111111111111")]
    pub system_program: TridentAccount,
}

/// Composite Account: composite_account
#[derive(Debug, Clone, TridentAccounts, Default)]
#[instruction_data(ProcessCustomTypesInstructionData)]
#[storage(FuzzAccounts)]
pub struct CompositeAccountAccounts {
    pub some_account: TridentAccount,

    #[account(mut, signer)]
    pub signer: TridentAccount,

    pub data_account_1: TridentAccount,
}

/// Instruction Data
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
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
}
