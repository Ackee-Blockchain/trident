use anyhow::Error;
use clap::Subcommand;
use fehler::throws;
use solana_sdk::{pubkey::Pubkey, signature::Signature};

mod account;
mod program;
mod transaction;

#[derive(Subcommand)]
pub enum ExplorerCommand {
    /// Show the contents of an account
    Account {
        /// Ed25519 pubkey, PDA or hash of a pubkey
        pubkey: Pubkey,
    },
    Program {
        /// Address of a program to show
        pubkey: Pubkey,
    },
    /// Show the contents of a transaction
    Transaction {
        /// Signature of a transaction
        signature: Signature,
    },
}

#[throws]
pub async fn explorer(subcmd: ExplorerCommand) {
    match subcmd {
        ExplorerCommand::Account { pubkey } => account::view(pubkey).await?,
        ExplorerCommand::Program { pubkey } => program::view(pubkey).await?,
        ExplorerCommand::Transaction { signature } => transaction::view(signature).await?,
    }
}
