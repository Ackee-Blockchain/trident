use solana_cli_config::{Config, CONFIG_FILE};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

pub struct ExplorerConfig {
    json_rpc_url: String,
    rpc_client: RpcClient,
}

impl ExplorerConfig {
    pub fn new() -> Self {
        let json_rpc_url = if let Some(ref config_file) = *CONFIG_FILE {
            Config::load(config_file).unwrap_or_default().json_rpc_url
        } else {
            Config::default().json_rpc_url
        };

        let rpc_client =
            RpcClient::new_with_commitment(json_rpc_url.clone(), CommitmentConfig::confirmed());

        setup_logging(LogLevel::DEBUG);

        ExplorerConfig {
            json_rpc_url,
            rpc_client,
        }
    }

    pub fn json_rpc_url(&self) -> &String {
        &self.json_rpc_url
    }

    pub fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }
}

impl Default for ExplorerConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub enum LogLevel {
    ERROR,
    WARN,
    INFO,
    DEBUG,
    TRACE,
}

pub fn setup_logging(level: LogLevel) {
    match level {
        LogLevel::ERROR => solana_logger::setup_with_default("error"),
        LogLevel::WARN => solana_logger::setup_with_default("warn"),
        LogLevel::INFO => solana_logger::setup_with_default("info"),
        LogLevel::DEBUG => solana_logger::setup_with_default("debug"),
        LogLevel::TRACE => solana_logger::setup_with_default("trace"),
    }
}

pub fn reset_logging() {
    setup_logging(LogLevel::ERROR);
}
