use anyhow::Error;
use clap::Subcommand;
use fehler::throws;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use trdelnik_explorer::display::DisplayFormat;

mod account;
mod program;
mod transaction;

#[derive(Subcommand)]
pub enum ExplorerCommand {
    /// Show the contents of an account
    Account {
        /// Ed25519 pubkey, PDA or hash of a pubkey
        pubkey: Pubkey,
        /// Pretty-printed JSON output
        #[clap(long = "json-pretty", conflicts_with = "json")]
        jsonpretty: bool,
        /// JSON output
        #[clap(long, conflicts_with = "jsonpretty")]
        json: bool,
    },
    /// Show the details of a program
    Program {
        /// Address of a program to show
        pubkey: Pubkey,
        /// Pretty-printed JSON output
        #[clap(long = "json-pretty", conflicts_with = "json")]
        jsonpretty: bool,
        /// JSON output
        #[clap(long, conflicts_with = "jsonpretty")]
        json: bool,
    },
    /// Show the contents of a transaction
    Transaction {
        /// Signature of a transaction
        signature: Signature,
        /// Raw transaction without interpretation
        #[clap(short, long)]
        raw: bool,
        /// Pretty-printed JSON output
        #[clap(long = "json-pretty", conflicts_with = "json")]
        jsonpretty: bool,
        /// JSON output
        #[clap(long, conflicts_with = "jsonpretty")]
        json: bool,
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
                account::view(pubkey, DisplayFormat::JSONPretty).await?
            } else if json {
                account::view(pubkey, DisplayFormat::JSON).await?
            } else {
                account::view(pubkey, DisplayFormat::Cli).await?
            }
        }
        ExplorerCommand::Program {
            pubkey,
            jsonpretty,
            json,
        } => {
            if jsonpretty {
                program::view(pubkey, DisplayFormat::JSONPretty).await?
            } else if json {
                program::view(pubkey, DisplayFormat::JSON).await?
            } else {
                program::view(pubkey, DisplayFormat::Cli).await?
            }
        }
        ExplorerCommand::Transaction {
            signature,
            raw,
            jsonpretty,
            json,
        } => {
            if jsonpretty {
                transaction::view(signature, raw, DisplayFormat::JSONPretty).await?
            } else if json {
                transaction::view(signature, raw, DisplayFormat::JSON).await?
            } else {
                transaction::view(signature, raw, DisplayFormat::Cli).await?
            }
        }
    }
}
