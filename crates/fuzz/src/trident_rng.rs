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
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

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
        solana_sdk::signer::keypair::Keypair::new_from_array(bytes).pubkey()
    }

    pub(crate) fn gen_keypair(&mut self) -> Keypair {
        let mut bytes = [0; 32];
        self.rng.fill_bytes(&mut bytes);
        solana_sdk::signer::keypair::Keypair::new_from_array(bytes)
    }

    pub(crate) fn fill_bytes(&mut self, bytes: &mut [u8]) {
        self.rng.fill_bytes(bytes);
    }

    pub(crate) fn gen_bool(&mut self) -> bool {
        self.rng.gen_bool(0.5)
    }

    pub(crate) fn gen_log_uniform<T: LogUniform>(&mut self) -> T {
        T::gen_log_uniform(self)
    }

    pub(crate) fn gen_biased<T: BiasedValue>(&mut self) -> T {
        T::gen_biased(self)
    }
}

/// Log-uniform distribution: equal probability per order of magnitude.
pub trait LogUniform: Sized {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self;
}

impl LogUniform for u8 {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self {
        let pow = rng.gen_range(1u32..9);
        let divisor = rng.gen_range(131u32..247);
        (2u32.pow(pow) / divisor) as u8
    }
}

impl LogUniform for u16 {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self {
        let pow = rng.gen_range(1u32..17);
        let divisor = rng.gen_range(131u32..247);
        (2u32.pow(pow) / divisor) as u16
    }
}

impl LogUniform for u32 {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self {
        let pow = rng.gen_range(1u32..33);
        let divisor = rng.gen_range(131u64..247);
        (2u64.pow(pow) / divisor) as u32
    }
}

impl LogUniform for u64 {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self {
        let pow = rng.gen_range(7u32..71);
        let divisor = rng.gen_range(131u128..247);
        (2u128.pow(pow) / divisor) as u64
    }
}

impl LogUniform for u128 {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self {
        let pow = rng.gen_range(7u32..128);
        let divisor = rng.gen_range(131u128..247);
        2u128.saturating_pow(pow) / divisor
    }
}

impl LogUniform for i8 {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self {
        let magnitude = (u8::gen_log_uniform(rng) / 2) as i8;
        if rng.gen_bool() {
            -magnitude
        } else {
            magnitude
        }
    }
}

impl LogUniform for i16 {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self {
        let magnitude = (u16::gen_log_uniform(rng) / 2) as i16;
        if rng.gen_bool() {
            -magnitude
        } else {
            magnitude
        }
    }
}

impl LogUniform for i32 {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self {
        let magnitude = (u32::gen_log_uniform(rng) / 2) as i32;
        if rng.gen_bool() {
            -magnitude
        } else {
            magnitude
        }
    }
}

impl LogUniform for i64 {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self {
        let magnitude = (u64::gen_log_uniform(rng) / 2) as i64;
        if rng.gen_bool() {
            -magnitude
        } else {
            magnitude
        }
    }
}

impl LogUniform for i128 {
    fn gen_log_uniform(rng: &mut TridentRng) -> Self {
        let magnitude = (u128::gen_log_uniform(rng) / 2) as i128;
        if rng.gen_bool() {
            -magnitude
        } else {
            magnitude
        }
    }
}

/// Edge cases: 0, MAX, powers of 2, type boundaries. Used internally by BiasedValue.
trait InterestingValue: Sized {
    fn gen_interesting(rng: &mut TridentRng) -> Self;
}

impl InterestingValue for u8 {
    fn gen_interesting(rng: &mut TridentRng) -> Self {
        const VALUES: &[u8] = &[0, 1, 2, 16, 32, 64, 127, 128, 254, 255];
        VALUES[rng.gen_range(0..VALUES.len())]
    }
}

impl InterestingValue for u16 {
    fn gen_interesting(rng: &mut TridentRng) -> Self {
        const VALUES: &[u16] = &[
            0, 1, 2, 16, 32, 64, 127, 128, 254, 255, 256, 512, 1000, 1024, 4096, 32767, 32768,
            65534, 65535,
        ];
        VALUES[rng.gen_range(0..VALUES.len())]
    }
}

