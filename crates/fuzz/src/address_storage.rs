use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;

use crate::trident::Trident;

/// A storage container for managing and tracking public key addresses
///
/// `AddressStorage` provides a convenient way to store and retrieve addresses during fuzz testing.
/// It can generate random addresses or derive PDAs, and allows you to randomly select from stored addresses.
pub struct AddressStorage {
    addresses: Vec<Pubkey>,
}

/// Seeds and program ID for deriving Program Derived Addresses (PDAs)
///
/// This structure holds the necessary information to derive a PDA using
/// `Pubkey::try_find_program_address`.
pub struct PdaSeeds<'a> {
    pub seeds: &'a [&'a [u8]],
    pub program_id: Pubkey,
}

impl<'a> PdaSeeds<'a> {
    /// Creates a new `PdaSeeds` instance
    ///
    /// # Arguments
    /// * `seeds` - The seeds to use for PDA derivation
    /// * `program_id` - The program ID to use for PDA derivation
    ///
    /// # Returns
    /// A new `PdaSeeds` instance
    pub fn new(seeds: &'a [&'a [u8]], program_id: Pubkey) -> Self {
        Self { seeds, program_id }
    }
}

/// Derives a Program Derived Address (PDA) from seeds and program ID
///
/// # Arguments
/// * `seeds` - The seeds to use for PDA derivation
/// * `program_id` - The program ID to use for PDA derivation
///
/// # Returns
/// The derived PDA if successful, or `None` if derivation fails
fn derive_pda(seeds: &[&[u8]], program_id: &Pubkey) -> Option<Pubkey> {
    if let Some((address, _)) = Pubkey::try_find_program_address(seeds, program_id) {
        Some(address)
    } else {
        None
    }
}

impl Default for AddressStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl AddressStorage {
    /// Creates a new empty `AddressStorage` instance
    ///
    /// # Returns
    /// A new `AddressStorage` with no stored addresses
    fn new() -> Self {
        let addresses: Vec<Pubkey> = Vec::new();
        Self { addresses }
    }

    /// Inserts a new address into storage
    ///
    /// Generates a new address (either a PDA or random keypair) and stores it.
    /// If PDA seeds are provided, attempts to derive a PDA. If derivation fails
    /// or no seeds are provided, generates a random keypair address.
    ///
    /// # Arguments
    /// * `trident` - The Trident instance for random number generation
    /// * `seeds` - Optional PDA seeds for deriving a program-derived address
    ///
    /// # Returns
    /// The newly created and stored address
    pub fn insert(&mut self, trident: &mut Trident, seeds: Option<PdaSeeds>) -> Pubkey {
        let address = self.get_or_create_address(seeds, trident);
        self.addresses.push(address);
        address
    }

    /// Inserts an existing address into storage
    ///
    /// Stores a pre-existing address without generating a new one.
    /// Useful when you need to track addresses created elsewhere.
    ///
    /// # Arguments
    /// * `address` - The address to store
    pub fn insert_with_address(&mut self, address: Pubkey) {
        self.addresses.push(address);
    }

    /// Retrieves a random address from storage
    ///
    /// Randomly selects one of the stored addresses using Trident's RNG.
    /// This is useful for fuzzing operations that need to work with previously
    /// created accounts.
    ///
    /// # Arguments
    /// * `trident` - The Trident instance for random number generation
    ///
    /// # Returns
    /// * `Some(Pubkey)` - A randomly selected address from storage
    /// * `None` - If the storage is empty
    pub fn get(&self, trident: &mut Trident) -> Option<Pubkey> {
        if self.is_empty() {
            return None;
        }
        let accounts_num = self.addresses.len();
        let account_id = trident.random_from_range(0..accounts_num);
        Some(self.addresses[account_id])
    }

    /// Retrieves a random address from storage, excluding specified addresses
    ///
    /// Randomly selects one of the stored addresses using Trident's RNG, ensuring
    /// the selected address is not in the exclusion list. This is useful for fuzzing
    /// operations that need distinct accounts (e.g., sender and receiver must be different).
    ///
    /// # Arguments
    /// * `trident` - The Trident instance for random number generation
    /// * `except_addresses` - Slice of addresses to exclude from selection
    ///
    /// # Returns
    /// * `Some(Pubkey)` - A randomly selected address that is not in the exclusion list
    /// * `None` - If storage is empty or all addresses are in the exclusion list
    ///
    /// # Examples
    /// ```ignore
    /// let sender = storage.get(&mut trident)?;
    /// // Get a different address for receiver
    /// let receiver = storage.get_except(&mut trident, &[sender])?;
    /// ```
    pub fn get_except(&self, trident: &mut Trident, except_addresses: &[Pubkey]) -> Option<Pubkey> {
        if self.is_empty() {
            return None;
        }

        let accounts_num = self.addresses.len();

        // If all addresses would be excluded, return None
        if except_addresses.len() >= accounts_num {
            let all_excluded = self
                .addresses
                .iter()
                .all(|addr| except_addresses.contains(addr));
            if all_excluded {
                return None;
            }
        }

        // Try to find a valid address by random sampling
        // We try up to accounts_num times to find a non-excluded address
        for _ in 0..accounts_num {
            let account_id = trident.random_from_range(0..accounts_num);
            let candidate = self.addresses[account_id];

            if !except_addresses.contains(&candidate) {
                return Some(candidate);
            }
        }

        // Fallback: if random sampling failed, do a linear search
        // This should rarely happen but ensures we return a valid address if one exists
        self.addresses
            .iter()
            .find(|addr| !except_addresses.contains(addr))
            .copied()
    }

    /// Checks if the storage is empty
    ///
    /// # Returns
    /// `true` if no addresses are stored, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.addresses.is_empty()
    }

    /// Returns the number of stored addresses
    ///
    /// # Returns
    /// The count of addresses currently in storage
    pub fn len(&self) -> usize {
        self.addresses.len()
    }

    /// Gets an existing address or creates a new one
    ///
    /// Internal helper method that either derives a PDA from the provided seeds
    /// or generates a random keypair address.
    ///
    /// # Arguments
    /// * `seeds` - Optional PDA seeds for deriving a program-derived address
    /// * `trident` - The Trident instance for random number generation
    ///
    /// # Returns
    /// A derived PDA if seeds are provided and derivation succeeds, otherwise a random address
    fn get_or_create_address(&self, seeds: Option<PdaSeeds>, trident: &mut Trident) -> Pubkey {
        match seeds {
            Some(seeds) => {
                if let Some(pubkey) = derive_pda(seeds.seeds, &seeds.program_id) {
                    pubkey
                } else {
                    let mut secret = [0; 32];
                    trident.random_bytes(&mut secret);
                    solana_sdk::signer::keypair::Keypair::new_from_array(secret).pubkey()
                }
            }
            None => {
                let mut secret = [0; 32];
                trident.random_bytes(&mut secret);
                solana_sdk::signer::keypair::Keypair::new_from_array(secret).pubkey()
            }
        }
    }
}
