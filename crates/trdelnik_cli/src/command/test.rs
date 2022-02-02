use anyhow::Error;
use fehler::throws;
use trdelnik::*;

#[throws]
pub async fn test(root: String) {
    let commander = Commander::with_root(root);
    commander.build_programs().await?;
    commander.generate_program_client_lib_rs().await?;
    commander.run_tests().await?;
    println!("Trdelnik CLI: Test command finished");
}

