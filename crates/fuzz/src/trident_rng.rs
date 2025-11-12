use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Alphanumeric;
use rand::distributions::Distribution;

use rand::rngs::SmallRng;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;
use sha2::Digest;
use sha2::Sha256;
use solana_sdk::pubkey::Pubkey;

pub struct TridentRng {
    seed: [u8; 32],
    rng: SmallRng,
}

impl Default for TridentRng {
    fn default() -> Self {
        Self {
            seed: [0; 32],
            rng: SmallRng::from_seed([0; 32]),
        }
    }
}

impl TridentRng {
    pub(crate) fn set_master_seed_for_debug(&mut self, seed: [u8; 32]) {
        self.seed = seed;
        self.rng = SmallRng::from_seed(self.seed);
    }

    pub(crate) fn set_master_seed_and_thread_id(&mut self, seed: [u8; 32], thread_id: usize) {
        let mut thread_hasher = Sha256::new();
        thread_hasher.update(thread_id.to_le_bytes());
        let thread_hash = thread_hasher.finalize();

        let mut combined_hasher = Sha256::new();
        combined_hasher.update(seed);
        combined_hasher.update(thread_hash);
        let final_hash = combined_hasher.finalize();

        self.seed = final_hash.into();
        self.rng = SmallRng::from_seed(self.seed);
    }

    pub(crate) fn rotate_seed(&mut self) {
        let mut temp_rng = SmallRng::from_seed(self.seed);
        let mut new_seed = [0; 32];
        temp_rng.fill_bytes(&mut new_seed);

        self.seed = new_seed;
        self.rng = SmallRng::from_seed(self.seed);
    }

    pub(crate) fn get_seed(&self) -> [u8; 32] {
        self.seed
    }

    pub(crate) fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.rng.gen_range(range)
    }

    pub(crate) fn gen_string(&mut self, length: usize) -> String {
        Alphanumeric
            .sample_iter(&mut self.rng)
            .take(length)
            .map(char::from)
            .collect()
    }

    pub(crate) fn gen_pubkey(&mut self) -> Pubkey {
        let mut bytes = [0; 32];
        self.rng.fill_bytes(&mut bytes);
        Pubkey::new_from_array(bytes)
    }

    pub(crate) fn fill_bytes(&mut self, bytes: &mut [u8]) {
        self.rng.fill_bytes(bytes);
    }

    pub(crate) fn gen_bool(&mut self) -> bool {
        self.rng.gen_bool(0.5)
    }
}
