// tomls
pub const TRIDENT_TOML: &str = "Trident.toml";
pub const ANCHOR_TOML: &str = "Anchor.toml";

// fuzz
pub const DEFAULT_LOOP_COUNT: u64 = 0;

// honggfuzz
pub const HONGGFUZZ_FUZZER_LOOPCOUNT_DEFAULT: u64 = 10; // less stable has to be lower than afl's default

// afl
pub const AFL_FUZZER_LOOPCOUNT_DEFAULT: u64 = 20;