impl InterestingValue for u32 {
    fn gen_interesting(rng: &mut TridentRng) -> Self {
        const VALUES: &[u32] = &[
            0,
            1,
            2,
            16,
            32,
            64,
            127,
            128,
            255,
            256,
            512,
            1000,
            1024,
            4096,
            32767,
            32768,
            65535,
            65536,
            100_000,
            1_000_000,
            0x7FFF_FFFF,
            0x8000_0000,
            0xFFFF_FFFE,
            0xFFFF_FFFF,
        ];
        VALUES[rng.gen_range(0..VALUES.len())]
    }
}

impl InterestingValue for u64 {
    fn gen_interesting(rng: &mut TridentRng) -> Self {
        const VALUES: &[u64] = &[
            0,
            1,
            2,
            16,
            32,
            64,
            127,
            128,
            255,
            256,
            512,
            1000,
            1024,
            4096,
            32767,
            32768,
            65535,
            65536,
            100_000,
            1_000_000,
            0x7FFF_FFFF,
            0x8000_0000,
            0xFFFF_FFFF,
            0x1_0000_0000,
            0x7FFF_FFFF_FFFF_FFFF,
            0x8000_0000_0000_0000,
            0xFFFF_FFFF_FFFF_FFFE,
            0xFFFF_FFFF_FFFF_FFFF,
        ];
        VALUES[rng.gen_range(0..VALUES.len())]
    }
}

impl InterestingValue for u128 {
    fn gen_interesting(rng: &mut TridentRng) -> Self {
        const VALUES: &[u128] = &[
            0,
            1,
            2,
            127,
            128,
            255,
            256,
            65535,
            65536,
            0xFFFF_FFFF,
            0x1_0000_0000,
            0xFFFF_FFFF_FFFF_FFFF,
            0x1_0000_0000_0000_0000,
            0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF,
            0x8000_0000_0000_0000_0000_0000_0000_0000,
            u128::MAX - 1,
            u128::MAX,
        ];
        VALUES[rng.gen_range(0..VALUES.len())]
    }
}

impl InterestingValue for i8 {
    fn gen_interesting(rng: &mut TridentRng) -> Self {
        const VALUES: &[i8] = &[-128, -127, -1, 0, 1, 2, 64, 126, 127];
        VALUES[rng.gen_range(0..VALUES.len())]
    }
}

impl InterestingValue for i16 {
    fn gen_interesting(rng: &mut TridentRng) -> Self {
        const VALUES: &[i16] = &[
            i16::MIN,
            i16::MIN + 1,
            -256,
            -255,
            -128,
            -127,
            -1,
            0,
            1,
            127,
            128,
            255,
            256,
            i16::MAX - 1,
            i16::MAX,
        ];
        VALUES[rng.gen_range(0..VALUES.len())]
    }
}

impl InterestingValue for i32 {
    fn gen_interesting(rng: &mut TridentRng) -> Self {
        const VALUES: &[i32] = &[
            i32::MIN,
            i32::MIN + 1,
            -65536,
            -65535,
            -32768,
            -32767,
            -256,
            -255,
            -128,
            -127,
            -1,
            0,
            1,
            127,
            128,
            255,
            256,
            32767,
            32768,
            65535,
            65536,
            i32::MAX - 1,
            i32::MAX,
        ];
        VALUES[rng.gen_range(0..VALUES.len())]
    }
}

impl InterestingValue for i64 {
    fn gen_interesting(rng: &mut TridentRng) -> Self {
        const VALUES: &[i64] = &[
            i64::MIN,
            i64::MIN + 1,
            -0x8000_0000,
            -0x7FFF_FFFF,
            -65536,
            -65535,
            -32768,
            -32767,
            -256,
            -128,
            -1,
            0,
            1,
            128,
            255,
            256,
            32767,
            32768,
            65535,
            65536,
            0x7FFF_FFFF,
            0x8000_0000,
            i64::MAX - 1,
            i64::MAX,
        ];
        VALUES[rng.gen_range(0..VALUES.len())]
    }
}

