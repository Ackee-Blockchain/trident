use crate::constants::*;
use crate::fork::CachedForkAccount;
use crate::fuzz::FuzzCluster;
use crate::fuzz::FuzzFork;
use solana_client::rpc_client::RpcClient;
use solana_sdk::account::AccountSharedData;
use solana_sdk::account::ReadableAccount;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

// ============================================================================
// RPC Client
// ============================================================================

/// Create an RPC client for the given cluster.
fn create_rpc_client(cluster: &FuzzCluster) -> RpcClient {
    let timeout = Duration::from_secs(RPC_TIMEOUT_SECS);
    RpcClient::new_with_timeout_and_commitment(
        cluster.rpc_url(),
        timeout,
        CommitmentConfig::confirmed(),
    )
}

/// Fetch a single account from RPC.
fn fetch_account(
    client: &RpcClient,
    address: &Pubkey,
    cluster: &FuzzCluster,
) -> Result<AccountSharedData, Box<dyn std::error::Error>> {
    let account = client.get_account(address).map_err(|e| {
        format!(
            "Failed to fetch account {} from {}: {}",
            address,
            cluster.as_str(),
            e
        )
    })?;
    Ok(AccountSharedData::from(account))
}

// ============================================================================
// Program Data Extraction
// ============================================================================

/// Extract program data address from a v3 upgradeable program account.
fn get_program_data_address(account: &AccountSharedData) -> Option<Pubkey> {
    use solana_loader_v3_interface::state::UpgradeableLoaderState;

    if let Ok(UpgradeableLoaderState::Program {
        programdata_address,
    }) = bincode::deserialize::<UpgradeableLoaderState>(account.data())
    {
        Some(programdata_address)
    } else {
        None
    }
}

// ============================================================================
// RPC Fetching
// ============================================================================

/// Fetch an account and all related accounts from RPC.
pub(crate) fn fetch_from_rpc(
    address: &Pubkey,
    cluster: &FuzzCluster,
) -> Result<Vec<(Pubkey, AccountSharedData)>, Box<dyn std::error::Error>> {
    eprintln!(
        "Fetching account {} from {} (timeout: {}s)...",
        address,
        cluster.as_str(),
        RPC_TIMEOUT_SECS
    );

    let client = create_rpc_client(cluster);
    let account = fetch_account(&client, address, cluster)?;
    let owner = *account.owner();

    // Handle different account types
    match owner {
        solana_sdk::bpf_loader::ID => {
            eprintln!("BPF Loader v2 not supported. Skipping: {}", address);
            Ok(vec![])
        }

        solana_sdk::bpf_loader_upgradeable::ID => {
            eprintln!("  Detected v3 upgradeable program");
            fetch_v3_program(&client, address, account, cluster)
        }

        solana_sdk::loader_v4::ID => {
            eprintln!("  Warning: v4 loader not fully supported");
            Ok(vec![(*address, account)])
        }

        _ => Ok(vec![(*address, account)]),
    }
}

/// Fetch a v3 upgradeable program and its data account.
fn fetch_v3_program(
    client: &RpcClient,
    address: &Pubkey,
    account: AccountSharedData,
    cluster: &FuzzCluster,
) -> Result<Vec<(Pubkey, AccountSharedData)>, Box<dyn std::error::Error>> {
    if let Some(data_address) = get_program_data_address(&account) {
        eprintln!("  Fetching program data: {}", data_address);
        let data_account = fetch_account(client, &data_address, cluster)?;
        Ok(vec![(*address, account), (data_address, data_account)])
    } else {
        eprintln!("  Warning: Not a program account, treating as single account");
        Ok(vec![(*address, account)])
    }
}

// ============================================================================
// Cache Operations
// ============================================================================

/// Save multiple accounts to cache.
fn cache_accounts(
    accounts: &[(Pubkey, AccountSharedData)],
    cluster: &FuzzCluster,
) -> Result<(), Box<dyn std::error::Error>> {
    for (address, account) in accounts {
        save_to_cache(address, cluster, account)?;
        eprintln!("Cached: {}", address);
    }
    Ok(())
}

/// Ensure program data account is cached for v3 programs.
fn ensure_program_data_cached(
    account: &AccountSharedData,
    cluster: &FuzzCluster,
) -> Result<(), Box<dyn std::error::Error>> {
    if *account.owner() != solana_sdk::bpf_loader_upgradeable::ID {
        return Ok(());
    }

    let Some(data_address) = get_program_data_address(account) else {
        return Ok(());
    };

    if is_cached(&data_address, cluster) {
        return Ok(());
    }

    eprintln!("  Program data not cached, fetching: {}", data_address);
    let client = create_rpc_client(cluster);
    let data_account = fetch_account(&client, &data_address, cluster)?;
    save_to_cache(&data_address, cluster, &data_account)?;
    eprintln!("Cached: {}", data_address);

    Ok(())
}

// ============================================================================
// Fork Processing
// ============================================================================

/// Fetch or load a fork, ensuring all related accounts are cached.
fn process_fork(fork: &FuzzFork) -> Result<(), Box<dyn std::error::Error>> {
    if needs_fetch(fork) {
        let accounts = fetch_from_rpc(&fork.address, &fork.cluster)?;
        cache_accounts(&accounts, &fork.cluster)?;
        eprintln!(
            "✓ Loaded {} account{} for {}",
            accounts.len(),
            if accounts.len() == 1 { "" } else { "s" },
            fork.address
        );
    } else {
        eprintln!("✓ Cache hit: {}", fork.address);

        // Verify program data is cached for v3 programs
        if let Ok(cached) = load_from_cache(&fork.address) {
            if let Ok(account) = cached.to_account_shared_data() {
                ensure_program_data_cached(&account, &fork.cluster)?;
            }
        }
    }

    Ok(())
}

