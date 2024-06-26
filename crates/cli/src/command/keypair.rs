use anyhow::Error;
use clap::Subcommand;
use fehler::throws;
use solana_sdk::signer::Signer;
use trident_client::___private::{keypair as other_keypair, program_keypair, system_keypair};

#[derive(Subcommand)]
pub enum KeyPairCommand {
    Program { n: usize },
    System { n: usize },
    Other { n: usize },
}

#[throws]
pub fn keypair(subcmd: KeyPairCommand) {
    let kp = match subcmd {
        KeyPairCommand::Program { n } => program_keypair(n),
        KeyPairCommand::System { n } => system_keypair(n),
        KeyPairCommand::Other { n } => other_keypair(n),
    };

    println!("PubKey: {:?}", kp.pubkey());
    println!("KeyPair: {:?}", kp.to_bytes());
}