impl InterestingValue for i128 {
    fn gen_interesting(rng: &mut TridentRng) -> Self {
        const VALUES: &[i128] = &[
            i128::MIN,
            i128::MIN + 1,
            -0x8000_0000_0000_0000,
            -0x8000_0000,
            -65536,
            -256,
            -128,
            -1,
            0,
            1,
            128,
            255,
            256,
            65535,
            0x7FFF_FFFF,
            0x8000_0000,
            0x7FFF_FFFF_FFFF_FFFF,
            0x8000_0000_0000_0000,
            i128::MAX - 1,
            i128::MAX,
        ];
        VALUES[rng.gen_range(0..VALUES.len())]
    }
}

/// Biased distribution: 25% edge cases + 25% near edges + 25% log-uniform + 25% uniform.
pub trait BiasedValue: Sized {
    fn gen_biased(rng: &mut TridentRng) -> Self;
}

/// Offset percentage for "near interesting" values (5%)
const OFFSET_PERCENT: u64 = 5;
/// Minimum offset to ensure values near 0 still get some variation
const MIN_OFFSET: u64 = 1;

impl BiasedValue for u8 {
    fn gen_biased(rng: &mut TridentRng) -> Self {
        let roll = rng.gen_range(0u8..100);
        match roll {
            0..25 => u8::gen_interesting(rng),
            25..50 => {
                let base = u8::gen_interesting(rng);
                let offset = ((base as u64 * OFFSET_PERCENT) / 100).max(MIN_OFFSET) as u8;
                let delta = rng.gen_range(0..=offset.saturating_mul(2));
                base.saturating_add(delta).saturating_sub(offset)
            }
            50..75 => u8::gen_log_uniform(rng),
            _ => rng.gen_range(0..=u8::MAX),
        }
    }
}

impl BiasedValue for u16 {
    fn gen_biased(rng: &mut TridentRng) -> Self {
        let roll = rng.gen_range(0u8..100);
        match roll {
            0..25 => u16::gen_interesting(rng),
            25..50 => {
                let base = u16::gen_interesting(rng);
                let offset = ((base as u64 * OFFSET_PERCENT) / 100).max(MIN_OFFSET) as u16;
                let delta = rng.gen_range(0..=offset.saturating_mul(2));
                base.saturating_add(delta).saturating_sub(offset)
            }
            50..75 => u16::gen_log_uniform(rng),
            _ => rng.gen_range(0..=u16::MAX),
        }
    }
}

impl BiasedValue for u32 {
    fn gen_biased(rng: &mut TridentRng) -> Self {
        let roll = rng.gen_range(0u8..100);
        match roll {
            0..25 => u32::gen_interesting(rng),
            25..50 => {
                let base = u32::gen_interesting(rng);
                let offset = ((base as u64 * OFFSET_PERCENT) / 100).max(MIN_OFFSET) as u32;
                let delta = rng.gen_range(0..=offset.saturating_mul(2));
                base.saturating_add(delta).saturating_sub(offset)
            }
            50..75 => u32::gen_log_uniform(rng),
            _ => rng.gen_range(0..=u32::MAX),
        }
    }
}

impl BiasedValue for u64 {
    fn gen_biased(rng: &mut TridentRng) -> Self {
        let roll = rng.gen_range(0u8..100);
        match roll {
            0..25 => u64::gen_interesting(rng),
            25..50 => {
                let base = u64::gen_interesting(rng);
                let offset =
                    ((base as u128 * OFFSET_PERCENT as u128) / 100).max(MIN_OFFSET as u128) as u64;
                let delta = rng.gen_range(0..=offset.saturating_mul(2));
                base.saturating_add(delta).saturating_sub(offset)
            }
            50..75 => u64::gen_log_uniform(rng),
            _ => rng.gen_range(0..=u64::MAX),
        }
    }
}

