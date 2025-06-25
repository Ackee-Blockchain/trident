use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Distribution;
use rand::distributions::Standard;

use rand::rngs::SmallRng;
use rand::{Rng, RngCore, SeedableRng};

pub struct TridentRng {
    seed: [u8; 32],
    rng: SmallRng,
}

impl TridentRng {
    pub fn new(seed: [u8; 32]) -> Self {
        Self {
            seed,
            rng: SmallRng::from_seed(seed),
        }
    }
    pub fn rotate_seed(&mut self) {
        let mut new_seed = [0; 32];
        self.rng.fill_bytes(&mut new_seed);
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
        }
    }
    pub fn get_seed(&self) -> [u8; 32] {
        self.seed
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
}
