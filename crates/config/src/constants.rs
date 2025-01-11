// tomls
pub const TRIDENT_TOML: &str = "Trident.toml";
pub const ANCHOR_TOML: &str = "Anchor.toml";

// honggfuzz
pub const CARGO_TARGET_DIR_DEFAULT_HFUZZ: &str = "trident-tests/fuzzing/honggfuzz/hfuzz_target";
pub const HFUZZ_WORKSPACE_DEFAULT_HFUZZ: &str = "trident-tests/fuzzing/honggfuzz/hfuzz_workspace";
pub const CARGO_TARGET_DIR_ENV: &str = "CARGO_TARGET_DIR";
pub const HFUZZ_WORKSPACE_ENV: &str = "HFUZZ_WORKSPACE";

// afl
pub const CARGO_TARGET_DIR_DEFAULT_AFL: &str = "trident-tests/fuzzing/afl/afl_target";

pub const AFL_WORKSPACE_DEFAULT_IN: &str = "trident-tests/fuzzing/afl/afl_workspace/in";

pub const AFL_WORKSPACE_DEFAULT_OUT: &str = "trident-tests/fuzzing/afl/afl_workspace/out";

pub const DEFAULT_SEED_FILENAME: &str = "trident-seed";
pub const DEFAULT_SEED: &str = "trident";
