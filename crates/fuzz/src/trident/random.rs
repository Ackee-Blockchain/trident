use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::trident::Trident;
use crate::trident_rng::BiasedValue;
use crate::trident_rng::LogUniform;

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

    /// Generates a random Solana keypair
    ///
    /// Creates a cryptographically secure random Ed25519 keypair,
    /// useful for generating test signers, authority accounts, and
    /// testing signature verification.
    ///
    /// # Returns
    /// A randomly generated Keypair with a valid public/private key pair
    ///
    /// # Example
    /// ```rust, ignore
    /// let signer = trident.random_keypair();
    /// let authority = trident.random_keypair();
    /// ```
    pub fn random_keypair(&mut self) -> Keypair {
        self.rng.gen_keypair()
    }

    /// Generates a random value with log-uniform distribution
    ///
    /// Problem with uniform: if you sample u64 uniformly, there are only 1000
    /// numbers below 1000, but 9 quintillion above 2^63. So 99.99% of samples
    /// are astronomically large - you'll almost never see small values.
    ///
    /// Log-uniform gives equal probability to each order of magnitude:
    /// range 1-10 is as likely as 1M-10M or 1B-10B. This ensures you actually
    /// test small, medium, AND large values.
    ///
    /// Supported types: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
    ///
    /// # Returns
    /// A random value of type T with log-uniform distribution
    ///
    /// # Example
    /// ```rust, ignore
    /// let amount: u64 = trident.random_log_uniform();
    /// let balance: u128 = trident.random_log_uniform();
    /// ```
    pub fn random_log_uniform<T: LogUniform>(&mut self) -> T {
        self.rng.gen_log_uniform()
    }

    /// Generates a random value with a biased distribution optimized for fuzzing
    ///
    /// Most bugs occur at boundaries and edge cases. This method combines
    /// four strategies to maximize bug-finding while maintaining exploration:
    /// - 25%: Exact edge cases (0, 1, MAX, MIN, powers of 2, type boundaries)
    /// - 25%: Near edge cases (±5% offset to catch off-by-one errors)
    /// - 25%: Log-uniform (covers all magnitudes fairly)
    /// - 25%: Uniform random (ensures full range exploration)
    ///
    /// The percentage-based offset scales with value size, so it works correctly
    /// for all types: ±5% of 100 is ±5, ±5% of u64::MAX is huge.
    ///
    /// This is the recommended default for numeric fuzzing inputs.
    ///
    /// Supported types: u8, u16, u32, u64, u128, i8, i16, i32, i64, i128
    ///
    /// # Returns
    /// A random value optimized for finding edge-case bugs
    ///
    /// # Example
    /// ```rust, ignore
    /// let amount: u64 = trident.random_biased();
    /// let index: u32 = trident.random_biased();
    /// ```
    pub fn random_biased<T: BiasedValue>(&mut self) -> T {
        self.rng.gen_biased()
    }
}
