use std::{
    io::Read,
    path::{Path, PathBuf},
};

use anchor_lang_idl_spec::Idl;
use anyhow::Error;
use fehler::throws;
use pretty_assertions::assert_str_eq;

#[throws]
#[tokio::test]
pub async fn generate_program_client() {
    let expected_client_code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expected_source_codes/expected_program_client_code.rs"
    ));

    let idl = read_idl()?;

    let mut use_tokens: Vec<syn::ItemUse> = vec![];

    use_tokens.push(syn::parse_quote! {use trident_client::prelude::*;});
    use_tokens.push(syn::parse_quote! {use trident_client::test::*;});

    let client_code = trident_client::___private::program_client_generator::generate_source_code(
        &vec![idl],
        &use_tokens,
    );
    let client_code =
        trident_client::___private::Commander::format_program_code(&client_code).await?;

    assert_str_eq!(client_code, expected_client_code);
}

#[throws]
fn read_idl() -> Idl {
    let current_dir = std::env::current_dir()?;

    let anchor_idl_path: PathBuf = [
        current_dir.as_ref(),
        Path::new("tests/anchor_idl/example.json"),
    ]
    .iter()
    .collect();

    let mut idl_file = std::fs::File::open(&anchor_idl_path)?;

    let mut json_content = String::new();
    idl_file.read_to_string(&mut json_content)?;

    // Parse the string of data into an Idl struct
    match serde_json::from_str::<Idl>(&json_content) {
        Ok(parsed_idl) => parsed_idl,
        Err(e) => {
            panic!("Failed to parse {}: {}", anchor_idl_path.display(), e);
            // Continue to the next file on failure
        }
    }
}
