// COVERAGE CONSTANTS
pub const COVERAGE_IGNORE_REGEX: &str = "trident-tests|trident/crates";

pub const PROFRAW_FILENAME: &str = "fuzz-cov-build-%p-%30m.profraw";
pub const JSON_REPORT_FILENAME: &str = "coverage-report.json";
pub const HTML_REPORT_DIRNAME: &str = "coverage-report";
pub const COVERAGE_RUSTFLAGS: &str =
    "-C instrument-coverage -C link-arg=-fuse-ld=lld -C link-arg=-lprofiler";

// COVERAGE SERVER
pub const SETUP_DYNAMIC_COVERAGE: &str = "/setup-dynamic-coverage";
pub const DISPLAY_FINAL_REPORT: &str = "/display-final-report";