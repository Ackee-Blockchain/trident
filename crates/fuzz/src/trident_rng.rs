use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Alphanumeric;
use rand::distributions::Distribution;
use rand::distributions::Standard;

use rand::rngs::SmallRng;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;
use sha2::Digest;
use sha2::Sha256;

pub struct TridentRng {
    seed: [u8; 32],
    rng: SmallRng,
    thread_id: Option<usize>,
}

impl TridentRng {
    pub fn new(seed: [u8; 32]) -> Self {
        Self {
            seed,
            rng: SmallRng::from_seed(seed),
            thread_id: None,
        }
    }
    pub fn override_seed(&mut self, seed: [u8; 32]) {
        self.seed = seed;
        self.rng = SmallRng::from_seed(seed);
    }
    pub fn rotate_seed(&mut self) {
        let mut temp_rng = SmallRng::from_seed(self.seed);
        let mut new_seed = [0; 32];
        temp_rng.fill_bytes(&mut new_seed);

        if let Some(thread_id) = self.thread_id {
            let mut thread_hasher = Sha256::new();
            thread_hasher.update(thread_id.to_le_bytes());
            let thread_hash = thread_hasher.finalize();

            let mut combined_hasher = Sha256::new();
            combined_hasher.update(new_seed);
            combined_hasher.update(thread_hash);
            let final_hash = combined_hasher.finalize();

            new_seed.copy_from_slice(&final_hash);
        }

        self.seed = new_seed;
        self.rng = SmallRng::from_seed(self.seed);
    }

    pub fn random() -> Self {
        let mut seed = [0; 32];
        if let Err(err) = getrandom::fill(&mut seed) {
            panic!("from_entropy failed: {}", err);
        }
        Self {
            seed,
            rng: SmallRng::from_seed(seed),
            thread_id: None,
        }
    }
    pub fn get_seed(&self) -> [u8; 32] {
        self.seed
    }

    pub fn set_thread_id(&mut self, thread_id: usize) {
        self.thread_id = Some(thread_id);
    }
    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.rng.gen_range(range)
    }

    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest);
    }

    pub fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        self.rng.gen()
    }

    pub fn gen_string(&mut self, length: usize) -> String {
        Alphanumeric
            .sample_iter(&mut self.rng)
            .take(length)
            .map(char::from)
            .collect()
    }
}
