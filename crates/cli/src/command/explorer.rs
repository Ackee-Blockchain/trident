use anyhow::Error;
use clap::Subcommand;
use fehler::throws;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use trdelnik_explorer::display::{AccountDisplayFormat, ProgramDisplayFormat};

mod account;
mod program;
mod transaction;

#[derive(Subcommand)]
pub enum ExplorerCommand {
    /// Show the contents of an account
    Account {
        /// Ed25519 pubkey, PDA or hash of a pubkey
        pubkey: Pubkey,
        #[clap(long = "json-pretty")]
        jsonpretty: bool,
        #[clap(long, conflicts_with = "jsonpretty")]
        json: bool,
    },
    Program {
        /// Address of a program to show
        pubkey: Pubkey,
        #[clap(long = "json-pretty")]
        jsonpretty: bool,
        #[clap(long, conflicts_with = "jsonpretty")]
        json: bool,
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
        ExplorerCommand::Account {
            pubkey,
            jsonpretty,
            json,
        } => {
            if jsonpretty {
                account::view(pubkey, AccountDisplayFormat::JSONPretty).await?
            } else if json {
                account::view(pubkey, AccountDisplayFormat::JSON).await?
            } else {
                account::view(pubkey, AccountDisplayFormat::Trdelnik).await?
            }
        }
        ExplorerCommand::Program {
            pubkey,
            jsonpretty,
            json,
        } => {
            if jsonpretty {
                program::view(pubkey, ProgramDisplayFormat::JSONPretty).await?
            } else if json {
                program::view(pubkey, ProgramDisplayFormat::JSON).await?
            } else {
                program::view(pubkey, ProgramDisplayFormat::Trdelnik).await?
            }
        }
        ExplorerCommand::Transaction { signature } => transaction::view(signature).await?,
    }
}
