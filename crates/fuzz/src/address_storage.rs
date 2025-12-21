use std::collections::HashMap;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;

use crate::trident::Trident;

/// A storage container for managing and tracking unique public key addresses
///
/// `AddressStorage` provides a convenient way to store and retrieve addresses during fuzz testing.
/// It can generate random addresses or derive PDAs, and allows you to randomly select from stored addresses.
///
/// Internally uses a `Vec` + `HashMap` combination for O(1) insert, remove, and random access.
/// Note: Insertion order is not preserved after removals due to swap-remove optimization.
pub struct AddressStorage {
    addresses: Vec<Pubkey>,
    indices: HashMap<Pubkey, usize>,
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
        Self {
            addresses: Vec::new(),
            indices: HashMap::new(),
        }
    }

    /// Inserts a new address into storage
    ///
    /// Generates a new address (either a PDA or random keypair) and stores it.
    /// If PDA seeds are provided, attempts to derive a PDA. If derivation fails
    /// or no seeds are provided, generates a random keypair address.
    /// If the address already exists, it will not be inserted again.
    ///
    /// # Arguments
    /// * `trident` - The Trident instance for random number generation
    /// * `seeds` - Optional PDA seeds for deriving a program-derived address
    ///
    /// # Returns
    /// The newly created address (or existing one if it was already stored)
    pub fn insert(&mut self, trident: &mut Trident, seeds: Option<PdaSeeds>) -> Pubkey {
        let address = self.get_or_create_address(seeds, trident);
        self.insert_unique(address);
        address
    }

    /// Inserts an existing address into storage
    ///
    /// Stores a pre-existing address without generating a new one.
    /// If the address already exists, it will not be inserted again.
    ///
    /// # Arguments
    /// * `address` - The address to store
    ///
    /// # Returns
    /// `true` if the address was inserted, `false` if it already existed
    pub fn insert_with_address(&mut self, address: Pubkey) -> bool {
        self.insert_unique(address)
    }

    /// Internal helper to insert an address if it doesn't already exist
    ///
    /// # Returns
    /// `true` if inserted, `false` if already present
    fn insert_unique(&mut self, address: Pubkey) -> bool {
        if self.indices.contains_key(&address) {
            return false;
        }
        let idx = self.addresses.len();
        self.addresses.push(address);
        self.indices.insert(address, idx);
        true
    }

    /// Removes an address from storage
    ///
    /// Uses swap-remove for O(1) removal. Note that this may change
    /// the order of elements in storage.
    ///
    /// # Arguments
    /// * `address` - The address to remove
    ///
    /// # Returns
    /// `true` if the address was removed, `false` if it wasn't found
    pub fn remove(&mut self, address: &Pubkey) -> bool {
        if let Some(idx) = self.indices.remove(address) {
            let last_idx = self.addresses.len() - 1;

            // If not removing the last element, update the swapped element's index
            if idx != last_idx {
                let swapped = self.addresses[last_idx];
                self.indices.insert(swapped, idx);
            }

            self.addresses.swap_remove(idx);
            true
        } else {
            false
        }
    }

    /// Checks if an address exists in storage
    ///
    /// # Arguments
    /// * `address` - The address to check
    ///
    /// # Returns
    /// `true` if the address is in storage, `false` otherwise
    pub fn contains(&self, address: &Pubkey) -> bool {
        self.indices.contains_key(address)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pubkey(byte: u8) -> Pubkey {
        Pubkey::new_from_array([byte; 32])
    }

    #[test]
    fn test_insert_and_contains() {
        let mut storage = AddressStorage::default();
        let addr1 = make_pubkey(1);
        let addr2 = make_pubkey(2);

        assert!(storage.insert_with_address(addr1));
        assert!(storage.insert_with_address(addr2));

        assert!(storage.contains(&addr1));
        assert!(storage.contains(&addr2));
        assert!(!storage.contains(&make_pubkey(3)));
    }

    #[test]
    fn test_insert_uniqueness() {
        let mut storage = AddressStorage::default();
        let addr = make_pubkey(1);

        assert!(storage.insert_with_address(addr));
        assert!(!storage.insert_with_address(addr)); // duplicate
        assert_eq!(storage.len(), 1);
    }

    #[test]
    fn test_remove() {
        let mut storage = AddressStorage::default();
        let addr1 = make_pubkey(1);
        let addr2 = make_pubkey(2);
        let addr3 = make_pubkey(3);

        storage.insert_with_address(addr1);
        storage.insert_with_address(addr2);
        storage.insert_with_address(addr3);

        assert_eq!(storage.len(), 3);

        assert!(storage.remove(&addr2));
        assert_eq!(storage.len(), 2);
        assert!(!storage.contains(&addr2));
        assert!(storage.contains(&addr1));
        assert!(storage.contains(&addr3));

        // Remove non-existent
        assert!(!storage.remove(&addr2));
    }

    #[test]
    fn test_remove_last_element() {
        let mut storage = AddressStorage::default();
        let addr1 = make_pubkey(1);
        let addr2 = make_pubkey(2);

        storage.insert_with_address(addr1);
        storage.insert_with_address(addr2);

        // Remove last element (no swap needed)
        assert!(storage.remove(&addr2));
        assert_eq!(storage.len(), 1);
        assert!(storage.contains(&addr1));
        assert!(!storage.contains(&addr2));
    }

    #[test]
    fn test_remove_only_element() {
        let mut storage = AddressStorage::default();
        let addr = make_pubkey(1);

        storage.insert_with_address(addr);
        assert!(storage.remove(&addr));
        assert!(storage.is_empty());
        assert!(!storage.contains(&addr));
    }

    #[test]
    fn test_is_empty_and_len() {
        let mut storage = AddressStorage::default();

        assert!(storage.is_empty());
        assert_eq!(storage.len(), 0);

        storage.insert_with_address(make_pubkey(1));
        assert!(!storage.is_empty());
        assert_eq!(storage.len(), 1);

        storage.insert_with_address(make_pubkey(2));
        assert_eq!(storage.len(), 2);
    }

    #[test]
    fn test_indices_consistency_after_multiple_removes() {
        let mut storage = AddressStorage::default();
        let addrs: Vec<Pubkey> = (0..5).map(make_pubkey).collect();

        for addr in &addrs {
            storage.insert_with_address(*addr);
        }

        // Remove middle elements
        storage.remove(&addrs[1]);
        storage.remove(&addrs[3]);

        assert_eq!(storage.len(), 3);
        assert!(storage.contains(&addrs[0]));
        assert!(!storage.contains(&addrs[1]));
        assert!(storage.contains(&addrs[2]));
        assert!(!storage.contains(&addrs[3]));
        assert!(storage.contains(&addrs[4]));

        // Verify indices are still valid by checking internal consistency
        for (idx, addr) in storage.addresses.iter().enumerate() {
            assert_eq!(storage.indices.get(addr), Some(&idx));
        }
    }
}
