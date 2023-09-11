use anyhow::Error;
use clap::Subcommand;
use fehler::throws;
use trdelnik_client::Cleaner;

#[derive(Subcommand)]
#[allow(non_camel_case_types)]
pub enum CleanCommand {
    /// Reverts trdelnik init -> removes .program_client, trdelnik-tests, Trdelnik.toml and updates members in Cargo.toml
    Full,
}

#[throws]
pub async fn clean(subcmd: CleanCommand) {
    let cleaner = Cleaner::new();
    match subcmd {
        CleanCommand::Full => {
            cleaner.clean_full().await?;
        }
    };
}
