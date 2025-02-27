use syn::parse::Result as ParseResult;
use syn::DeriveInput;

use crate::types::trident_fuzz_test_executor::TridentFuzzTestExecutor;

pub fn parse_trident_fuzz_test_executor(
    input: &DeriveInput,
) -> ParseResult<TridentFuzzTestExecutor> {
    Ok(TridentFuzzTestExecutor {
        ident: input.ident.clone(),
        generics: input.generics.params.iter().cloned().collect(),
        where_clause: input.generics.where_clause.clone(),
    })
}
