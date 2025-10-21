use trident_fuzz_metrics::TridentFuzzingData;

use trident_fuzz_metrics::types::Seed;

use crate::trident::Trident;

impl Trident {
    pub(crate) fn _set_master_seed_for_debug(&mut self, seed: Seed) {
        self.rng.set_master_seed_for_debug(seed);
        self.fuzzing_data.add_master_seed(&hex::encode(seed));
    }

    pub(crate) fn _set_master_seed_and_thread_id(&mut self, seed: Seed, thread_id: usize) {
        self.rng.set_master_seed_and_thread_id(seed, thread_id);
        self.fuzzing_data.add_master_seed(&hex::encode(seed));
    }

    pub(crate) fn _next_iteration(&mut self) {
        self.client.clear_accounts();
        self.rng.rotate_seed();
    }

    pub(crate) fn _get_fuzzing_data(&self) -> TridentFuzzingData {
        self.fuzzing_data.clone()
    }
}