impl BiasedValue for u128 {
    fn gen_biased(rng: &mut TridentRng) -> Self {
        let roll = rng.gen_range(0u8..100);
        match roll {
            0..25 => u128::gen_interesting(rng),
            25..50 => {
                let base = u128::gen_interesting(rng);
                let offset = (base / 100 * OFFSET_PERCENT as u128).max(MIN_OFFSET as u128);
                let delta = rng.gen_range(0..=offset.saturating_mul(2));
                base.saturating_add(delta).saturating_sub(offset)
            }
            50..75 => u128::gen_log_uniform(rng),
            _ => rng.gen_range(0..=u128::MAX),
        }
    }
}

impl BiasedValue for i8 {
    fn gen_biased(rng: &mut TridentRng) -> Self {
        let roll = rng.gen_range(0u8..100);
        match roll {
            0..25 => i8::gen_interesting(rng),
            25..50 => {
                let base = i8::gen_interesting(rng);
                let abs_base = base.unsigned_abs() as u64;
                let offset = ((abs_base * OFFSET_PERCENT) / 100).max(MIN_OFFSET) as i8;
                let delta = rng.gen_range(0..=offset.saturating_mul(2));
                base.saturating_add(delta).saturating_sub(offset)
            }
            50..75 => i8::gen_log_uniform(rng),
            _ => rng.gen_range(i8::MIN..=i8::MAX),
        }
    }
}

impl BiasedValue for i16 {
    fn gen_biased(rng: &mut TridentRng) -> Self {
        let roll = rng.gen_range(0u8..100);
        match roll {
            0..25 => i16::gen_interesting(rng),
            25..50 => {
                let base = i16::gen_interesting(rng);
                let abs_base = base.unsigned_abs() as u64;
                let offset = ((abs_base * OFFSET_PERCENT) / 100).max(MIN_OFFSET) as i16;
                let delta = rng.gen_range(0..=offset.saturating_mul(2));
                base.saturating_add(delta).saturating_sub(offset)
            }
            50..75 => i16::gen_log_uniform(rng),
            _ => rng.gen_range(i16::MIN..=i16::MAX),
        }
    }
}

impl BiasedValue for i32 {
    fn gen_biased(rng: &mut TridentRng) -> Self {
        let roll = rng.gen_range(0u8..100);
        match roll {
            0..25 => i32::gen_interesting(rng),
            25..50 => {
                let base = i32::gen_interesting(rng);
                let abs_base = base.unsigned_abs() as u64;
                let offset = ((abs_base * OFFSET_PERCENT) / 100).max(MIN_OFFSET) as i32;
                let delta = rng.gen_range(0..=offset.saturating_mul(2));
                base.saturating_add(delta).saturating_sub(offset)
            }
            50..75 => i32::gen_log_uniform(rng),
            _ => rng.gen_range(i32::MIN..=i32::MAX),
        }
    }
}

impl BiasedValue for i64 {
    fn gen_biased(rng: &mut TridentRng) -> Self {
        let roll = rng.gen_range(0u8..100);
        match roll {
            0..25 => i64::gen_interesting(rng),
            25..50 => {
                let base = i64::gen_interesting(rng);
                let abs_base = base.unsigned_abs() as u128;
                let offset =
                    ((abs_base * OFFSET_PERCENT as u128) / 100).max(MIN_OFFSET as u128) as i64;
                let delta = rng.gen_range(0..=offset.saturating_mul(2));
                base.saturating_add(delta).saturating_sub(offset)
            }
            50..75 => i64::gen_log_uniform(rng),
            _ => rng.gen_range(i64::MIN..=i64::MAX),
        }
    }
}

impl BiasedValue for i128 {
    fn gen_biased(rng: &mut TridentRng) -> Self {
        let roll = rng.gen_range(0u8..100);
        match roll {
            0..25 => i128::gen_interesting(rng),
            25..50 => {
                let base = i128::gen_interesting(rng);
                let abs_base = base.unsigned_abs();
                let offset =
                    (abs_base / 100 * OFFSET_PERCENT as u128).max(MIN_OFFSET as u128) as i128;
                let delta = rng.gen_range(0..=offset.saturating_mul(2));
                base.saturating_add(delta).saturating_sub(offset)
            }
            50..75 => i128::gen_log_uniform(rng),
            _ => rng.gen_range(i128::MIN..=i128::MAX),
        }
    }
}
