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
async fn test_fuzz_instructions() {
    let expected_fuzz_instructions_code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expected_source_codes/expected_fuzz_instructions.rs"
    ));

    let idl_2 = read_idl("dummy_2.json")?;
    let idl_dummy = read_idl("dummy_example.json")?;

    let fuzz_instructions_code =
        trident_client::___private::fuzz_instructions_generator::generate_source_code(&vec![
            idl_2, idl_dummy,
        ]);

    let fuzz_instructions_code =
        trident_client::___private::Commander::format_program_code_nightly(&fuzz_instructions_code)
            .await?;

    assert_str_eq!(fuzz_instructions_code, expected_fuzz_instructions_code);
}

#[throws]
#[tokio::test]
async fn test_fuzz_test() {
    let test_fuzz_expected = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/expected_source_codes/expected_test_fuzz.rs"
    ));

    let idl_2 = read_idl("dummy_2.json")?;
    let idl_dummy = read_idl("dummy_example.json")?;

    let test_fuzz = trident_client::___private::test_fuzz_generator::generate_source_code(&vec![
        idl_2, idl_dummy,
    ]);

    let test_fuzz =
        trident_client::___private::Commander::format_program_code_nightly(&test_fuzz).await?;

    assert_str_eq!(test_fuzz, test_fuzz_expected);
}

#[throws]
fn read_idl(_idl_name: &str) -> Idl {
    let current_dir = std::env::current_dir()?;

    let anchor_idl_path: PathBuf = [
        current_dir.as_ref(),
        Path::new(&format!("tests/anchor_idl/{}", _idl_name)),
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
