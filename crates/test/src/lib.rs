pub mod client;
mod reader;
pub mod temp_clone;

mod constants {
    // client
    pub const RETRY_LOCALNET_EVERY_MILLIS: u64 = 500;
    pub const DEFAULT_KEYPAIR_PATH: &str = "~/.config/solana/id.json";
}
