use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;
use solana_sdk::pubkey::Pubkey;

use crate::trident::Trident;

impl Trident {
    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.rng.gen_range(range)
    }

    pub fn gen_pubkey(&mut self) -> Pubkey {
        self.rng.gen_pubkey()
    }

    pub fn gen_string(&mut self, length: usize) -> String {
        self.rng.gen_string(length)
    }

    pub fn fill_bytes(&mut self, bytes: &mut [u8]) {
        self.rng.fill_bytes(bytes);
    }
}
