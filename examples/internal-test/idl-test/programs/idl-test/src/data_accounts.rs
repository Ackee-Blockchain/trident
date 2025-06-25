use crate::types::*;
use anchor_lang::prelude::*;

#[account]
pub struct DataAccount {
    pub unit_struct: UnitStruct,
    pub tuple_struct: TupleStruct,
    pub classic_struct: ClassicStruct,
    // pub lifetimes: Lifetimes<'static>,
    pub generic_struct: GenericStruct<u8>,
    pub optional_fields: OptionalFields,
    pub default_struct: DefaultStruct,
    pub nested_struct: NestedStruct,
    pub simple_enum: SimpleEnum,
    pub data_enum: DataEnum,
    pub multi_data_enum: MultiDataEnum,
    pub named_fields_enum: NamedFieldsEnum,
    pub generic_enum: GenericEnum<u8>,
    // pub lifetime_enum: LifetimeEnum<'static>,
    pub unit_variants: UnitVariants,
}

// Adding #[account] to all structs
#[account]
pub struct UnitStructAccount;

#[account]
pub struct TupleStructAccount(u8, u16, Pubkey);

#[account]
pub struct ClassicStructAccount {
    pub field1: u8,
    pub field2: u16,
    pub field3: Pubkey,
}

// #[account]
// pub struct Lifetimes<'a> {
//     pub reference: &'a str,
// }

// #[account]
// pub struct GenericStruct<T> {
//     pub value: T,
// }

#[account]
pub struct OptionalFieldsAccount {
    pub field1: Option<u8>,
    pub field2: Option<u16>,
    pub field3: Option<Pubkey>,
}

#[account]
pub struct NestedStructAccount {
    pub inner: ClassicStructAccount,
}
