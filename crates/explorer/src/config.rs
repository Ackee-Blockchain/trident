use solana_cli_config::{Config, CONFIG_FILE};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

pub struct ExplorerConfig {
    json_rpc_url: String,
    rpc_client: RpcClient,
    verbose: bool,
    colored: bool,
}

impl ExplorerConfig {
    pub fn new() -> Self {
        let json_rpc_url = if let Some(config_file) = &*CONFIG_FILE {
            Config::load(config_file).unwrap_or_default().json_rpc_url
        } else {
            Config::default().json_rpc_url
        };

        let rpc_client =
            RpcClient::new_with_commitment(json_rpc_url.clone(), CommitmentConfig::confirmed());

        solana_logger::setup_with("solana=debug");

        ExplorerConfig {
            json_rpc_url,
            rpc_client,
            verbose: true,
            colored: true,
        }
    }

    pub fn json_rpc_url(&self) -> &String {
        &self.json_rpc_url
    }

    pub fn set_json_rpc_url(&mut self, new_json_rpc_url: String) {
        self.json_rpc_url = new_json_rpc_url;
        self.rpc_client = RpcClient::new_with_commitment(
            self.json_rpc_url.clone(),
            CommitmentConfig::confirmed(),
        );
    }

    pub fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }

    pub fn set_verbose(&mut self, new_verbose: bool) {
        self.verbose = new_verbose;
    }

    pub fn colored(&self) -> bool {
        self.colored
    }

    pub fn set_colored(&mut self, new_colored: bool) {
        self.colored = new_colored;
    }
}

impl Default for ExplorerConfig {
    fn default() -> Self {
        Self::new()
    }
}
