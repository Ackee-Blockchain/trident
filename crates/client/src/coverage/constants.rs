// AFL COVERAGE CONSTANTS
pub const AFL_PROFRAW_FILENAME: &str = "afl-fuzz-cov-%p-%30m.profraw";
pub const AFL_COVERAGE_FILENAME: &str = "afl-fuzz-cov.json";
pub const AFL_COVERAGE_RUSTFLAGS: &str = "-C instrument-coverage
                                          --cfg=coverage
                                          --cfg=coverage_nightly
                                          --cfg=trybuild_no_target
                                          -C link-arg=-fuse-ld=lld
                                          -C link-arg=-lprofiler";

//HONGGFUZZ COVERAGE CONSTANTS
pub const HONGGFUZZ_PROFRAW_FILENAME: &str = "honggfuzz-cov-%p-%30m.profraw";
pub const HONGGFUZZ_COVERAGE_FILENAME: &str = "honggfuzz-cov.json";
pub const HONGGFUZZ_COVERAGE_RUSTFLAGS: &str =
    "-C instrument-coverage -C link-arg=-fuse-ld=lld -C link-arg=-lprofiler";

// GENERAL COVERAGE CONSTANTS
pub const COVERAGE_IGNORE_REGEX: &str = "trident";
