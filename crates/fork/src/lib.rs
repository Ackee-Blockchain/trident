use bincode::deserialize;
use solana_client::rpc_client::RpcClient;
use solana_loader_v3_interface::state::UpgradeableLoaderState;
use solana_pubkey::Pubkey;
// use solana_sdk::account::Account;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use trident_config::fuzz::_FuzzForkProgram;
use trident_config::ClusterType;

// use solana_sdk::bpf_loader::ID as loader_v2_id;
// use solana_sdk::bpf_loader_deprecated::ID as loader_v1_id;
// use solana_sdk::bpf_loader_upgradeable::ID as loader_v3_id;
// use solana_sdk::loader_v4::ID as loader_v4_id;

pub const LOADER_V3_ID: &str = "BPFLoaderUpgradeab1e11111111111111111111111";

pub const LOADER_V4_ID: &str = "LoaderV411111111111111111111111111111111111";
pub const LOADER_V2_ID: &str = "BPFLoader2111111111111111111111111111111111";
pub const LOADER_V1_ID: &str = "BPFLoader1111111111111111111111111111111111";

// RPC URL constants
pub const MAINNET_RPC_URL: &str = "https://api.mainnet-beta.solana.com";
pub const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";
pub const TESTNET_RPC_URL: &str = "https://api.testnet.solana.com";

pub const CACHE_FOLDER: &str = ".cache-trident-programs";

