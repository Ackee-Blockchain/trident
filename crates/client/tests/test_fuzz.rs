use anyhow::Error;
use cargo_metadata::camino::Utf8PathBuf;
use fehler::throws;
use pretty_assertions::assert_str_eq;

const PROGRAM_NAME: &str = "fuzz_example3";

#[throws]
#[tokio::test]
async fn test_fuzz_instructions() {
    let expanded_fuzz_example3 = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_data/expanded_fuzz_example3.rs"
    ));

    let expected_fuzz_instructions_code = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_data/expected_fuzz_instructions.rs"
    ));

    let program_idl =
        trdelnik_client::idl::parse_to_idl_program(PROGRAM_NAME.to_owned(), expanded_fuzz_example3)
            .await?;

    let idl = trdelnik_client::idl::Idl {
        programs: vec![program_idl],
    };

    let fuzz_instructions_code = trdelnik_client::fuzzer_generator::generate_source_code(&idl);
    let fuzz_instructions_code =
        trdelnik_client::Commander::format_program_code(&fuzz_instructions_code).await?;

    assert_str_eq!(fuzz_instructions_code, expected_fuzz_instructions_code);
}

#[throws]
#[tokio::test]
async fn test_account_snapshots() {
    let expanded_fuzz_example3 = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_data/expanded_fuzz_example3.rs"
    ));

    let expected_accounts_snapshots = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/test_data/expected_accounts_snapshots.rs"
    ));

    let mut program_path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    program_path.push_str("/tests/test_program/fuzz_example3/src/lib.rs");

    let path = Utf8PathBuf::from(program_path);

    let codes_libs_pairs = vec![(expanded_fuzz_example3.to_string(), path)];

    let fuzzer_snapshots =
        trdelnik_client::snapshot_generator::generate_snapshots_code(codes_libs_pairs).unwrap();
    let fuzzer_snapshots =
        trdelnik_client::Commander::format_program_code(&fuzzer_snapshots).await?;

    assert_str_eq!(fuzzer_snapshots, expected_accounts_snapshots);
}
#[throws]
#[tokio::test]
async fn test_fuzz_instruction_macros() {
    // this will automatically created expanded code within the same directory
    // with ".expanded.rs" extension, if the file does not exist already.
    // Do not perform any formatting command on the expanded code
    // the test will then fail
    macrotest::expand("tests/test_data/fuzzer_macros/fuzz_instructions.rs");
}

#[throws]
#[tokio::test]
async fn test_test_fuzz_macros() {
    // this will automatically created expanded code within the same directory
    // with ".expanded.rs" extension, if the file does not exist already.
    // Do not perform any formatting command on the expanded code
    // the test will then fail
    macrotest::expand("tests/test_data/fuzzer_macros/test_fuzz.rs");
}
