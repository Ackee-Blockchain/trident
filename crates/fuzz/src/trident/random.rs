use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;
use solana_sdk::pubkey::Pubkey;

use crate::trident::Trident;

impl Trident {
    /// Generates a random value within the specified range
    ///
    /// This method uses the internal RNG to generate a random value of type T
    /// within the given range. The range can be inclusive or exclusive.
    ///
    /// # Arguments
    /// * `range` - The range to sample from (e.g., 0..10, 0..=9)
    ///
    /// # Returns
    /// A random value of type T within the specified range
    ///
    /// # Example
    /// ```rust, ignore
    /// let random_u64 = trident.random_from_range(1..=100);
    /// let random_f64 = trident.random_from_range(0.0..1.0);
    /// ```
    pub fn random_from_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.rng.gen_range(range)
    }

    /// Generates a random Solana public key
    ///
    /// Creates a cryptographically random 32-byte public key,
    /// useful for generating test accounts and addresses.
    ///
    /// # Returns
    /// A randomly generated Pubkey
    pub fn random_pubkey(&mut self) -> Pubkey {
        self.rng.gen_pubkey()
    }

    /// Generates a random string of the specified length
    ///
    /// Creates a random string containing alphanumeric characters,
    /// useful for generating test data like names, symbols, or URIs.
    ///
    /// # Arguments
    /// * `length` - The desired length of the generated string
    ///
    /// # Returns
    /// A random string of the specified length
    pub fn random_string(&mut self, length: usize) -> String {
        self.rng.gen_string(length)
    }

    /// Fills a byte slice with random data
    ///
    /// This method fills the provided mutable byte slice with
    /// cryptographically random data, useful for generating
    /// random seeds, nonces, or other binary data.
    ///
    /// # Arguments
    /// * `bytes` - A mutable byte slice to fill with random data
    pub fn random_bytes(&mut self, bytes: &mut [u8]) {
        self.rng.fill_bytes(bytes);
    }

    /// Generates a random boolean value
    ///
    /// Creates a random boolean value, useful for testing with boolean inputs.
    ///
    /// # Returns
    /// A random boolean value
    ///
    /// # Example
    /// ```rust, ignore
    /// let random_bool = trident.random_bool();
    /// ```
    pub fn random_bool(&mut self) -> bool {
        self.rng.gen_bool()
    }
}
