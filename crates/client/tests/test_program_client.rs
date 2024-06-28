use anyhow::Error;
use fehler::throws;
use pretty_assertions::assert_str_eq;
use trident_client::___private::ProgramData;

#[throws]
#[tokio::test]
pub async fn generate_program_client() {
    // Generate with this command:
    // `trident/examples/escrow/programs/escrow$ cargo expand > escrow_expanded.rs`
    // and the content copy to `test_data/expanded_escrow.rs`
    let code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_data/expanded_source_codes/expanded_escrow.rs"
    ));

    // for this test we do not need path
    let path = String::default();

    // You can copy the content from the `program_client` crate from an example
    // after you've called `makers trident test`.
    let expected_client_code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_data/expected_source_codes/expected_program_client_code.rs"
    ));

    let program_idl = trident_client::___private::parse_to_idl_program("escrow".to_owned(), code)?;

    let program_data = ProgramData {
        code: code.to_string(),
        path: path.into(),
        program_idl,
    };
    let program_data = vec![program_data];

    let use_modules: Vec<syn::ItemUse> = vec![syn::parse_quote! { use trident_client::*; }];
    let client_code = trident_client::___private::program_client_generator::generate_source_code(
        &program_data,
        &use_modules,
    );
    let client_code =
        trident_client::___private::Commander::format_program_code(&client_code).await?;

    assert_str_eq!(client_code, expected_client_code);
}
