use anchor_lang::prelude::*;

mod data_accounts;
mod types;

use crate::data_accounts::*;
use crate::types::*;

declare_id!("HtD1eaPZ1JqtxcirNtYt3aAhUMoJWZ2Ddtzu4NDZCrhN");

#[program]
pub mod idl_test {
    use super::*;
    use types::DataEnum;

    #[allow(clippy::too_many_arguments)]
    pub fn process_rust_types(
        _ctx: Context<Initialize>,
        _input_u8: u8,
        _input_u16: u16,
        _input_u32: u32,
        _input_u64: u64,
        _input_i8: i8,
        _input_i16: i16,
        _input_i32: i32,
        _input_i64: i64,
        _input_i128: i128,
        _input_f32: f32,
        _input_f64: f64,
        _input_string: String,
        _input_vec: Vec<u8>,
        _input_vec_string: Vec<String>,
        _input_bool: bool,
    ) -> Result<()> {
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn process_custom_types(
        _ctx: Context<Initialize>,
        _input_classic: ClassicStruct,
        _input_optional: OptionalFields,
        _input_tuple: TupleStruct,
        _input_enum: SimpleEnum,
        _input_data_enum: DataEnum,
        _input_multi_data_enum: MultiDataEnum,
        _input_named_fields_enum: NamedFieldsEnum,
        _input_generic_enum: GenericEnum<u8>,
        // _input_lifetime_enum: LifetimeEnum,
        _input_unit_variants: UnitVariants,
        _input_nested: NestedStruct,
        _input_default: DefaultStruct,
        // _input_lifetimes: Lifetimes<'static>,
        _input_generic_struct: GenericStruct<u32>,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub composite_account_nested: NestedInitialize<'info>,
    pub signer: Signer<'info>,
    pub data_account_1: Account<'info, DataAccount>,
    pub data_account_2: Account<'info, UnitStructAccount>,
    pub data_account_3: Account<'info, TupleStructAccount>,
    pub data_account_4: Account<'info, ClassicStructAccount>,
    pub data_account_5: Account<'info, NestedStructAccount>,
    pub data_account_6: Account<'info, OptionalFieldsAccount>,
    pub composite_account: InnerInitialize<'info>,
}

#[derive(Accounts)]
pub struct InnerInitialize<'info> {
    /// CHECK: we test here
    pub some_account: AccountInfo<'info>,
    pub signer: Signer<'info>,
    pub data_account_1: Account<'info, DataAccount>,
}

#[derive(Accounts)]
pub struct NestedInitialize<'info> {
    /// CHECK: we test here
    pub some_account: AccountInfo<'info>,
    pub nested_inner: NestedInnerInitialize<'info>,
}

#[derive(Accounts)]
pub struct NestedInnerInitialize<'info> {
    /// CHECK: we test here
    pub some_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
