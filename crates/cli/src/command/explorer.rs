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
        #[clap(long = "hide-lamports")]
        /// Hide lamports in the output
        hidelamports: bool,
        /// Hide data in the output
        #[clap(long = "hide-data")]
        hidedata: bool,
        #[clap(long = "hide-owner")]
        /// Hide owner in the output
        hideowner: bool,
        #[clap(long = "hide-executable")]
        /// Hide executable in the output
        hideexecutable: bool,
        /// Hide rent epoch in the output
        #[clap(long = "hide-rent-epoch")]
        hiderentepoch: bool,
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
        /// Hide program account in the output
        #[clap(long = "hide-program-account")]
        hideprogramaccount: bool,
        /// Hide programdata account in the output
        #[clap(long = "hide-programdata-account")]
        hideprogramdataaccount: bool,
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
        /// Hide overview in the output
        #[clap(long = "hide-overview")]
        hideoverview: bool,
        /// Hide transaction content in the output
        #[clap(long = "hide-transaction")]
        hidetransaction: bool,
        /// Hide log messages in the output
        #[clap(long = "hide-log-messages", conflicts_with = "raw")]
        hidelogmessages: bool,
    },
}

#[throws]
pub async fn explorer(subcmd: ExplorerCommand) {
    match subcmd {
        ExplorerCommand::Account {
            pubkey,
            jsonpretty,
            json,
            hidelamports,
            hidedata,
            hideowner,
            hideexecutable,
            hiderentepoch,
        } => {
            if jsonpretty {
                account::view(
                    pubkey,
                    hidelamports,
                    hidedata,
                    hideowner,
                    hideexecutable,
                    hiderentepoch,
                    DisplayFormat::JSONPretty,
                )
                .await?
            } else if json {
                account::view(
                    pubkey,
                    hidelamports,
                    hidedata,
                    hideowner,
                    hideexecutable,
                    hiderentepoch,
                    DisplayFormat::JSON,
                )
                .await?
            } else {
                account::view(
                    pubkey,
                    hidelamports,
                    hidedata,
                    hideowner,
                    hideexecutable,
                    hiderentepoch,
                    DisplayFormat::Cli,
                )
                .await?
            }
        }
        ExplorerCommand::Program {
            pubkey,
            jsonpretty,
            json,
            hideprogramaccount,
            hideprogramdataaccount,
        } => {
            if jsonpretty {
                program::view(
                    pubkey,
                    hideprogramaccount,
                    hideprogramdataaccount,
                    DisplayFormat::JSONPretty,
                )
                .await?
            } else if json {
                program::view(
                    pubkey,
                    hideprogramaccount,
                    hideprogramdataaccount,
                    DisplayFormat::JSON,
                )
                .await?
            } else {
                program::view(
                    pubkey,
                    hideprogramaccount,
                    hideprogramdataaccount,
                    DisplayFormat::Cli,
                )
                .await?
            }
        }
        ExplorerCommand::Transaction {
            signature,
            raw,
            jsonpretty,
            json,
            hideoverview,
            hidetransaction,
            hidelogmessages,
        } => {
            if jsonpretty {
                transaction::view(
                    signature,
                    raw,
                    hideoverview,
                    hidetransaction,
                    hidelogmessages,
                    DisplayFormat::JSONPretty,
                )
                .await?
            } else if json {
                transaction::view(
                    signature,
                    raw,
                    hideoverview,
                    hidetransaction,
                    hidelogmessages,
                    DisplayFormat::JSON,
                )
                .await?
            } else {
                transaction::view(
                    signature,
                    raw,
                    hideoverview,
                    hidetransaction,
                    hidelogmessages,
                    DisplayFormat::Cli,
                )
                .await?
            }
        }
    }
}
