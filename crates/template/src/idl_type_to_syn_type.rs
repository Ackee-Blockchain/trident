use quote::format_ident;
use syn::parse_quote;
use trident_idl_spec::{DefinedType, IdlArrayLen, IdlType};

pub(crate) fn idl_type_to_syn_type(idl_type: &IdlType) -> (syn::Type, bool) {
    match idl_type {
        IdlType::Bool => (parse_quote!(bool), false),
        IdlType::U8 => (parse_quote!(u8), false),
        IdlType::I8 => (parse_quote!(i8), false),
        IdlType::U16 => (parse_quote!(u16), false),
        IdlType::I16 => (parse_quote!(i16), false),
        IdlType::U32 => (parse_quote!(u32), false),
        IdlType::I32 => (parse_quote!(i32), false),
        IdlType::F32 => (parse_quote!(f32), false),
        IdlType::U64 => (parse_quote!(u64), false),
        IdlType::I64 => (parse_quote!(i64), false),
        IdlType::F64 => (parse_quote!(f64), false),
        IdlType::U128 => (parse_quote!(u128), false),
        IdlType::I128 => (parse_quote!(i128), false),
        IdlType::U256 => (parse_quote!(u256), false), // Assuming custom type for u256
        IdlType::I256 => (parse_quote!(i256), false), // Assuming custom type for i256
        IdlType::Bytes => (parse_quote!(Vec<u8>), false),
        IdlType::String => (parse_quote!(String), false),
        IdlType::Pubkey | IdlType::PublicKey => (parse_quote!(TridentPubkey), false),
        IdlType::Option(inner) => {
            let (inner_type, is_custom) = idl_type_to_syn_type(inner);
            (parse_quote!(Option<#inner_type>), is_custom)
        }
        IdlType::Vec(inner) => {
            let (inner_type, is_custom) = idl_type_to_syn_type(inner);
            (parse_quote!(Vec<#inner_type>), is_custom)
        }
        IdlType::Array(inner, len) => {
            let (inner_type, is_custom) = idl_type_to_syn_type(inner);
            let len = match len {
                IdlArrayLen::Generic(_generic) => {
                    panic!("Generic within Array len not supported")
                }
                IdlArrayLen::Value(len) => len,
            };
            (parse_quote!([#inner_type;#len]), is_custom)
        }
        // Handle defined types
        IdlType::Defined(inner) => match inner {
            DefinedType::Simple(name) => {
                let name_ident: syn::Ident = format_ident!("{}", &name);
                (parse_quote!(#name_ident), true)
            }
            DefinedType::Complex { name, generics: _ } => {
                let name_ident: syn::Ident = format_ident!("{}", &name);
                (parse_quote!(#name_ident), true)
            }
        },
        // Handle generic types
        IdlType::Generic(name) => {
            let name_ident: syn::Ident = format_ident!("{}", name);
            (parse_quote!(#name_ident), true)
        }
        _ => panic!("We reached the end :("),
    }
}