#[derive(Debug, thiserror::Error)]
pub enum ForkError {
    #[error("RPC Error: {0}")]
    Rpc(#[from] solana_client::client_error::ClientError),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid program address: {0}")]
    InvalidAddress(String),
    #[error("Account is not a program: {0}")]
    NotAProgram(String),
    #[error("Program not found: {0}")]
    ProgramNotFound(String),
    #[error("Unsupported program loader: {0}")]
    UnsupportedLoader(String),
    #[error("Failed to deserialize program data: {0}")]
    DeserializationError(String),
    #[error("Bincode error: {0}")]
    BincodeError(#[from] bincode::Error),
}

pub struct ProgramForker {
    cache_dir: PathBuf,
}

impl ProgramForker {
    pub fn new() -> Result<Self, ForkError> {
        let cache_dir = std::env::current_dir()?.join(CACHE_FOLDER);

        // Create cache directory if it doesn't exist
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }

        // Clean up any stale lock files (from previous crashes)
        Self::cleanup_stale_locks(&cache_dir)?;

        Ok(Self { cache_dir })
    }

    /// Clean up any stale lock files that might be left from previous crashes
    fn cleanup_stale_locks(cache_dir: &PathBuf) -> Result<(), ForkError> {
        if let Ok(entries) = fs::read_dir(cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "lock" {
                        // Remove stale lock files
                        let _ = fs::remove_file(&path);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn fork_programs(
        &self,
        fork_programs: &[_FuzzForkProgram],
    ) -> Result<Vec<ForkedProgram>, ForkError> {
        let mut forked_programs = Vec::new();

        for program_config in fork_programs {
            let forked_program = self.fork_program_thread_safe(program_config)?;
            forked_programs.push(forked_program);
        }

        Ok(forked_programs)
    }

    fn fork_program_thread_safe(
        &self,
        program_config: &_FuzzForkProgram,
    ) -> Result<ForkedProgram, ForkError> {
        let address = Pubkey::from_str(&program_config.address)
            .map_err(|_| ForkError::InvalidAddress(program_config.address.clone()))?;

        let cache_file = self.cache_dir.join(format!("{}.so", address));
        let lock_file = self.cache_dir.join(format!("{}.lock", address));

        // If file exists and we don't want to overwrite, use cached version
        if !program_config.overwrite && cache_file.exists() {
            println!(
                "Thread {:?}: Using cached program: {}",
                thread::current().id(),
                address
            );
            let data = fs::read(&cache_file)?;
            return Ok(ForkedProgram { address, data });
        }

        // Thread-safe forking using file locking
        self.fork_with_lock(program_config, &cache_file, &lock_file)
    }

    fn fork_with_lock(
        &self,
        program_config: &_FuzzForkProgram,
        cache_file: &PathBuf,
        lock_file: &PathBuf,
    ) -> Result<ForkedProgram, ForkError> {
        let address = Pubkey::from_str(&program_config.address)
            .map_err(|_| ForkError::InvalidAddress(program_config.address.clone()))?;

        // Try to acquire lock
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 100; // 10 seconds maximum wait
        const RETRY_DELAY: Duration = Duration::from_millis(100);

        loop {
            // Try to create lock file exclusively
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(lock_file)
            {
                Ok(_lock_file) => {
                    // We got the lock, perform the forking
                    let result = self.fork_program_locked(program_config, cache_file);

                    // Release lock by deleting the lock file
                    let _ = fs::remove_file(lock_file);

                    return result;
                }
                Err(_) => {
                    // Lock file exists, another thread is forking
                    attempts += 1;
                    if attempts == 1 {
                        println!(
                            "Thread {:?}: Waiting for another thread to fork program: {}",
                            thread::current().id(),
                            address
                        );
                    }

                    if attempts >= MAX_ATTEMPTS {
                        return Err(ForkError::Io(std::io::Error::new(
                            std::io::ErrorKind::TimedOut,
                            format!("Timeout waiting for fork lock for program: {}", address),
                        )));
                    }

                    // Wait a bit and try again
                    thread::sleep(RETRY_DELAY);

                    // Check if the file now exists (other thread completed)
                    if cache_file.exists() {
                        println!(
                            "Thread {:?}: Using program forked by another thread: {}",
                            thread::current().id(),
                            address
                        );
                        let data = fs::read(cache_file)?;
                        return Ok(ForkedProgram { address, data });
                    }
                }
            }
        }
    }

    fn fork_program_locked(
        &self,
        program_config: &_FuzzForkProgram,
        cache_file: &PathBuf,
    ) -> Result<ForkedProgram, ForkError> {
        let address = Pubkey::from_str(&program_config.address)
            .map_err(|_| ForkError::InvalidAddress(program_config.address.clone()))?;

        // Double-check if file exists (another thread might have completed while we were waiting)
        if !program_config.overwrite && cache_file.exists() {
            let data = fs::read(cache_file)?;
            return Ok(ForkedProgram { address, data });
        }

        // Actually fork the program
        println!(
            "Thread {:?}: Forking program {} from cluster {:?}",
            thread::current().id(),
            address,
            program_config.cluster
        );

        let rpc_client = self.get_rpc_client(&program_config.cluster);

        // Get the program account
        let program_account = rpc_client
            .get_account(&address)
            .map_err(|_| ForkError::ProgramNotFound(address.to_string()))?;

        // Get the program binary data based on the loader type
        let program_data = self.get_program_binary_data(&rpc_client, &address, &program_account)?;

        // Cache the program
        fs::write(cache_file, &program_data)?;
        println!(
            "Thread {:?}: Cached program: {} ({} bytes)",
            thread::current().id(),
            address,
            program_data.len()
        );

        Ok(ForkedProgram {
            address,
            data: program_data,
        })
    }

    fn get_program_binary_data(
        &self,
        rpc_client: &RpcClient,
        program_address: &Pubkey,
        program_account: &Account,
    ) -> Result<Vec<u8>, ForkError> {
        // Check if the account is executable
        if !program_account.executable {
            return Err(ForkError::NotAProgram(program_address.to_string()));
        }

        // Get the owner (loader) of the program
        let program_owner = &program_account.owner;

        // Check which loader owns the program
        if *program_owner == loader_v3_id {
            // BPFLoaderUpgradeable (v3)
            println!("Program uses BPFLoaderUpgradeable (v3)");

            // Deserialize the program account to get the program data address
            let state: UpgradeableLoaderState =
                deserialize(&program_account.data).map_err(|e| {
                    ForkError::DeserializationError(format!(
                        "Failed to deserialize program account: {}",
                        e
                    ))
                })?;

            // Extract the program data account address
            let programdata_address = match state {
                UpgradeableLoaderState::Program {
                    programdata_address,
                } => programdata_address,
                _ => {
                    return Err(ForkError::DeserializationError(
                        "Program account does not contain a programdata address".to_string(),
                    ))
                }
            };

            println!("Program data account address: {}", programdata_address);

            // Get the program data account
            let programdata_account =
                rpc_client.get_account(&programdata_address).map_err(|_| {
                    ForkError::ProgramNotFound(format!(
                        "Program data account not found: {}",
                        programdata_address
                    ))
                })?;

            // Deserialize the program data account
            let programdata_state: UpgradeableLoaderState = deserialize(&programdata_account.data)
                .map_err(|e| {
                    ForkError::DeserializationError(format!(
                        "Failed to deserialize program data account: {}",
                        e
                    ))
                })?;

            // Extract the actual program binary
            match programdata_state {
                UpgradeableLoaderState::ProgramData {
                    slot: _,
                    upgrade_authority_address: _,
                } => {
                    // The program binary starts after the metadata
                    // We need to calculate the offset where the program binary starts
                    let offset =
                        UpgradeableLoaderState::programdata_data_offset().map_err(|e| {
                            ForkError::DeserializationError(format!(
                                "Failed to get program data offset: {:?}",
                                e
                            ))
                        })?;

                    if programdata_account.data.len() <= offset {
                        return Err(ForkError::DeserializationError(
                            "Program data account too small".to_string(),
                        ));
                    }

                    // Extract the program binary (skipping the metadata)
                    Ok(programdata_account.data[offset..].to_vec())
                }
                _ => Err(ForkError::DeserializationError(
                    "Program data account has invalid state".to_string(),
                )),
            }
        } else if *program_owner == loader_v4_id {
            // BPFLoaderV4 (v4)
            println!("Program uses BPFLoaderV4 (v4)");

            // For now, we'll just return the program account data
            // In a real implementation, you'd need to handle v4 loader properly
            // This is a placeholder implementation
            println!("Warning: BPFLoaderV4 support is experimental");

            // For v4, the program binary is stored directly in the program account
            // But we need to skip the metadata
            // This is a simplification - in a real implementation, you'd use proper deserialization

            // For now, just return the account data as is
            // TODO: Implement proper v4 loader support
            Err(ForkError::UnsupportedLoader(format!(
                "Unsupported program loader: {}",
                program_owner
            )))
        } else if *program_owner == loader_v2_id {
            // BPFLoader (v2)
            println!("Program uses BPFLoader (v2)");
            println!("Warning: BPFLoader v2 is deprecated");

            // For v2, the program binary is stored directly in the program account
            Err(ForkError::UnsupportedLoader(format!(
                "Unsupported program loader: {}",
                program_owner
            )))
        } else if *program_owner == loader_v1_id {
            // BPFLoaderDeprecated (v1)
            println!("Program uses BPFLoaderDeprecated (v1)");
            println!("Warning: BPFLoaderDeprecated v1 is deprecated");

            // For v1, the program binary is stored directly in the program account
            Err(ForkError::UnsupportedLoader(format!(
                "Unsupported program loader: {}",
                program_owner
            )))
        } else {
            // Unknown loader
            Err(ForkError::UnsupportedLoader(format!(
                "Unsupported program loader: {}",
                program_owner
            )))
        }
    }

    fn get_rpc_client(&self, cluster: &ClusterType) -> RpcClient {
        let rpc_url = match cluster {
            ClusterType::Mainnet | ClusterType::MainnetShort => MAINNET_RPC_URL,
            ClusterType::Devnet | ClusterType::DevnetShort => DEVNET_RPC_URL,
            ClusterType::Testnet | ClusterType::TestnetShort => TESTNET_RPC_URL,
            ClusterType::Localnet | ClusterType::LocalnetShort => "http://localhost:8899",
        };

        RpcClient::new(rpc_url.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct ForkedProgram {
    pub address: Pubkey,
    pub data: Vec<u8>,
}

impl Default for ProgramForker {
    fn default() -> Self {
        Self::new().expect("Failed to create ProgramForker")
    }
}

// Helper function to get or create forked programs
pub fn get_forked_programs(
    fork_programs: &[_FuzzForkProgram],
) -> Result<Vec<ForkedProgram>, ForkError> {
    let forker = ProgramForker::new()?;
    forker.fork_programs(fork_programs)
}

// Helper function to check if cache directory exists
pub fn cache_exists() -> bool {
    std::env::current_dir()
        .map(|cwd| cwd.join(CACHE_FOLDER).exists())
        .unwrap_or(false)
}

// Helper function to clear cache
pub fn clear_cache() -> Result<(), ForkError> {
    let cache_dir = std::env::current_dir()?.join(CACHE_FOLDER);
    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_forker_creation() {
        let forker = ProgramForker::new();
        assert!(forker.is_ok());

        let forker = forker.unwrap();
        assert!(forker.cache_dir.exists());
        assert!(forker.cache_dir.ends_with(CACHE_FOLDER));
    }

    #[test]
    fn test_cache_functions() {
        // Create cache
        let _forker = ProgramForker::new().unwrap();
        assert!(cache_exists());

        // Clear cache
        clear_cache().unwrap();
        assert!(!cache_exists());
    }

    #[test]
    fn test_cluster_rpc_urls() {
        let forker = ProgramForker::new().unwrap();

        // Test different cluster types
        let mainnet_client = forker.get_rpc_client(&ClusterType::Mainnet);
        let devnet_client = forker.get_rpc_client(&ClusterType::Devnet);
        let testnet_client = forker.get_rpc_client(&ClusterType::Testnet);

        // Just verify the clients are created (we can't test actual RPC calls without network)
        assert!(mainnet_client.url().contains("mainnet"));
        assert!(devnet_client.url().contains("devnet"));
        assert!(testnet_client.url().contains("testnet"));
    }
}
