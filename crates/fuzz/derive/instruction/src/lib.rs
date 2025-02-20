use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;
use trident_syn::parser::trident_instruction::parse_trident_instruction;

#[proc_macro_derive(TridentInstruction, attributes(program_id, discriminator))]
pub fn derive_trident_instruction(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);

    match parse_trident_instruction(&input) {
        Ok(instruction) => instruction.to_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
