use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(FuzzDeserialize)]
pub fn fuzz_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fuzz_deser_impls = match &input.data {
        Data::Enum(enum_data) => {
            let fuzz_deser_impl = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;
                let snapshot_name = format_ident!("{}Snapshot", variant_name);
                quote! {
                    impl<'info> FuzzDeserialize<'info> for #variant_name {
                        type Ix = #snapshot_name<'info>;
                        fn deserialize_option(
                            &self,
                            _program_id: &anchor_lang::prelude::Pubkey,
                            accounts: &'info mut [Option<AccountInfo<'info>>],
                        ) -> Result<Self::Ix, FuzzingError> {
                            Self::Ix::deserialize_option(_program_id,accounts)
                        }
                    }
                }
            });

            quote! { #(#fuzz_deser_impl)* }
        }
        _ => panic!("DisplayIx can only be derived for enums"),
    };

    TokenStream::from(fuzz_deser_impls)
}
