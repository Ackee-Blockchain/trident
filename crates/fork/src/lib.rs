use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

// RPC URL constants - you can fill these in later
pub const MAINNET_RPC_URL: &str = "https://api.mainnet-beta.solana.com";
pub const DEVNET_RPC_URL: &str = "https://api.devnet.solana.com";
pub const TESTNET_RPC_URL: &str = "https://api.testnet.solana.com";

pub const CACHE_FOLDER: &str = ".cache-trident-programs";

#[derive(Debug, Clone)]
pub enum ClusterType {
    Mainnet,
    Devnet,
    Testnet,
    Localnet,
}

impl std::str::FromStr for ClusterType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet" | "m" => Ok(Self::Mainnet),
            "devnet" | "d" => Ok(Self::Devnet),
            "testnet" | "t" => Ok(Self::Testnet),
            "localnet" | "l" => Ok(Self::Localnet),
            _ => Err(format!("Invalid cluster type: {}", s)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForkProgramConfig {
    pub cluster: ClusterType,
    pub address: String,
    pub overwrite: bool,
}

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
        fork_programs: &[ForkProgramConfig],
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
        program_config: &ForkProgramConfig,
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
        program_config: &ForkProgramConfig,
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
        program_config: &ForkProgramConfig,
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

        let account = rpc_client
            .get_account(&address)
            .map_err(|_| ForkError::ProgramNotFound(address.to_string()))?;

        // Verify it's a program (executable)
        if !account.executable {
            return Err(ForkError::NotAProgram(address.to_string()));
        }

        // Cache the program
        fs::write(cache_file, &account.data)?;
        println!(
            "Thread {:?}: Cached program: {} ({} bytes)",
            thread::current().id(),
            address,
            account.data.len()
        );

        Ok(ForkedProgram {
            address,
            data: account.data,
        })
    }

    fn get_rpc_client(&self, cluster: &ClusterType) -> RpcClient {
        let rpc_url = match cluster {
            ClusterType::Mainnet => MAINNET_RPC_URL,
            ClusterType::Devnet => DEVNET_RPC_URL,
            ClusterType::Testnet => TESTNET_RPC_URL,
            ClusterType::Localnet => "http://localhost:8899",
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
    fork_programs: &[ForkProgramConfig],
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
