use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;

/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct ClassicStruct {
    pub field1: u8,

    pub field2: u16,

    pub field3: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct ClassicStructAccount {
    pub field1: u8,

    pub field2: u16,

    pub field3: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct DataAccount {
    pub unit_struct: UnitStruct,

    pub tuple_struct: TupleStruct,

    pub classic_struct: ClassicStruct,

    pub generic_struct: GenericStruct,

    pub optional_fields: OptionalFields,

    pub default_struct: DefaultStruct,

    pub nested_struct: NestedStruct,

    pub simple_enum: SimpleEnum,

    pub data_enum: DataEnum,

    pub multi_data_enum: MultiDataEnum,

    pub named_fields_enum: NamedFieldsEnum,

    pub generic_enum: GenericEnum,

    pub unit_variants: UnitVariants,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub enum DataEnum {
    Integer(i32),

    Float(f64),

    Text(String),

    Pubkey(TridentPubkey),
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct DefaultStruct {
    pub field1: u8,

    pub field2: u16,

    pub field3: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub enum GenericEnum {
    Value(T),

    None,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct GenericStruct {
    pub value: T,

    pub key: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub enum MultiDataEnum {
    Pair(i32, i32),

    Triple(i32, i32, i32),

    Pubkey(TridentPubkey, TridentPubkey),
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub enum NamedFieldsEnum {
    Point {
        x: f64,

        y: f64,
    },

    Circle {
        radius: f64,
    },

    Pubkey {
        pubkey1: TridentPubkey,

        pubkey2: TridentPubkey,
    },
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct NestedStruct {
    pub inner: ClassicStruct,

    pub key: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct NestedStructAccount {
    pub inner: ClassicStructAccount,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct OptionalFields {
    pub field1: Option<u8>,

    pub field2: Option<u16>,

    pub field3: Option<TridentPubkey>,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct OptionalFieldsAccount {
    pub field1: Option<u8>,

    pub field2: Option<u16>,

    pub field3: Option<TridentPubkey>,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub enum SimpleEnum {
    #[default]
    Variant1,

    Variant2,

    Pubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct TupleStruct {
    pub field_0: u8,

    pub field_1: u16,

    pub field_2: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct TupleStructAccount {
    pub field_0: u8,

    pub field_1: u16,

    pub field_2: TridentPubkey,
}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct UnitStruct {}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub struct UnitStructAccount {}

#[derive(Debug, BorshDeserialize, BorshSerialize, Clone, Default)]
pub enum UnitVariants {
    #[default]
    VariantA,

    VariantB,

    VariantC,
}
