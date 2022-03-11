use solana_cli_config::Config;
use solana_client::nonblocking::rpc_client::RpcClient;

struct ExplorerConfig {
    solana_config: Config,
    rpc_client: RpcClient,
    verbose: bool,
    colored: bool,
}

// impl Config {
//     fn new() {
//         let config = {
//             let cli_config = if let Some(config_file) = matches.value_of("config_file") {
//                 solana_cli_config::Config::load(config_file).unwrap_or_default()
//             } else {
//                 solana_cli_config::Config::default()
//             };

//             Config {
//                 json_rpc_url: matches
//                     .value_of("json_rpc_url")
//                     .unwrap_or(&cli_config.json_rpc_url)
//                     .to_string(),
//                 keypair: read_keypair_file(
//                     matches
//                         .value_of("keypair")
//                         .unwrap_or(&cli_config.keypair_path),
//                 )?,
//                 verbose: matches.is_present("verbose"),
//             }
//         };
//     }
// }
