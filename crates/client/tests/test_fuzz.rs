use anyhow::Error;
use cargo_metadata::camino::Utf8PathBuf;
use fehler::throws;
use pretty_assertions::assert_str_eq;
use trident_client::___private::ProgramData;

const PROGRAM_NAME: &str = "fuzz_example3";

#[throws]
#[tokio::test]
async fn test_snapshots_and_instructions() {
    let expanded_fuzz_example3 = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_data/expanded_source_codes/expanded_fuzz_example3.rs"
    ));

    let expected_accounts_snapshots = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_data/expected_source_codes/expected_accounts_snapshots.rs"
    ));
    let expected_fuzz_instructions_code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_data/expected_source_codes/expected_fuzz_instructions.rs"
    ));

    let mut program_path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    program_path.push_str("/tests/test_program/fuzz_example3/src/lib.rs");

    let path = Utf8PathBuf::from(program_path);

    let program_idl = trident_client::___private::parse_to_idl_program(
        PROGRAM_NAME.to_owned(),
        expanded_fuzz_example3,
    )?;

    let code = expanded_fuzz_example3.to_string();

    let program_data = ProgramData {
        code,
        path,
        program_idl,
    };

    let program_data = vec![program_data];

    let fuzzer_snapshots =
        trident_client::___private::snapshot_generator::generate_snapshots_code(&program_data)
            .unwrap();
    let fuzzer_snapshots =
        trident_client::___private::Commander::format_program_code(&fuzzer_snapshots).await?;

    let fuzz_instructions_code =
        trident_client::___private::fuzzer_generator::generate_source_code(&program_data);
    let fuzz_instructions_code =
        trident_client::___private::Commander::format_program_code(&fuzz_instructions_code).await?;

    assert_str_eq!(fuzzer_snapshots, expected_accounts_snapshots);
    assert_str_eq!(fuzz_instructions_code, expected_fuzz_instructions_code);
}