/// Load a fork and all related accounts from cache.
fn load_with_related(
    address: &Pubkey,
) -> Result<Vec<(Pubkey, AccountSharedData)>, Box<dyn std::error::Error>> {
    let cached = load_from_cache(address)?;
    let account = cached.to_account_shared_data()?;
    let mut results = vec![(*address, account.clone())];

    // Load program data for v3 programs
    if *account.owner() == solana_sdk::bpf_loader_upgradeable::ID {
        if let Some(data_address) = get_program_data_address(&account) {
            if let Ok(cached_data) = load_from_cache(&data_address) {
                if let Ok(data_account) = cached_data.to_account_shared_data() {
                    results.push((data_address, data_account));
                }
            }
        }
    }

    Ok(results)
}

/// Process all forks - fetch from RPC if needed and cache everything.
/// Called once in the main thread before parallel fuzzing.
pub(crate) fn process_forks(forks: &[FuzzFork]) -> Result<(), Box<dyn std::error::Error>> {
    if forks.is_empty() {
        return Ok(());
    }

    eprintln!("\nProcessing {} fork(s)...", forks.len());

    for (idx, fork) in forks.iter().enumerate() {
        eprintln!("\n[{}/{}] {}", idx + 1, forks.len(), fork.address);

        if let Err(e) = process_fork(fork) {
            eprintln!("✗ Failed: {}", e);
            eprintln!("  Continuing...");
        }
    }

    Ok(())
}

/// Load all forks from cache (no RPC calls).
/// Called by worker threads during parallel fuzzing.
pub(crate) fn load_forks_from_cache(
    forks: &[FuzzFork],
) -> Result<Vec<(Pubkey, AccountSharedData)>, Box<dyn std::error::Error>> {
    let mut results = Vec::with_capacity(forks.len());

    for fork in forks {
        match load_with_related(&fork.address) {
            Ok(accounts) => results.extend(accounts),
            Err(e) => eprintln!("Warning: Failed to load {}: {}", fork.address, e),
        }
    }

    Ok(results)
}

// ============================================================================
// Cache Storage
// ============================================================================

fn cache_dir() -> PathBuf {
    PathBuf::from(FUZZ_ARTIFACTS_DIR).join(FORK_CACHE_DIR)
}

fn cache_path(address: &Pubkey) -> PathBuf {
    cache_dir().join(format!("{}.json", address))
}

fn ensure_cache_dir() -> Result<(), std::io::Error> {
    let dir = cache_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    Ok(())
}

fn load_from_cache(address: &Pubkey) -> Result<CachedForkAccount, Box<dyn std::error::Error>> {
    let path = cache_path(address);
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

fn save_to_cache(
    address: &Pubkey,
    cluster: &FuzzCluster,
    account: &AccountSharedData,
) -> Result<(), Box<dyn std::error::Error>> {
    ensure_cache_dir()?;
    let cached = CachedForkAccount::new(*address, cluster, account);
    let content = serde_json::to_string_pretty(&cached)?;
    fs::write(cache_path(address), content)?;
    Ok(())
}

fn is_cached(address: &Pubkey, cluster: &FuzzCluster) -> bool {
    load_from_cache(address)
        .map(|cached| cached.cluster == cluster.as_str())
        .unwrap_or(false)
}

fn needs_fetch(fork: &FuzzFork) -> bool {
    fork.overwrite || !is_cached(&fork.address, &fork.cluster)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_dir() {
        let dir = cache_dir();
        assert_eq!(dir, PathBuf::from(".fuzz-artifacts").join("fork-cache"));
    }

    #[test]
    fn test_cache_path() {
        let pubkey = Pubkey::new_unique();
        let path = cache_path(&pubkey);
        assert!(path.ends_with(format!("{}.json", pubkey)));
    }

    #[test]
    fn test_cluster_urls() {
        assert_eq!(
            FuzzCluster::Mainnet.rpc_url(),
            "https://api.mainnet-beta.solana.com"
        );
        assert_eq!(
            FuzzCluster::Devnet.rpc_url(),
            "https://api.devnet.solana.com"
        );
        assert_eq!(
            FuzzCluster::Custom("https://custom.rpc.com".to_string()).rpc_url(),
            "https://custom.rpc.com"
        );
    }

    #[test]
    fn test_cluster_parse() {
        // Full names
        assert!(matches!(
            FuzzCluster::parse("mainnet"),
            FuzzCluster::Mainnet
        ));
        assert!(matches!(FuzzCluster::parse("devnet"), FuzzCluster::Devnet));
        assert!(matches!(
            FuzzCluster::parse("testnet"),
            FuzzCluster::Testnet
        ));

        // Short versions
        assert!(matches!(FuzzCluster::parse("m"), FuzzCluster::Mainnet));
        assert!(matches!(FuzzCluster::parse("d"), FuzzCluster::Devnet));
        assert!(matches!(FuzzCluster::parse("t"), FuzzCluster::Testnet));

        // Custom
        match FuzzCluster::parse("https://custom.rpc.com") {
            FuzzCluster::Custom(url) => assert_eq!(url, "https://custom.rpc.com"),
            _ => panic!("Expected Custom cluster"),
        }
    }
}
