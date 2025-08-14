use anchor_lang::prelude::*;

// Unit Struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UnitStruct;

// Tuple Struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TupleStruct(u8, u16, Pubkey);

// Classic Struct
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ClassicStruct {
    pub field1: u8,
    pub field2: u16,
    pub field3: Pubkey,
}

// Struct with Lifetime Parameters
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Lifetimes<'a> {
    pub reference: &'a str,
    pub key: Pubkey,
}

// Struct with Generic Parameters
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GenericStruct<T> {
    pub value: T,
    pub key: Pubkey,
}

// Struct with Optional Fields
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OptionalFields {
    pub field1: Option<u8>,
    pub field2: Option<u16>,
    pub field3: Option<Pubkey>,
}

// Struct with Default Values
#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct DefaultStruct {
    pub field1: u8,
    pub field2: u16,
    pub field3: Pubkey,
}

// Struct with Nested Structs
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NestedStruct {
    pub inner: ClassicStruct,
    pub key: Pubkey,
}

// Simple Enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum SimpleEnum {
    Variant1,
    Variant2,
    Pubkey,
}

// Enum with Associated Data
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum DataEnum {
    Integer(i32),
    Float(f64),
    Text(String),
    Pubkey(Pubkey),
}

// Enum with Multiple Associated Values
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum MultiDataEnum {
    Pair(i32, i32),
    Triple(i32, i32, i32),
    Pubkey(Pubkey, Pubkey),
}

// Enum with Named Fields
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum NamedFieldsEnum {
    Point { x: f64, y: f64 },
    Circle { radius: f64 },
    Pubkey { pubkey1: Pubkey, pubkey2: Pubkey },
}

// Enum with Generic Parameters
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum GenericEnum<T> {
    Value(T),
    None,
}

// Enum with Lifetime Parameters
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum LifetimeEnum<'a> {
    Borrowed(&'a str),
    Owned(String),
}

// Enum with Unit Variants
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum UnitVariants {
    VariantA,
    VariantB,
    VariantC,
}
