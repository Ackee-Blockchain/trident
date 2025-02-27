use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// File containing all custom types which can be used
/// in transactions and instructions or invariant checks.
///
/// You can define your own custom types here.
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct ClassicStruct {
    field1: u8,
    field2: u16,
    field3: TridentPubkey,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct ClassicStructAccount {
    field1: u8,
    field2: u16,
    field3: TridentPubkey,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
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
    Pubkey(TridentPubkey),
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct DefaultStruct {
    field1: u8,
    field2: u16,
    field3: TridentPubkey,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum GenericEnum {
    Value(T),
    None,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct GenericStruct {
    value: T,
    key: TridentPubkey,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum MultiDataEnum {
    Pair(i32, i32),
    Triple(i32, i32, i32),
    Pubkey(TridentPubkey, TridentPubkey),
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
        pubkey1: TridentPubkey,
        pubkey2: TridentPubkey,
    },
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct NestedStruct {
    inner: ClassicStruct,
    key: TridentPubkey,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct NestedStructAccount {
    inner: ClassicStructAccount,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct OptionalFields {
    field1: Option<u8>,
    field2: Option<u16>,
    field3: Option<TridentPubkey>,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct OptionalFieldsAccount {
    field1: Option<u8>,
    field2: Option<u16>,
    field3: Option<TridentPubkey>,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum SimpleEnum {
    Variant1,
    Variant2,
    Pubkey,
}
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct TupleStruct(pub u8, pub u16, pub TridentPubkey);
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct TupleStructAccount(pub u8, pub u16, pub TridentPubkey);
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct UnitStruct;
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct UnitStructAccount;
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub enum UnitVariants {
    VariantA,
    VariantB,
    VariantC,
}
