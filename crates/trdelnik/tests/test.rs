use anyhow::Error;
use fehler::throws;
use pretty_assertions::assert_str_eq;

#[throws]
#[tokio::test]
pub async fn generate_program_client() {
    // Generate with this command:
    // `trdelnik/examples/state_machine/programs/turnstile$ cargo expand > turnstile_expanded.rs`
    // and the content copy to `test_data/expanded_anchor_program.rs`
    let expanded_anchor_program = include_str!("test_data/expanded_anchor_program.rs");

    // You can copy the content from the `program_client` crate from an example
    // after you've called `makers trdelnik test`.
    let expected_client_code = include_str!("test_data/expected_client_code.rs");

    let program_idl =
        trdelnik::idl::parse_to_idl_program("turnstile".to_owned(), expanded_anchor_program)
            .await?;
    let idl = trdelnik::idl::Idl {
        programs: vec![program_idl],
    };

    let client_code = trdelnik::program_client_generator::generate_source_code(idl);
    let client_code = trdelnik::Commander::format_program_code(&client_code).await?;

    assert_str_eq!(client_code, expected_client_code);
}
